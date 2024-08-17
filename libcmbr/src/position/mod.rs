use bit_vec::BitVec;
use phf::phf_map;
use shakmaty::{Bitboard, Chess, Color, Position};

pub type CompressedPosition = Vec<u8>;

pub trait CompressedPositionConvertable<T> {
    fn to_pytorch_position(s: &T) -> CompressedPosition;
    fn from_pytorch_position(position: &mut CompressedPosition) -> String;
}

impl CompressedPositionConvertable<Chess> for Chess {
    fn to_pytorch_position(s: &Chess) -> CompressedPosition {
        let mut encoded = CompressedPosition::new();

        let occupied = s
            .board()
            .occupied()
            .0
            .to_ne_bytes()
            .into_iter()
            .collect::<Vec<u8>>();

        occupied.iter().for_each(|byte| encoded.push(*byte));

        let piece_lookup = phf_map! {
            'K' => 0, 'Q' => 1, 'R' => 2, 'B' => 3, 'N' => 4, 'P' => 5,
            'k' => 6, 'q' => 7, 'r' => 8, 'b' => 9, 'n' => 10, 'p' => 11u8,
        };

        // FIXME: Don't generate a fen. Performance issue
        let fen = format!("{}", s.board().board_fen(shakmaty::Bitboard(0)));

        let mut bitvec = BitVec::with_capacity(1024);

        for char in fen.chars() {
            if char == ' ' {
                break;
            }
            if char == '/' || !char.is_alphabetic() {
                continue;
            }

            let lookup = piece_lookup[&char];
            // FIXME: This is also probably bad
            for i in 0..4 {
                bitvec.push(lookup & (1 << i) != 0);
                // bitvec.set(bitvec_i + i, lookup & (1 << i) != 0);
            }
        }

        bitvec.push(s.turn() == Color::White);

        let can_castle_wk = s
            .castles()
            .rook(shakmaty::Color::White, shakmaty::CastlingSide::KingSide)
            .is_some();
        let can_castle_bk = s
            .castles()
            .rook(shakmaty::Color::Black, shakmaty::CastlingSide::KingSide)
            .is_some();
        let can_castle_wq = s
            .castles()
            .rook(shakmaty::Color::White, shakmaty::CastlingSide::QueenSide)
            .is_some();
        let can_castle_bq = s
            .castles()
            .rook(shakmaty::Color::Black, shakmaty::CastlingSide::QueenSide)
            .is_some();

        bitvec.push(can_castle_wk);
        bitvec.push(can_castle_wq);
        bitvec.push(can_castle_bk);
        bitvec.push(can_castle_bq);

        let square = s.ep_square(shakmaty::EnPassantMode::Legal);
        let square_bits = if square.is_some() {
            square.unwrap() as u8
        } else {
            u8::MAX
        };

        for i in 0..8 {
            bitvec.push(square_bits & (1 << i) != 0);
        }

        // let bits_to_bytes = (bitvec_i + 7) / 8;
        let bitvec_bytes = bitvec.to_bytes();

        bitvec_bytes.iter().for_each(|byte| encoded.push(*byte));

        return encoded;
    }

    fn from_pytorch_position(position: &mut CompressedPosition) -> String {
        let piece_lookup = phf_map! {
            0u8 => 'K', 1u8 => 'Q', 2u8 => 'R', 3u8 => 'B', 4u8 => 'N', 5u8 => 'P',
            6u8 => 'k', 7u8 => 'q', 8u8 => 'r', 9u8 => 'b', 10u8 => 'n', 11u8 => 'p',
        };

        let occupied = Bitboard(u64::from_ne_bytes(position[0..8].try_into().unwrap()));
        let number_of_pieces = occupied.0.count_ones();

        position.drain(0..8);

        let bitvec = BitVec::from_bytes(&position[0..position.len()]);

        let mut fen = String::with_capacity(64);

        let mut read_piece_count = 0;
        let mut empty = 0;
        let mut bitvec_i = 0;

        // FIXME: Optimize this to only loop through occupied squares
        for rank in (0..8).rev() {
            for file in 0..8 {
                'occupancy_check: {
                    let i = rank * 8 + file;
                    let bit = occupied.0 & (1 << i) != 0;

                    if !bit {
                        empty += 1;

                        break 'occupancy_check;
                    } else if empty != 0 {
                        let empty_as_char = (empty + b'0') as char;
                        fen.push(empty_as_char);

                        empty = 0;
                    }

                    // SAFE: Safe
                    let piece = unsafe {
                        (bitvec.get_unchecked(read_piece_count * 4 + 0) as u8) << 0
                            | (bitvec.get_unchecked(read_piece_count * 4 + 1) as u8) << 1
                            | (bitvec.get_unchecked(read_piece_count * 4 + 2) as u8) << 2
                            | (bitvec.get_unchecked(read_piece_count * 4 + 3) as u8) << 3
                    };

                    read_piece_count += 1;

                    println!("{:08b} | read_piece_count: {read_piece_count} | number_of_pieces {number_of_pieces}", piece);

                    let piece_char = piece_lookup[&piece];
                    fen.push(piece_char);
                }

                if empty == 8 || (file == 7 && empty != 0) {
                    let empty_as_char = (empty + b'0') as char;
                    fen.push(empty_as_char);

                    empty = 0;
                }

                if file == 7 {
                    empty = 0;
                    fen.push('/');
                }
            }
        }

        debug_assert!(read_piece_count == number_of_pieces as usize);
        fen.pop();

