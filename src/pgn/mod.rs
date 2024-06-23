// pub mod ast;
// pub mod errors;
// pub mod headers;
// pub mod lex;
// pub mod tokens;
// mod tree;

use memmap2::Mmap;
use pgn_lexer::parser::{self, Token};

use std::{error::Error, fs::File};

pub fn parse_pgn<'a>(input_file: String) -> Result<(), Box<dyn Error>> {
    let file = File::open(input_file)?;
    let mmap = unsafe { Mmap::map(&file) }?;

    let mut bytes = &mmap[..];
    if bytes[0..3] == [239u8, 187u8, 191u8] {
        bytes = &bytes[3..];
    }

    let tokens = parser::PGNTokenIterator::new(bytes).collect::<Vec<Token>>();

    for t in tokens {
        println!("{t}");
    }

    Ok(())
}
