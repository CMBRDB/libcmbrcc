#[cfg(test)]
mod cmbr_tests {
    #[allow(unused_imports)]
    use crate::pgn::{self, lex_pgn, Token};
    use crate::{
        cmbr::{san_to_cmbr, Cmbr},
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
        let mut cmbrs: Vec<Cmbr> = vec![];

        for game in ast {
            for (_, variation) in game.0 .1 {
                let mut board = Chess::new();

                for token in variation.0 {
                    match token {
                        PgnToken::Token(t) => match t {
                            Token::Move(san) => {
                                let cmbr = san_to_cmbr(&mut board, san).unwrap();
                                cmbrs.push(cmbr);
                            }
                            _ => {}
                        },

                        _ => {}
                    }
                }
            }
        }

        let expected_vec: Vec<Cmbr> = vec![
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

    #[cfg(feature = "benchmark")]
    #[bench]
    fn bench_san_cmbr(b: &mut Bencher) {
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
        let ast = pgn::parse_pgn(&mut mmap);

        b.iter(|| {
            for game in &ast {
                // This clone is fucking it up
                for (_, variation) in (&game).0 .1.clone() {
                    let mut board = Chess::new();

                    for token in variation.0 {
                        match token {
                            PgnToken::Token(t) => match t {
                                Token::Move(san) => {
                                    san_to_cmbr(&mut board, san).unwrap();
                                }
                                _ => {}
                            },

                            _ => {}
                        }
                    }
                }
            }
        });
    }
}
