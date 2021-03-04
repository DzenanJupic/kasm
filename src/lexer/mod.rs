use std::collections::HashMap;

use code_token::CodeToken;

use crate::{Error, IRS, RAM, Result};
use crate::instruction::Instruction;
use crate::lexer::code_line::CodeLine;
use crate::lexer::jump_point::JumpPoint;

pub mod code_token;
pub mod code_line;
pub mod jump_point;

type CodeLineIndex = usize;

#[derive(Debug)]
pub struct Document {
    code_lines: Vec<(CodeLineIndex, CodeLine)>
}

impl Document {
    pub fn as_ram(&self) -> RAM {
        self.code_lines
            .iter()
            .map(|(_, cl)| cl.as_urs_irs())
            .collect()
    }

    pub fn from_str(s: &str) -> Result<Self> {
        let mut doc = Self::parse(s)?;

        doc.check()?;
        doc.resolve_jump_points()?;

        Ok(doc)
    }

    fn parse(s: &str) -> Result<Self> {
        let mut code_lines = Vec::new();

        for (i, line) in s.lines().enumerate() {
            let line_i = i + 1;

            let code_line = CodeLine::from_str(line)
                .map_err(|err| Error::ParsingFailed { s: line.to_owned(), line: line_i, err })?;

            if let Some(code_line) = code_line {
                code_lines.push((line_i, code_line));
            }
        }

        Ok(Self {
            code_lines
        })
    }

    fn check(&self) -> Result<()> {
        for &(i, ref code_line) in self.code_lines.iter() {
            code_line
                .check()
                .map_err(|err| Error::InvalidTokenArrangement {
                    line: i,
                    err,
                })?;
        }

        Ok(())
    }

    fn resolve_jump_points(&mut self) -> Result<()> {
        let jump_point_declarations = self.get_jump_point_declarations();

        for &mut (i, ref mut cl) in self.code_lines.iter_mut() {
            if let CodeLine::DoubleToken(_, ct @ CodeToken::JumpPoint(_)) = cl {
                let val;
                match ct {
                    CodeToken::JumpPoint(jp) => {
                        val = jump_point_declarations.get(jp.as_ref())
                            .ok_or(Error::UndefinedJumpPoint { name: jp.as_ref().to_owned(), line: i })?;
                    }
                    _ => unreachable!(),
                }
                *ct = CodeToken::Val(*val as IRS);
            }
        }

        Ok(())
    }

    fn get_jump_point_declarations(&mut self) -> HashMap<String, usize> {
        self.code_lines
            .iter_mut()
            .enumerate()
            .filter_map(|(i, (_, cl))| {
                match cl {
                    CodeLine::SingleToken(ct @ CodeToken::JumpPointDeclaration(_)) => {
                        let ct = std::mem::replace(ct, CodeToken::Inst(Instruction::NOOP));
                        match ct {
                            CodeToken::JumpPointDeclaration(JumpPoint(jp)) => Some((jp, i)),
                            _ => unreachable!()
                        }
                    }
                    _ => None
                }
            })
            .collect::<HashMap<String, usize>>()
    }
}
