//lib.rs
pub mod constants;
use std::{format, println};

use crate::constants::*;

#[derive(Debug, PartialEq)]
enum Instruction {
    Nop,
    Ldi { dest: u8, k: u8 },
    Add { dest: u8, src: u8 },
    Sub { dest: u8, src: u8 },
    Cp { dest: u8, src: u8 },
    Unknown(u16),
}

fn decode(word: u16) -> Instruction {
    match word {
        0x0000 => Instruction::Nop,
        w if w & 0xF000 == 0xE000 => {
            Instruction::Ldi {
                dest: ((w >> 4) & 0x000F) as u8 + 16, //R16 to R31
                k: (((w >> 4) & 0x00F0) | (w & 0x000F)) as u8,
            }
        }
        w if w & 0xFC00 == 0x0C00 => {
            let (dest, src) = decode_dest_src(w);
            Instruction::Add {
                dest: dest,
                src: src,
            }
        }
        w if w & 0xFC00 == 0x1800 => {
            let (dest, src) = decode_dest_src(w);
            Instruction::Sub {
                dest: dest,
                src: src,
            }
        }
        w if w & 0xFC00 == 0x1400 => {
            let (dest, src) = decode_dest_src(w);
            Instruction::Cp {
                dest: dest,
                src: src,
            }
        }
        _ => Instruction::Unknown(word),
    }
}

fn overflowing_add(a: u8, b: u8) -> (u8, bool) {
    let res = a as u16 + b as u16;
    (res as u8, res > u8::MAX as u16)
}

fn overflowing_sub(a: u8, b: u8) -> (u8, bool) {
    ((a as u16 + 256 - b as u16) as u8, a < b)
}

fn decode_dest_src(word: u16) -> (u8, u8) {
    (
        (((word >> 4) & 0x001F) as u8),
        (((word >> 5) & 0x0010) | (word & 0x000F)) as u8,
    )
}

