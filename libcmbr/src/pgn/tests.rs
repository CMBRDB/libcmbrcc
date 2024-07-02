extern crate test;

#[cfg(test)]
mod pgn_tests {
    #[allow(unused_imports)]
    use crate::pgn::{self, lex_pgn, Token, VecDeque};
    use memmap2::Mmap;
    use project_root::get_project_root;
    use std::fs::File;

    #[cfg(feature = "benchmark")]
    use super::test::Bencher;

    #[test]
    fn test_ast() {
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

        // SAFE: Safe
        let file = unsafe { file.unwrap_unchecked() };
        let mmap = unsafe { Mmap::map(&file) };

        if mmap.is_err() {
            panic!("[ERROR] {}", mmap.err().unwrap());
        }

        let mut mmap = mmap.unwrap();

        let ast = crate::pgn::parse_pgn(&mut mmap);

        let ast_expected = "[PgnGame(([TagSymbol([69]), TagString([69]), TagSymbol([83]), TagString([83]), Result([42])], LiteMap { values: [(0, PgnVariation([Token(MoveNumber(1, false)), Token(Move([101, 52])), VariationPointer(1), VariationPointer(2), Token(MoveNumber(1, true)), Token(Move([101, 53])), Token(MoveNumber(2, false)), Token(Move([78, 102, 51])), VariationPointer(3), VariationPointer(5), Token(Commentary([32, 67, 111, 109, 109, 101, 110, 116, 32]))])), (1, PgnVariation([Token(MoveNumber(1, false)), Token(Move([100, 52]))])), (2, PgnVariation([Token(MoveNumber(1, false)), Token(Move([99, 52]))])), (3, PgnVariation([Token(MoveNumber(2, false)), Token(Move([78, 99, 51])), Token(Move([100, 53])), VariationPointer(16)])), (5, PgnVariation([Token(MoveNumber(2, false)), Token(Move([100, 51]))])), (16, PgnVariation([Token(MoveNumber(2, true)), Token(Move([100, 54]))]))], _key_type: PhantomData<u16>, _value_type: PhantomData<libcmbr::pgn::ast::PgnVariation> })), PgnGame(([TagSymbol([69]), TagString([69]), TagSymbol([83]), TagString([83]), Result([42])], LiteMap { values: [(0, PgnVariation([Token(MoveNumber(1, false)), Token(Move([101, 52])), VariationPointer(1), VariationPointer(2), Token(MoveNumber(1, true)), Token(Move([101, 53])), Token(MoveNumber(2, false)), Token(Move([78, 102, 51])), VariationPointer(3), VariationPointer(5), Token(Commentary([32, 67, 111, 109, 109, 101, 110, 116, 32]))])), (1, PgnVariation([Token(MoveNumber(1, false)), Token(Move([100, 52]))])), (2, PgnVariation([Token(MoveNumber(1, false)), Token(Move([99, 52]))])), (3, PgnVariation([Token(MoveNumber(2, false)), Token(Move([78, 99, 51])), Token(Move([100, 53])), VariationPointer(16)])), (5, PgnVariation([Token(MoveNumber(2, false)), Token(Move([100, 51]))])), (16, PgnVariation([Token(MoveNumber(2, true)), Token(Move([100, 54]))]))], _key_type: PhantomData<u16>, _value_type: PhantomData<libcmbr::pgn::ast::PgnVariation> }))]";

        assert_eq!(format!("{:?}", ast), ast_expected);
    }

    #[cfg(feature = "benchmark")]
    #[bench]
    fn bench_ast(b: &mut Bencher) {
        let file_path = get_project_root().unwrap().join("data/twic1544.pgn");
        let file = File::open(file_path.clone());

        if file.is_err() {
            panic!(
                "[ERROR] {}. File path: {:?}",
                file.err().unwrap(),
                file_path
            );
        }

        // SAFE: Safe
        let file = unsafe { file.unwrap_unchecked() };
        let mmap = unsafe { Mmap::map(&file) };

        if mmap.is_err() {
            panic!("[ERROR] {}", mmap.err().unwrap());
        }

        let mut mmap = mmap.unwrap();

        b.iter(|| {
            pgn::parse_pgn(&mut mmap);
        });
    }

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

        // SAFE: Safe
        let file = unsafe { file.unwrap_unchecked() };
        let mmap = unsafe { Mmap::map(&file) };

        if mmap.is_err() {
            panic!("[ERROR] {}", mmap.err().unwrap());
        }

        let mut mmap = mmap.unwrap();

        let tokens = lex_pgn(&mut mmap);

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

        // SAFE: Safe
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
