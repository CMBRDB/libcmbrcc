mod error;
mod flags;
mod tests;
mod u24_impl;

use std::error::Error;

use error::{LibCmbrError, LibCmbrErrorType};
pub use flags::*;
use shakmaty::san::{San::*, SanPlus, Suffix};
use shakmaty::{Chess, Color, Position, Role, Square};
pub use u24_impl::*;

pub type Cmbr = u24;

fn shakmaty_move_to_cmbr(
    role: &Role,
    from: &Square,
    to: &Square,
    capture: &bool,
    promotion: &Option<Role>,
    suffix: &Option<Suffix>,
    color: u8,
) -> Cmbr {
    let mut cmbr = 0u32;

    let role_to_cmbr: &[u32] = &[
        0b0000, 0b0001, 0b0010, 0b0011, 0b0100, 0b0101, 0b1000, 0b1001, 0b1010, 0b1011, 0b1100,
        0b1101,
    ];

    let promotion_to_cmbr: &[u32] = &[0, 0b01010000, 0b01000000, 0b01100000, 0b01110000];

    if promotion.is_some() {
        // SAFE: Safe
        cmbr |= promotion_to_cmbr[unsafe { promotion.unwrap_unchecked() } as usize - 1];
    }

    if suffix.is_some() {
        // SAFE: Safe
        cmbr |= shakmaty_suffix_to_flag(unsafe { suffix.unwrap_unchecked() }) as u32;
    }

    if *capture {
        cmbr |= 0b00000100;
    }

    cmbr |= role_to_cmbr[*role as usize - 1 + 6 * color as usize] << 8;
    cmbr |= (*from as u32) << (8 + 4);
    cmbr |= (*to as u32) << (8 + 4 + 6);

    cmbr.into()
}

fn shakmaty_suffix_to_flag(suffix: Suffix) -> u8 {
    return match suffix {
        Suffix::Check => CmbrFlags::FlagCheck,
        Suffix::Checkmate => CmbrFlags::FlagMate,
    };
}

/// Inputs a SAN string and generates a CMBR-MV from it
pub fn san_to_cmbr(board: &mut Chess, san_bytes: &[u8]) -> Result<Cmbr, Box<dyn Error>> {
    // safe if called correctly.
    let san_string = unsafe { std::str::from_utf8_unchecked(san_bytes) };
    let san: SanPlus = san_string.parse()?;
    let san_move = san.san.to_move(board)?;
    let color = board.turn();

    let cmbr_move = match &san_move {
        shakmaty::Move::Normal {
            role,
            from,
            capture,
            to,
            promotion,
        } => shakmaty_move_to_cmbr(
            role,
            &from,
            &to,
            &capture.is_some(),
            promotion,
            &san.suffix,
            (color == Color::Black) as u8,
        ),

        #[rustfmt::skip]
        shakmaty::Move::Castle { king: _, rook: _ } => {
            let mut cmbr = 0u32;

            let side = if let Castle(side) = san.san {side} else {unreachable!()};
            cmbr |= if color == Color::Black {1u32 << 3} else {0};

            cmbr |= match side {
                shakmaty::CastlingSide::KingSide  => 0b100u32,
                shakmaty::CastlingSide::QueenSide => 0b101u32,
            };

            if san.suffix.is_some() {
                // SAFE: Safe
                cmbr |= (shakmaty_suffix_to_flag(unsafe { san.suffix.unwrap_unchecked() }) as u32) as u32;
            }

            cmbr.into()
        }

        shakmaty::Move::Put { role: _, to: _ } => {
            return Err(Box::new(LibCmbrError::new(
                LibCmbrErrorType::CrazyHouseNotSupported,
            )));
        }

        shakmaty::Move::EnPassant { from, to } => shakmaty_move_to_cmbr(
            &Role::Pawn,
            &from,
            &to,
            &true,
            &None,
            &san.suffix,
            (color == Color::Black) as u8,
        ),
    };

    // SAFE: Safe
    board.play_unchecked(&san_move);

    return Ok(cmbr_move);
}
