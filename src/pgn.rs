use std::error::Error;

#[rustfmt::skip]
pub use smartstring::alias::String;
use super::tree::Tree;
use super::utils;
use micromap::Map;
use smol_str::SmolStr;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct PgnGame {
    headers: PgnHeaders,
    main_variation: PgnVariation,
}

pub type PgnHeaders = Map<String, String, 24>;
pub type PgnMove = SmolStr;
pub type PgnVariation = Tree<PgnMove>;

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
}

pub fn parse_pgn(input_filename: &str) -> Result<Vec<PgnGame>, Box<dyn Error>> {
    let lines = utils::read_lines(input_filename)?;

    for line in lines.flatten() {
        println!("{line}");
    }

    Ok(Vec::new())
}
