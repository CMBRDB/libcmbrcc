#[rustfmt::skip]
pub use smartstring::alias::String;
use crate::tree::Tree;
use micromap::Map;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct PgnGame {
    headers: PgnHeaders,
    comments: Vec<PgnComment>,
    main_variation: PgnVariation,
    result: PgnResult,
}

pub type PgnHeaders = Map<String, String, 24>;

// Move string, half move count (odd = white, even = black)
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct PgnMove {
    pub value: String,
    pub half_move_location: u16,
}

pub type PgnVariation = Tree<PgnMove>;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
#[repr(C)]
pub enum PgnResult {
    #[default]
    Undefined,
    WhiteWon,
    BlackWon,
    Draw,
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct PgnComment {
    pub comment: String,
    pub half_move_location: u16,
}

#[macro_export]
macro_rules! pgn_move_to_half_move {
    ($arg:expr) => {
        (2 * $arg - 1)
    };
}

impl PgnGame {
    #[inline(always)]
    pub fn new() -> Self {
        return Self::default();
    }

    #[inline(always)]
    pub fn get_headers(&self) -> &PgnHeaders {
        return &self.headers;
    }

    #[inline(always)]
    pub fn get_headers_mut(&mut self) -> &mut PgnHeaders {
        return &mut self.headers;
    }

    #[inline(always)]
    pub fn get_result(&self) -> &PgnResult {
        return &self.result;
    }

    #[inline(always)]
    pub fn get_result_mut(&mut self) -> &mut PgnResult {
        return &mut self.result;
    }

    #[inline(always)]
    pub fn get_comments(&self) -> &Vec<PgnComment> {
        return &self.comments;
    }

    #[inline(always)]
    pub fn get_comments_mut(&mut self) -> &mut Vec<PgnComment> {
        return &mut self.comments;
    }

    #[inline(always)]
    pub fn get_main_variation(&self) -> &PgnVariation {
        return &self.main_variation;
    }

    #[inline(always)]
    pub fn get_main_variation_mut(&mut self) -> &mut PgnVariation {
        return &mut self.main_variation;
    }
}
