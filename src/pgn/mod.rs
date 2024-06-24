pub mod ast;
pub use ast::*;

use std::collections::VecDeque;

use memmap2::Mmap;
use pgn_lexer::parser::{self, Token};

pub fn parse_pgn<'a>(input_mmap: &'a mut Mmap) -> VecDeque<Token<'a>> {
    let mut bytes = &input_mmap[..];
    if bytes[0..3] == [239u8, 187u8, 191u8] {
        bytes = &bytes[3..];
    }

    let tokens = parser::PGNTokenIterator::new(bytes);

    return tokens.collect();
}
