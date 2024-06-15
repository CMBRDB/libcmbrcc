#[cfg(test)]
mod pgn_tests {
    use crate::pgn::{parse_pgn, parse_pgn_header};
    use smartstring::alias::String;

    #[test]
    fn test_header_parsing() {
        let tests: [(&str, (String, String)); 12] = [
            (
                "[Event \"Fischer - Spassky\"]",
                ("Event".into(), "Fischer - Spassky".into()),
            ),
            (
                "[Site \"Sveti Stefan / Belgrade YUG\"]",
                ("Site".into(), "Sveti Stefan / Belgrade YUG".into()),
            ),
            (
                "[Date \"1992.09.02\"]",
                ("Date".into(), "1992.09.02".into()),
            ),
            (
                "[EventDate \"1992.09.02\"]",
                ("EventDate".into(), "1992.09.02".into()),
            ),
            ("[Round \"1\"]", ("Round".into(), "1".into())),
            ("[Result \"1-0\"]", ("Result".into(), "1-0".into())),
            (
                "[White \"Robert James Fischer\"]",
                ("White".into(), "Robert James Fischer".into()),
            ),
            (
                "[Black \"Boris Spassky\"]",
                ("Black".into(), "Boris Spassky".into()),
            ),
            ("[ECO \"C95\"]", ("ECO".into(), "C95".into())),
            ("[WhiteElo \"?\"]", ("WhiteElo".into(), "?".into())),
            ("[BlackElo \"?\"]", ("BlackElo".into(), "?".into())),
            ("[PlyCount \"99\"]", ("PlyCount".into(), "99".into())),
        ];

        for test in tests {
            unsafe {
                eprintln!("Testing line: `{}`", test.0);
                assert_eq!(Some(test.1), parse_pgn_header(test.0));
            }
        }
    }

