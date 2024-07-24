use shakmaty::{Chess, Position, Square};
use bit_vec::BitVec;

pub type PytorchPosition = Vec<u8>;

pub trait PytorchPositionConvertable<T> {
    fn to_pytorch_position(s: &T) -> PytorchPosition; 
    fn from_pytorch_position(position: PytorchPosition) -> T;
}

impl PytorchPositionConvertable<Chess> for Chess {
    fn to_pytorch_position(s: &Chess) -> PytorchPosition {
        let mut position = [0u8; 1024];
        let mut position_i = 0;
        
        let occupancy = s.board().occupied().0;
        let occupancy_bytes = occupancy.to_ne_bytes();
        for byte in occupancy_bytes {
            position[position_i] = byte;
            position_i += 1;
        }

        let mut pieces_bitvec = BitVec::from_elem(1024, false);
        let mut pieces_bitvec_i = 0;

        for square in 0..64u8 {
            // SAFE: Safe
            let square: Square = unsafe { std::mem::transmute(square) };
            let piece_at = s.board().piece_at(square);

            if piece_at.is_none() {
                continue;
            }

            // SAFE: Safe
            let piece_at = unsafe { piece_at.unwrap_unchecked() };
            let piece_bits = (piece_at.color as u8) << 3 | (piece_at.role as u8);
            let bits_to_append_n = 8 - piece_bits.trailing_zeros();

            for j in 0..bits_to_append_n {
                pieces_bitvec.set(pieces_bitvec_i, (piece_bits & (1 << j)) != 0);
                pieces_bitvec_i += 1;
            }
        }

        let bitvec_bytes = &pieces_bitvec.to_bytes()[0..(pieces_bitvec_i + (8 - (pieces_bitvec_i % 8))) / 8];
        for byte in bitvec_bytes {
            position[position_i] = *byte;
            position_i += 1;
        }

        let can_castle_wk = s.castles().rook(shakmaty::Color::White, shakmaty::CastlingSide::KingSide).is_some();
        let can_castle_bk = s.castles().rook(shakmaty::Color::Black, shakmaty::CastlingSide::KingSide).is_some();
        let can_castle_wq = s.castles().rook(shakmaty::Color::White, shakmaty::CastlingSide::QueenSide).is_some();
        let can_castle_bq = s.castles().rook(shakmaty::Color::Black, shakmaty::CastlingSide::QueenSide).is_some();

        let mut castle_en_passant_mask = BitVec::from_elem(16, false);
        castle_en_passant_mask.set(0, can_castle_wk);
        castle_en_passant_mask.set(1, can_castle_bk);
        castle_en_passant_mask.set(2, can_castle_wq);
        castle_en_passant_mask.set(3, can_castle_bq);

        let ep_square = s.ep_square(shakmaty::EnPassantMode::Legal);
        let ep_bits = if ep_square.is_none() {
            0b11111111
        } else {
            ep_square.unwrap() as u8
        };

        for j in 0..8 {
            castle_en_passant_mask.set(j + 4, ep_bits & (1 << j) != 0);
        }

        let castle_en_passant_mask = castle_en_passant_mask.to_bytes();

        for byte in castle_en_passant_mask {
            position[position_i] = byte;
            position_i += 1;
        }

        let position_truncated = position[0..position_i].to_vec();
        return position_truncated;
    }

    fn from_pytorch_position(_position: PytorchPosition) -> Self {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use shakmaty::{fen::Fen, Chess, FromSetup};
    use super::*;

    #[test]
    pub fn test_to_pytorch_position() {
        let starting_fen = Fen::from_ascii(b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        let starting_setup = starting_fen.as_setup();
        let starting = Chess::from_setup(starting_setup.clone(), shakmaty::CastlingMode::Standard).unwrap();

        let pytorch_position = Chess::to_pytorch_position(&starting);

        println!("{pytorch_position:?} {}", pytorch_position.len());
    }
}