        bitvec_i += read_piece_count * 4;

        let turn = if bitvec.get(bitvec_i) == Some(true) {
            " w "
        } else {
            " b "
        };

        fen.push_str(turn);

        bitvec_i += 1;

        let can_castle_wk = if bitvec.get(bitvec_i + 0).unwrap() {
            "K"
        } else {
            ""
        };
        let can_castle_wq = if bitvec.get(bitvec_i + 1).unwrap() {
            "Q"
        } else {
            ""
        };
        let can_castle_bk = if bitvec.get(bitvec_i + 2).unwrap() {
            "k"
        } else {
            ""
        };
        let can_castle_bq = if bitvec.get(bitvec_i + 3).unwrap() {
            "q"
        } else {
            ""
        };

        if can_castle_wk.is_empty()
            && can_castle_wq.is_empty()
            && can_castle_bk.is_empty()
            && can_castle_bq.is_empty()
        {
            fen.pop();
        }

        fen.push_str(can_castle_wk);
        fen.push_str(can_castle_wq);
        fen.push_str(can_castle_bk);
        fen.push_str(can_castle_bq);

        bitvec_i += 4;

        let en_passant_bits = unsafe {
            (bitvec.get_unchecked(bitvec_i + 0) as u8) << 0
                | (bitvec.get(bitvec_i + 1).unwrap() as u8) << 1
                | (bitvec.get(bitvec_i + 2).unwrap() as u8) << 2
                | (bitvec.get(bitvec_i + 3).unwrap() as u8) << 3
                | (bitvec.get(bitvec_i + 4).unwrap() as u8) << 4
                | (bitvec.get(bitvec_i + 5).unwrap() as u8) << 5
                | (bitvec.get(bitvec_i + 6).unwrap() as u8) << 6
        };

        let en_pessant_squre = if en_passant_bits == u8::MAX & !(1 << 7) {
            " -"
        } else {
            &format!(
                " {}{}",
                ((en_passant_bits % 8) + b'a') as char,
                ((en_passant_bits / 8) + b'1') as char
            )
        };

        fen.push_str(en_pessant_squre);

        return fen;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shakmaty::{fen::Fen, Chess, FromSetup};

    #[cfg(feature = "benchmark")]
    extern crate test;
    #[cfg(feature = "benchmark")]
    use test::Bencher;

    #[test]
    pub fn test_to_and_from_pytorch_position() {
        let fens_to_test: &[&[u8]] = &[
            b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -",
            b"rnbqkb1r/pppppppp/8/8/8/8/PPPPPPPP/RNBQKB1R w KQkq -",
            b"5k2/1pp1nb2/1r1p4/1P6/2P3R1/6P1/P1B5/b1K5 w -",
            b"1rbqk2r/p1p1pnbp/1p1p2p1/n5PQ/R3pP1P/1PP5/3PB3/1NB1K1NR w Kk -",
            b"1rbqk2r/p1pppnbp/2n2pp1/Pp5Q/4PP1P/1P6/2PPB1P1/RNB1K1NR w KQk b6",
            b"r1b1k2r/1n1q1p1p/p1p1p1p1/1p1pP1b1/1N1P1P1P/P1P1Q1P1/1P1nN1B1/R1B1K2R b KQkq -",
        ];

        for fen_str in fens_to_test {
            let fen = Fen::from_ascii(fen_str).unwrap();
            let setup = fen.as_setup();
            let position =
                Chess::from_setup(setup.clone(), shakmaty::CastlingMode::Standard).unwrap();

            let mut pytorch_position = Chess::to_pytorch_position(&position);
            let pytorch_fen = Chess::from_pytorch_position(&mut pytorch_position);

            println!(
                "Pytorch length: {} | Fen length: {} | Absolute diff: {} | Percent change: {:.2}%",
                pytorch_position.len(),
                fen_str.len(),
                pytorch_position.len().abs_diff(fen_str.len()),
                (1.0 - (fen_str.len() as f64) / (pytorch_position.len() as f64)) * 100.0
            );

            assert_eq!(&pytorch_fen, std::str::from_utf8(fen_str).unwrap());
        }
    }

    #[cfg(feature = "benchmark")]
    #[bench]
    fn bench_to_pytorch_position(b: &mut Bencher) {
        let fens_to_test: &[&[u8]] = &[
            b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -",
            b"rnbqkb1r/pppppppp/8/8/8/8/PPPPPPPP/RNBQKB1R w KQkq -",
            b"5k2/1pp1nb2/1r1p4/1P6/2P3R1/6P1/P1B5/b1K5 w -",
            b"1rbqk2r/p1p1pnbp/1p1p2p1/n5PQ/R3pP1P/1PP5/3PB3/1NB1K1NR w Kk -",
            b"1rbqk2r/p1pppnbp/2n2pp1/Pp5Q/4PP1P/1P6/2PPB1P1/RNB1K1NR w KQk b6",
            b"r1b1k2r/1n1q1p1p/p1p1p1p1/1p1pP1b1/1N1P1P1P/P1P1Q1P1/1P1nN1B1/R1B1K2R b KQkq -",
        ];

        for fen_str in fens_to_test {
            let fen = Fen::from_ascii(fen_str).unwrap();
            let setup = fen.as_setup();
            let position =
                Chess::from_setup(setup.clone(), shakmaty::CastlingMode::Standard).unwrap();

            b.iter(|| {
                let pytorch_position = Chess::to_pytorch_position(&position);
            });

        }
    }
}
