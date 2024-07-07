use super::{CmbrFile, SanToCmbrMvConvertor};
use crate::cmbr::CmbrVariation;
use crate::pgn::{PgnGame, PgnToken};
use crate::{cmbr::CmbrGame, error::LibCmbrError};
use pgn_lexer::parser::Token;

use shakmaty::Chess;
use std::str::from_utf8_unchecked;

use phf::phf_map;

macro_rules! move_to_halfmove {
    ($move:expr, $is_black:expr) => {
        (2 * $move) - ($is_black == false) as u16
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

impl CmbrFile {
    // TODO(#22): Write tests for CmbrFile::from_ast
    pub fn from_ast(
        ast: Vec<PgnGame>,
        convertor: &mut SanToCmbrMvConvertor,
        is_compressed: bool,
    ) -> Result<Self, LibCmbrError> {
        debug_assert!(is_compressed == false);

        let mut file = CmbrFile::new(is_compressed);
        let mut board = Chess::new();

        (0..ast.len()).into_iter().for_each(|game_i| {
            file.games.insert(game_i as u32, CmbrGame::new());

            // SAFE: Safe
            let cmbr_game = unsafe { file.games.get_mut(&(game_i as u32)).unwrap_unchecked() };
            let game = &ast[game_i];
            board = Chess::new();

            {
                let mut current_key: &[u8] = &[];

                for header in &game.0 .0 {
                    match header {
                        Token::Result(r) => cmbr_game.result = r[r.len() - 1] as char,
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
                                    eprintln!("[WARN] Not finishing convertion of N{game_i} due to invalid san. Error: {}", cmbrmv.err().unwrap());
                                    skip_game = true;
                                    break;
                                }

                                // SAFE: Safe
                                let cmbrmv = unsafe { cmbrmv.unwrap_unchecked() };
                                cmbr_variation.moves.push(cmbrmv);
                                current_move_number += 1;
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
