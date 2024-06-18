pub use smartstring::alias::String;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub enum PgnToken {
    Header(String, String),
    HalfMoveNumber(u16),
    PgnMove(String),
    VariationStart,
    VariationEnd,
    Comment(String),
    Result(PgnResult),
    NGA(String),

    #[default]
    None,
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
#[repr(C)]
pub enum PgnResult {
    #[default]
    Undefined,
    WhiteWon,
    BlackWon,
    Draw,
}

#[macro_export]
macro_rules! pgn_move_to_half_move {
    ($arg:expr) => {
        (2 * $arg - 1)
    };
}
