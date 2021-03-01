use std::io::Write;
use std::str::FromStr;

use anyhow::{Context, ensure, Result};

use crate::{Cmd, RAM};
use crate::command::Command;

pub fn to_file(path: impl AsRef<std::path::Path>, cmds: RAM) -> Result<()> {
    use num_traits::FromPrimitive;

    let mut file = std::fs::File::create(path)?;

    for (cmd, value) in cmds {
        let cmd = Command::from_u16(cmd)
            .with_context(||
                format!("Fatal: to_file got `({0}, {1})`, but {0} is not a valid command!", cmd, value)
            )?;

        let line = format!("{:02} {:03}\t\t; {}\n", cmd as u8, value, cmd.to_string());
        file.write_all(line.as_bytes())?;
    }

    Ok(())
}

pub fn parse_file(path: impl AsRef<std::path::Path>, compile: bool) -> Result<RAM> {
    let file = std::fs::read_to_string(path)?;
    parse_document(&file, compile)
}

pub fn parse_document(s: &str, compile: bool) -> Result<RAM> {
    let mut vec = Vec::new();

    for (i, line) in s.lines().enumerate() {
        let cmd_value = parse_line(line, compile)
            .with_context(|| format!("Failed to parse line {}", i))?;

        if let Some((cmd, value)) = cmd_value {
            vec.push((cmd as u16, value))
        }
    }

    Ok(vec)
}

pub fn parse_line(s: &str, compile: bool) -> Result<Option<Cmd>> {
    let code = remove_comments(s);
    let (cmd, value) = match split_into_cmd_value(code)? {
        Some(cmd_value) => cmd_value,
        None => return Ok(None)
    };

    let cmd = parse_command(cmd, compile)?;
    let value = parse_value(value, cmd)?;

    Ok(Some((cmd, value)))
}


fn remove_comments(s: &str) -> &str {
    match s.split_once(';') {
        Some((code, _comment)) => code,
        None => s
    }
}

fn split_into_cmd_value(code: &str) -> Result<Option<(&str, Option<&str>)>> {
    let mut code_parts = code.split_ascii_whitespace();

    let cmd = match code_parts.next() {
        Some(cmd) => cmd,
        None => return Ok(None)
    };
    let value = code_parts.next();

    ensure!(code_parts.next().is_none(), "The line contains more then a command an a value!");

    Ok(Some((cmd, value)))
}

fn parse_command(s: &str, compile: bool) -> Result<Command> {
    use num_traits::FromPrimitive;

    if compile {
        Command::from_str(&s.to_uppercase())
            .with_context(|| format!("Failed to parse the command `{}`!", s))
    } else {
        Command::from_u8(s.parse::<u8>()?)
            .with_context(|| format!("Failed to parse the command `{}`!", s))
    }
}

fn parse_value(s: Option<&str>, cmd: Command) -> Result<i16> {
    let value = if cmd.takes_value() {
        s
            .with_context(|| format!("`{:?}` takes a value!", cmd))?
            .parse::<i16>()?
    } else {
        0
    };

    Ok(value)
}
