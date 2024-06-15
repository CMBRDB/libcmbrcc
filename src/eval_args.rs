use super::pgn;
use super::Cli;

pub fn eval_args(cli: &Cli) {
    match cli.command.as_ref().unwrap() {
        crate::CommandE::Cmbr2pgn(_args) => {
            // TODO(#1): Implement CMBR2PGN
        }

        crate::CommandE::Pgn2cmbr(args) => {
            let pgn = pgn::parse_pgn(&args.input);
        }

        crate::CommandE::License => {
            println!("libcmbr, cmbrcc  Copyright (C) 2024 datawater");
            println!("This program comes with ABSOLUTELY NO WARRANTY;");
            println!("This is free software, and you are welcome to redistribute it");
            println!("under the conditions of the GPL-3.0 License;");
            println!("\nsee https://github.com/datawater/cmbr");

            std::process::exit(0);
        }
    }
}