    #[test]
    fn test_pgn_parsing_no_variations() {
        let expected_output = "Ok([PgnGame { headers: {\"Event\": \"Fischer - Spassky\", \"Site\": \"Sveti Stefan / Belgrade YUG\", \"Date\": \"1992.09.02\", \"EventDate\": \"1992.09.02\", \"Round\": \"1\", \"Result\": \"1-0\", \"White\": \"Robert James Fischer\", \"Black\": \"Boris Spassky\", \"ECO\": \"C95\", \"WhiteElo\": \"?\", \"BlackElo\": \"?\", \"PlyCount\": \"99\"}, comments: [], main_variation: Tree { children: [Tree { children: [], value: PgnMove { value: \"e4\", half_move_location: 1 } }, Tree { children: [], value: PgnMove { value: \"e5\", half_move_location: 2 } }, Tree { children: [], value: PgnMove { value: \"Nf3\", half_move_location: 3 } }, Tree { children: [], value: PgnMove { value: \"Nc6\", half_move_location: 4 } }, Tree { children: [], value: PgnMove { value: \"Bb5\", half_move_location: 5 } }, Tree { children: [], value: PgnMove { value: \"a6\", half_move_location: 6 } }, Tree { children: [], value: PgnMove { value: \"Ba4\", half_move_location: 7 } }, Tree { children: [], value: PgnMove { value: \"Nf6\", half_move_location: 8 } }, Tree { children: [], value: PgnMove { value: \"O-O\", half_move_location: 9 } }, Tree { children: [], value: PgnMove { value: \"Be7\", half_move_location: 10 } }, Tree { children: [], value: PgnMove { value: \"Re1\", half_move_location: 11 } }, Tree { children: [], value: PgnMove { value: \"b5\", half_move_location: 12 } }, Tree { children: [], value: PgnMove { value: \"Bb3\", half_move_location: 13 } }, Tree { children: [], value: PgnMove { value: \"O-O\", half_move_location: 14 } }, Tree { children: [], value: PgnMove { value: \"c3\", half_move_location: 15 } }, Tree { children: [], value: PgnMove { value: \"d6\", half_move_location: 16 } }, Tree { children: [], value: PgnMove { value: \"h3\", half_move_location: 17 } }, Tree { children: [], value: PgnMove { value: \"Nb8\", half_move_location: 18 } }, Tree { children: [], value: PgnMove { value: \"d4\", half_move_location: 19 } }, Tree { children: [], value: PgnMove { value: \"Nbd7\", half_move_location: 20 } }, Tree { children: [], value: PgnMove { value: \"Nbd2\", half_move_location: 21 } }, Tree { children: [], value: PgnMove { value: \"Bb7\", half_move_location: 22 } }, Tree { children: [], value: PgnMove { value: \"Bc2\", half_move_location: 23 } }, Tree { children: [], value: PgnMove { value: \"Re8\", half_move_location: 24 } }, Tree { children: [], value: PgnMove { value: \"Nf1\", half_move_location: 25 } }, Tree { children: [], value: PgnMove { value: \"Bf8\", half_move_location: 26 } }, Tree { children: [], value: PgnMove { value: \"Ng3\", half_move_location: 27 } }, Tree { children: [], value: PgnMove { value: \"g6\", half_move_location: 28 } }, Tree { children: [], value: PgnMove { value: \"Bg5\", half_move_location: 29 } }, Tree { children: [], value: PgnMove { value: \"h6\", half_move_location: 30 } }, Tree { children: [], value: PgnMove { value: \"Bd2\", half_move_location: 31 } }, Tree { children: [], value: PgnMove { value: \"Bg7\", half_move_location: 32 } }, Tree { children: [], value: PgnMove { value: \"a4\", half_move_location: 33 } }, Tree { children: [], value: PgnMove { value: \"c5\", half_move_location: 34 } }, Tree { children: [], value: PgnMove { value: \"d5\", half_move_location: 35 } }, Tree { children: [], value: PgnMove { value: \"c4\", half_move_location: 36 } }, Tree { children: [], value: PgnMove { value: \"b4\", half_move_location: 37 } }, Tree { children: [], value: PgnMove { value: \"Nh7\", half_move_location: 38 } }, Tree { children: [], value: PgnMove { value: \"Be3\", half_move_location: 39 } }, Tree { children: [], value: PgnMove { value: \"h5\", half_move_location: 40 } }, Tree { children: [], value: PgnMove { value: \"Qd2\", half_move_location: 41 } }, Tree { children: [], value: PgnMove { value: \"Rf8\", half_move_location: 42 } }, Tree { children: [], value: PgnMove { value: \"Ra3\", half_move_location: 43 } }, Tree { children: [], value: PgnMove { value: \"Ndf6\", half_move_location: 44 } }, Tree { children: [], value: PgnMove { value: \"Rea1\", half_move_location: 45 } }, Tree { children: [], value: PgnMove { value: \"Qd7\", half_move_location: 46 } }, Tree { children: [], value: PgnMove { value: \"R1a2\", half_move_location: 47 } }, Tree { children: [], value: PgnMove { value: \"Rfc8\", half_move_location: 48 } }, Tree { children: [], value: PgnMove { value: \"Qc1\", half_move_location: 49 } }, Tree { children: [], value: PgnMove { value: \"Bf8\", half_move_location: 50 } }, Tree { children: [], value: PgnMove { value: \"Qa1\", half_move_location: 51 } }, Tree { children: [], value: PgnMove { value: \"Qe8\", half_move_location: 52 } }, Tree { children: [], value: PgnMove { value: \"Nf1\", half_move_location: 53 } }, Tree { children: [], value: PgnMove { value: \"Be7\", half_move_location: 54 } }, Tree { children: [], value: PgnMove { value: \"N1d2\", half_move_location: 55 } }, Tree { children: [], value: PgnMove { value: \"Kg7\", half_move_location: 56 } }, Tree { children: [], value: PgnMove { value: \"Nb1\", half_move_location: 57 } }, Tree { children: [], value: PgnMove { value: \"Nxe4\", half_move_location: 58 } }, Tree { children: [], value: PgnMove { value: \"Bxe4\", half_move_location: 59 } }, Tree { children: [], value: PgnMove { value: \"f5\", half_move_location: 60 } }, Tree { children: [], value: PgnMove { value: \"Bc2\", half_move_location: 61 } }, Tree { children: [], value: PgnMove { value: \"Bxd5\", half_move_location: 62 } }, Tree { children: [], value: PgnMove { value: \"axb5\", half_move_location: 63 } }, Tree { children: [], value: PgnMove { value: \"axb5\", half_move_location: 64 } }, Tree { children: [], value: PgnMove { value: \"Ra7\", half_move_location: 65 } }, Tree { children: [], value: PgnMove { value: \"Kf6\", half_move_location: 66 } }, Tree { children: [], value: PgnMove { value: \"Nbd2\", half_move_location: 67 } }, Tree { children: [], value: PgnMove { value: \"Rxa7\", half_move_location: 68 } }, Tree { children: [], value: PgnMove { value: \"Rxa7\", half_move_location: 69 } }, Tree { children: [], value: PgnMove { value: \"Ra8\", half_move_location: 70 } }, Tree { children: [], value: PgnMove { value: \"g4\", half_move_location: 71 } }, Tree { children: [], value: PgnMove { value: \"hxg4\", half_move_location: 72 } }, Tree { children: [], value: PgnMove { value: \"hxg4\", half_move_location: 73 } }, Tree { children: [], value: PgnMove { value: \"Rxa7\", half_move_location: 74 } }, Tree { children: [], value: PgnMove { value: \"Qxa7\", half_move_location: 75 } }, Tree { children: [], value: PgnMove { value: \"f4\", half_move_location: 76 } }, Tree { children: [], value: PgnMove { value: \"Bxf4\", half_move_location: 77 } }, Tree { children: [], value: PgnMove { value: \"exf4\", half_move_location: 78 } }, Tree { children: [], value: PgnMove { value: \"Nh4\", half_move_location: 79 } }, Tree { children: [], value: PgnMove { value: \"Bf7\", half_move_location: 80 } }, Tree { children: [], value: PgnMove { value: \"Qd4+\", half_move_location: 81 } }, Tree { children: [], value: PgnMove { value: \"Ke6\", half_move_location: 82 } }, Tree { children: [], value: PgnMove { value: \"Nf5\", half_move_location: 83 } }, Tree { children: [], value: PgnMove { value: \"Bf8\", half_move_location: 84 } }, Tree { children: [], value: PgnMove { value: \"Qxf4\", half_move_location: 85 } }, Tree { children: [], value: PgnMove { value: \"Kd7\", half_move_location: 86 } }, Tree { children: [], value: PgnMove { value: \"Nd4\", half_move_location: 87 } }, Tree { children: [], value: PgnMove { value: \"Qe1+\", half_move_location: 88 } }, Tree { children: [], value: PgnMove { value: \"Kg2\", half_move_location: 89 } }, Tree { children: [], value: PgnMove { value: \"Bd5+\", half_move_location: 90 } }, Tree { children: [], value: PgnMove { value: \"Be4\", half_move_location: 91 } }, Tree { children: [], value: PgnMove { value: \"Bxe4+\", half_move_location: 92 } }, Tree { children: [], value: PgnMove { value: \"Nxe4\", half_move_location: 93 } }, Tree { children: [], value: PgnMove { value: \"Be7\", half_move_location: 94 } }, Tree { children: [], value: PgnMove { value: \"Nxb5\", half_move_location: 95 } }, Tree { children: [], value: PgnMove { value: \"Nf8\", half_move_location: 96 } }, Tree { children: [], value: PgnMove { value: \"Nbxd6\", half_move_location: 97 } }, Tree { children: [], value: PgnMove { value: \"Ne6\", half_move_location: 98 } }, Tree { children: [], value: PgnMove { value: \"Qe5\", half_move_location: 99 } }], value: PgnMove { value: \"\", half_move_location: 0 } }, result: WhiteWon }])";

        assert_eq!(
            expected_output,
            format!("{:?}", parse_pgn("data/fischer_spassky_1992.pgn"))
        );
    }
}
