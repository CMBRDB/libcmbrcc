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

        let ast_expected = "[PgnGame { global_tokens: [TagSymbol([69]), TagString([69]), TagSymbol([83]), TagString([83]), Result([42])], variations: LiteMap { values: [(0, PgnVariation([Token(MoveNumber(1, false)), Token(Move([101, 52])), VariationPointer(2), VariationPointer(3), Token(MoveNumber(1, true)), Token(Move([101, 53])), Token(MoveNumber(2, false)), Token(Move([78, 102, 51])), VariationPointer(5), VariationPointer(11), Token(MoveNumber(2, true)), Token(Move([78, 99, 54])), Token(MoveNumber(3, false)), Token(Move([66, 99, 52])), Token(Move([66, 99, 53])), Token(MoveNumber(4, false)), Token(Move([79, 45, 79]))])), (2, PgnVariation([Token(MoveNumber(1, false)), Token(Move([100, 52]))])), (3, PgnVariation([Token(MoveNumber(1, false)), Token(Move([99, 52]))])), (5, PgnVariation([Token(MoveNumber(2, false)), Token(Move([78, 99, 51])), Token(Move([100, 53])), VariationPointer(42)])), (11, PgnVariation([Token(MoveNumber(2, false)), Token(Move([100, 51]))])), (42, PgnVariation([Token(MoveNumber(2, true)), Token(Move([100, 54]))]))], _key_type: PhantomData<u32>, _value_type: PhantomData<libcmbr::pgn::ast::PgnVariation> } }, PgnGame { global_tokens: [TagSymbol([69]), TagString([69]), TagSymbol([83]), TagString([83]), Result([42])], variations: LiteMap { values: [(0, PgnVariation([Token(MoveNumber(1, false)), Token(Move([101, 52])), VariationPointer(2), VariationPointer(3), Token(MoveNumber(1, true)), Token(Move([101, 53])), Token(MoveNumber(2, false)), Token(Move([78, 102, 51])), VariationPointer(5), VariationPointer(11), Token(Commentary([32, 67, 111, 109, 109, 101, 110, 116, 32]))])), (2, PgnVariation([Token(MoveNumber(1, false)), Token(Move([100, 52]))])), (3, PgnVariation([Token(MoveNumber(1, false)), Token(Move([99, 52]))])), (5, PgnVariation([Token(MoveNumber(2, false)), Token(Move([78, 99, 51])), Token(Move([100, 53])), VariationPointer(42)])), (11, PgnVariation([Token(MoveNumber(2, false)), Token(Move([100, 51]))])), (42, PgnVariation([Token(MoveNumber(2, true)), Token(Move([100, 54]))]))], _key_type: PhantomData<u32>, _value_type: PhantomData<libcmbr::pgn::ast::PgnVariation> } }]";

        assert_eq!(format!("{:?}", ast), ast_expected);
    }

    #[cfg(feature = "benchmark")]
    #[bench]
    fn bench_ast(b: &mut Bencher) {
        let file_path = get_project_root()
            .unwrap()
            .join("data/fischer_spassky_1992.pgn");
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

    #[cfg(feature = "benchmark")]
    #[bench]
    fn bench_lex(b: &mut Bencher) {
        let file_path = get_project_root()
            .unwrap()
            .join("data/fischer_spassky_1992.pgn");
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
