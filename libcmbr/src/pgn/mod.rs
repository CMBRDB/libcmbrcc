pub mod ast;
pub use ast::*;
mod tests;

use std::collections::VecDeque;

use memmap2::Mmap;
use pgn_lexer::parser;
pub use pgn_lexer::parser::Token;

/// Lexes a PGN file (Generates a `Vec<Token>`) from the given Mmap
pub fn lex_pgn<'a>(input_mmap: &'a mut Mmap) -> VecDeque<Token<'a>> {
    let mut bytes = &input_mmap[..];
    if bytes[0..3] == [239u8, 187u8, 191u8] {
        bytes = &bytes[3..];
    }

    let tokens = parser::PGNTokenIterator::new(bytes);

    return tokens.collect();
}

/// First lexes mmap, then generates AST and returns
pub fn parse_pgn<'a>(input_mmap: &'a mut Mmap) -> Vec<PgnGame<'a>> {
    return build_pgn_ast(&mut lex_pgn(input_mmap));
}
