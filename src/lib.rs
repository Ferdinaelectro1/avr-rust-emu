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
    Rjmp { k: i16 },
    BRBS { s: u8, k: i8 },
    BRBC { s: u8, k: i8 },
    Inc { rd: u8 },
    Dec { rd: u8 },
    And { rd: u8, rr: u8 },
    Or { rd: u8, rr: u8 },
    Eor { rd: u8, rr: u8 },
    Mov { rd: u8, rr: u8 },
    Adc { rd: u8, rr: u8 },
    Sbc { rd: u8, rr: u8 },
    Subi { rd: u8, k: u8 },
    Cpi { rd: u8, k: u8 },
    Andi { rd: u8, k: u8 },
    Ori { rd: u8, k: u8 },
    Sbci { rd: u8, k: u8 },
    Cpc { rd: u8, rr: u8 },
    Com { rd: u8 },
    Neg { rd: u8 },
    Lsr { rd: u8 },
    Asr { rd: u8 },
    Ror { rd: u8 },
    Swap { rd: u8 },
    Adiw { rd: u8, k: u8 },
    Sbiw { rd: u8, k: u8 },
    Movw { rd: u8, rr: u8 },
    Mul { rd: u8, rr: u8 },
    Muls { rd: u8, rr: u8 },
    Mulsu { rd: u8, rr: u8 },
    Bset { s: u8 },
    Bclr { s: u8 },
    Bst { rd: u8, b: u8 },
    Bld { rd: u8, b: u8 },
    Fmul { rd: u8, rr: u8 },
    Fmuls { rd: u8, rr: u8 },
    Fmulsu { rd: u8, rr: u8 },
    In { rd: u8, a: u8 },
    Out { a: u8, rr: u8 },
    Push { rr: u8 },
    Pop { rd: u8 },
    Lds { rd: u8 },
    Sts { rr: u8 },
    Rcall { k: i16 },
    Ret,
    Jmp,
    Call,
    Ijmp,
    Icall,
    Reti,
    Sbi { a: u8, b: u8 },
    Cbi { a: u8, b: u8 },
    Unknown(u16),
}

