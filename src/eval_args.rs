use super::Cli;
use crate::pgn::parse_pgn;

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
            let file_name = args.input.clone();
            let file = File::open(&file_name);

            if file.is_err() {
                eprintln!("[ERROR] {}. File name: {file_name}", file.err().unwrap());
                exit(1);
            }

            let file = unsafe { file.unwrap_unchecked() };
            let mmap = unsafe { Mmap::map(&file) };

            if mmap.is_err() {
                eprintln!("[ERROR] {}. File name: {file_name}", mmap.err().unwrap());
                exit(1);
            }

            let mut mmap = unsafe { mmap.unwrap_unchecked() };

            std::hint::black_box(parse_pgn(&mut mmap));

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
