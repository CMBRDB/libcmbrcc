use std::collections::HashMap;

use super::u24;
use crate::utils::def_enum;
use litemap::LiteMap;

pub type CmbrMv = u24;

def_enum! (
    #[doc = "An enum donating the flags that a CMBR-MV Can have"]
    pub CmbrMvFlags => u8 {
        FlagNone  => 0,
        FlagCheck   => 1 << 0,
        FlagMate    => 1 << 1,
        FlagCapture => 1 << 2,
        FlagNag     => 1 << 3, // If this flag is set, the first 8 bits of the CMBR are replaced with a NAG index (https://w.wiki/AWUT)

        FlagPromotesBishop => (1 << 6) | 0b000000,
        FlagPromotesKnight => (1 << 6) | 0b010000,
        FlagPromotesRook   => (1 << 6) | 0b100000,
        FlagPromotesQueen  => (1 << 6) | 0b110000,

        FlagIsVariationPointer => 1 << 7 // If this flag is set, the first 16 bits of the CMBR are replaced with an index to the table of variations
});

def_enum! (
    #[doc = "An enum donating the Piece that a CMBR-MV Can have"]
    pub CmbrMvPiece => u8 {
        WhitePawn => 0b0000,
        WhiteKnight => 0b0001,
        WhiteBishop => 0b0010,
        WhiteRook => 0b0011,
        WhiteQueen => 0b0100,
        WhiteKing => 0b0101,
        WhiteShortCastle => 0b0110,
        WhiteLongCaslte => 0b0111,
        BlackPawn => 0b1000,
        BlackKnight => 0b1001,
        BlackBishop => 0b1010,
        BlackRook => 0b1011,
        BlackQueen => 0b1100,
        BlackKing => 0b1101,
        BlackShortCastle => 0b1110,
        BlackLongCaslte => 0b1111,
});

/// A Struct denoting the structure of a CMBR file.
#[cfg_attr(feature = "bitcode", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CmbrFile {
    // CMBR!
    magic_bytes: &'static str,
    pub is_compressed: bool,
    // Game Id
    pub games: HashMap<u32, CmbrGame>,
}

/// A Struct denoting the structure of a game represented in CMBR
#[cfg_attr(feature = "bitcode", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CmbrGame {
    pub headers: LiteMap<String, String>,
    /// Possible values: 'w', 'b', 'd', 'u'.
    ///     'w': White won;
    ///     'b': Black won;
    ///     'd': Draw;
    ///     'u': Undefined.
    pub result: char,
    /// Variation pointer (main variation is 0)
    pub variations: LiteMap<u16, CmbrVariation>,
    pub crc64: u64,
}

/// A Struct denoting the structure of a variation represented in CMBR
#[cfg_attr(feature = "bitcode", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CmbrVariation {
    pub starts_at: u16,
    pub moves: Vec<CmbrMv>,
    /// The u16 denotes of which half move the comment is on
    pub comments: LiteMap<u16, String>,
}

impl CmbrFile {
    pub fn new(is_compressed: bool) -> Self {
        if is_compressed {
            panic!("Currently creating compressed CMBRs is not supported");
        }

        return Self {
            magic_bytes: "CMBR!",
            is_compressed,
            games: HashMap::with_capacity(16),
        };
    }
}

impl CmbrGame {
    pub fn new() -> Self {
        return Self {
            headers: LiteMap::with_capacity(7),
            variations: LiteMap::with_capacity(1),
            result: 'u',
            crc64: 0,
        };
    }
}

impl CmbrVariation {
    pub fn new(starts_at: u16) -> Self {
        return Self {
            starts_at,
            // https://chess.stackexchange.com/a/4899
            moves: Vec::with_capacity(79),
            comments: LiteMap::new(),
        };
    }
}
