pub mod define_structs;
pub mod errors;
pub mod headers;

use std::error::Error;

pub use define_structs::*;
use errors::{Location, PgnError};
pub use headers::*;

use crate::{pgn_move_to_half_move, utils};

pub fn parse_pgn(input_filename: &str) -> Result<Vec<PgnGame>, Box<dyn Error + '_>> {
    // TODO(#7): Add variations support
    // TODO(#8): Add comments support

    let lines = utils::read_lines(input_filename)?;

    let mut vec_games: Vec<PgnGame> = Vec::with_capacity(1024);
    let mut game_index = 0;
    let mut half_move_count: u16 = 0;
    vec_games.insert(0, PgnGame::default());

    for (i, line_) in lines.flatten().enumerate() {
        let line = line_.trim();
        let mut defer = false;

        let mut line_len = line.len();
        let bytes = line.as_bytes();

        if line_len == 0 {
            continue;
        }

        let last_two = bytes[line_len - 2];
        let last_one = bytes[line_len - 1];

        if last_two == b'/' || last_two == b'-' || last_one == b'*' {
            if last_two == b'/' {
                *(vec_games[game_index].get_result_mut()) = PgnResult::Draw;
                line_len -= 7;
            } else if last_one == b'0' {
                *(vec_games[game_index].get_result_mut()) = PgnResult::WhiteWon;
                line_len -= 3;
            } else if last_one == b'1' {
                *(vec_games[game_index].get_result_mut()) = PgnResult::BlackWon;
                line_len -= 3;
            } else {
                *(vec_games[game_index].get_result_mut()) = PgnResult::Undefined;
                line_len -= 1;
            };

            defer = true;
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
                vec_games[game_index]
                    .get_headers_mut()
                    .insert(header.0, header.1);
            }

            continue;
        }

        let mut line_char_index = 0;
        while line_char_index < line_len {
            let char = bytes[line_char_index];
            let line_char_index_start = line_char_index;

            // TODO: (MAYBE) Replace half-move count parsing with a smarter method
            // TODO: (MAYBE) Abstract this away to see the performance overhead of this
            // This is basically extra parsing because half-move count can be easily calculated by setting it at the start of a variation and then just incrementing it
            if char.is_ascii_digit() {
                let mut num_buffer = String::new();

                while line_char_index < line_len && bytes[line_char_index] != b'.' {
                    num_buffer.push(bytes[line_char_index] as char);
                    line_char_index += 1;
                }
                line_char_index += 1;

                let parsed_num = num_buffer.as_str().parse::<u16>();

                if parsed_num.is_err() {
                    return Err(Box::new(PgnError {
                        code: 101,
                        message: format!("Couldn't parse number: {num_buffer}"),
                        location: Location {
                            file: input_filename,
                            line: line_len as u32,
                            col: line_char_index_start as u32,
                        },
                    }));
                }

                half_move_count = pgn_move_to_half_move!(parsed_num.unwrap());

                if line_char_index + 1 < line_len && bytes[line_char_index] == b'.' {
                    half_move_count += 1;
                    line_char_index += 2;
                }
            } else if char.is_ascii_alphabetic() {
                let mut buffer = String::new();

                while line_char_index < line_len
                    && (bytes[line_char_index].is_ascii_alphanumeric()
                        || bytes[line_char_index].is_ascii_punctuation())
                {
                    buffer.push(bytes[line_char_index] as char);
                    line_char_index += 1;
                }

                (*vec_games[game_index].get_main_variation_mut()).insert(PgnMove {
                    value: buffer,
                    half_move_location: half_move_count,
                });

                half_move_count += 1;
            }

            line_char_index += 1;
            continue;
        }

        if defer == true {
            half_move_count = 0;
            game_index += 1;
            vec_games.insert(game_index, PgnGame::default());

            #[allow(unused_assignments)]
            defer = false;
        }
    }

    vec_games.remove(game_index);

    Ok(vec_games)
}
