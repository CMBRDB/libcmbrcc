use std::error::Error;

#[rustfmt::skip]
pub use smartstring::alias::String;
use super::tree::Tree;
use super::utils;
use micromap::Map;
use smol_str::SmolStr;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct PgnGame {
    headers: PgnHeaders,
    main_variation: PgnVariation,
}

pub type PgnHeaders = Map<String, String, 24>;
pub type PgnMove = SmolStr;
pub type PgnVariation = Tree<PgnMove>;

impl PgnGame {
    #[inline(always)]
    pub fn new() -> Self {
        return Self::default();
    }

    #[inline(always)]
    pub fn get_headers(&self) -> &PgnHeaders {
        return &self.headers;
    }

    #[inline(always)]
    pub fn get_headers_mut(&mut self) -> &mut PgnHeaders {
        return &mut self.headers;
    }
}

pub unsafe fn parse_pgn_header(line: &str) -> Option<(String, String)> {
    let mut to_return: (String, String) = (std::mem::zeroed(), std::mem::zeroed());
    let mut key_or_value = 0;

    let chars = line.as_bytes();
    let mut index = 0;

    while index < line.len() {
        let char = chars[index];

        if char.is_ascii_alphabetic() {
            let mut buffer = String::new();

            while chars[index] != b' ' {
                if index + 1 == line.len() && chars[index + 1] != b' ' {
                    return None;
                }

                buffer.push(chars[index] as char);
                index += 1;
            }

            if key_or_value == 0 {
                to_return.0 = buffer;
                key_or_value += 1;
            } else {
                // Something went wrong
                return None;
            }
        }

        if char == b'"' {
            index += 1;

            let mut buffer = String::new();

            while chars[index] != b'"' {
                if index + 1 == line.len() && chars[index + 1] != b'"' {
                    return None;
                }

                buffer.push(chars[index] as char);
                index += 1;
            }

            if key_or_value == 1 {
                to_return.1 = buffer;
                key_or_value += 1;
            } else {
                // Something went wrong
                return None;
            }
        }

        index += 1;
    }

    Some(to_return)
}

pub fn parse_pgn(input_filename: &str) -> Result<Vec<PgnGame>, Box<dyn Error>> {
    let lines = utils::read_lines(input_filename)?;

    let mut vec_games: Vec<PgnGame> = Vec::with_capacity(1024);
    let mut index = 0;
    vec_games.insert(0, PgnGame::default());

    for (i, line_) in lines.flatten().enumerate() {
        let line = line_.trim();

        let line_len = line.len();
        let bytes = line.as_bytes();

        if line_len == 0 {
            continue;
        }

        if bytes[0] == b'[' {
            let header;
            unsafe {
                header = parse_pgn_header(&line);
            }

            if header == None {
                eprintln!("[WARNING] Couldn't parse header on line: {i}. Skipping. View the lines content down bellow.");
                eprintln!("{line}");
                continue;
            }

            unsafe {
                let header = header.unwrap_unchecked();
                vec_games[index].headers.insert(header.0, header.1);
            }

            continue;
        }

        let last_two = bytes[line_len - 2];
        let last_one = bytes[line_len - 1];

        if last_two == b'/' || last_two == b'-' || last_one == b'*' {
            index += 1;
            vec_games.insert(index, PgnGame::default());

            continue;
        }
    }

    vec_games.remove(index);

    println!("{vec_games:?}");

    Ok(vec_games)
}
