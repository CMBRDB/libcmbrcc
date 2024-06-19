// TODO(#3): Define and Write documentation for the CMBR Standard.
// TODO(#6): Seperate the cli and libcmbr
// TODO(#12): Implement turing the tokens into an AST

mod eval_args;
mod pgn;
mod tests;
mod utils;

use clap::{Args, Command, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Generator, Shell};
use std::io::IsTerminal;
use std::process::exit;
use std::thread::available_parallelism;

use std::panic;

#[derive(Subcommand, Clone, Debug, PartialEq, Eq)]
pub enum CommandE {
    Cmbr2pgn(Cmbr2PgnArgs),
    Pgn2cmbr(Pgn2CmbrArgs),
    License,
}

#[derive(Args, Debug, Clone, PartialEq, Eq)]
pub struct Cmbr2PgnArgs {
    input_file: String,

    #[arg(short, long)]
    output: String,

    #[arg(
        short = 'T',
        default_value = "1",
        help = "Uses # threads. Pass 0 to use all"
    )]
    threads_n: u16,
}

#[derive(Args, Debug, Clone, PartialEq, Eq)]
pub struct Pgn2CmbrArgs {
    input: String,

    #[arg(short, long)]
    output: String,

    #[arg(
        short = 'c',
        long,
        default_value = "false",
        help = "Enable Zstandard compression."
    )]
    enable_compression: bool,

    #[arg(
        long,
        default_value = "9",
        help = "Specifies Zstandard compression level. (Ranges 1 to 22)",
        required = false
    )]
    zstd_compression_level: u8,

    #[arg(
        short = 'T',
        default_value = "1",
        help = "Uses <THREADS_N> threads. Pass 0 to use all",
        required = false
    )]
    threads_n: u16,
}

#[derive(Parser, Debug, Clone, PartialEq, Eq)]
#[command(author, version, about, long_about = None, name="cmbrcc", arg_required_else_help = true)]
pub struct Cli {
    #[arg(long = "generate", value_enum)]
    generator: Option<Shell>,

    #[command(subcommand)]
    command: Option<CommandE>,
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
}

fn validate_args(cli: &mut Cli) {
    match cli.command.as_mut().unwrap() {
        CommandE::Pgn2cmbr(args) => {
            if args.zstd_compression_level < 1 || args.zstd_compression_level > 22 {
                eprintln!("[ERROR] Zstd compression level should be between 1 and 22. Provided value is {}", args.zstd_compression_level);
                exit(1);
            }

            if args.threads_n == 0 {
                args.threads_n = available_parallelism().unwrap().get().try_into().unwrap();
            }
        }

        CommandE::Cmbr2pgn(args) => {
            if args.threads_n == 0 {
                args.threads_n = available_parallelism().unwrap().get().try_into().unwrap();
            }
        }

        #[allow(unreachable_patterns)]
        _ => {}
    }
}

fn main() {
    let mut cli = Cli::parse();

    if let Some(generator) = cli.generator {
        let mut cmd = Cli::command();

        eprintln!("Generating completion file for {generator:?}...");
        print_completions(generator, &mut cmd);

        return;
    }

    validate_args(&mut cli);
    eval_args::eval_args(&cli);
}