fn decode(word: u16) -> Instruction {
    match word {
        0x0000 => Instruction::Nop,
        w if w & 0xF000 == 0xE000 => {
            let (reg, imm) = decode_reg_imm(w);
            Instruction::Ldi { dest: reg, k: imm }
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
        w if w & 0xF000 == 0xC000 => Instruction::Rjmp {
            k: sign_extend(w, 12),
        },
        w if w & 0xFC00 == 0xF000 => Instruction::BRBS {
            s: (w & 0x0007) as u8,
            k: sign_extend(w >> 3, 7) as i8,
        },
        w if w & 0xFC00 == 0xF400 => Instruction::BRBC {
            s: (w & 0x0007) as u8,
            k: sign_extend(w >> 3, 7) as i8,
        },
        w if w & 0xFE0F == 0x9403 => Instruction::Inc { rd: decode_dest(w) },
        w if w & 0xFE0F == 0x940A => Instruction::Dec { rd: decode_dest(w) },
        w if w & 0xFC00 == 0x2000 => {
            let (dest, src) = decode_dest_src(w);
            Instruction::And { rd: dest, rr: src }
        }
        w if w & 0xFC00 == 0x2800 => {
            let (dest, src) = decode_dest_src(w);
            Instruction::Or { rd: dest, rr: src }
        }
        w if w & 0xFC00 == 0x2400 => {
            let (dest, src) = decode_dest_src(w);
            Instruction::Eor { rd: dest, rr: src }
        }
        w if w & 0xFC00 == 0x2C00 => {
            let (dest, src) = decode_dest_src(w);
            Instruction::Mov { rd: dest, rr: src }
        }
        w if w & 0xFC00 == 0x1C00 => {
            let (dest, src) = decode_dest_src(w);
            Instruction::Adc { rd: dest, rr: src }
        }
        w if w & 0xFC00 == 0x0800 => {
            let (dest, src) = decode_dest_src(w);
            Instruction::Sbc { rd: dest, rr: src }
        }
        w if w & 0xF000 == 0x5000 => {
            let (reg, imm) = decode_reg_imm(w);
            Instruction::Subi { rd: reg, k: imm }
        }
        w if w & 0xF000 == 0x3000 => {
            let (reg, imm) = decode_reg_imm(w);
            Instruction::Cpi { rd: reg, k: imm }
        }
        w if w & 0xF000 == 0x7000 => {
            let (reg, imm) = decode_reg_imm(w);
            Instruction::Andi { rd: reg, k: imm }
        }
        w if w & 0xF000 == 0x6000 => {
            let (reg, imm) = decode_reg_imm(w);
            Instruction::Ori { rd: reg, k: imm }
        }
        w if w & 0xF000 == 0x4000 => {
            let (reg, imm) = decode_reg_imm(w);
            Instruction::Sbci { rd: reg, k: imm }
        }
        w if w & 0xFC00 == 0x0400 => {
            let (dest, src) = decode_dest_src(w);
            Instruction::Cpc { rd: dest, rr: src }
        }
        w if w & 0xFE0F == 0x9400 => Instruction::Com { rd: decode_dest(w) },
        w if w & 0xFE0F == 0x9401 => Instruction::Neg { rd: decode_dest(w) },
        w if w & 0xFE0F == 0x9406 => Instruction::Lsr { rd: decode_dest(w) },
        w if w & 0xFE0F == 0x9405 => Instruction::Asr { rd: decode_dest(w) },
        w if w & 0xFE0F == 0x9407 => Instruction::Ror { rd: decode_dest(w) },
        w if w & 0xFE0F == 0x9402 => Instruction::Swap { rd: decode_dest(w) },
        w if w & 0xFF00 == 0x9600 => Instruction::Adiw {
            rd: 24 + 2 * ((w >> 4) & 0x0003) as u8,
            k: (((w >> 2) & 0x0030) | (w & 0x000F)) as u8,
        },
        w if w & 0xFF00 == 0x9700 => Instruction::Sbiw {
            rd: 24 + 2 * ((w >> 4) & 0x0003) as u8,
            k: (((w >> 2) & 0x0030) | (w & 0x000F)) as u8,
        },
        w if w & 0xFF00 == 0x0100 => Instruction::Movw {
            rd: 2 * ((w >> 4) & 0x000F) as u8,
            rr: 2 * (w & 0x000F) as u8,
        },
        w if w & 0xFC00 == 0x9C00 => {
            let (dest, src) = decode_dest_src(w);
            Instruction::Mul { rd: dest, rr: src }
        }
        w if w & 0xFF00 == 0x0200 => Instruction::Muls {
            rd: (((w >> 4) & 0x000F) + 16) as u8,
            rr: ((w & 0x000F) + 16) as u8,
        },
        w if w & 0xFF88 == 0x0300 => Instruction::Mulsu {
            rd: (((w >> 4) & 0x0007) + 16) as u8,
            rr: ((w & 0x0007) + 16) as u8,
        },
        w if w & 0xFF8F == 0x9408 => Instruction::Bset {
            s: ((w >> 4) & 0x0007) as u8,
        },
        w if w & 0xFF8F == 0x9488 => Instruction::Bclr {
            s: ((w >> 4) & 0x0007) as u8,
        },
        w if w & 0xFE08 == 0xF800 => Instruction::Bld {
            rd: ((w >> 4) & 0x001F) as u8,
            b: (w & 0x0007) as u8,
        },
        w if w & 0xFE08 == 0xFA00 => Instruction::Bst {
            rd: ((w >> 4) & 0x001F) as u8,
            b: (w & 0x0007) as u8,
        },
        w if w & 0xFF88 == 0x0308 => Instruction::Fmul {
            rd: (((w >> 4) & 0x0007) + 16) as u8,
            rr: ((w & 0x0007) + 16) as u8,
        },
        w if w & 0xFF88 == 0x0380 => Instruction::Fmuls {
            rd: (((w >> 4) & 0x0007) + 16) as u8,
            rr: ((w & 0x0007) + 16) as u8,
        },
        w if w & 0xFF88 == 0x0388 => Instruction::Fmulsu {
            rd: (((w >> 4) & 0x0007) + 16) as u8,
            rr: ((w & 0x0007) + 16) as u8,
        },
        w if w & 0xF800 == 0xB000 => Instruction::In {
            rd: ((w >> 4) & 0x001F) as u8,
            a: (((w >> 5) & 0x0030) | (w & 0x000F)) as u8,
        },
        w if w & 0xF800 == 0xB800 => Instruction::Out {
            a: (((w >> 5) & 0x0030) | (w & 0x000F)) as u8,
            rr: ((w >> 4) & 0x001F) as u8,
        },
        w if w & 0xFE0F == 0x920F => Instruction::Push { rr: decode_dest(w) },
        w if w & 0xFE0F == 0x900F => Instruction::Pop { rd: decode_dest(w) },
        w if w & 0xFE0F == 0x9000 => Instruction::Lds { rd: decode_dest(w) },
        w if w & 0xFE0F == 0x9200 => Instruction::Sts { rr: decode_dest(w) },
        w if w & 0xF000 == 0xD000 => Instruction::Rcall {
            k: sign_extend(w, 12),
        },
        w if w & 0xFFFF == 0x9508 => Instruction::Ret,
        w if w & 0xFE0E == 0x940C => Instruction::Jmp,
        w if w & 0xFE0E == 0x940E => Instruction::Call,
        w if w & 0xFFFF == 0x9409 => Instruction::Ijmp,
        w if w & 0xFFFF == 0x9509 => Instruction::Icall,
        w if w & 0xFFFF == 0x9518 => Instruction::Reti,
        w if w & 0xFF00 == 0x9800 => Instruction::Cbi {
            a: ((w >> 3) & 0x001F) as u8,
            b: (w & 0x0007) as u8,
        },
        w if w & 0xFF00 == 0x9A00 => Instruction::Sbi {
            a: ((w >> 3) & 0x001F) as u8,
            b: (w & 0x0007) as u8,
        },
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

fn decode_reg_imm(word: u16) -> (u8, u8) {
    (
        ((word >> 4) & 0x000F) as u8 + 16, //R16 to R31
        (((word >> 4) & 0x00F0) | (word & 0x000F)) as u8,
    )
}

fn decode_dest(word: u16) -> u8 {
    ((word >> 4) & 0x001F) as u8
}

fn sign_extend(value: u16, bits: u32) -> i16 {
    let raw = (value & ((1 << bits) - 1)) as i16;
    let k = if raw & (1 << (bits - 1)) != 0 {
        raw - (1 << bits)
    } else {
        raw
    };
    return k;
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
                self.update_flags_add(op_a, op_b, res, carry, false);
                Ok(())
            }
            Instruction::Sub { dest, src } => {
                let op_a = self.registers[dest as usize];
                let op_b = self.registers[src as usize];
                let (res, carry) = overflowing_sub(op_a, op_b);
                self.registers[dest as usize] = res;
                self.update_flags_sub(op_a, op_b, res, carry, false);
                Ok(())
            }
            Instruction::Cp { dest, src } => {
                let op_a = self.registers[dest as usize];
                let op_b = self.registers[src as usize];
                let (res, carry) = overflowing_sub(op_a, op_b);
                self.update_flags_sub(op_a, op_b, res, carry, false);
                Ok(())
            }
            Instruction::Ldi { dest, k } => {
                self.registers[dest as usize] = k;
                Ok(())
            }
            Instruction::Rjmp { k } => {
                self.pc = (self.pc as i32 + k as i32).rem_euclid(FLASH_SIZE as i32) as u16;
                Ok(())
            }
            Instruction::BRBS { s, k } => {
                if (1 << s) & self.sreg != 0 {
                    self.pc = (self.pc as i16 + k as i16).rem_euclid(FLASH_SIZE as i16) as u16;
                }
                Ok(())
            }
            Instruction::BRBC { s, k } => {
                if (1 << s) & self.sreg == 0 {
                    self.pc = (self.pc as i16 + k as i16).rem_euclid(FLASH_SIZE as i16) as u16;
                }
                Ok(())
            }
            Instruction::Inc { rd } => {
                self.registers[rd as usize] = self.registers[rd as usize].wrapping_add(1);
                self.update_flags_incdec(self.registers[rd as usize], true);
                Ok(())
            }
            Instruction::Dec { rd } => {
                self.registers[rd as usize] = self.registers[rd as usize].wrapping_sub(1);
                self.update_flags_incdec(self.registers[rd as usize], false);
                Ok(())
            }
            Instruction::And { rd, rr } => {
                self.registers[rd as usize] &= self.registers[rr as usize];
                self.update_flags_logic(self.registers[rd as usize]);
                Ok(())
            }
            Instruction::Or { rd, rr } => {
                self.registers[rd as usize] |= self.registers[rr as usize];
                self.update_flags_logic(self.registers[rd as usize]);
                Ok(())
            }
            Instruction::Eor { rd, rr } => {
                self.registers[rd as usize] ^= self.registers[rr as usize];
                self.update_flags_logic(self.registers[rd as usize]);
                Ok(())
            }
            Instruction::Mov { rd, rr } => {
                self.registers[rd as usize] = self.registers[rr as usize];
                Ok(())
            }
            Instruction::Adc { rd, rr } => {
                let op_a = self.registers[rd as usize];
                let op_b = self.registers[rr as usize];
                let last_carry_on = self.sreg & FLAG_CARRY != 0;
                let (res, c1) = overflowing_add(op_a, op_b);
                let (res, c2) = overflowing_add(res, if last_carry_on { 1 } else { 0 });
                self.registers[rd as usize] = res;
                self.update_flags_add(op_a, op_b, res, c1 | c2, last_carry_on);
                Ok(())
            }
            Instruction::Sbc { rd, rr } => {
                let op_a = self.registers[rd as usize];
                let op_b = self.registers[rr as usize];
                let last_carry_on = self.sreg & FLAG_CARRY != 0;
                let last_zero_off = self.sreg & FLAG_ZERO == 0;
                let (res, c1) = overflowing_sub(op_a, op_b);
                let (res, c2) = overflowing_sub(res, if last_carry_on { 1 } else { 0 });
                self.registers[rd as usize] = res;
                self.update_flags_sub(op_a, op_b, res, c1 | c2, last_carry_on);
                if last_zero_off {
                    self.sreg &= !(FLAG_ZERO);
                }
                Ok(())
            }
            Instruction::Subi { rd, k } => {
                let op_a = self.registers[rd as usize];
                let op_b = k;
                let (res, carry) = overflowing_sub(op_a, op_b);
                self.registers[rd as usize] = res;
                self.update_flags_sub(op_a, op_b, res, carry, false);
                Ok(())
            }
            Instruction::Cpi { rd, k } => {
                let op_a = self.registers[rd as usize];
                let op_b = k;
                let (res, carry) = overflowing_sub(op_a, op_b);
                self.update_flags_sub(op_a, op_b, res, carry, false);
                Ok(())
            }
            Instruction::Andi { rd, k } => {
                self.registers[rd as usize] &= k;
                self.update_flags_logic(self.registers[rd as usize]);
                Ok(())
            }
            Instruction::Ori { rd, k } => {
                self.registers[rd as usize] |= k;
                self.update_flags_logic(self.registers[rd as usize]);
                Ok(())
            }
            Instruction::Sbci { rd, k } => {
                let op_a = self.registers[rd as usize];
                let op_b = k;
                let last_carry_on = self.sreg & FLAG_CARRY != 0;
                let last_zero_off = self.sreg & FLAG_ZERO == 0;
                let (res, c1) = overflowing_sub(op_a, op_b);
                let (res, c2) = overflowing_sub(res, if last_carry_on { 1 } else { 0 });
                self.registers[rd as usize] = res;
                self.update_flags_sub(op_a, op_b, res, c1 | c2, last_carry_on);
                if last_zero_off {
                    self.sreg &= !(FLAG_ZERO);
                }
                Ok(())
            }
            Instruction::Cpc { rd, rr } => {
                let op_a = self.registers[rd as usize];
                let op_b = self.registers[rr as usize];
                let last_carry_on = self.sreg & FLAG_CARRY != 0;
                let last_zero_off = self.sreg & FLAG_ZERO == 0;
                let (res, c1) = overflowing_sub(op_a, op_b);
                let (res, c2) = overflowing_sub(res, if last_carry_on { 1 } else { 0 });
                self.update_flags_sub(op_a, op_b, res, c1 | c2, last_carry_on);
                if last_zero_off {
                    self.sreg &= !(FLAG_ZERO);
                }
                Ok(())
            }
            Instruction::Com { rd } => {
                self.registers[rd as usize] = !self.registers[rd as usize];
                self.update_flags_logic(self.registers[rd as usize]);
                self.sreg |= FLAG_CARRY;
                Ok(())
            }
            Instruction::Neg { rd } => {
                let op_a = 0;
                let op_b = self.registers[rd as usize];
                let (res, carry) = overflowing_sub(op_a, op_b);
                self.registers[rd as usize] = res;
                self.update_flags_sub(op_a, op_b, res, carry, false);
                Ok(())
            }
            Instruction::Asr { rd } => {
                let carry = self.registers[rd as usize] & 0x01 != 0;
                let old_seven_bit = self.registers[rd as usize] & 0x80;
                self.registers[rd as usize] = self.registers[rd as usize] >> 1;
                self.registers[rd as usize] |= old_seven_bit;
                self.update_flags_shift(self.registers[rd as usize], carry);
                Ok(())
            }
            Instruction::Lsr { rd } => {
                let carry = self.registers[rd as usize] & 0x01 != 0;
                self.registers[rd as usize] = self.registers[rd as usize] >> 1;
                self.update_flags_shift(self.registers[rd as usize], carry);
                Ok(())
            }
            Instruction::Ror { rd } => {
                let old_carry_on = self.sreg & FLAG_CARRY != 0;
                let carry = self.registers[rd as usize] & 0x01 != 0;
                self.registers[rd as usize] = self.registers[rd as usize] >> 1;
                if old_carry_on {
                    self.registers[rd as usize] |= 0x80; //turn on 7 bit
                }
                self.update_flags_shift(self.registers[rd as usize], carry);
                Ok(())
            }
            Instruction::Swap { rd } => {
                self.registers[rd as usize] = ((self.registers[rd as usize] >> 4) & 0x0F)
                    | ((self.registers[rd as usize] << 4) & 0xF0);
                Ok(())
            }
            Instruction::Adiw { rd, k } => {
                let rdh = self.registers[(rd + 1) as usize] as u16;
                let rdl = self.registers[rd as usize] as u16;
                let rdw = (rdh << 8) | rdl;
                let res = rdw.wrapping_add(k as u16);
                self.registers[(rd + 1) as usize] = ((res >> 8) & 0x00FF) as u8;
                self.registers[rd as usize] = (res & 0x00FF) as u8;
                self.update_flags_word(rdw, res, true);
                Ok(())
            }
            Instruction::Sbiw { rd, k } => {
                let rdh = self.registers[(rd + 1) as usize] as u16;
                let rdl = self.registers[rd as usize] as u16;
                let rdw = (rdh << 8) | rdl;
                let res = rdw.wrapping_sub(k as u16);
                self.registers[(rd + 1) as usize] = ((res >> 8) & 0x00FF) as u8;
                self.registers[rd as usize] = (res & 0x00FF) as u8;
                self.update_flags_word(rdw, res, false);
                Ok(())
            }
            Instruction::Movw { rd, rr } => {
                self.registers[rd as usize] = self.registers[rr as usize];
                self.registers[(rd + 1) as usize] = self.registers[(rr + 1) as usize];
                Ok(())
            }
            Instruction::Mul { rd, rr } => {
                let res = self.registers[rd as usize] as u16 * self.registers[rr as usize] as u16;
                self.registers[0] = (res & 0x00FF) as u8;
                self.registers[1] = ((res >> 8) & 0x00FF) as u8;
                self.update_flags_mul(res);
                Ok(())
            }
            Instruction::Muls { rd, rr } => {
                let res = (self.registers[rd as usize] as i8) as i16
                    * (self.registers[rr as usize] as i8) as i16;
                self.registers[0] = (res & 0x00FF) as u8;
                self.registers[1] = ((res >> 8) & 0x00FF) as u8;
                self.update_flags_mul(res as u16);
                Ok(())
            }
            Instruction::Mulsu { rd, rr } => {
                let signed_res =
                    self.registers[rd as usize] as i8 as i16 * self.registers[rr as usize] as i16;
                self.registers[0] = (signed_res & 0x00FF) as u8;
                self.registers[1] = ((signed_res >> 8) & 0x00FF) as u8;
                self.update_flags_mul(signed_res as u16);
                Ok(())
            }
            Instruction::Bset { s } => {
                self.sreg |= 1u8 << s;
                Ok(())
            }
            Instruction::Bclr { s } => {
                self.sreg &= !(1u8 << s);
                Ok(())
            }
            Instruction::Bld { rd, b } => {
                if self.sreg & FLAG_BIT_COPY != 0 {
                    self.registers[rd as usize] |= 1 << b;
                } else {
                    self.registers[rd as usize] &= !(1 << b);
                }
                Ok(())
            }
            Instruction::Bst { rd, b } => {
                if self.registers[rd as usize] & (1 << b) != 0 {
                    self.sreg |= FLAG_BIT_COPY;
                } else {
                    self.sreg &= !FLAG_BIT_COPY;
                }
                Ok(())
            }
            Instruction::Fmul { rd, rr } => {
                let a = self.registers[rd as usize] as u16;
                let b = self.registers[rr as usize] as u16;
                let product = a * b; // max 255 * 255 = 65025, fits in u16
                let res = product << 1;
                self.registers[0] = res as u8;
                self.registers[1] = (res >> 8) as u8;
                self.update_flags_fmul(product, res);
                Ok(())
            }
            Instruction::Fmuls { rd, rr } => {
                let a = self.registers[rd as usize] as i8 as i16;
                let b = self.registers[rr as usize] as i8 as i16;
                let product = (a * b) as u16;
                let res = product << 1;
                self.registers[0] = res as u8;
                self.registers[1] = (res >> 8) as u8;
                self.update_flags_fmul(product, res);
                Ok(())
            }
            Instruction::Fmulsu { rd, rr } => {
                let a = self.registers[rd as usize] as i8 as i16;
                let b = self.registers[rr as usize] as i16; // unsigned: u8 -> i16 directly
                let product = (a * b) as u16;
                let res = product << 1;
                self.registers[0] = res as u8;
                self.registers[1] = (res >> 8) as u8;
                self.update_flags_fmul(product, res);
                Ok(())
            }
            Instruction::Out { a, rr } => {
                self.write_data(a as u16 + IO_START, self.registers[rr as usize])?;
                Ok(())
            }
            Instruction::In { rd, a } => {
                self.registers[rd as usize] = self.read_data(a as u16 + IO_START)?;
                Ok(())
            }
            Instruction::Pop { rd } => {
                self.sp = self.sp.wrapping_add(1);
                self.registers[rd as usize] = self.read_data(self.sp)?;
                Ok(())
            }
            Instruction::Push { rr } => {
                self.write_data(self.sp, self.registers[rr as usize])?;
                self.sp = self.sp.wrapping_sub(1);
                Ok(())
            }
            Instruction::Sts { rr } => {
                let k = self.fetch()?;
                self.write_data(k, self.registers[rr as usize])?;
                Ok(())
            }
            Instruction::Lds { rd } => {
                let k = self.fetch()?;
                self.registers[rd as usize] = self.read_data(k)?;
                Ok(())
            }
            Instruction::Rcall { k } => {
                let next_instruction = self.pc;
                self.write_data(self.sp, (next_instruction & 0x00FF) as u8)?;
                self.sp = self.sp.wrapping_sub(1);
                self.write_data(self.sp, ((next_instruction >> 8) & 0x00FF) as u8)?;
                self.sp = self.sp.wrapping_sub(1);
                self.pc = (self.pc as i32 + k as i32).rem_euclid(FLASH_SIZE as i32) as u16;
                Ok(())
            }
            Instruction::Ret => {
                self.sp = self.sp.wrapping_add(1);
                let pc_ret_h = self.read_data(self.sp)?;
                self.sp = self.sp.wrapping_add(1);
                let pc_ret_l = self.read_data(self.sp)?;
                self.pc = ((pc_ret_h as u16) << 8) | (pc_ret_l as u16);
                Ok(())
            }
            Instruction::Jmp => {
                let k = self.fetch()?;
                self.pc = k;
                Ok(())
            }
            Instruction::Ijmp => {
                let z_l = self.registers[30];
                let z_h = self.registers[31];
                self.pc = ((z_h as u16) << 8) | (z_l as u16);
                Ok(())
            }
            Instruction::Call => {
                let dest = self.fetch()?;
                let ret = self.pc;
                self.write_data(self.sp, (ret & 0x00FF) as u8)?;
                self.sp = self.sp.wrapping_sub(1);
                self.write_data(self.sp, ((ret >> 8) & 0x00FF) as u8)?;
                self.sp = self.sp.wrapping_sub(1);
                self.pc = dest;
                Ok(())
            }
            Instruction::Icall => {
                let z_l = self.registers[30];
                let z_h = self.registers[31];
                self.write_data(self.sp, (self.pc & 0x00FF) as u8)?;
                self.sp = self.sp.wrapping_sub(1);
                self.write_data(self.sp, ((self.pc >> 8) & 0x00FF) as u8)?;
                self.sp = self.sp.wrapping_sub(1);
                self.pc = ((z_h as u16) << 8) | (z_l as u16);
                Ok(())
            }
            Instruction::Reti => {
                self.sp = self.sp.wrapping_add(1);
                let pc_h = self.read_data(self.sp)?;
                self.sp = self.sp.wrapping_add(1);
                let pc_l = self.read_data(self.sp)?;
                self.pc = ((pc_h as u16) << 8) | (pc_l as u16);
                self.sreg |= FLAG_INTERRUPT_ENABLE;
                Ok(())
            }
            Instruction::Sbi { a, b } => {
                let reg_value = self.read_data(a as u16 + IO_START)?;
                self.write_data(a as u16 + IO_START, reg_value | (1 << b))?;
                Ok(())
            }
            Instruction::Cbi { a, b } => {
                let reg_value = self.read_data(a as u16 + IO_START)?;
                self.write_data(a as u16 + IO_START, reg_value & (!(1 << b)))?;
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

    fn update_flags_add(&mut self, op_a: u8, op_b: u8, res: u8, carry: bool, in_carry: bool) {
        self.sreg &= FLAG_BIT_COPY | FLAG_INTERRUPT_ENABLE; //We clear all flags without T and I flags
        if carry {
            self.sreg |= FLAG_CARRY;
        }
        if res & 0x80 != 0 {
            self.sreg |= FLAG_NEGATIVE;
        }
        if (((op_a & 0x0F) + (op_b & 0x0F) + if in_carry { 1 } else { 0 }) & 0x10) != 0 {
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

    fn update_flags_sub(&mut self, op_a: u8, op_b: u8, res: u8, carry: bool, in_carry: bool) {
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
        if (op_a & 0x0F) < ((op_b & 0x0F) + if in_carry { 1 } else { 0 }) {
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

    fn update_flags_incdec(&mut self, res: u8, is_inc: bool) {
        self.sreg &= FLAG_INTERRUPT_ENABLE | FLAG_BIT_COPY | FLAG_CARRY | FLAG_HALF_CARRY;
        if res == 0 {
            self.sreg |= FLAG_ZERO;
        }
        let overflow_value = if is_inc { 0x80 } else { 0x7F };
        if res == overflow_value {
            self.sreg |= FLAG_OVERFLOW;
        }
        if res & 0x80 != 0 {
            self.sreg |= FLAG_NEGATIVE;
        }
        if ((self.sreg & FLAG_NEGATIVE) != 0) != ((self.sreg & FLAG_OVERFLOW) != 0) {
            self.sreg |= FLAG_SIGN;
        }
    }

    fn update_flags_logic(&mut self, res: u8) {
        self.sreg &= FLAG_INTERRUPT_ENABLE | FLAG_BIT_COPY | FLAG_CARRY | FLAG_HALF_CARRY;
        if res == 0 {
            self.sreg |= FLAG_ZERO;
        }
        if res & 0x80 != 0 {
            self.sreg |= FLAG_NEGATIVE;
            self.sreg |= FLAG_SIGN;
        }
    }

    fn update_flags_shift(&mut self, res: u8, carry: bool) {
        self.sreg &= FLAG_INTERRUPT_ENABLE | FLAG_BIT_COPY | FLAG_HALF_CARRY;
        if carry {
            self.sreg |= FLAG_CARRY;
            self.sreg |= FLAG_SIGN; //Sign is just Neg xor Over but Over = Neg xor Carry
        }
        if res == 0 {
            self.sreg |= FLAG_ZERO;
        }
        if res & 0x80 != 0 {
            self.sreg |= FLAG_NEGATIVE;
        }
        let negative_on = self.sreg & FLAG_NEGATIVE != 0;
        let carry_on = self.sreg & FLAG_CARRY != 0;
        if negative_on != carry_on {
            self.sreg |= FLAG_OVERFLOW;
        }
    }

    fn update_flags_word(&mut self, old: u16, res: u16, is_add: bool) {
        self.sreg &= FLAG_INTERRUPT_ENABLE | FLAG_BIT_COPY | FLAG_HALF_CARRY;
        if res == 0 {
            self.sreg |= FLAG_ZERO;
        }
        if res & 0x8000 != 0 {
            self.sreg |= FLAG_NEGATIVE;
        }
        if is_add {
            if (old & 0x8000 == 0) && (res & 0x8000 != 0) {
                self.sreg |= FLAG_OVERFLOW;
            }
            if (old & 0x8000 != 0) && (res & 0x8000 == 0) {
                self.sreg |= FLAG_CARRY;
            }
        } else {
            if (old & 0x8000 != 0) && (res & 0x8000 == 0) {
                self.sreg |= FLAG_OVERFLOW;
            }
            if (old & 0x8000 == 0) && (res & 0x8000 != 0) {
                self.sreg |= FLAG_CARRY;
            }
        }
        if ((self.sreg & FLAG_NEGATIVE) != 0) != ((self.sreg & FLAG_OVERFLOW) != 0) {
            self.sreg |= FLAG_SIGN;
        }
    }

    fn update_flags_mul(&mut self, res: u16) {
        self.sreg &= !(FLAG_CARRY | FLAG_ZERO); //Clear C and Z flags
        if res == 0 {
            self.sreg |= FLAG_ZERO;
        }
        if res & 0x8000 != 0 {
            self.sreg |= FLAG_CARRY;
        }
    }

    fn update_flags_fmul(&mut self, product: u16, res: u16) {
        self.sreg &= !(FLAG_CARRY | FLAG_ZERO);
        if product & 0x8000 != 0 {
            self.sreg |= FLAG_CARRY;
        }
        if res == 0 {
            self.sreg |= FLAG_ZERO;
        }
    }

    pub fn pc(&self) -> u16 {
        self.pc
    }
}

#[cfg(test)]
mod tests;
