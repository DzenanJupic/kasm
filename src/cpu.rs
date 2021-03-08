use std::io::Write;

use byte_slice_cast::AsByteSlice;
use num_traits::FromPrimitive;

use crate::{DATA_REGISTERS, Error, IRS, RAM, Result, URS};
use crate::instruction::Instruction;
use crate::interrupt::Interrupt;

#[derive(Debug)]
pub struct CPU<W> {
    A: IRS,
    BZ: URS,
    Rx: [IRS; DATA_REGISTERS],

    ram: RAM,
    stdout: W,
}

#[derive(Clone, Debug)]
pub enum ExecResult {
    None,
    Ended,
    HitBreakPoint,
    Print(String),
}

impl<W> CPU<W>
    where W: Write {
    pub fn new(ram: RAM, stdout: W) -> Self {
        Self {
            A: 0,
            BZ: 0,
            Rx: [0; DATA_REGISTERS],
            ram,
            stdout,
        }
    }

    pub fn A(&self) -> IRS {
        self.A
    }

    pub fn BZ(&self) -> URS {
        self.BZ
    }

    pub fn Rx(&self) -> &[IRS; DATA_REGISTERS] {
        &self.Rx
    }

    pub fn ram(&self) -> &RAM {
        &self.ram
    }

    pub fn set_ram(&mut self, ram: RAM) {
        self.ram = ram;
    }

    pub fn set_BZ(&mut self, BZ: URS) {
        self.BZ = BZ;
    }

    pub fn step_to_breakpoint(&mut self, max_steps: u64) -> Result<()> {
        let mut steps = 0;

        loop {
            log::debug!("step | {:?}", self.BZ);
            match self.step()? {
                ExecResult::Ended | ExecResult::HitBreakPoint => break,
                ExecResult::Print(t) => self.println(t.as_bytes())?,
                _ => {
                    steps += 1;
                    if steps >= max_steps {
                        return Err(Error::TooManySteps(steps));
                    }
                }
            }
        }

        Ok(())
    }

    pub fn step_to_end(&mut self, max_steps: u64) -> Result<()> {
        let mut steps = 0;

        loop {
            match self.step()? {
                ExecResult::Ended => break,
                ExecResult::Print(t) => self.println(t.as_bytes())?,
                _ => {
                    steps += 1;
                    if steps >= max_steps {
                        return Err(Error::TooManySteps(steps));
                    }
                }
            }
        }

        Ok(())
    }

    pub fn step(&mut self) -> Result<ExecResult> {
        let (inst, value) = self.next_instruction()?;
        self.exec(inst, value)
    }

    fn next_instruction(&self) -> Result<(Instruction, IRS)> {
        let &(inst, value) = self.ram
            .get(self.BZ as usize)
            .ok_or(Error::NoMoreInstructions { BZ: self.BZ })?;

        let inst = Instruction::from_u64(inst)
            .ok_or(Error::InvalidInstruction { inst, BZ: self.BZ })?;

        Ok((inst, value))
    }

    pub fn exec(&mut self, inst: Instruction, value: IRS) -> Result<ExecResult> {
        use Instruction::*;

        match inst {
            LOAD => {
                self.A = self.get_rx(value)?;
                self.BZ += 1;
                Ok(ExecResult::None)
            }
            DLOAD => {
                self.A = value;
                self.BZ += 1;
                Ok(ExecResult::None)
            }
            STORE => {
                *self.get_rx_mut(value)? = self.A;
                self.BZ += 1;
                Ok(ExecResult::None)
            }
            ADD => self.calc(value, |a, rx| a.wrapping_add(rx)),
            SUB => self.calc(value, |a, rx| a.wrapping_sub(rx)),
            MULT => self.calc(value, |a, rx| a.wrapping_sub(rx)),
            DIV => {
                if value == 0 {
                    return Err(Error::DivideByZero { lhs: self.A, BZ: self.BZ });
                }
                self.calc(value, |a, rx| a.wrapping_div(rx))
            }
            JUMP => self.jump(value, |_| true),
            JGE => self.jump(value, |a| a >= 0),
            JGT => self.jump(value, |a| a > 0),
            JLE => self.jump(value, |a| a <= 0),
            JLT => self.jump(value, |a| a < 0),
            JEQ => self.jump(value, |a| a == 0),
            JNE => self.jump(value, |a| a != 0),
            END => {
                self.BZ += 1;
                Ok(ExecResult::Ended)
            }
            BP => {
                self.BZ += 1;
                Ok(ExecResult::HitBreakPoint)
            }
            NOOP => {
                self.BZ += 1;
                Ok(ExecResult::None)
            }
            INT => self.handle_interrupt(value)
        }
    }

    fn calc<F: FnOnce(IRS, IRS) -> IRS>(&mut self, i: IRS, op: F) -> Result<ExecResult> {
        self.A = op(self.A, self.Rx[self.check_rx_index(i)?]);
        self.BZ += 1;

        Ok(ExecResult::None)
    }

    fn jump<F: FnOnce(IRS) -> bool>(&mut self, addr: IRS, cond: F) -> Result<ExecResult> {
        if cond(self.A) {
            self.BZ = addr as URS;
        } else {
            self.BZ += 1;
        }

        Ok(ExecResult::None)
    }

    fn handle_interrupt(&mut self, int: IRS) -> Result<ExecResult> {
        use Interrupt::*;
        let int = Interrupt::from_i64(int)
            .ok_or(Error::InvalidInterrupt { int, BZ: self.BZ })?;

        let res = match int {
            Print => {
                let bytes = shorten_rx_to_last_val(&self.Rx).as_byte_slice();
                let string = String::from_utf8_lossy(bytes).to_string();
                Ok(ExecResult::Print(string))
            }
            PrintBytes => {
                let bytes = shorten_rx_to_last_val(&self.Rx).as_byte_slice();
                Ok(ExecResult::Print(format!("{:?}", bytes)))
            }
            DumpA => Ok(ExecResult::Print(self.A.to_string())),
            DumpBZ => Ok(ExecResult::Print(self.BZ.to_string())),
            DumpRx => Ok(ExecResult::Print(format!("{:?}", self.Rx))),
            DumpRam => Ok(ExecResult::Print(format!("{:?}", self.ram))),
        };

        self.BZ += 1;
        res
    }

    fn get_rx(&self, i: IRS) -> Result<IRS> {
        let i = self.check_rx_index(i)?;
        Ok(self.Rx[i])
    }

    fn get_rx_mut(&mut self, i: IRS) -> Result<&mut IRS> {
        let i = self.check_rx_index(i)?;
        Ok(&mut self.Rx[i])
    }

    fn check_rx_index(&self, i: IRS) -> Result<usize> {
        if !index_can_be_converted_to_usize(i) || i.is_negative() || !index_is_in_range(i) {
            Err(Error::InvalidRxIndex {
                i,
                len: DATA_REGISTERS,
                BZ: self.BZ,
            })
        } else {
            Ok(i as usize)
        }
    }

    fn println(&mut self, bytes: &[u8]) -> Result<()> {
        self.stdout.write_all(bytes)?;
        self.stdout.write_all(b"\n")?;

        Ok(())
    }
}

fn index_can_be_converted_to_usize(i: IRS) -> bool {
    ((i as usize) as IRS) == i
}

fn index_is_in_range(i: IRS) -> bool {
    0 <= i && (i as usize) < DATA_REGISTERS
}

fn shorten_rx_to_last_val(rx: &[IRS]) -> &[IRS] {
    let index = rx
        .iter()
        .enumerate()
        .rev()
        .find(|(_i, &val)| val != 0)
        .map(|(i, _val)| i)
        .unwrap_or(9);

    &rx[..=index]
}
