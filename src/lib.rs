//lib.rs
pub mod constants;
use crate::constants::*;

#[derive(Debug,PartialEq)]
enum Instruction {
    Nop,
    Ldi {dest: u8,k: u8},
    Add {dest: u8,src: u8},
    Unknown(u16)
}

fn decode(word: u16) -> Instruction {
    match word {
        0x0000 => Instruction::Nop,
        w if w & 0xF000 == 0xE000 => {
            Instruction::Ldi { 
                dest: ((w >> 4) & 0x000F) as u8 + 16, //R16 to R31
                k: (((w >> 4) & 0x00F0) | (w & 0x000F)) as u8
            }
        },
        w if w & 0xFC00 == 0x0C00 => {
            Instruction::Add { 
                dest: ((w >> 4) & 0x001F) as u8,
                src:  (((w >> 5) & 0x0010) | (w & 0x000F)) as u8
            }
        }
        _ => Instruction::Unknown(word)
    }
}

pub struct Cpu {
    flash : [u16;FLASH_SIZE],
    sram  : [u8;SRAM_SIZE],
    registers : [u8;CPU_REGISTERS_SIZE],
    pc : u16,
    sp : u16,
    sreg: u8,
    io: [u8; IO_SIZE]
}

impl Cpu {
    pub fn new() -> Self {
        Self { 
            flash: [0;FLASH_SIZE], 
            sram: [0;SRAM_SIZE], 
            registers: [0;CPU_REGISTERS_SIZE], 
            pc: 0, 
            sp: 0, 
            sreg: 0, 
            io: [0; IO_SIZE]
        }
    }

    fn write_data(&mut self,address: u16,value: u8) -> Result<(),String> {
        match address {
            REGISTERS_START..=REGISTERS_END => {
                self.registers[address as usize] = value;
            },
            SPL_ADDR => {
                self.sp = self.sp & 0xFF00;
                self.sp |= value as u16;
            },
            SPH_ADDR => {
                self.sp = self.sp & 0x00FF;
                self.sp |= (value as u16) << 8;
            },
            SREG_ADDR => {
                self.sreg = value;
            },
            IO_START..=IO_END => {
                self.io[(address - IO_START) as usize] = value;
            },
            SRAM_START..=RAMEND => {
                self.sram[(address - SRAM_START) as usize] = value;
            },
            _ => return Err(String::from("Out of Adress Space"))
        }
        Ok(())
    }

    fn read_data(&self, address: u16) -> Result<u8, String> {
        match address {
            REGISTERS_START..=REGISTERS_END => {
                Ok(self.registers[address as usize])
            },
            SPL_ADDR => {
                Ok((self.sp & 0x00FF) as u8)
            },
            SPH_ADDR => {
                Ok(((self.sp & 0xFF00) >> 8) as u8)
            },
            SREG_ADDR => {
                Ok(self.sreg)
            },
            IO_START..=IO_END => {
                Ok(self.io[(address - IO_START) as usize])
            },
            SRAM_START..=RAMEND => {
                Ok(self.sram[(address - SRAM_START) as usize])
            },
            _ => Err(String::from("Out of Adress Space"))
        }
    }
}

#[cfg(test)]
mod tests;