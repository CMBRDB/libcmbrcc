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
        let expected_output = "Ok([Header(\"Event\", \"Fischer - Spassky\"), Header(\"Site\", \"Sveti Stefan / Belgrade YUG\"), Header(\"Date\", \"1992.09.02\"), Header(\"EventDate\", \"1992.09.02\"), Header(\"Round\", \"1\"), Header(\"Result\", \"1-0\"), Header(\"White\", \"Robert James Fischer\"), Header(\"Black\", \"Boris Spassky\"), Header(\"ECO\", \"C95\"), Header(\"WhiteElo\", \"?\"), Header(\"BlackElo\", \"?\"), Header(\"PlyCount\", \"99\"), HalfMoveNumber(1), PgnMove(\"e4\"), HalfMoveNumber(2), PgnMove(\"e5\"), HalfMoveNumber(3), PgnMove(\"Nf3\"), HalfMoveNumber(4), PgnMove(\"Nc6\"), HalfMoveNumber(5), PgnMove(\"Bb5\"), HalfMoveNumber(6), PgnMove(\"a6\"), HalfMoveNumber(7), PgnMove(\"Ba4\"), HalfMoveNumber(8), PgnMove(\"Nf6\"), HalfMoveNumber(9), PgnMove(\"O-O\"), HalfMoveNumber(10), PgnMove(\"Be7\"), HalfMoveNumber(11), PgnMove(\"Re1\"), HalfMoveNumber(12), PgnMove(\"b5\"), HalfMoveNumber(13), PgnMove(\"Bb3\"), HalfMoveNumber(14), PgnMove(\"O-O\"), HalfMoveNumber(15), PgnMove(\"c3\"), HalfMoveNumber(16), PgnMove(\"d6\"), HalfMoveNumber(17), PgnMove(\"h3\"), HalfMoveNumber(18), PgnMove(\"Nb8\"), HalfMoveNumber(19), PgnMove(\"d4\"), HalfMoveNumber(20), PgnMove(\"Nbd7\"), HalfMoveNumber(21), PgnMove(\"Nbd2\"), HalfMoveNumber(22), PgnMove(\"Bb7\"), HalfMoveNumber(23), PgnMove(\"Bc2\"), HalfMoveNumber(24), PgnMove(\"Re8\"), HalfMoveNumber(25), PgnMove(\"Nf1\"), HalfMoveNumber(26), PgnMove(\"Bf8\"), HalfMoveNumber(27), PgnMove(\"Ng3\"), HalfMoveNumber(28), PgnMove(\"g6\"), HalfMoveNumber(29), PgnMove(\"Bg5\"), HalfMoveNumber(30), PgnMove(\"h6\"), HalfMoveNumber(31), PgnMove(\"Bd2\"), HalfMoveNumber(32), PgnMove(\"Bg7\"), HalfMoveNumber(33), PgnMove(\"a4\"), HalfMoveNumber(34), PgnMove(\"c5\"), HalfMoveNumber(35), PgnMove(\"d5\"), HalfMoveNumber(36), PgnMove(\"c4\"), HalfMoveNumber(37), PgnMove(\"b4\"), HalfMoveNumber(38), PgnMove(\"Nh7\"), HalfMoveNumber(39), PgnMove(\"Be3\"), HalfMoveNumber(40), PgnMove(\"h5\"), HalfMoveNumber(41), PgnMove(\"Qd2\"), HalfMoveNumber(42), PgnMove(\"Rf8\"), HalfMoveNumber(43), PgnMove(\"Ra3\"), HalfMoveNumber(44), PgnMove(\"Ndf6\"), HalfMoveNumber(45), PgnMove(\"Rea1\"), HalfMoveNumber(46), PgnMove(\"Qd7\"), HalfMoveNumber(47), PgnMove(\"R1a2\"), HalfMoveNumber(48), PgnMove(\"Rfc8\"), HalfMoveNumber(49), PgnMove(\"Qc1\"), HalfMoveNumber(50), PgnMove(\"Bf8\"), HalfMoveNumber(51), PgnMove(\"Qa1\"), HalfMoveNumber(52), PgnMove(\"Qe8\"), HalfMoveNumber(53), PgnMove(\"Nf1\"), HalfMoveNumber(54), PgnMove(\"Be7\"), HalfMoveNumber(55), PgnMove(\"N1d2\"), HalfMoveNumber(56), PgnMove(\"Kg7\"), HalfMoveNumber(57), PgnMove(\"Nb1\"), HalfMoveNumber(58), PgnMove(\"Nxe4\"), HalfMoveNumber(59), PgnMove(\"Bxe4\"), HalfMoveNumber(60), PgnMove(\"f5\"), HalfMoveNumber(61), PgnMove(\"Bc2\"), HalfMoveNumber(62), PgnMove(\"Bxd5\"), HalfMoveNumber(63), PgnMove(\"axb5\"), HalfMoveNumber(64), PgnMove(\"axb5\"), HalfMoveNumber(65), PgnMove(\"Ra7\"), HalfMoveNumber(66), PgnMove(\"Kf6\"), HalfMoveNumber(67), PgnMove(\"Nbd2\"), HalfMoveNumber(68), PgnMove(\"Rxa7\"), HalfMoveNumber(69), PgnMove(\"Rxa7\"), HalfMoveNumber(70), PgnMove(\"Ra8\"), HalfMoveNumber(71), PgnMove(\"g4\"), HalfMoveNumber(72), PgnMove(\"hxg4\"), HalfMoveNumber(73), PgnMove(\"hxg4\"), HalfMoveNumber(74), PgnMove(\"Rxa7\"), HalfMoveNumber(75), PgnMove(\"Qxa7\"), HalfMoveNumber(76), PgnMove(\"f4\"), HalfMoveNumber(77), PgnMove(\"Bxf4\"), HalfMoveNumber(78), PgnMove(\"exf4\"), HalfMoveNumber(79), PgnMove(\"Nh4\"), HalfMoveNumber(80), PgnMove(\"Bf7\"), HalfMoveNumber(81), PgnMove(\"Qd4+\"), HalfMoveNumber(82), PgnMove(\"Ke6\"), HalfMoveNumber(83), PgnMove(\"Nf5\"), HalfMoveNumber(84), PgnMove(\"Bf8\"), HalfMoveNumber(85), PgnMove(\"Qxf4\"), HalfMoveNumber(86), PgnMove(\"Kd7\"), HalfMoveNumber(87), PgnMove(\"Nd4\"), HalfMoveNumber(88), PgnMove(\"Qe1+\"), HalfMoveNumber(89), PgnMove(\"Kg2\"), HalfMoveNumber(90), PgnMove(\"Bd5+\"), HalfMoveNumber(91), PgnMove(\"Be4\"), HalfMoveNumber(92), Result(WhiteWon), PgnMove(\"Bxe4+\"), HalfMoveNumber(93), PgnMove(\"Nxe4\"), HalfMoveNumber(94), PgnMove(\"Be7\"), HalfMoveNumber(95), PgnMove(\"Nxb5\"), HalfMoveNumber(96), PgnMove(\"Nf8\"), HalfMoveNumber(97), PgnMove(\"Nbxd6\"), HalfMoveNumber(98), PgnMove(\"Ne6\"), HalfMoveNumber(99), PgnMove(\"Qe5\"), HalfMoveNumber(100)])";

        assert_eq!(
            expected_output,
            format!("{:?}", parse_pgn("data/fischer_spassky_1992.pgn"))
        );
    }
}
