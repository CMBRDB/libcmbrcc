use super::Cli;
use libcmbr::cmbr::{san_to_cmbr, u24};
use libcmbr::pgn::parse_pgn;

use memmap2::Mmap;
use std::fs::File;
use std::io::Write;

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

            // SAFE: Safe
            let file = unsafe { file.unwrap_unchecked() };
            let mmap = unsafe { Mmap::map(&file) };

            if mmap.is_err() {
                eprintln!("[ERROR] {}. File name: {file_name}", mmap.err().unwrap());
                exit(1);
            }

            // SAFE: Safe
            let mut mmap = unsafe { mmap.unwrap_unchecked() };

            let ast = parse_pgn(&mut mmap);
            let mut cmbrs = vec![];

            for game in ast {
                for (_, variation) in game.0 .1 {
                    let mut board = libcmbr::ChessBoard::new();

                    for token in variation.0 {
                        match token {
                            libcmbr::pgn::PgnToken::Token(t) => match t {
                                libcmbr::pgn::Token::Move(san) => {
                                    let cmbr = san_to_cmbr(&mut board, san).unwrap();
                                    cmbrs.push(cmbr);
                                }
                                _ => {}
                            },
                            _ => {}
                        }
                    }
                }
            }

            let mut f = File::create(&args.output).unwrap();
            f.write_all(unsafe {
                std::slice::from_raw_parts(
                    cmbrs.as_ptr() as *const u8,
                    cmbrs.len() * std::mem::size_of::<u24>(),
                )
            })
            .unwrap();

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
