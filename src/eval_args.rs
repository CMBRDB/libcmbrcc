use crate::pgn::pgn_tokens_to_ast;

use super::pgn;
use super::Cli;

use memmap2::Mmap;
use std::fs::File;

pub fn eval_args(cli: &Cli) {
    use std::process::exit;

    match cli.command.as_ref().unwrap() {
        crate::CommandE::Cmbr2pgn(_args) => {
            // TODO(#1): Implement CMBR2PGN
        }

        crate::CommandE::Pgn2cmbr(args) => {
            // TODO(#14): Implement PGN2CMBR

            let file = File::open(args.input.clone());

            if file.is_err() {
                println!("[ERROR] {}", file.err().unwrap());
                exit(1);
            }

            let file = unsafe { file.unwrap_unchecked() };
            let mmap = unsafe { Mmap::map(&file) };

            if mmap.is_err() {
                println!("[ERROR] {}", mmap.err().unwrap());
                exit(1);
            }

            let mut mmap = unsafe { mmap.unwrap_unchecked() };
            let mut tokens = pgn::parse_pgn(&mut mmap);

            std::hint::black_box(pgn_tokens_to_ast(&mut tokens));

            // TODO(#15): Implement the 3rd and final step of processing PGN files - Conversion.
         }

        crate::CommandE::License => {
            println!("libcmbr, cmbrcc  Copyright (C) 2024 datawater");
            println!("This program comes with ABSOLUTELY NO WARRANTY;");
            println!("This is free software, and you are welcome to redistribute it,");
            println!("under the conditions of the GPL-3.0 License;");
            println!("\nSee https://github.com/datawater/cmbr");

            exit(0);
        }
    }
}
