pub mod errors;
pub mod headers;
pub mod tokens;

use crate::{pgn_move_to_half_move, utils};
use errors::{Location, PgnError};
pub use headers::*;
use tokens::*;

use std::error::Error;

// NOTE: After profiling I've found that this function only uses 0.5% of the total samples.
//       it does NOT cause significant overhead therefore isn't worth optimizing.
// NOTE(#2): With the current parsing, it takes 150-200ms to parse a 6.4MB file, so
//           31-42 MB/s of parisng speed.
fn parse_half_move_count<'a>(
    bytes: &'a [u8],
    line_char_index: &'a mut usize,
    line_len: usize,
    input_filename: &'a str,
    line_char_index_start: u32,
    line_i: u32,
) -> Result<u16, Box<dyn Error + 'a>> {
    let mut num_buffer = String::new();

    while *line_char_index < line_len && bytes[*line_char_index] != b'.' {
        num_buffer.push(bytes[*line_char_index] as char);
        *line_char_index += 1;
    }

    *line_char_index += 1;

    let parsed_num = num_buffer.as_str().parse::<u16>();

    if parsed_num.is_err() {
        return Err(Box::new(PgnError {
            code: 101,
            message: format!("Couldn't parse number: {num_buffer}"),
            location: Location {
                file: input_filename,
                line: line_i as u32,
                col: line_char_index_start,
            },
        }));
    }

    let mut half_move_count = pgn_move_to_half_move!(parsed_num.unwrap());

    if *line_char_index + 1 < line_len && bytes[(*line_char_index) + 1] == b'.' {
        half_move_count += 1;
        *line_char_index += 2;
    }

    Ok(half_move_count)
}

pub fn parse_pgn(input_filename: &str) -> Result<Vec<PgnToken>, Box<dyn Error + '_>> {
    let lines = utils::read_lines(input_filename)?;

    let mut tokens_vec: Vec<PgnToken> = Vec::with_capacity(1024);
    let mut in_comment = false;
    let mut comment_string = String::new();
    let mut half_move_number = 0;
    let mut last_was_move_not_number = false;

    for (i, line_) in lines.flatten().enumerate() {
        let line = line_.trim();

        let mut line_len = line.len();
        let bytes = line.as_bytes();

        if line_len == 0 {
            continue;
        }

        let last_two = bytes[line_len - 2];
        let last_one = bytes[line_len - 1];

        if last_two == b'/' || last_two == b'-' || last_one == b'*' {
            if last_two == b'/' {
                tokens_vec.push(PgnToken::Result(PgnResult::Draw));
                line_len -= 7;
            } else if last_one == b'0' {
                tokens_vec.push(PgnToken::Result(PgnResult::WhiteWon));
                line_len -= 3;
            } else if last_one == b'1' {
                tokens_vec.push(PgnToken::Result(PgnResult::BlackWon));
                line_len -= 3;
            } else {
                tokens_vec.push(PgnToken::Result(PgnResult::Undefined));
                line_len -= 1;
            };
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
                tokens_vec.push(PgnToken::Header(header.0, header.1));
            }

            continue;
        }

        let mut line_char_index = 0;
        while line_char_index < line_len {
            let char = bytes[line_char_index];
            let line_char_index_start = line_char_index;

            if in_comment == true {
                while line_char_index < line_len && bytes[line_char_index] != b'}' {
                    comment_string.push(bytes[line_char_index] as char);
                    line_char_index += 1;
                }

                if line_char_index == line_len {
                    if bytes[line_char_index - 1] == b'}' {
                        in_comment = false;
                        tokens_vec.push(PgnToken::Comment(comment_string.clone()));

                        comment_string = String::new();
                    }
                } else if bytes[line_char_index] == b'}' {
                    in_comment = false;
                    tokens_vec.push(PgnToken::Comment(comment_string.clone()));

                    comment_string = String::new();
                }

                line_char_index += 1;
            }

            if char.is_ascii_digit() {
                last_was_move_not_number = true;

                let result = parse_half_move_count(
                    bytes,
                    &mut line_char_index,
                    line_len,
                    input_filename,
                    line_char_index_start as u32,
                    i as u32,
                );

                if result.is_err() {
                    // TODO(#11): Return the error instead of panicing
                    panic!("Coudln't parse half move count: {:?}", result);
                }

                match tokens_vec.last().unwrap() {
                    PgnToken::HalfMoveNumber(_) => {
                        tokens_vec.remove(tokens_vec.len() - 1);
                    }

                    _ => {}
                }

                half_move_number = result.unwrap();
                tokens_vec.push(PgnToken::HalfMoveNumber(half_move_number));
            } else if char.is_ascii_alphabetic() {
                let mut buffer = String::new();

                while line_char_index < line_len
                    && (bytes[line_char_index].is_ascii_alphanumeric()
                        || bytes[line_char_index].is_ascii_punctuation())
                {
                    buffer.push(bytes[line_char_index] as char);
                    line_char_index += 1;
                }

                if last_was_move_not_number {
                    tokens_vec.push(PgnToken::PgnMove(buffer));
                }

                last_was_move_not_number = true;
                half_move_number += 1;
                tokens_vec.push(PgnToken::HalfMoveNumber(half_move_number));
            } else if char == b'{' {
                in_comment = true;
                line_char_index += 1;

                while line_char_index < line_len && bytes[line_char_index] != b'}' {
                    comment_string.push(bytes[line_char_index] as char);
                    line_char_index += 1;
                }

                if line_char_index == line_len {
                    if bytes[line_char_index - 1] == b'}' {
                        in_comment = false;
                        tokens_vec.push(PgnToken::Comment(comment_string.clone()));

                        comment_string = String::new();
                    }
                } else if bytes[line_char_index] == b'}' {
                    in_comment = false;
                    tokens_vec.push(PgnToken::Comment(comment_string.clone()));

                    comment_string = String::new();
                }

                line_char_index += 1;
            } else if char == b'$' {
                // TODO(#13): Support NAG parsing
                // http://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm#c8.2.4
                eprintln!(
                    "[WARNING] NAG parsing is not supported. encountered NAG on line {}",
                    i
                );

                while line_char_index < line_len && bytes[line_char_index] != b' ' {
                    line_char_index += 1;
                }
            } else if char == b'(' {
                tokens_vec.push(PgnToken::VariationStart);
            } else if char == b')' {
                tokens_vec.push(PgnToken::VariationEnd);
            }

            line_char_index += 1;
            continue;
        }
    }

    Ok(tokens_vec)
}
