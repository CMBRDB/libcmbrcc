#[cfg(feature = "benchmark")]
#[feature(test)]
extern crate test;

#[cfg(test)]
mod pgn_tests {
    use memmap2::Mmap;
    use project_root::get_project_root;
    use std::fs::File;

    #[cfg(feature = "benchmark")]
    use super::test::Bencher;

    use crate::pgn;

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

        let file = unsafe { file.unwrap_unchecked() };
        let mmap = unsafe { Mmap::map(&file) };

        if mmap.is_err() {
            panic!("[ERROR] {}", mmap.err().unwrap());
        }

        let mut mmap = mmap.unwrap();

        let ast = pgn::parse_pgn(&mut mmap);

        let ast_expected = "[PgnGame(([TagSymbol([69]), TagString([69]), TagSymbol([83]), TagString([83]), Result([42])], LiteMap { values: [(0, PgnVariation([Token(MoveNumber(1, false)), Token(Move([101, 52])), VariationPointer(1), VariationPointer(2), Token(MoveNumber(1, true)), Token(Move([101, 53])), Token(MoveNumber(2, false)), Token(Move([78, 102, 51])), VariationPointer(3), VariationPointer(5), Token(Commentary([32, 67, 111, 109, 109, 101, 110, 116, 32]))])), (1, PgnVariation([Token(MoveNumber(1, false)), Token(Move([100, 52]))])), (2, PgnVariation([Token(MoveNumber(1, false)), Token(Move([99, 52]))])), (3, PgnVariation([Token(MoveNumber(2, false)), Token(Move([78, 99, 51])), Token(Move([100, 53])), VariationPointer(16)])), (5, PgnVariation([Token(MoveNumber(2, false)), Token(Move([100, 51]))])), (16, PgnVariation([Token(MoveNumber(2, true)), Token(Move([100, 54]))]))], _key_type: PhantomData<u16>, _value_type: PhantomData<cmbr::pgn::ast::PgnVariation> })), PgnGame(([TagSymbol([69]), TagString([69]), TagSymbol([83]), TagString([83]), Result([42])], LiteMap { values: [(0, PgnVariation([Token(MoveNumber(1, false)), Token(Move([101, 52])), VariationPointer(1), VariationPointer(2), Token(MoveNumber(1, true)), Token(Move([101, 53])), Token(MoveNumber(2, false)), Token(Move([78, 102, 51])), VariationPointer(3), VariationPointer(5), Token(Commentary([32, 67, 111, 109, 109, 101, 110, 116, 32]))])), (1, PgnVariation([Token(MoveNumber(1, false)), Token(Move([100, 52]))])), (2, PgnVariation([Token(MoveNumber(1, false)), Token(Move([99, 52]))])), (3, PgnVariation([Token(MoveNumber(2, false)), Token(Move([78, 99, 51])), Token(Move([100, 53])), VariationPointer(16)])), (5, PgnVariation([Token(MoveNumber(2, false)), Token(Move([100, 51]))])), (16, PgnVariation([Token(MoveNumber(2, true)), Token(Move([100, 54]))]))], _key_type: PhantomData<u16>, _value_type: PhantomData<cmbr::pgn::ast::PgnVariation> }))]";

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
}
