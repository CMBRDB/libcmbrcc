use super::{CmbrFile, SanToCmbrMvConvertor};
use crate::cmbr::CmbrGame;
use crate::cmbr::CmbrVariation;
use crate::pgn::{PgnGame, PgnToken};
use pgn_lexer::parser::Token;

// use lz4_flex::{compress_prepend_size, decompress_size_prepended};
use phf::phf_map;

use shakmaty::fen::Fen;
use shakmaty::zobrist::{Zobrist32, ZobristHash};
use shakmaty::{CastlingSide, Chess, Color, Position};

use std::collections::HashMap;
use std::error::Error;
use std::io::Write;
use std::str::from_utf8_unchecked;

macro_rules! move_to_halfmove {
    ($move:expr, $is_black:expr) => {
        (2 * $move) - 1 - ($is_black == false) as u16
    };
}

static MOVE_ANNOTATION_TO_NAG: phf::Map<&[u8], u8> = phf_map! {
    b"!" => 1,
    b"?" => 2,
    b"!!" => 3,
    b"??" => 4,
    b"!?" => 5,
    b"?!" => 6,
};

static RESULT_TO_CHAR: phf::Map<&[u8], char> = phf_map! {
    b"*" => 'u',
    b"1-0" => 'w',
    b"0-1" => 'b',
    b"1/2-1/2" => 'd',
};

fn get_fen_from_board(board: &Chess) -> String {
    let mut fen = board.board().board_fen(board.promoted()).to_string();
    fen.push_str(if board.turn() == Color::White {
        " w "
    } else {
        " b "
    });

    let castles = board.castles();

    if castles.has(Color::White, CastlingSide::KingSide) {
        fen.push('K')
    };
    if castles.has(Color::White, CastlingSide::QueenSide) {
        fen.push('Q')
    };
    if castles.has(Color::Black, CastlingSide::KingSide) {
        fen.push('k')
    };
    if castles.has(Color::Black, CastlingSide::QueenSide) {
        fen.push('q')
    };

    let ep_square = board.ep_square(shakmaty::EnPassantMode::Legal);
    if ep_square.is_some() {
        fen.push(' ');
        // SAFE: Safe
        fen.push_str(&unsafe { ep_square.unwrap_unchecked() }.to_string());
    }

    return fen;
}

