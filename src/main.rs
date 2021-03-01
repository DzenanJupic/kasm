#![feature(str_split_once, core_intrinsics, intrinsics)]
#![feature(array_methods)]
#![allow(non_snake_case)]

use std::path::PathBuf;

use clap::Clap;

use crate::command::Command;
use crate::cpu::CPU;

mod command;
mod cpu;
mod interrupt;
mod lexer;
mod error;
mod shell;

type RAM = Vec<RawCmd>;
type Cmd = (Command, i16);
type RawCmd = (u16, i16);


#[derive(Clap)]
#[clap(version = "0.1.0", author = "Dzenan Jupic <info@dzenanjupic.de>", about = "A klett asm compiler")]
pub struct Args {
    #[clap(about = "The file to run", required_unless_present = "interactive")]
    file: Option<PathBuf>,
    #[clap(short = 'C', long, about = "Only compile the file, without running it", conflicts_with = "interactive")]
    compile_only: bool,
    #[clap(short = 'R', long, about = "Only run the file, without compiling it")]
    run_only: bool,
    #[clap(short = 's', long, about = "Don't run the commands, just load them into RAM", requires = "interactive", conflicts_with = "compile-only")]
    step: bool,
    #[clap(short, long, about = "Open an interactive shell (after running the file)")]
    interactive: bool,
}

fn main() -> anyhow::Result<()> {
    let args: Args = Args::parse();

    let mut cpu = match args.file {
        Some(ref path) => file_to_cpu(path, &args)?,
        None => cpu::CPU::default()
    };

    if !args.compile_only && !args.step {
        cpu.step_to_end()?;
    }

    if args.interactive {
        shell::run_shell(args, cpu);
    }

    Ok(())
}

fn file_to_cpu(path: &PathBuf, args: &Args) -> anyhow::Result<CPU> {
    let cmds = lexer::parse_file(&path, !args.run_only)?;

    if !args.run_only {
        lexer::to_file(path.with_extension("kbin"), cmds.clone())?;
    }

    Ok(cpu::CPU::with_ram(cmds))
}
