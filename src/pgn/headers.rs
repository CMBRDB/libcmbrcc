pub use smartstring::alias::String;

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