impl CmbrFile {
    // TODO(#22): Write tests for CmbrFile::from_ast
    // TODO(#28): Split the Headers, Moves and the encountered positions into seperate files and compress them seperatly
    // TODO: Reduce the memory footprint of the program
    // Currently the program uses 33x amount of memory as the input file.
    pub fn from_ast(
        ast: Vec<PgnGame>,
        convertor: &mut SanToCmbrMvConvertor,
        is_compressed: bool,
    ) -> Result<Self, Box<dyn Error>> {
        debug_assert!(is_compressed == false);

        let mut file = CmbrFile::new(is_compressed);
        let mut board = Chess::new();

        let fen = get_fen_from_board(&board);

        let _ = file.encountered_positions.try_insert(
            board
                .zobrist_hash::<Zobrist32>(shakmaty::EnPassantMode::Legal)
                .0,
            fen,
        );

        let len = ast.len();

        (0..len).into_iter().for_each(|game_i| {
            if game_i % 1000 == 0 || game_i == len {
                print!("{}\r", game_i as f64 / len as f64 * 100.0);
                let _ = std::io::stdout().flush();
            }

            file.games.insert(game_i as u32, CmbrGame::new());

            // SAFE: Safe
            let cmbr_game = unsafe { file.games.get_mut(&(game_i as u32)).unwrap_unchecked() };
            let game = &ast[game_i];

            // TODO: Support fen headers in libcmbr
            board = Chess::new();

            let _ = cmbr_game.encountered_positions.try_insert(0, board
                .zobrist_hash::<Zobrist32>(shakmaty::EnPassantMode::Legal)
                .0);

            {
                let mut current_key: &[u8] = &[];

                for header in &game.0 .0 {
                    match header {
                        Token::Result(r) => cmbr_game.result = RESULT_TO_CHAR[r],
                        Token::TagSymbol(k) => current_key = k,

                        // SAFE: Safe
                        Token::TagString(v) => unsafe {
                            let _ = cmbr_game.headers.insert(
                                from_utf8_unchecked(current_key).to_owned(),
                                from_utf8_unchecked(v).to_owned(),
                            );
                        },

                        _ => {}
                    }
                }
            }

            let variations = &game.0 .1;
            let variations_iter = variations.iter();

            let mut variation_pointers: HashMap<u16, u16> = HashMap::with_capacity(1);
            variation_pointers.insert(0, 0);

            for (id, variation) in variations_iter {
                if variation.0.len() == 0 {
                    eprintln!("[WARN] Empty variation on game N{game_i}. Skipping game");
                    break;
                }

                // SAFE: Safe. Empty variations shouldn't get to this point
                let start_at =
                    if let PgnToken::Token(Token::MoveNumber(number, is_black)) = variation.0[0] {
                        move_to_halfmove!(number, is_black)
                    } else {
                        0
                    };

                let variation_pointer = *variation_pointers.get(id).unwrap() as u32;
                let positions_pointer = (variation_pointer << 16) | start_at as u32;

                let zobrist_hash = cmbr_game.encountered_positions.get(&positions_pointer);
                if zobrist_hash.is_none() {
                    eprintln!("[WARN] Skipping game: {game_i}");
                    break;
                }

                // SAFE: Safe
                let zobrist_hash = unsafe { zobrist_hash.unwrap_unchecked() };
                let fen = file.encountered_positions.get(zobrist_hash).unwrap();
                // SAFE: Safe
                let fen: Fen = fen.parse().unwrap();

                board = fen.into_position(shakmaty::CastlingMode::Standard).unwrap();

                let cmbr_variation = CmbrVariation::new(start_at);
                cmbr_game.variations.insert(*id, cmbr_variation);

                let cmbr_variation = cmbr_game.variations.get_mut(id).unwrap();
                let mut current_move_number = start_at;
                let mut skip_game = false;

                for token in &variation.0 {
                    if let PgnToken::VariationPointer(p) = token {
                        cmbr_variation
                            .moves
                            .push((((*p as u32) << 8) | 0b10000000).into());

                        variation_pointers.insert(*p, *id);

                        continue;
                    }


                    if let PgnToken::Token(t) = token {
                        match t {
                            Token::NAG(n) => {
                                let mut nag_numeral =
                                    // SAFE: Safe
                                    u32::from_str_radix(unsafe { from_utf8_unchecked(*n) }, 10)
                                        .unwrap();

                                nag_numeral <<= 8;
                                nag_numeral |= 0b00001000;

                                cmbr_variation.moves.push(nag_numeral.into());
                            }

                            Token::Move(m) => {
                                // TODO(#23): Handle errors in CmbrFile::from_ast
                                let cmbrmv = convertor
                                    .san_to_cmbr(&mut board, m);

                                if cmbrmv.is_err() {
                                    // TODO(#24): Skip game instead of not finishing convertion if invalid san occurs
                                    eprintln!("[WARN] Not finishing convertion of N{game_i} due to invalid san. SAN: {} | Fen: {}",
                                        std::str::from_utf8(m).unwrap(),
                                        get_fen_from_board(&board));
                                    skip_game = true;
                                    break;
                                }

                                // SAFE: Safe
                                let cmbrmv = unsafe { cmbrmv.unwrap_unchecked() };
                                cmbr_variation.moves.push(cmbrmv);

                                let hash = board
                                    .zobrist_hash::<Zobrist32>(shakmaty::EnPassantMode::Legal)
                                    .0;

                                let fen = get_fen_from_board(&board);
                                let _ = file.encountered_positions.try_insert(hash, fen);

                                current_move_number += 1;
                                let _ = cmbr_game.encountered_positions.insert(((*id as u32) << 16) | current_move_number as u32, hash);
                            }

                            Token::MoveAnnotation(an) => cmbr_variation.moves.push(
                                (((MOVE_ANNOTATION_TO_NAG[an] as u32) << 8) as u32 | 0b10000000)
                                    .into(),
                            ),

                            Token::MoveNumber(number, is_black) => {
                                current_move_number = move_to_halfmove!(number, *is_black);
                            }

                            Token::Commentary(c) => {
                                cmbr_variation
                                    .comments
                                    .insert(
                                        current_move_number,
                                        // SAFE: Safe
                                        unsafe { from_utf8_unchecked(c) }.to_owned(),
                                    );
                            }

                            _ => {}
                        }
                    }
                }

                if skip_game {
                    break;
                }
            }
        });

        Ok(file)
    }
}