pub struct Cpu {
    flash: [u16; FLASH_SIZE],
    sram: [u8; SRAM_SIZE],
    registers: [u8; CPU_REGISTERS_SIZE],
    pc: u16,
    sp: u16,
    sreg: u8,
    io: [u8; IO_SIZE],
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            flash: [0; FLASH_SIZE],
            sram: [0; SRAM_SIZE],
            registers: [0; CPU_REGISTERS_SIZE],
            pc: 0,
            sp: 0,
            sreg: 0,
            io: [0; IO_SIZE],
        }
    }

    fn write_data(&mut self, address: u16, value: u8) -> Result<(), String> {
        match address {
            REGISTERS_START..=REGISTERS_END => {
                self.registers[address as usize] = value;
            }
            SPL_ADDR => {
                self.sp = self.sp & 0xFF00;
                self.sp |= value as u16;
            }
            SPH_ADDR => {
                self.sp = self.sp & 0x00FF;
                self.sp |= (value as u16) << 8;
            }
            SREG_ADDR => {
                self.sreg = value;
            }
            IO_START..=IO_END => {
                self.io[(address - IO_START) as usize] = value;
            }
            SRAM_START..=RAMEND => {
                self.sram[(address - SRAM_START) as usize] = value;
            }
            _ => return Err(String::from("Out of Adress Space")),
        }
        Ok(())
    }

    fn read_data(&self, address: u16) -> Result<u8, String> {
        match address {
            REGISTERS_START..=REGISTERS_END => Ok(self.registers[address as usize]),
            SPL_ADDR => Ok((self.sp & 0x00FF) as u8),
            SPH_ADDR => Ok(((self.sp & 0xFF00) >> 8) as u8),
            SREG_ADDR => Ok(self.sreg),
            IO_START..=IO_END => Ok(self.io[(address - IO_START) as usize]),
            SRAM_START..=RAMEND => Ok(self.sram[(address - SRAM_START) as usize]),
            _ => Err(String::from("Out of Adress Space")),
        }
    }

    fn fetch(&mut self) -> Result<u16, String> {
        if self.pc as usize >= self.flash.len() {
            return Err(String::from("Invalid value of pc"));
        }
        let word: u16 = self.flash[self.pc as usize];
        self.pc += 1;
        Ok(word)
    }

    fn execute(&mut self, instruction: Instruction) -> Result<(), String> {
        match instruction {
            Instruction::Nop => Ok(()),
            Instruction::Unknown(inst) => Err(format!("Error Invalid Instruction [{:#X}]", inst)),
            Instruction::Add { dest, src } => {
                let op_a = self.registers[dest as usize];
                let op_b = self.registers[src as usize];
                let (res, carry) = overflowing_add(op_a, op_b);
                self.registers[dest as usize] = res;
                self.update_flags_add(op_a, op_b, res, carry);
                Ok(())
            }
            Instruction::Sub { dest, src } => {
                let op_a = self.registers[dest as usize];
                let op_b = self.registers[src as usize];
                let (res, carry) = overflowing_sub(op_a, op_b);
                self.registers[dest as usize] = res;
                self.update_flags_sub(op_a, op_b, res, carry);
                Ok(())
            }
            Instruction::Cp { dest, src } => {
                let op_a = self.registers[dest as usize];
                let op_b = self.registers[src as usize];
                let (res, carry) = overflowing_sub(op_a, op_b);
                self.update_flags_sub(op_a, op_b, res, carry);
                Ok(())
            }
            Instruction::Ldi { dest, k } => {
                self.registers[dest as usize] = k;
                Ok(())
            }
        }
    }

    pub fn step(&mut self) -> Result<(), String> {
        let instruction_word = self.fetch()?;
        let instruction = decode(instruction_word);
        self.execute(instruction)
    }

    pub fn load_program(&mut self, words: &[u16]) {
        self.flash[..words.len()].copy_from_slice(words);
    }

    pub fn print_cpu(&self) {
        let width = 40;
        // Flags in display order, from bit 7 (I) down to bit 0 (C)
        let flags = [
            ('I', FLAG_INTERRUPT_ENABLE),
            ('T', FLAG_BIT_COPY),
            ('H', FLAG_HALF_CARRY),
            ('S', FLAG_SIGN),
            ('V', FLAG_OVERFLOW),
            ('N', FLAG_NEGATIVE),
            ('Z', FLAG_ZERO),
            ('C', FLAG_CARRY),
        ];

        println!("{:-^width$}", "CPU");
        println!("PC  : {:#06X}        SP : {:#06X}", self.pc, self.sp);

        let names: Vec<String> = flags.iter().map(|(name, _)| name.to_string()).collect();
        let values: Vec<&str> = flags
            .iter()
            .map(|(_, mask)| if self.sreg & mask != 0 { "1" } else { "." })
            .collect();
        println!("SREG: {:#04X}        {}", self.sreg, names.join(" "));
        println!("                  {}", values.join(" "));

        println!("{:-^width$}", "REGISTERS");
        print!("    ");
        for col in 0..8 {
            print!(" +{col} ");
        }
        println!();
        for (row, chunk) in self.registers.chunks(8).enumerate() {
            print!("R{:02}:", row * 8);
            for value in chunk {
                print!(" {value:02X} ");
            }
            println!();
        }
        println!("{:-<width$}", "");
    }

    fn update_flags_add(&mut self, op_a: u8, op_b: u8, res: u8, carry: bool) {
        self.sreg &= FLAG_BIT_COPY | FLAG_INTERRUPT_ENABLE; //We clear all flags without T and I flags
        if carry {
            self.sreg |= FLAG_CARRY;
        }
        if res & 0x80 != 0 {
            self.sreg |= FLAG_NEGATIVE;
        }
        if (((op_a & 0x0F) + (op_b & 0x0F)) & 0x10) != 0 {
            self.sreg |= FLAG_HALF_CARRY;
        }
        if ((res & 0x80 != 0) && (op_a & 0x80 == 0) && (op_b & 0x80 == 0))
            || ((res & 0x80 == 0) && (op_a & 0x80 != 0) && (op_b & 0x80 != 0))
        {
            self.sreg |= FLAG_OVERFLOW;
        }
        if res == 0 {
            self.sreg |= FLAG_ZERO;
        }
        let negative_on = (self.sreg & FLAG_NEGATIVE) != 0;
        let overflow_on = (self.sreg & FLAG_OVERFLOW) != 0;
        if negative_on != overflow_on {
            self.sreg |= FLAG_SIGN;
        }
    }

    fn update_flags_sub(&mut self, op_a: u8, op_b: u8, res: u8, carry: bool) {
        self.sreg &= FLAG_BIT_COPY | FLAG_INTERRUPT_ENABLE; //We clear all flags without T and I flags
        if carry {
            self.sreg |= FLAG_CARRY;
        }
        if res == 0 {
            self.sreg |= FLAG_ZERO;
        }
        if res & 0x80 != 0 {
            self.sreg |= FLAG_NEGATIVE;
        }
        if (op_a & 0x0F) < (op_b & 0x0F) {
            self.sreg |= FLAG_HALF_CARRY;
        }
        if ((res & 0x80 != 0) && (op_b & 0x80 != 0) && (op_a & 0x80 == 0))
            || ((res & 0x80 == 0) && (op_b & 0x80 == 0) && (op_a & 0x80 != 0))
        {
            self.sreg |= FLAG_OVERFLOW;
        }
        let negative_on = (self.sreg & FLAG_NEGATIVE) != 0;
        let overflow_on = (self.sreg & FLAG_OVERFLOW) != 0;
        if negative_on != overflow_on {
            self.sreg |= FLAG_SIGN;
        }
    }
}

#[cfg(test)]
mod tests;
