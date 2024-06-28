// TODO: Define and Write documentation for the CMBR Standard.
// TODO: Seperate the cli and libcmbr
#![feature(test)]
#![allow(non_upper_case_globals)]

mod cmbr;
mod eval_args;
mod pgn;
mod tests;

use lexopt::prelude::*;
use std::{process::exit, thread::available_parallelism};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandE {
    Cmbr2pgn(Cmbr2PgnArgs),
    Pgn2cmbr(Pgn2CmbrArgs),
    License,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cmbr2PgnArgs {
    input: String,
    output: String,
    threads_n: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pgn2CmbrArgs {
    input: String,
    output: String,
    enable_compression: bool,
    zstd_compression_level: u8,
    threads_n: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cli {
    command: Option<CommandE>,
}

fn print_usage() {
    // TODO: A better usage/help-command..

    println!("\nUsage: cmbr {{COMMAND}} [OPTIONS]");
    println!("note: Options inside of square brackets ([]) are optional\n");
    println!("Commands:");
    println!("  cmbr2pgn --input {{INPUT_FILE}} [--output {{OUTPUT_FILE}} --threads_n {{AMOUNT_OF_THREADS}} --enable_compression ]");
    println!("  pgncmbr  --input {{INPUT_FILE}} [--output {{OUTPUT_FILE}} --threads_n {{AMOUNT_OF_THREADS}} ]");
    println!("  license");
}

fn parse_args() -> Cli {
    let mut parser = lexopt::Parser::from_env();
    let mut command = None;

    while let Some(arg) = parser.next().unwrap() {
        match arg {
            Short('h') | Long("help") => {
                print_usage();
                exit(0);
            }

            Short('T') | Long("threads_n") => {
                let threads_n = parser.value().unwrap().parse().unwrap();

                if let Some(CommandE::Cmbr2pgn(ref mut args)) = command {
                    args.threads_n = threads_n;
                } else if let Some(CommandE::Pgn2cmbr(ref mut args)) = command {
                    args.threads_n = threads_n;
                }
            }

            Short('o') | Long("output") => {
                let output = parser.value().unwrap().into_string().unwrap();

                if let Some(CommandE::Cmbr2pgn(ref mut args)) = command {
                    args.output = output;
                } else if let Some(CommandE::Pgn2cmbr(ref mut args)) = command {
                    args.output = output;
                }
            }

            Short('i') | Long("input") => {
                let input = parser.value().unwrap().into_string().unwrap();

                if let Some(CommandE::Cmbr2pgn(ref mut args)) = command {
                    args.input = input.clone();
                } else if let Some(CommandE::Pgn2cmbr(ref mut args)) = command {
                    args.input = input.clone();
                }
            }

            Short('c') | Long("enable_compression") => {
                let enable_compression = parser.value().unwrap().parse().unwrap();

                if let Some(CommandE::Pgn2cmbr(ref mut args)) = command {
                    args.enable_compression = enable_compression;
                }
            }

            Value(val) => {
                if command.is_none() {
                    let cmd = val.to_str().unwrap();

                    match cmd {
                        "cmbr2pgn" => {
                            command = Some(CommandE::Cmbr2pgn(Cmbr2PgnArgs {
                                input: String::new(),
                                output: String::new(),
                                threads_n: 1,
                            }));
                        }

                        "pgn2cmbr" => {
                            command = Some(CommandE::Pgn2cmbr(Pgn2CmbrArgs {
                                input: String::new(),
                                output: String::new(),
                                enable_compression: false,
                                zstd_compression_level: 9,
                                threads_n: 1,
                            }));
                        }

                        "license" => {
                            command = Some(CommandE::License);
                        }

                        _ => {
                            eprintln!("[ERROR] Unknown command: {}", cmd);
                            exit(1);
                        }
                    }
                }
            }

            _ => {
                eprintln!("[ERROR] Unknown argument: {:?}", arg);
                exit(1);
            }
        }
    }

    Cli { command }
}

fn validate_args(cli: &mut Cli) {
    if cli.command.is_none() {
        eprintln!("[ERROR] Expected a subcommand. Run `cmbrcc --help` for help.");
        exit(1);
    }

    match cli.command.as_mut().unwrap() {
        CommandE::Pgn2cmbr(args) => {
            if args.zstd_compression_level < 1 || args.zstd_compression_level > 22 {
                eprintln!("[ERROR] Zstd compression level should be between 1 and 22. Provided value is {}", args.zstd_compression_level);
                exit(1);
            }

            if args.threads_n == 0 {
                args.threads_n = available_parallelism().unwrap().get().try_into().unwrap();
            }

            if args.input.is_empty() {
                eprintln!("[ERROR] Expected an input file name\nRun `cmbrcc --help` for help.");
                exit(1);
            }
        }

        CommandE::Cmbr2pgn(args) => {
            if args.threads_n == 0 {
                args.threads_n = available_parallelism().unwrap().get().try_into().unwrap();
            }

            if args.input.is_empty() {
                eprintln!("[ERROR] Expected an input file name");
                exit(1);
            }
        }

        #[allow(unreachable_patterns)]
        _ => {}
    }
}

fn main() {
    let mut cli = parse_args();

    validate_args(&mut cli);
    eval_args::eval_args(&cli);
}
