use std::convert::TryFrom;
use std::fmt::{Debug, Display};
use std::io::Write;
use std::num::NonZeroUsize;

use byte_slice_cast::{AsByteSlice, ToByteSlice};
use num_traits::{AsPrimitive, FromPrimitive, Num, Signed};

use crate::{DATA_REGISTERS, Error, RAM, URS};
use crate::instruction::Instruction;
use crate::interrupt::Interrupt;

#[derive(Debug)]
pub struct CPU<IRS, W> {
    A: IRS,
    BZ: URS,
    Rx: [IRS; DATA_REGISTERS],

    ram: RAM<IRS>,
    stdout: W,
}

#[derive(Clone, Debug)]
pub enum ExecResult {
    None,
    Ended,
    HitBreakPoint,
    Print(String),
    NotFinished
}

impl<IRS, W> CPU<IRS, W>
    where IRS: Default + Copy {
    pub fn new(ram: RAM<IRS>, stdout: W) -> Self {
        Self {
            A: IRS::default(),
            BZ: 0,
            Rx: [IRS::default(); DATA_REGISTERS],
            ram,
            stdout,
        }
    }

    pub fn reset_registers(&mut self) {
        self.A = IRS::default();
        self.BZ = 0;
        self.Rx = [IRS::default(); DATA_REGISTERS];
    }
}

impl<IRS, W> CPU<IRS, W>
    where IRS: Copy {
    pub fn A(&self) -> IRS {
        self.A
    }

    pub fn BZ(&self) -> URS {
        self.BZ
    }
}

impl<IRS, W> CPU<IRS, W> {
    pub fn Rx(&self) -> &[IRS; DATA_REGISTERS] {
        &self.Rx
    }

    pub fn ram(&self) -> &RAM<IRS> {
        &self.ram
    }

    pub fn stdout(&self) -> &W {
        &self.stdout
    }

    pub fn BZ_mut(&mut self) -> &mut URS {
        &mut self.BZ
    }

    pub fn ram_mut(&mut self) -> &mut RAM<IRS> {
        &mut self.ram
    }
}


impl<IRS, W> CPU<IRS, W>
    where
        IRS: Debug + Display + PartialOrd + ToByteSlice,
        IRS: Num + Signed + AsPrimitive<URS> + AsPrimitive<usize>,
        W: Write,
        usize: AsPrimitive<IRS>,
        Instruction: TryFrom<IRS>,
        Interrupt: TryFrom<IRS> {
    pub fn step_to_breakpoint(&mut self, max_steps: NonZeroUsize) -> Result<ExecResult, Error<IRS>> {
        for _ in 0..max_steps.get() {
            match self.step()? {
                res @ ExecResult::Ended |
                res @ ExecResult::HitBreakPoint => return Ok(res),
                ExecResult::Print(t) => self.println(&t)?,
                _ => {}
            }
        }

        Ok(ExecResult::NotFinished)
    }

    pub fn step_to_end(&mut self, max_steps: NonZeroUsize) -> Result<ExecResult, Error<IRS>> {
        for _ in 0..max_steps.get() {
            match self.step()? {
                res @ ExecResult::Ended => return Ok(res),
                ExecResult::Print(t) => self.println(&t)?,
                _ => {}
            }
        }

        Ok(ExecResult::NotFinished)
    }

    pub fn step(&mut self) -> Result<ExecResult, Error<IRS>> {
        let (inst, value) = self.next_instruction()?;
        self.exec(inst, value)
    }

    fn next_instruction(&self) -> Result<(Instruction, IRS), Error<IRS>> {
        let &(inst, value) = self.ram
            .get(self.BZ as usize)
            .ok_or(Error::NoMoreInstructions { BZ: self.BZ })?;

        let inst = Instruction::from_usize(inst)
            .ok_or(Error::InvalidInstruction { inst, BZ: self.BZ })?;

        Ok((inst, value))
    }

    pub fn exec(&mut self, inst: Instruction, value: IRS) -> Result<ExecResult, Error<IRS>> {
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
            ADD => self.calc(value, |a, rx| a + rx),
            SUB => self.calc(value, |a, rx| a - rx),
            MULT => self.calc(value, |a, rx| a * rx),
            DIV => {
                if self.get_rx(value)? == IRS::zero() {
                    return Err(Error::DivideByZero { lhs: self.A, BZ: self.BZ });
                }
                self.calc(value, |a, rx| a / rx)
            }
            JUMP => self.jump(value, |_| true),
            JGE => self.jump(value, |a| a >= IRS::zero()),
            JGT => self.jump(value, |a| a > IRS::zero()),
            JLE => self.jump(value, |a| a <= IRS::zero()),
            JLT => self.jump(value, |a| a < IRS::zero()),
            JEQ => self.jump(value, |a| a == IRS::zero()),
            JNE => self.jump(value, |a| a != IRS::zero()),
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

    fn calc<F: FnOnce(IRS, IRS) -> IRS>(&mut self, i: IRS, op: F) -> Result<ExecResult, Error<IRS>> {
        self.A = op(self.A, self.Rx[self.check_rx_index(i)?]);
        self.BZ += 1;

        Ok(ExecResult::None)
    }

    fn jump<F: FnOnce(IRS) -> bool>(&mut self, addr: IRS, cond: F) -> Result<ExecResult, Error<IRS>> {
        if cond(self.A) {
            self.BZ = addr.as_();
        } else {
            self.BZ += 1;
        }

        Ok(ExecResult::None)
    }

    fn handle_interrupt(&mut self, int: IRS) -> Result<ExecResult, Error<IRS>> {
        use Interrupt::*;
        let int = Interrupt::try_from(int)
            .map_err(|_| Error::InvalidInterrupt { int, BZ: self.BZ })?;

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

    fn get_rx(&self, i: IRS) -> Result<IRS, Error<IRS>> {
        let i = self.check_rx_index(i)?;
        Ok(self.Rx[i])
    }

    fn get_rx_mut(&mut self, i: IRS) -> Result<&mut IRS, Error<IRS>> {
        let i = self.check_rx_index(i)?;
        Ok(&mut self.Rx[i])
    }

    fn check_rx_index(&self, i: IRS) -> Result<usize, Error<IRS>> {
        if !index_can_be_converted_to_usize(i) || i.is_negative() || !index_is_in_range(i) {
            Err(Error::InvalidRxIndex {
                i,
                len: DATA_REGISTERS,
                BZ: self.BZ,
            })
        } else {
            Ok(i.as_())
        }
    }

    pub fn println(&mut self, s: &str) -> Result<(), Error<IRS>> {
        writeln!(self.stdout, "{}", s)?;
        Ok(())
    }
}

fn index_can_be_converted_to_usize<IRS>(i: IRS) -> bool
    where
        IRS: AsPrimitive<usize> + PartialEq,
        usize: AsPrimitive<IRS> {
    let us: usize = i.as_();
    let irs: IRS = us.as_();
    irs == i
}

fn index_is_in_range<IRS: Num + PartialOrd + AsPrimitive<usize>>(i: IRS) -> bool {
    IRS::zero() <= i && (i.as_()) < DATA_REGISTERS
}

fn shorten_rx_to_last_val<IRS: Num + Copy>(rx: &[IRS]) -> &[IRS] {
    let index = rx
        .iter()
        .enumerate()
        .rev()
        .find(|(_i, &val)| val != IRS::zero())
        .map(|(i, _val)| i)
        .unwrap_or(9);

    &rx[..=index]
}
