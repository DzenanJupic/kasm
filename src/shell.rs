use std::io::Write;

use clap::Clap;
use strum::VariantNames;

use crate::cpu::CPU;

macro_rules! try_continue {
    ($res:expr) => { try_continue!($res => continue;); };
    ($res:expr => $($ret:tt)+) => {
        match $res {
            Ok(val) => val,
            Err(e) => {
                println!("Error: {}", e);
                $($ret)+
            }
        }
    };
}

type IsShellCmd = bool;
type IsEnded = bool;

#[derive(Clap)]
#[clap(about = "A klett asm shell")]
enum RunTimeCmd {
    #[clap(about = "Display the runtime command help")]
    Help,
    #[clap(about = "Display all available interrupts")]
    Interrupts,
    #[clap(about = "Display all available instructions")]
    Instructions,
    #[clap(about = "Switch to run-only, if currently not in that mode, otherwise turn it of")]
    ToggleRunOnly,
    #[clap(about = "Tells weather you're in run-only mode")]
    IsRunOnly,
    #[clap(about = "Exits the shell")]
    End,
}

impl RunTimeCmd {
    fn parse(cmd: &str) -> Result<Self, clap::Error> {
        let cmd = Self::try_parse_from(&["", &cmd[1..]]);

        match cmd {
            Ok(cmd) => Ok(cmd),
            Err(e) if e.kind == clap::ErrorKind::DisplayHelp => Ok(RunTimeCmd::Help),
            Err(e) => Err(e)
        }
    }

    fn exec(self, args: &mut crate::Args) -> Result<IsEnded, clap::Error> {
        match self {
            RunTimeCmd::Help => {
                use clap::IntoApp;
                RunTimeCmd::into_app().print_help()?;
            }
            RunTimeCmd::Interrupts => {
                for (i, variant) in crate::interrupt::Interrupt::VARIANTS.iter().enumerate() {
                    println!("{:02}: {}", i, variant);
                }
            }
            RunTimeCmd::Instructions => {
                for (i, variant) in crate::command::Command::VARIANTS.iter().enumerate() {
                    println!("{:02}: {}", i, variant);
                }
            },
            RunTimeCmd::ToggleRunOnly => args.run_only = !args.run_only,
            RunTimeCmd::IsRunOnly => println!("{}", args.run_only),
            RunTimeCmd::End => return Ok(true)
        }

        Ok(false)
    }
}

pub fn run_shell(mut args: crate::Args, mut cpu: CPU) {
    let mut buffer = String::new();

    loop {
        let is_shell_cmd = try_continue!(read_line_to_buffer(&mut buffer));

        if is_shell_cmd {
            let cmd = try_continue!(RunTimeCmd::parse(&buffer));
            let is_ended = try_continue!(cmd.exec(&mut args));

            if is_ended { break; }
        } else {
            let cmd_value = try_continue!(crate::lexer::parse_line(&buffer, !args.run_only));

            if let Some((cmd, value)) = cmd_value {
                let ended = try_continue!(cpu.exec(cmd, value));
                if ended { break; }
            }
        }
    }
}

fn read_line_to_buffer(buffer: &mut String) -> std::io::Result<IsShellCmd> {
    buffer.clear();
    print!("\n> ");
    std::io::stdout().flush()?;
    std::io::stdin().read_line(buffer)?;

    if buffer.starts_with('/') {
        buffer.pop();                                       // pop \n
        if buffer.ends_with('\r') { buffer.pop(); }     // pop \r

        Ok(true)
    } else {
        Ok(false)
    }
}
