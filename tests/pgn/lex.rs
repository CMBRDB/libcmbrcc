extern crate test;

#[cfg(test)]
mod pgn_tests {
    use memmap2::Mmap;
    use pgn_lexer::parser::Token;
    use project_root::get_project_root;
    use std::{collections::VecDeque, fs::File};

    #[cfg(feature = "benchmark")]
    use super::test::Bencher;

    use crate::pgn;

    #[test]
    fn test_lex() {
        let file_path = get_project_root()
            .unwrap()
            .join("data/with_varation_and_comments.pgn");
        let file = File::open(file_path.clone());

        if file.is_err() {
            panic!(
                "[ERROR] {}. File path: {:?}",
                file.err().unwrap(),
                file_path
            );
        }

        let file = unsafe { file.unwrap_unchecked() };
        let mmap = unsafe { Mmap::map(&file) };

        if mmap.is_err() {
            panic!("[ERROR] {}", mmap.err().unwrap());
        }

        let mut mmap = mmap.unwrap();

        let tokens = pgn::lex_pgn(&mut mmap);

        let tokens_expected = VecDeque::from([
            Token::TagSymbol(b"E"),
            Token::TagString(b"E"),
            Token::TagSymbol(b"S"),
            Token::TagString(b"S"),
            Token::MoveNumber(1, false),
            Token::Move(b"e4"),
            Token::StartVariation(b"("),
            Token::MoveNumber(1, false),
            Token::Move(b"d4"),
            Token::EndVariation(b")"),
            Token::StartVariation(b"("),
            Token::MoveNumber(1, false),
            Token::Move(b"c4"),
            Token::EndVariation(b")"),
            Token::MoveNumber(1, true),
            Token::Move(b"e5"),
            Token::MoveNumber(2, false),
            Token::Move(b"Nf3"),
            Token::StartVariation(b"("),
            Token::MoveNumber(2, false),
            Token::Move(b"Nc3"),
            Token::Move(b"d5"),
            Token::StartVariation(b"("),
            Token::MoveNumber(2, true),
            Token::Move(b"d6"),
            Token::EndVariation(b")"),
            Token::EndVariation(b")"),
            Token::StartVariation(b"("),
            Token::MoveNumber(2, false),
            Token::Move(b"d3"),
            Token::EndVariation(b")"),
            Token::Commentary(b" Comment "),
            Token::Result(b"*"),
            Token::TagSymbol(b"E"),
            Token::TagString(b"E"),
            Token::TagSymbol(b"S"),
            Token::TagString(b"S"),
            Token::MoveNumber(1, false),
            Token::Move(b"e4"),
            Token::StartVariation(b"("),
            Token::MoveNumber(1, false),
            Token::Move(b"d4"),
            Token::EndVariation(b")"),
            Token::StartVariation(b"("),
            Token::MoveNumber(1, false),
            Token::Move(b"c4"),
            Token::EndVariation(b")"),
            Token::MoveNumber(1, true),
            Token::Move(b"e5"),
            Token::MoveNumber(2, false),
            Token::Move(b"Nf3"),
            Token::StartVariation(b"("),
            Token::MoveNumber(2, false),
            Token::Move(b"Nc3"),
            Token::Move(b"d5"),
            Token::StartVariation(b"("),
            Token::MoveNumber(2, true),
            Token::Move(b"d6"),
            Token::EndVariation(b")"),
            Token::EndVariation(b")"),
            Token::StartVariation(b"("),
            Token::MoveNumber(2, false),
            Token::Move(b"d3"),
            Token::EndVariation(b")"),
            Token::Commentary(b" Comment "),
            Token::Result(b"*"),
        ]);

        assert_eq!(tokens_expected, tokens);
    }

    #[cfg(feature = "benchmark")]
    #[bench]
    fn bench_lex(b: &mut Bencher) {
        let file_path = get_project_root().unwrap().join("data/twic1544.pgn");
        let file = File::open(file_path.clone());

        if file.is_err() {
            panic!(
                "[ERROR] {}. File path: {:?}",
                file.err().unwrap(),
                file_path
            );
        }

        let file = unsafe { file.unwrap_unchecked() };
        let mmap = unsafe { Mmap::map(&file) };

        if mmap.is_err() {
            panic!("[ERROR] {}", mmap.err().unwrap());
        }

        let mut mmap = mmap.unwrap();

        b.iter(|| {
            pgn::lex_pgn(&mut mmap);
        });
    }
}
