use anyhow::Result;
use byte_slice_cast::AsByteSlice;
use num_traits::FromPrimitive;

use crate::command::Command;
use crate::error::Error;
use crate::interrupt::Interrupt;
use crate::RAM;

type Ended = bool;

#[derive(Clone, Debug)]
pub struct CPU {
    BZ: u16,
    SR: i16,
    A: i16,
    Rx: [i16; u16::MAX as usize],
    ram: RAM,
}

impl CPU {
    pub fn with_ram(ram: RAM) -> Self {
        Self {
            BZ: 0,
            SR: 0,
            A: 0,
            Rx: [0; u16::MAX as usize],
            ram,
        }
    }

    pub fn step_to_end(&mut self) -> Result<()> {
        if !self.ram.is_empty() {
            while !self.step()? {}
        }

        Ok(())
    }

    pub fn step(&mut self) -> Result<Ended> {
        let (cmd, value) = self.next_cmd()?;
        self.exec(cmd, value)
    }

    pub fn exec(&mut self, cmd: Command, value: i16) -> Result<Ended> {
        use Command::*;

        let mut ended = false;

        match cmd {
            LOAD => {
                self.A = self.Rx[clip(value) as usize];
                self.SR = 0;
            }
            DLOAD => {
                self.A = value;
                self.SR = 0;
            }
            STORE => {
                self.Rx[clip(value) as usize] = self.A;
                self.SR = 0;
            }
            ADD => self.calc(value, |lhs, rhs| lhs.overflowing_add(rhs)),
            SUB => self.calc(value, |lhs, rhs| lhs.overflowing_sub(rhs)),
            MULT => self.calc(value, |lhs, rhs| lhs.overflowing_mul(rhs)),
            DIV => {
                if self.Rx[clip(value) as usize] == 0 {
                    self.SR = -1;
                    self.A = i16::MAX;
                    anyhow::bail!(Error::DivideByZero);
                } else {
                    self.calc(value, |lhs, rhs| (lhs / rhs, false));
                    self.SR = 0;
                }
            }
            JUMP => self.jmp(value, |_| true),
            JGE => self.jmp(value, |cpu| cpu.A >= 0),
            JGT => self.jmp(value, |cpu| cpu.A > 0),
            JLE => self.jmp(value, |cpu| cpu.A <= 0),
            JLT => self.jmp(value, |cpu| cpu.A < 0),
            JEQ => self.jmp(value, |cpu| cpu.A == 0),
            JNE => self.jmp(value, |cpu| cpu.A != 0),
            END => ended = true,

            NOOP => self.SR = 0,
            INT => self.handle_interrupt(value as u16)?,
        }

        Ok(ended)
    }

    fn next_cmd(&mut self) -> Result<(Command, i16)> {
        let (cmd, value) = self.ram
            .get(self.BZ as usize)
            .copied()
            .ok_or(Error::NoMoreInstructions { BZ: self.BZ })?;

        let cmd = Command::from_u16(cmd)
            .ok_or(Error::InvalidCommand { cmd, BZ: self.BZ })?;

        self.BZ = self.BZ.wrapping_add(1);
        Ok((cmd, value))
    }

    fn calc<F: FnOnce(i16, i16) -> (i16, bool)>(&mut self, value: i16, func: F) {
        let (res, overflowed) = func(self.A, self.Rx[clip(value) as usize]);
        self.A = res;

        if overflowed {
            self.SR = 1;
        } else {
            if self.A.is_negative() {
                self.SR = -1;
            } else {
                self.SR = 0;
            }
        }
    }

    #[inline]
    fn jmp<F: FnOnce(&CPU) -> bool>(&mut self, value: i16, cond: F) {
        if cond(self) {
            self.BZ = value as u16;
            self.SR = 0;
        } else {
            self.SR = -1;
        }
    }

    fn handle_interrupt(&mut self, int: u16) -> Result<()> {
        use Interrupt::*;

        let int: Interrupt = Interrupt::from_u16(int)
            .ok_or(Error::InvalidInterrupt { int, BZ: self.BZ })?;

        match int {
            Print => {
                let values = shorten_to_last_val(self.Rx.as_slice());
                let bytes = values.as_byte_slice();
                let string = String::from_utf8_lossy(bytes);

                println!("{}", string);
            }
            PrintBytes => {
                let values = shorten_to_last_val(self.Rx.as_slice());
                let bytes = values.as_byte_slice();
                println!("{:?}", bytes);
            },
            DumpRegisters => println!(
                "BZ: {}\nSR: {}\nA: {}\nRx: {:?}\nram: {:?}",
                self.BZ, self.SR, self.A, shorten_to_last_val(self.Rx.as_slice()), self.ram
            ),
            StoreRam => {
                let index = clip(self.A) as usize;

                if self.ram.len() <= index {
                    self.ram.extend(
                        (0..=index - self.ram.len()).map(|_| (Command::NOOP as u16, 0))
                    );
                }

                self.ram[index] = (self.Rx[0] as u16, self.Rx[1]);
            }
            StoreBZ => self.BZ = self.A as u16,
            Step => {
                let (cmd, value) = self.next_cmd()?;
                let ended = self.exec(cmd, value)?;
                anyhow::ensure!(!ended, Error::EndedInInterrupt);
            }
            Exec => {
                self.step_to_end()?;
            }
            ClearRam => self.ram.clear(),
            Clear => {
                self.BZ = 0;
                self.A = 0;
                self.SR = 0;
                self.Rx = [0; u16::MAX as usize];
                self.ram.clear();
            }
        }

        Ok(())
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            BZ: 0,
            SR: 0,
            A: 0,
            Rx: [0; u16::MAX as usize],
            ram: Vec::default(),
        }
    }
}

#[inline]
fn clip(value: i16) -> u16 {
    (value as u16).min(u16::MAX - 1)
}

fn shorten_to_last_val(slice: &[i16]) -> &[i16] {
    let last_val = slice
        .iter()
        .enumerate()
        .rev()
        .find(|(_i, &v)| v != 0)
        .map(|(i, _v)| i)
        .unwrap_or(0)
        .max(9);

    &slice[..=last_val as usize]
} 
