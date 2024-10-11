#[cfg(test)]
mod cmbr_tests {
    #[allow(unused_imports)]
    use crate::pgn::{self, lex_pgn, Token};
    use crate::{
        cmbr::{CmbrMv, SanToCmbrMvConvertor},
        pgn::PgnToken,
    };
    use memmap2::Mmap;
    use project_root::get_project_root;
    use shakmaty::Chess;
    use std::fs::File;

    #[cfg(feature = "benchmark")]
    extern crate test;
    #[cfg(feature = "benchmark")]
    use test::Bencher;

    #[test]
    fn test_san_cmbr() {
        let file_path = get_project_root().unwrap().join("data/promotion.pgn");
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

        let ast = pgn::parse_pgn(&mut mmap);
        let mut convertor = SanToCmbrMvConvertor::new(/* 16MB */ 16 * 1024 * 1024);
        let mut cmbrs: Vec<CmbrMv> = vec![];

        for game in ast {
            for (_, variation) in game.variations {
                let mut board = Chess::new();

                for token in variation.0 {
                    if let PgnToken::Token(t) = token {
                        if let Token::Move(san) = t {
                            let cmbr = convertor.san_to_cmbr(&mut board, san).unwrap();
                            cmbrs.push(cmbr);
                        }
                    }
                }
            }
        }

        let expected_vec: Vec<CmbrMv> = vec![
            0b011100001100000000000000.into(),
            0b101101110101100000000000.into(),
            0b100100011100000000000000.into(),
            0b101100110100100000000000.into(),
            0b101101100100000000000100.into(),
            0b100010111101101000000000.into(),
            0b110110101101000000000100.into(),
            0b101101111110100100000000.into(),
            0b111111110110000001110101.into(),
        ];

        assert_eq!(expected_vec, cmbrs);
    }

    // TODO: Implement bench_san_cmbr
    // #[cfg(feature = "benchmark")]
    // #[bench]
    // fn bench_san_cmbr(_b: &mut Bencher) {
    //
    // }
}
