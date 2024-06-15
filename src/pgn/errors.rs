use std::{error::Error, fmt};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Location<'a> {
    pub file: &'a str,
    pub line: u32,
    pub col: u32,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct PgnError<'a> {
    pub code: usize,
    pub message: String,
    pub location: Location<'a>,
}

impl<'a> Error for PgnError<'a> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl<'a> fmt::Display for PgnError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = match self.code {
            0 => format!("Success! `{}` Location: {:?}", self.message, self.location),
            101 => format!(
                "Couldn't parse number. `{}`. Location in file: {:?}",
                self.message, self.location
            ),
            _ => format!(
                "Girl you really fucked up, this error shouldn't exist. `{}` Location: {:?}",
                self.message, self.location
            ),
        };

        return writeln!(f, "{err_msg}");
    }
}
