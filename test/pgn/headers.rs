#[cfg(test)]
mod tests_header {
    use crate::pgn::parse_pgn_header;
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
}
