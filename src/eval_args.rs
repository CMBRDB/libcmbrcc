use super::Cli;
use libcmbr::cmbr::{CmbrFile, SanToCmbrMvConvertor};
use libcmbr::pgn::parse_pgn;

use memmap2::Mmap;
use std::fs::File;
use std::io::Write;

pub fn eval_args(cli: &Cli) {
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
                std::process::exit(1);
            }

            // SAFE: Safe
            let file = unsafe { file.unwrap_unchecked() };
            let mmap = unsafe { Mmap::map(&file) };

            if mmap.is_err() {
                eprintln!("[ERROR] {}. File name: {file_name}", mmap.err().unwrap());
                std::process::exit(1);
            }

            // SAFE: Safe
            let mut mmap = unsafe { mmap.unwrap_unchecked() };

            let ast = parse_pgn(&mut mmap);
            let mut convertor = SanToCmbrMvConvertor::new(args.table_mem_limit);

            let cmbr_file =
                CmbrFile::from_ast(ast, &mut convertor, args.enable_compression).unwrap();

            cfg_if::cfg_if! {
                if #[cfg(feature = "bitcode")] {
                    let mut f = File::create(&args.output).unwrap();
                    let serialized = bitcode::serialize(&cmbr_file).unwrap();
                    f.write(&serialized[..]).unwrap();
                }
            };
        }

        crate::CommandE::License => {
            println!("libcmbr, cmbrcc  Copyright (C) 2024 datawater");
            println!("This program comes with ABSOLUTELY NO WARRANTY;");
            println!("This is free software, and you are welcome to redistribute it,");
            println!("under the conditions of the GPL-3.0 License;");
            println!("\nSee https://github.com/datawater/cmbr");

            std::process::exit(0);
        }
    }
}
