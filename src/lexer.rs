use std::collections::HashMap;
use std::io::Write;
use std::str::FromStr;

use anyhow::{Context, ensure, Result};

use crate::{Inst, RAM};
use crate::instruction::Instruction;

enum InstOrFunction<'a> {
    Inst(Inst),
    Function(&'a str),
}

pub fn to_file(path: impl AsRef<std::path::Path>, ram: RAM) -> Result<()> {
    use num_traits::FromPrimitive;

    let mut file = std::fs::File::create(path)?;

    for (cmd, value) in ram {
        let cmd = Instruction::from_u16(cmd)
            .with_context(||
                format!("Fatal: [to_file] got `({0}, {1})`, but {0} is not a valid instruction!", cmd, value)
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
    let mut ram = Vec::new();
    // todo: use fn_names to resolve jmp instructions
    let mut fn_names = HashMap::new();

    for (i, line) in s.lines().enumerate() {
        let inst_or_fn = parse_line_or_function(line, compile)
            .with_context(|| format!("Failed to parse line {}", i))?;

        if let Some(inst_or_fn) = inst_or_fn {
            match inst_or_fn {
                InstOrFunction::Inst((inst, value)) => {
                    ram.push((inst as u16, value));
                }
                InstOrFunction::Function(fn_name) => {
                    fn_names.insert(fn_name, ram.len());
                    ram.push((Instruction::NOOP as u16, 0));
                }
            }
        }
    }

    Ok(ram)
}

pub fn parse_line(s: &str, compile: bool) -> Result<Option<Inst>> {
    let code = remove_comments(s);
    let (cmd, value) = match split_into_cmd_value(code)? {
        Some(cmd_value) => cmd_value,
        None => return Ok(None)
    };

    let cmd = parse_instruction(cmd, compile)?;
    let value = parse_value(value, cmd)?;

    Ok(Some((cmd, value)))
}

fn parse_line_or_function(s: &str, compile: bool) -> Result<Option<InstOrFunction>> {
    let code = remove_comments(s);

    match split_into_function(code) {
        Ok(Some(func)) => {
            if let Ok(name) = parse_function(func) {
                return Ok(Some(InstOrFunction::Function(name)))
            }
        }
        Ok(None) => return Ok(None),
        _ => {}
    }

    parse_line(s, compile)
        .map(|opt| {
            opt.map(InstOrFunction::Inst)
        })
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

    ensure!(code_parts.next().is_none(), "The line contains more then an instruction an a value!");

    Ok(Some((cmd, value)))
}

fn split_into_function(s: &str) -> Result<Option<&str>> {
    let mut parts = s.split_ascii_whitespace();

    let function = match parts.next() {
        Some(func) => func,
        None => return Ok(None)
    };

    ensure!(parts.next().is_none(), "The line contains morethen an function declaration!");

    Ok(Some(function))
}

fn parse_instruction(s: &str, compile: bool) -> Result<Instruction> {
    use num_traits::FromPrimitive;

    if compile {
        Instruction::from_str(&s.to_uppercase())
            .with_context(|| format!("Failed to parse the instruction `{}`!", s))
    } else {
        Instruction::from_u8(s.parse::<u8>()?)
            .with_context(|| format!("Failed to parse the instruction `{}`!", s))
    }
}

fn parse_value(s: Option<&str>, inst: Instruction) -> Result<i16> {
    let value = if inst.takes_value() {
        s
            .with_context(|| format!("`{:?}` takes a value!", inst))?
            .parse::<i16>()?
    } else {
        0
    };

    Ok(value)
}

fn parse_function(s: &str) -> Result<&str> {
    ensure!(s.starts_with('.'), "Function declarations must start with a `.`");
    ensure!(s.ends_with(':'), "Function declarations must end with a `:`");
    ensure!(s.chars().count() > 2, "Function names must contain at least one char");

    s
        .get(1..s.len() - 1)
        .context("Invalid function name")
}
