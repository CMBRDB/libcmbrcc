use crate::utils::def_enum;

def_enum! (
    #[doc = "An enum donating the flags that a CMBR-MV Can have"]
    pub CmbrFlags => u8{
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
    pub CmbrPiece => u8{
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
