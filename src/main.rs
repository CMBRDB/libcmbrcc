mod eval_args;
mod utils;

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
    table_mem_limit: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pgn2CmbrArgs {
    input: String,
    output: String,
    enable_compression: bool,
    zstd_compression_level: u8,
    threads_n: u16,
    table_mem_limit: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cli {
    command: Option<CommandE>,
}

#[derive(Debug)]
enum ParseMemoryError {
    InvalidFormat,
    UnknownUnit,
}

fn parse_memory_amount(s: &str) -> Result<u64, ParseMemoryError> {
    let pos = s
        .find(|c: char| !c.is_digit(10))
        .ok_or(ParseMemoryError::InvalidFormat)?;

    let (number_str, unit_str) = s.split_at(pos);
    let number: u64 = number_str
        .parse()
        .map_err(|_| ParseMemoryError::InvalidFormat)?;

    let bytes = match unit_str {
        "G" | "GB" => number * 1024 * 1024 * 1024,
        "M" | "MB" => number * 1024 * 1024,
        "K" | "KB" => number * 1024,
        _ => return Err(ParseMemoryError::UnknownUnit),
    };

    Ok(bytes)
}

fn print_usage() {
    // TODO(#20): A better usage/help command
    println!("\nUsage: cmbr {{COMMAND}} [OPTIONS]");
    println!("note: Options inside of square brackets ([]) are optional\n");
    println!("Commands:");
    println!("  cmbr2pgn --input {{INPUT_FILE}} [--output {{OUTPUT_FILE}} --threads_n {{AMOUNT_OF_THREADS}} --enable_compression --table-memory-limit {{LIMIT}} ]");
    println!("  pgn2cmbr  --input {{INPUT_FILE}} [--output {{OUTPUT_FILE}} --threads_n {{AMOUNT_OF_THREADS}} --table-memory-limit {{LIMIT}} ]");
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

            Short('T') | Long("amount-of-threads") => {
                let threads_n = parser.value().unwrap().parse();

                if threads_n.is_err() {
                    eprintln!("Invalid thread amount. Run `cmbrcc --help` for help.");
                    std::process::exit(1);
                }

                let threads_n = threads_n.unwrap();

                if let Some(CommandE::Cmbr2pgn(ref mut args)) = command {
                    args.threads_n = threads_n;
                } else if let Some(CommandE::Pgn2cmbr(ref mut args)) = command {
                    args.threads_n = threads_n;
                } else {
                    eprintln!("Invalid option --amount-of-threads for this subcommand. Run `cmbrcc --help` for help.");
                    std::process::exit(1);
                }
            }

            Short('M') | Long("table-memory-limit") => {
                let table_memory_limit =
                    parse_memory_amount(&parser.value().unwrap().into_string().unwrap());

                if table_memory_limit.is_err() {
                    eprintln!("Invalid memory limit amount. Expected values like this: 2GB, 1G, 1024MB, 5M, 1024KB, 5K. Run `cmbrcc --help` for help.");
                    std::process::exit(1);
                }

                if let Some(CommandE::Cmbr2pgn(ref mut args)) = command {
                    args.table_mem_limit = table_memory_limit.unwrap();
                } else if let Some(CommandE::Pgn2cmbr(ref mut args)) = command {
                    args.table_mem_limit = table_memory_limit.unwrap();
                } else {
                    eprintln!("Invalid option --table-memory-limit for this subcommand. Run `cmbrcc --help` for help.");
                    std::process::exit(1);
                }
            }

            Short('o') | Long("output") => {
                let output = parser.value().unwrap().into_string().unwrap();

                if let Some(CommandE::Cmbr2pgn(ref mut args)) = command {
                    args.output = output;
                } else if let Some(CommandE::Pgn2cmbr(ref mut args)) = command {
                    args.output = output;
                } else {
                    eprintln!("Invalid option --output for this subcommand. Run `cmbrcc --help` for help.");
                    std::process::exit(1);
                }
            }

            Short('i') | Long("input") => {
                let input = parser.value().unwrap().into_string().unwrap();

                if let Some(CommandE::Cmbr2pgn(ref mut args)) = command {
                    args.input = input.clone();
                } else if let Some(CommandE::Pgn2cmbr(ref mut args)) = command {
                    args.input = input.clone();
                } else {
                    eprintln!(
                        "Invalid option --input for this subcommand. Run `cmbrcc --help` for help."
                    );
                    std::process::exit(1);
                }
            }

            Short('c') | Long("enable-compression") => {
                let enable_compression = parser.value().unwrap().parse();

                if enable_compression.is_err() {
                    eprintln!("Invalid option for enable-compression (Expected `true` or `false`). Run `cmbrcc --help` for help.");
                    std::process::exit(1);
                }

                if let Some(CommandE::Pgn2cmbr(ref mut args)) = command {
                    args.enable_compression = enable_compression.unwrap();
                } else {
                    eprintln!("Invalid option --enable-compression for this subcommand. Run `cmbrcc --help` for help.");
                    std::process::exit(1);
                }
            }

            Value(val) => {
                if command.is_none() {
                    let cmd = val.to_str().unwrap();
                    let mem = utils::get_free_memory();

                    let mem = if mem.is_none() {
                        // 1MB
                        1048576
                    } else {
                        mem.unwrap() * 1024 / 8
                    };

                    match cmd {
                        "cmbr2pgn" => {
                            command = Some(CommandE::Cmbr2pgn(Cmbr2PgnArgs {
                                input: String::new(),
                                output: String::new(),
                                threads_n: 1,
                                table_mem_limit: mem,
                            }));
                        }

                        "pgn2cmbr" => {
                            command = Some(CommandE::Pgn2cmbr(Pgn2CmbrArgs {
                                input: String::new(),
                                output: String::new(),
                                enable_compression: false,
                                zstd_compression_level: 9,
                                threads_n: 1,
                                table_mem_limit: 0,
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
