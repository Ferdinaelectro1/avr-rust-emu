use std::assert_eq;

use super::*;
use crate::hex::load_hex;

#[test]
fn write_regs_test() {
    let mut cpu = Cpu::new();
    assert_eq!(cpu.write_data(REGISTERS_START, 0xFF), Ok(()));
    assert_eq!(cpu.registers[0], 0xFF);
    assert_eq!(cpu.write_data(REGISTERS_START + 3, 0xFF), Ok(()));
    assert_eq!(cpu.registers[3], 0xFF);
    assert_eq!(cpu.write_data(REGISTERS_END, 0xFF), Ok(()));
    assert_eq!(cpu.registers[CPU_REGISTERS_SIZE - 1], 0xFF);
}

#[test]
fn write_io_test() {
    let mut cpu = Cpu::new();
    assert_eq!(cpu.write_data(IO_START, 0xFF), Ok(()));
    assert_eq!(cpu.io[0], 0xFF);
    assert_eq!(cpu.write_data(IO_START + 3, 0xFF), Ok(()));
    assert_eq!(cpu.io[3], 0xFF);
    assert_eq!(cpu.write_data(IO_END, 0xFF), Ok(()));
    assert_eq!(cpu.io[IO_SIZE - 1], 0xFF);
}

#[test]
fn write_sram_test() {
    let mut cpu = Cpu::new();
    assert_eq!(cpu.write_data(SRAM_START, 0xFF), Ok(()));
    assert_eq!(cpu.sram[0], 0xFF);
    assert_eq!(cpu.write_data(SRAM_START + 3, 0xFF), Ok(()));
    assert_eq!(cpu.sram[3], 0xFF);
    assert_eq!(cpu.write_data(RAMEND, 0xFF), Ok(()));
    assert_eq!(cpu.sram[SRAM_SIZE - 1], 0xFF);
}

#[test]
fn write_out_of_address_space_test() {
    let mut cpu = Cpu::new();
    assert!(cpu.write_data(RAMEND + 1, 0xFF).is_err());
}

#[test]
fn sreg_write_replaces_whole_value() {
    let mut cpu = Cpu::new();
    assert_eq!(cpu.write_data(SREG_ADDR, 0xFF), Ok(()));
    assert_eq!(cpu.sreg, 0xFF);
    // Writing 0x00 must clear every flag, not just add bits
    assert_eq!(cpu.write_data(SREG_ADDR, 0x00), Ok(()));
    assert_eq!(cpu.sreg, 0x00);
}

#[test]
fn spl_write_keeps_high_byte() {
    let mut cpu = Cpu::new();
    cpu.sp = 0x08FF;
    assert_eq!(cpu.write_data(SPL_ADDR, 0x12), Ok(()));
    assert_eq!(cpu.sp, 0x0812);
}

#[test]
fn sph_write_keeps_low_byte() {
    let mut cpu = Cpu::new();
    cpu.sp = 0x08FF;
    assert_eq!(cpu.write_data(SPH_ADDR, 0x01), Ok(()));
    assert_eq!(cpu.sp, 0x01FF);
}

#[test]
fn sp_and_sreg_writes_do_not_touch_io_array() {
    let mut cpu = Cpu::new();
    assert_eq!(cpu.write_data(SPL_ADDR, 0xFF), Ok(()));
    assert_eq!(cpu.write_data(SPH_ADDR, 0xFF), Ok(()));
    assert_eq!(cpu.write_data(SREG_ADDR, 0xFF), Ok(()));
    assert_eq!(cpu.io[(SPL_ADDR - IO_START) as usize], 0);
    assert_eq!(cpu.io[(SPH_ADDR - IO_START) as usize], 0);
    assert_eq!(cpu.io[(SREG_ADDR - IO_START) as usize], 0);
}

#[test]
fn read_regs_test() {
    let mut cpu = Cpu::new();
    cpu.registers[0] = 0xFF;
    assert_eq!(cpu.read_data(REGISTERS_START), Ok(0xFF));
    cpu.registers[3] = 0xFF;
    assert_eq!(cpu.read_data(REGISTERS_START + 3), Ok(0xFF));
    cpu.registers[CPU_REGISTERS_SIZE - 1] = 0xFF;
    assert_eq!(cpu.read_data(REGISTERS_END), Ok(0xFF));
}

#[test]
fn read_io_test() {
    let mut cpu = Cpu::new();
    cpu.io[0] = 0xFF;
    assert_eq!(cpu.read_data(IO_START), Ok(0xFF));
    cpu.io[3] = 0xFF;
    assert_eq!(cpu.read_data(IO_START + 3), Ok(0xFF));
    cpu.io[IO_SIZE - 1] = 0xFF;
    assert_eq!(cpu.read_data(IO_END), Ok(0xFF));
}

#[test]
fn read_sram_test() {
    let mut cpu = Cpu::new();
    cpu.sram[0] = 0xFF;
    assert_eq!(cpu.read_data(SRAM_START), Ok(0xFF));
    cpu.sram[3] = 0xFF;
    assert_eq!(cpu.read_data(SRAM_START + 3), Ok(0xFF));
    cpu.sram[SRAM_SIZE - 1] = 0xFF;
    assert_eq!(cpu.read_data(RAMEND), Ok(0xFF));
}

#[test]
fn read_out_of_address_space_test() {
    let cpu = Cpu::new();
    assert!(cpu.read_data(RAMEND + 1).is_err());
}

#[test]
fn sreg_read_replaces_whole_value() {
    let mut cpu = Cpu::new();
    cpu.sreg = 0xF2;
    assert_eq!(cpu.read_data(SREG_ADDR), Ok(0xF2));
}

#[test]
fn spl_read_returns_low_byte() {
    let mut cpu = Cpu::new();
    cpu.sp = 0x1234;
    assert_eq!(cpu.read_data(SPL_ADDR), Ok(0x34));
}

#[test]
fn sph_read_keeps_high_byte() {
    let mut cpu = Cpu::new();
    cpu.sp = 0x1234;
    assert_eq!(cpu.read_data(SPH_ADDR), Ok(0x12));
}

//NOP
#[test]
fn test_decode_nop() {
    assert_eq!(decode(0x0000), Instruction::Nop);
}

//LDI dest,k
#[test]
fn test_decode_ldi() {
    assert_eq!(decode(0xE005), Instruction::Ldi { dest: 16, k: 5 });
    assert_eq!(decode(0xEFFF), Instruction::Ldi { dest: 31, k: 255 });
}

//ADD dest,src
#[test]
fn test_decode_add() {
    assert_eq!(decode(0x0F01), Instruction::Add { dest: 16, src: 17 });
    assert_eq!(decode(0x0C00), Instruction::Add { dest: 0, src: 0 });
    assert_eq!(decode(0x0FFF), Instruction::Add { dest: 31, src: 31 });
}

// SUB dest, src
#[test]
fn test_decode_sub() {
    // SUB R16, R17
    assert_eq!(decode(0x1B01), Instruction::Sub { dest: 16, src: 17 });
    // SUB R0, R0
    assert_eq!(decode(0x1800), Instruction::Sub { dest: 0, src: 0 });
    // SUB R31, R31
    assert_eq!(decode(0x1BFF), Instruction::Sub { dest: 31, src: 31 });
}

// CP dest, src
#[test]
fn test_decode_cp() {
    // CP R16, R17
    assert_eq!(decode(0x1701), Instruction::Cp { dest: 16, src: 17 });
    // CP R0, R0
    assert_eq!(decode(0x1400), Instruction::Cp { dest: 0, src: 0 });
    // CP R31, R31
    assert_eq!(decode(0x17FF), Instruction::Cp { dest: 31, src: 31 });
}

//RJMP k
#[test]
fn test_decode_rjmp() {
    assert_eq!(decode(0xC000), Instruction::Rjmp { k: 0 });
    assert_eq!(decode(0xC001), Instruction::Rjmp { k: 1 });
    assert_eq!(decode(0xCFFF), Instruction::Rjmp { k: -1 });
    assert_eq!(decode(0xC7FF), Instruction::Rjmp { k: 2047 });
    assert_eq!(decode(0xC800), Instruction::Rjmp { k: -2048 });
}

//BRBS s, k
//BRBS s, k
#[test]
fn test_decode_branches() {
    // BREQ +1
    assert_eq!(decode(0xF009), Instruction::BRBS { s: 1, k: 1 });
    // BRNE -2
    assert_eq!(decode(0xF7F1), Instruction::BRBC { s: 1, k: -2 });
    // BRIE -1
    assert_eq!(decode(0xF3FF), Instruction::BRBS { s: 7, k: -1 });
    // BRCS -64 (lowest offset)
    assert_eq!(decode(0xF200), Instruction::BRBS { s: 0, k: -64 });
    // BRCS +63 (highest offset)
    assert_eq!(decode(0xF1F8), Instruction::BRBS { s: 0, k: 63 });
}

//INC Rd
//DEC Rd
#[test]
fn test_decode_inc_dec() {
    // INC R16 / INC R0 / INC R31
    assert_eq!(decode(0x9503), Instruction::Inc { rd: 16 });
    assert_eq!(decode(0x9403), Instruction::Inc { rd: 0 });
    assert_eq!(decode(0x95F3), Instruction::Inc { rd: 31 });
    // DEC R16 / DEC R0 / DEC R31
    assert_eq!(decode(0x950A), Instruction::Dec { rd: 16 });
    assert_eq!(decode(0x940A), Instruction::Dec { rd: 0 });
    assert_eq!(decode(0x95FA), Instruction::Dec { rd: 31 });
}

//And rd, rr
//Eor rd, rr
//Or  rd, rr
#[test]
fn test_decode_logic() {
    // AND R16, R17 / EOR R16, R17 / OR R16, R17
    assert_eq!(decode(0x2301), Instruction::And { rd: 16, rr: 17 });
    assert_eq!(decode(0x2701), Instruction::Eor { rd: 16, rr: 17 });
    assert_eq!(decode(0x2B01), Instruction::Or { rd: 16, rr: 17 });
    // AND R0, R0 (lowest registers)
    assert_eq!(decode(0x2000), Instruction::And { rd: 0, rr: 0 });
    // AND R31, R31 (highest registers)
    assert_eq!(decode(0x23FF), Instruction::And { rd: 31, rr: 31 });
}

//Mov Rd, Rr
#[test]
fn test_decode_mov() {
    // MOV R16, R17 / MOV R0, R17
    assert_eq!(decode(0x2F01), Instruction::Mov { rd: 16, rr: 17 });
    assert_eq!(decode(0x2E01), Instruction::Mov { rd: 0, rr: 17 });
    // MOV R0, R0 (lowest registers)
    assert_eq!(decode(0x2C00), Instruction::Mov { rd: 0, rr: 0 });
    // MOV R31, R31 (highest registers)
    assert_eq!(decode(0x2FFF), Instruction::Mov { rd: 31, rr: 31 });
}

//Adc rd, rr
//Sbc rd, rr
#[test]
fn test_decode_adc_sbc() {
    assert_eq!(decode(0x1F01), Instruction::Adc { rd: 16, rr: 17 });
    assert_eq!(decode(0x1F23), Instruction::Adc { rd: 18, rr: 19 });
    assert_eq!(decode(0x1C00), Instruction::Adc { rd: 0, rr: 0 });
    assert_eq!(decode(0x1FFF), Instruction::Adc { rd: 31, rr: 31 });
    assert_eq!(decode(0x0B01), Instruction::Sbc { rd: 16, rr: 17 });
    assert_eq!(decode(0x0800), Instruction::Sbc { rd: 0, rr: 0 });
    assert_eq!(decode(0x0BFF), Instruction::Sbc { rd: 31, rr: 31 });
}

//Subi rd, k
//Cpi  rd, k
#[test]
fn test_decode_subi_cpi() {
    // SUBI R16, 5 / SUBI R16, 0x80 / SUBI R16, 0xFF / SUBI R31, 0xFF
    assert_eq!(decode(0x5005), Instruction::Subi { rd: 16, k: 5 });
    assert_eq!(decode(0x5800), Instruction::Subi { rd: 16, k: 0x80 });
    assert_eq!(decode(0x5F0F), Instruction::Subi { rd: 16, k: 0xFF });
    assert_eq!(decode(0x5FFF), Instruction::Subi { rd: 31, k: 0xFF });
    // a constant with different high and low nibbles : SUBI R19, 0xAC
    assert_eq!(decode(0x5A3C), Instruction::Subi { rd: 19, k: 0xAC });
    // CPI R16, 5 / CPI R16, 0 / CPI R31, 0xFF
    assert_eq!(decode(0x3005), Instruction::Cpi { rd: 16, k: 5 });
    assert_eq!(decode(0x3000), Instruction::Cpi { rd: 16, k: 0 });
    assert_eq!(decode(0x3FFF), Instruction::Cpi { rd: 31, k: 0xFF });
    // CPI R19, 0xAC
    assert_eq!(decode(0x3A3C), Instruction::Cpi { rd: 19, k: 0xAC });
}

//ANDI Rd, K
//ORI  Rd, K
#[test]
fn test_decode_andi_ori() {
    // ANDI R16, 0x0F / ANDI R16, 0 / ANDI R31, 0xFF
    assert_eq!(decode(0x700F), Instruction::Andi { rd: 16, k: 0x0F });
    assert_eq!(decode(0x7000), Instruction::Andi { rd: 16, k: 0 });
    assert_eq!(decode(0x7FFF), Instruction::Andi { rd: 31, k: 0xFF });
    // a constant with different high and low nibbles : ANDI R19, 0xAC
    assert_eq!(decode(0x7A3C), Instruction::Andi { rd: 19, k: 0xAC });
    // ORI R16, 0x0F / ORI R31, 0xFF / ORI R19, 0xAC
    assert_eq!(decode(0x600F), Instruction::Ori { rd: 16, k: 0x0F });
    assert_eq!(decode(0x6FFF), Instruction::Ori { rd: 31, k: 0xFF });
    assert_eq!(decode(0x6A3C), Instruction::Ori { rd: 19, k: 0xAC });
}

//SBCI Rd, k
//CPC  Rd, Rr
#[test]
fn test_decode_sbci_cpc() {
    // SBCI R16, 5 / SBCI R16, 0 / SBCI R31, 0xFF
    assert_eq!(decode(0x4005), Instruction::Sbci { rd: 16, k: 5 });
    assert_eq!(decode(0x4000), Instruction::Sbci { rd: 16, k: 0 });
    assert_eq!(decode(0x4FFF), Instruction::Sbci { rd: 31, k: 0xFF });
    // a constant with different high and low nibbles : SBCI R19, 0xAC
    assert_eq!(decode(0x4A3C), Instruction::Sbci { rd: 19, k: 0xAC });
    // CPC R0, R0 / CPC R16, R17 / CPC R31, R31
    assert_eq!(decode(0x0400), Instruction::Cpc { rd: 0, rr: 0 });
    assert_eq!(decode(0x0701), Instruction::Cpc { rd: 16, rr: 17 });
    assert_eq!(decode(0x07FF), Instruction::Cpc { rd: 31, rr: 31 });
    // CPC R17, R19 : both register numbers have the split bit set or mixed
    assert_eq!(decode(0x0713), Instruction::Cpc { rd: 17, rr: 19 });
}

//COM Rd
//NEG Rd
#[test]
fn test_decode_com_neg() {
    // COM R16 / COM R0 / COM R31
    assert_eq!(decode(0x9500), Instruction::Com { rd: 16 });
    assert_eq!(decode(0x9400), Instruction::Com { rd: 0 });
    assert_eq!(decode(0x95F0), Instruction::Com { rd: 31 });
    // NEG R16 / NEG R0 / NEG R31
    assert_eq!(decode(0x9501), Instruction::Neg { rd: 16 });
    assert_eq!(decode(0x9401), Instruction::Neg { rd: 0 });
    assert_eq!(decode(0x95F1), Instruction::Neg { rd: 31 });
}

//ASR rd
//LSR rd
//ROR rd
#[test]
fn test_decode_shifts() {
    // ASR / LSR / ROR on R16
    assert_eq!(decode(0x9505), Instruction::Asr { rd: 16 });
    assert_eq!(decode(0x9506), Instruction::Lsr { rd: 16 });
    assert_eq!(decode(0x9507), Instruction::Ror { rd: 16 });
    // bounds : R0 and R31
    assert_eq!(decode(0x9405), Instruction::Asr { rd: 0 });
    assert_eq!(decode(0x9406), Instruction::Lsr { rd: 0 });
    assert_eq!(decode(0x9407), Instruction::Ror { rd: 0 });
    assert_eq!(decode(0x95F5), Instruction::Asr { rd: 31 });
    assert_eq!(decode(0x95F6), Instruction::Lsr { rd: 31 });
    assert_eq!(decode(0x95F7), Instruction::Ror { rd: 31 });
}

//SWAP Rd
#[test]
fn test_decode_swap() {
    // SWAP R16 / R0 / R31
    assert_eq!(decode(0x9502), Instruction::Swap { rd: 16 });
    assert_eq!(decode(0x9402), Instruction::Swap { rd: 0 });
    assert_eq!(decode(0x95F2), Instruction::Swap { rd: 31 });
}

//ADIW rd, k
//SBIW rd, k
#[test]
fn test_decode_adiw_sbiw() {
    // the four register pairs, K = 1
    assert_eq!(decode(0x9601), Instruction::Adiw { rd: 24, k: 1 });
    assert_eq!(decode(0x9611), Instruction::Adiw { rd: 26, k: 1 });
    assert_eq!(decode(0x9621), Instruction::Adiw { rd: 28, k: 1 });
    assert_eq!(decode(0x9631), Instruction::Adiw { rd: 30, k: 1 });
    // constants : 0, 63 and 42 (both halves non-zero, different)
    assert_eq!(decode(0x9600), Instruction::Adiw { rd: 24, k: 0 });
    assert_eq!(decode(0x96CF), Instruction::Adiw { rd: 24, k: 63 });
    assert_eq!(decode(0x96FF), Instruction::Adiw { rd: 30, k: 63 });
    assert_eq!(decode(0x968A), Instruction::Adiw { rd: 24, k: 42 });
    // SBIW
    assert_eq!(decode(0x9701), Instruction::Sbiw { rd: 24, k: 1 });
    assert_eq!(decode(0x9731), Instruction::Sbiw { rd: 30, k: 1 });
    assert_eq!(decode(0x97FF), Instruction::Sbiw { rd: 30, k: 63 });
    assert_eq!(decode(0x978A), Instruction::Sbiw { rd: 24, k: 42 });
}

//MOVW Rd, Rr
#[test]
fn test_decode_movw() {
    // MOVW R24, R26 / R26, R24
    assert_eq!(decode(0x01CD), Instruction::Movw { rd: 24, rr: 26 });
    assert_eq!(decode(0x01DC), Instruction::Movw { rd: 26, rr: 24 });
    // bounds : R0,R0 and R30,R30
    assert_eq!(decode(0x0100), Instruction::Movw { rd: 0, rr: 0 });
    assert_eq!(decode(0x01FF), Instruction::Movw { rd: 30, rr: 30 });
    // destination and source with different nibbles
    assert_eq!(decode(0x0108), Instruction::Movw { rd: 0, rr: 16 });
    assert_eq!(decode(0x0189), Instruction::Movw { rd: 16, rr: 18 });
    assert_eq!(decode(0x01EF), Instruction::Movw { rd: 28, rr: 30 });
}

//MUL Rd, Rr
//MULS Rd, Rr
//MULSU Rd, Rr
#[test]
fn test_decode_mul_muls_mulsu() {
    // MUL R16, R17 / MUL R0, R0 / MUL R31, R31
    assert_eq!(decode(0x9F01), Instruction::Mul { rd: 16, rr: 17 });
    assert_eq!(decode(0x9C00), Instruction::Mul { rd: 0, rr: 0 });
    assert_eq!(decode(0x9FFF), Instruction::Mul { rd: 31, rr: 31 });
    // MULS R16, R17 / MULS R16, R16 / MULS R31, R31
    assert_eq!(decode(0x0201), Instruction::Muls { rd: 16, rr: 17 });
    assert_eq!(decode(0x0200), Instruction::Muls { rd: 16, rr: 16 });
    assert_eq!(decode(0x02FF), Instruction::Muls { rd: 31, rr: 31 });
    // MULSU R16, R17 / MULSU R16, R16 / MULSU R23, R23
    assert_eq!(decode(0x0301), Instruction::Mulsu { rd: 16, rr: 17 });
    assert_eq!(decode(0x0300), Instruction::Mulsu { rd: 16, rr: 16 });
    assert_eq!(decode(0x0377), Instruction::Mulsu { rd: 23, rr: 23 });
}

//Bset s
//Bclr s
#[test]
fn test_decode_bset_bclr() {
    // SEC / SEI / SES : BSET 0, 7, 4
    assert_eq!(decode(0x9408), Instruction::Bset { s: 0 });
    assert_eq!(decode(0x9478), Instruction::Bset { s: 7 });
    assert_eq!(decode(0x9448), Instruction::Bset { s: 4 });
    // CLC / CLI / CLS : BCLR 0, 7, 4
    assert_eq!(decode(0x9488), Instruction::Bclr { s: 0 });
    assert_eq!(decode(0x94F8), Instruction::Bclr { s: 7 });
    assert_eq!(decode(0x94C8), Instruction::Bclr { s: 4 });
}
#[test]
fn test_decode_bset_bclr_all_flags() {
    // the word is built from s, like the MOVW test
    for s in 0u8..8 {
        let bset = 0x9408u16 | ((s as u16) << 4);
        let bclr = 0x9488u16 | ((s as u16) << 4);
        assert_eq!(decode(bset), Instruction::Bset { s }, "BSET {s}");
        assert_eq!(decode(bclr), Instruction::Bclr { s }, "BCLR {s}");
    }
}

//BLD Rd s
//BST Rd s
#[test]
fn test_decode_bst_bld() {
    // BLD R16,0 / R16,7 / R0,0 / R31,7
    assert_eq!(decode(0xF900), Instruction::Bld { rd: 16, b: 0 });
    assert_eq!(decode(0xF907), Instruction::Bld { rd: 16, b: 7 });
    assert_eq!(decode(0xF800), Instruction::Bld { rd: 0, b: 0 });
    assert_eq!(decode(0xF9F7), Instruction::Bld { rd: 31, b: 7 });
    // BST R16,0 / R16,7 / R0,0 / R31,7
    assert_eq!(decode(0xFB00), Instruction::Bst { rd: 16, b: 0 });
    assert_eq!(decode(0xFB07), Instruction::Bst { rd: 16, b: 7 });
    assert_eq!(decode(0xFA00), Instruction::Bst { rd: 0, b: 0 });
    assert_eq!(decode(0xFBF7), Instruction::Bst { rd: 31, b: 7 });
}
#[test]
fn test_decode_bst_bld_all_registers_and_bits() {
    // the word is built from rd and b, like the MOVW test
    for rd in 0u8..32 {
        for b in 0u8..8 {
            let low = ((rd as u16) << 4) | b as u16;
            assert_eq!(
                decode(0xF800 | low),
                Instruction::Bld { rd, b },
                "BLD R{rd},{b}"
            );
            assert_eq!(
                decode(0xFA00 | low),
                Instruction::Bst { rd, b },
                "BST R{rd},{b}"
            );
        }
    }
}

//FMUL rd, rr
//FMULS rd, rr
//FMULSU rd, rr
#[test]
fn test_decode_fmul_family() {
    // FMUL R16,R16 / R16,R17 / R23,R23
    assert_eq!(decode(0x0308), Instruction::Fmul { rd: 16, rr: 16 });
    assert_eq!(decode(0x0309), Instruction::Fmul { rd: 16, rr: 17 });
    assert_eq!(decode(0x037F), Instruction::Fmul { rd: 23, rr: 23 });
    // FMULS R16,R16 / R23,R23
    assert_eq!(decode(0x0380), Instruction::Fmuls { rd: 16, rr: 16 });
    assert_eq!(decode(0x03F7), Instruction::Fmuls { rd: 23, rr: 23 });
    // FMULSU R16,R16 / R23,R23
    assert_eq!(decode(0x0388), Instruction::Fmulsu { rd: 16, rr: 16 });
    assert_eq!(decode(0x03FF), Instruction::Fmulsu { rd: 23, rr: 23 });
}
#[test]
fn test_decode_fmul_family_all_registers() {
    // the word is built from d and r, like the MOVW test
    for d in 0u8..8 {
        for r in 0u8..8 {
            let low = ((d as u16) << 4) | r as u16;
            let (rd, rr) = (d + 16, r + 16);
            assert_eq!(
                decode(0x0308 | low),
                Instruction::Fmul { rd, rr },
                "FMUL R{rd},R{rr}"
            );
            assert_eq!(
                decode(0x0380 | low),
                Instruction::Fmuls { rd, rr },
                "FMULS R{rd},R{rr}"
            );
            assert_eq!(
                decode(0x0388 | low),
                Instruction::Fmulsu { rd, rr },
                "FMULSU R{rd},R{rr}"
            );
        }
    }
}

//Unknown
#[test]
fn test_decode_unknown() {
    assert_eq!(decode(0xFFFF), Instruction::Unknown(0xFFFF));
}

#[test]
fn fetch_reads_first_word() {
    let mut cpu = Cpu::new();
    cpu.flash[0] = 0xE005;
    assert_eq!(cpu.fetch(), Ok(0xE005));
    assert_eq!(cpu.pc, 1);
}

#[test]
fn fetch_reads_at_pc() {
    let mut cpu = Cpu::new();
    cpu.flash[60] = 0xE005;
    cpu.pc = 60;
    assert_eq!(cpu.fetch(), Ok(0xE005));
    assert_eq!(cpu.pc, 61);
}

#[test]
fn fetch_fails_past_flash_end() {
    let mut cpu = Cpu::new();
    cpu.flash[FLASH_SIZE - 1] = 0xE005;
    cpu.pc = (FLASH_SIZE - 1) as u16;
    assert_eq!(cpu.fetch(), Ok(0xE005));
    assert_eq!(cpu.pc as usize, FLASH_SIZE);
    assert!(cpu.fetch().is_err());
    assert_eq!(cpu.pc as usize, FLASH_SIZE);
}

#[test]
fn test_execute_nop() {
    let mut cpu = Cpu::new();
    let program: [u16; 1] = [0x0000];
    cpu.load_program(&program);
    assert_eq!(cpu.step(), Ok(1));
    assert_eq!(cpu.pc, 1);
    // NOP must not touch any register
    assert_eq!(cpu.registers, [0; CPU_REGISTERS_SIZE]);
}

#[test]
fn test_execute_ldi() {
    let mut cpu = Cpu::new();
    //LDI R16, 5
    let program: [u16; 1] = [0xE005];
    cpu.load_program(&program);
    assert_eq!(cpu.step(), Ok(1));
    assert_eq!(cpu.registers[16], 5);
    assert_eq!(cpu.pc, 1);
}

#[test]
fn test_execute_unknown() {
    let mut cpu = Cpu::new();
    let program: [u16; 1] = [0xFFFF];
    cpu.load_program(&program);
    assert!(cpu.step().is_err());
    assert_eq!(cpu.pc, 1);
}

#[test]
fn test_execute_ldi_high_bound() {
    let mut cpu = Cpu::new();
    // LDI R31, 255
    cpu.load_program(&[0xEFFF]);
    assert_eq!(cpu.step(), Ok(1));
    assert_eq!(cpu.registers[31], 255);
    assert_eq!(cpu.pc, 1);
}

#[test]
fn test_execute_two_ldi() {
    let mut cpu = Cpu::new();
    // LDI R16, 5 / LDI R23, 23
    cpu.load_program(&[0xE005, 0xE177]);
    assert_eq!(cpu.step(), Ok(1));
    assert_eq!(cpu.step(), Ok(1));
    assert_eq!(cpu.registers[16], 5);
    assert_eq!(cpu.registers[23], 23);
    assert_eq!(cpu.pc, 2);
}

#[test]
fn test_step_past_flash_end() {
    let mut cpu = Cpu::new();
    cpu.pc = FLASH_SIZE as u16;
    assert!(cpu.step().is_err());
    // A failed fetch must not move pc
    assert_eq!(cpu.pc as usize, FLASH_SIZE);
}

fn run_program(cpu: &mut Cpu, programm: &[u16]) {
    cpu.load_program(programm);
    for _ in 0..programm.len() {
        assert!(cpu.step().is_ok());
    }
}

#[test]
fn test_execute_add_no_on_flags() {
    let mut cpu = Cpu::new();
    // LDI R16, 5
    // LDI R17, 7
    // ADD R16, R17
    cpu.sreg = 0xFF;
    run_program(&mut cpu, &[0xE005, 0xE017, 0x0F01]);
    assert_eq!(cpu.registers[16], 12);
    assert_eq!(cpu.registers[17], 7);
    assert_eq!(cpu.sreg, 0x00 | FLAG_INTERRUPT_ENABLE | FLAG_BIT_COPY);
}

#[test]
fn test_execute_add_carry_flag_on() {
    let mut cpu = Cpu::new();
    // LDI R16, 200
    // LDI R17, 100
    // ADD R16, R17
    cpu.sreg = 0xFF;
    run_program(&mut cpu, &[0xEC08, 0xE614, 0x0F01]);
    assert_eq!(cpu.registers[16], 44);
    assert_eq!(cpu.registers[17], 100);
    assert_eq!(cpu.sreg, FLAG_CARRY | FLAG_INTERRUPT_ENABLE | FLAG_BIT_COPY);
}

#[test]
fn test_execute_add_carry_and_zero_and_half_flags_on() {
    let mut cpu = Cpu::new();
    // LDI R16, 255
    // LDI R17, 1
    // ADD R16, R17
    cpu.sreg = 0xFF;
    run_program(&mut cpu, &[0xEF0F, 0xE011, 0x0F01]);
    assert_eq!(cpu.registers[16], 0);
    assert_eq!(cpu.registers[17], 1);
    assert_eq!(
        cpu.sreg,
        FLAG_CARRY | FLAG_ZERO | FLAG_HALF_CARRY | FLAG_INTERRUPT_ENABLE | FLAG_BIT_COPY
    );
}

#[test]
fn test_execute_add_negative_and_overflow_flags_on() {
    let mut cpu = Cpu::new();
    // LDI R16, 100
    // LDI R17, 50
    // ADD R16, R17
    cpu.sreg = 0xFF;
    run_program(&mut cpu, &[0xE604, 0xE312, 0x0F01]);
    assert_eq!(cpu.registers[16], 150);
    assert_eq!(cpu.registers[17], 50);
    assert_eq!(
        cpu.sreg,
        FLAG_NEGATIVE | FLAG_OVERFLOW | FLAG_INTERRUPT_ENABLE | FLAG_BIT_COPY
    );
}

#[test]
fn test_execute_add_same_register() {
    let mut cpu = Cpu::new();
    // LDI R16, 5
    // ADD R16, R16
    cpu.sreg = 0xFF;
    run_program(&mut cpu, &[0xE005, 0x0F00]);
    assert_eq!(cpu.registers[16], 10);
    assert_eq!(cpu.sreg, 0x00 | FLAG_INTERRUPT_ENABLE | FLAG_BIT_COPY);
}

#[test]
fn test_execute_add_negative_overflow_half_carry_flags_on() {
    let mut cpu = Cpu::new();
    // LDI R16, 127
    // LDI R17, 1
    // ADD R16, R17
    cpu.sreg = 0xFF;
    run_program(&mut cpu, &[0xE70F, 0xE011, 0x0F01]);
    assert_eq!(cpu.registers[16], 128);
    assert_eq!(cpu.registers[17], 1);
    assert_eq!(
        cpu.sreg,
        FLAG_NEGATIVE | FLAG_OVERFLOW | FLAG_HALF_CARRY | FLAG_INTERRUPT_ENABLE | FLAG_BIT_COPY
    );
}

#[test]
fn test_execute_add_carry_zero_overflow_sign_flags_on() {
    let mut cpu = Cpu::new();
    // LDI R16, 128
    // LDI R17, 128
    // ADD R16, R17
    cpu.sreg = 0xFF;
    run_program(&mut cpu, &[0xE800, 0xE810, 0x0F01]);
    assert_eq!(cpu.registers[16], 0);
    assert_eq!(cpu.registers[17], 128);
    assert_eq!(
        cpu.sreg,
        FLAG_CARRY | FLAG_ZERO | FLAG_OVERFLOW | FLAG_SIGN | FLAG_INTERRUPT_ENABLE | FLAG_BIT_COPY
    );
}

#[test]
fn test_execute_add_half_carry_flag_on() {
    let mut cpu = Cpu::new();
    // LDI R16, 8
    // LDI R17, 8
    // ADD R16, R17
    cpu.sreg = 0xFF;
    run_program(&mut cpu, &[0xE008, 0xE018, 0x0F01]);
    assert_eq!(cpu.registers[16], 16);
    assert_eq!(cpu.registers[17], 8);
    assert_eq!(
        cpu.sreg,
        FLAG_HALF_CARRY | FLAG_INTERRUPT_ENABLE | FLAG_BIT_COPY
    );
}

#[test]
fn test_execute_sub_half_carry_flag_on() {
    let mut cpu = Cpu::new();
    // LDI R16, 20
    // LDI R17, 5
    // SUB R16, R17
    cpu.sreg = 0xFF;
    run_program(&mut cpu, &[0xE104, 0xE015, 0x1B01]);
    assert_eq!(cpu.registers[16], 15);
    assert_eq!(cpu.registers[17], 5);
    assert_eq!(
        cpu.sreg,
        FLAG_HALF_CARRY | FLAG_BIT_COPY | FLAG_INTERRUPT_ENABLE
    );
}

#[test]
fn test_execute_sub_zero_flag_on() {
    let mut cpu = Cpu::new();
    // LDI R16, 5
    // LDI R17, 5
    // SUB R16, R17
    cpu.sreg = 0xFF;
    run_program(&mut cpu, &[0xE005, 0xE015, 0x1B01]);
    assert_eq!(cpu.registers[16], 0);
    assert_eq!(cpu.registers[17], 5);
    assert_eq!(cpu.sreg, FLAG_ZERO | FLAG_BIT_COPY | FLAG_INTERRUPT_ENABLE);
}

#[test]
fn test_execute_sub_borrow_negative_half_sign_flags_on() {
    let mut cpu = Cpu::new();
    // LDI R16, 5
    // LDI R17, 10
    // SUB R16, R17
    cpu.sreg = 0xFF;
    run_program(&mut cpu, &[0xE005, 0xE01A, 0x1B01]);
    assert_eq!(cpu.registers[16], 251);
    assert_eq!(cpu.registers[17], 10);
    assert_eq!(
        cpu.sreg,
        FLAG_CARRY
            | FLAG_NEGATIVE
            | FLAG_HALF_CARRY
            | FLAG_SIGN
            | FLAG_BIT_COPY
            | FLAG_INTERRUPT_ENABLE
    );
}

#[test]
fn test_execute_sub_carry_negative_overflow_flags_on() {
    let mut cpu = Cpu::new();
    // LDI R16, 127
    // LDI R17, 255
    // SUB R16, R17
    cpu.sreg = 0xFF;
    run_program(&mut cpu, &[0xE70F, 0xEF1F, 0x1B01]);
    assert_eq!(cpu.registers[16], 128);
    assert_eq!(cpu.registers[17], 255);
    assert_eq!(
        cpu.sreg,
        FLAG_CARRY | FLAG_NEGATIVE | FLAG_OVERFLOW | FLAG_BIT_COPY | FLAG_INTERRUPT_ENABLE
    );
}

#[test]
fn test_execute_sub_half_overflow_sign_flags_on() {
    let mut cpu = Cpu::new();
    // LDI R16, 128
    // LDI R17, 1
    // SUB R16, R17
    cpu.sreg = 0xFF;
    run_program(&mut cpu, &[0xE800, 0xE011, 0x1B01]);
    assert_eq!(cpu.registers[16], 127);
    assert_eq!(cpu.registers[17], 1);
    assert_eq!(
        cpu.sreg,
        FLAG_HALF_CARRY | FLAG_OVERFLOW | FLAG_SIGN | FLAG_BIT_COPY | FLAG_INTERRUPT_ENABLE
    );
}

#[test]
fn test_execute_sub_same_register() {
    let mut cpu = Cpu::new();
    // LDI R16, 5
    // SUB R16, R16
    cpu.sreg = 0xFF;
    run_program(&mut cpu, &[0xE005, 0x1B00]);
    assert_eq!(cpu.registers[16], 0);
    assert_eq!(cpu.sreg, FLAG_ZERO | FLAG_BIT_COPY | FLAG_INTERRUPT_ENABLE);
}

#[test]
fn test_execute_sub_clears_arithmetic_flags_keeps_t_and_i() {
    let mut cpu = Cpu::new();
    cpu.sreg = 0xFF;
    // LDI R16, 20 / LDI R17, 17 / SUB R16, R17  -> 3, no flag
    run_program(&mut cpu, &[0xE104, 0xE111, 0x1B01]);
    assert_eq!(cpu.registers[16], 3);
    assert_eq!(cpu.sreg, FLAG_BIT_COPY | FLAG_INTERRUPT_ENABLE);
}

#[test]
fn test_execute_add_clears_arithmetic_flags_keeps_t_and_i() {
    let mut cpu = Cpu::new();
    cpu.sreg = 0xFF;
    // LDI R16, 5 / LDI R17, 7 / ADD R16, R17  -> 12, no flag
    run_program(&mut cpu, &[0xE005, 0xE017, 0x0F01]);
    assert_eq!(cpu.registers[16], 12);
    assert_eq!(cpu.sreg, FLAG_BIT_COPY | FLAG_INTERRUPT_ENABLE);
}

#[test]
fn test_decode_dest_src() {
    assert_eq!(decode_dest_src(0x0F01), (16, 17));
    assert_eq!(decode_dest_src(0x0FFF), (31, 31));
    assert_eq!(decode_dest_src(0x0C00), (0, 0));
}

#[test]
fn test_execute_cp_zero_flag_on() {
    let mut cpu = Cpu::new();
    // LDI R16, 5
    // LDI R17, 5
    // CP R16, R17
    run_program(&mut cpu, &[0xE005, 0xE015, 0x1701]);
    assert_eq!(cpu.registers[16], 5);
    assert_eq!(cpu.registers[17], 5);
    assert_eq!(cpu.sreg, FLAG_ZERO);
}

#[test]
fn test_execute_cp_half_carry_flag_on() {
    let mut cpu = Cpu::new();
    // LDI R16, 20
    // LDI R17, 5
    // CP R16, R17
    run_program(&mut cpu, &[0xE104, 0xE015, 0x1701]);
    assert_eq!(cpu.registers[16], 20);
    assert_eq!(cpu.registers[17], 5);
    assert_eq!(cpu.sreg, FLAG_HALF_CARRY);
}

#[test]
fn test_execute_cp_borrow_negative_half_sign_flags_on() {
    let mut cpu = Cpu::new();
    // LDI R16, 5
    // LDI R17, 10
    // CP R16, R17
    run_program(&mut cpu, &[0xE005, 0xE01A, 0x1701]);
    assert_eq!(cpu.registers[16], 5);
    assert_eq!(cpu.registers[17], 10);
    assert_eq!(
        cpu.sreg,
        FLAG_CARRY | FLAG_NEGATIVE | FLAG_HALF_CARRY | FLAG_SIGN
    );
}

#[test]
fn test_execute_cp_carry_negative_overflow_flags_on() {
    let mut cpu = Cpu::new();
    // LDI R16, 127
    // LDI R17, 255
    // CP R16, R17
    run_program(&mut cpu, &[0xE70F, 0xEF1F, 0x1701]);
    assert_eq!(cpu.registers[16], 127);
    assert_eq!(cpu.registers[17], 255);
    assert_eq!(cpu.sreg, FLAG_CARRY | FLAG_NEGATIVE | FLAG_OVERFLOW);
}

#[test]
fn test_execute_cp_half_overflow_sign_flags_on() {
    let mut cpu = Cpu::new();
    // LDI R16, 128
    // LDI R17, 1
    // CP R16, R17
    run_program(&mut cpu, &[0xE800, 0xE011, 0x1701]);
    assert_eq!(cpu.registers[16], 128);
    assert_eq!(cpu.registers[17], 1);
    assert_eq!(cpu.sreg, FLAG_HALF_CARRY | FLAG_OVERFLOW | FLAG_SIGN);
}

#[test]
fn test_execute_cp_same_register() {
    let mut cpu = Cpu::new();
    // LDI R16, 5
    // CP R16, R16
    run_program(&mut cpu, &[0xE005, 0x1700]);
    assert_eq!(cpu.registers[16], 5);
    assert_eq!(cpu.sreg, FLAG_ZERO);
}

#[test]
fn test_execute_cp_clears_arithmetic_flags_keeps_t_and_i() {
    let mut cpu = Cpu::new();
    cpu.sreg = 0xFF;
    // LDI R16, 20 / LDI R17, 17 / CP R16, R17  -> no flag
    run_program(&mut cpu, &[0xE104, 0xE111, 0x1701]);
    assert_eq!(cpu.registers[16], 20);
    assert_eq!(cpu.sreg, FLAG_BIT_COPY | FLAG_INTERRUPT_ENABLE);
}

#[test]
fn test_decode_rjmp_neighbours_are_not_rjmp() {
    // RCALL (1101), IN/OUT (1011) and LDI (1110) sit next to RJMP (1100)
    assert!(!matches!(decode(0xD000), Instruction::Rjmp { .. }));
    assert!(!matches!(decode(0xB000), Instruction::Rjmp { .. }));
    assert!(!matches!(decode(0xE000), Instruction::Rjmp { .. }));
}

#[test]
fn test_execute_rjmp_zero_goes_to_next_instruction() {
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xC000]);
    assert_eq!(cpu.step(), Ok(2));
    assert_eq!(cpu.pc, 1);
}

#[test]
fn test_execute_rjmp_forward_skips_instruction() {
    let mut cpu = Cpu::new();
    // RJMP +1 / LDI R16, 5 (skipped) / LDI R17, 7
    cpu.load_program(&[0xC001, 0xE005, 0xE017]);
    assert_eq!(cpu.step(), Ok(2));
    assert_eq!(cpu.step(), Ok(1));
    assert_eq!(cpu.registers[16], 0);
    assert_eq!(cpu.registers[17], 7);
    assert_eq!(cpu.pc, 3);
}

#[test]
fn test_execute_rjmp_loops_on_itself() {
    let mut cpu = Cpu::new();
    // RJMP -1
    cpu.load_program(&[0xCFFF]);
    for _ in 0..3 {
        assert_eq!(cpu.step(), Ok(2));
        assert_eq!(cpu.pc, 0);
    }
}

#[test]
fn test_execute_rjmp_backward() {
    let mut cpu = Cpu::new();
    // LDI R16, 5 / RJMP -2
    cpu.load_program(&[0xE005, 0xCFFE]);
    assert_eq!(cpu.step(), Ok(1));
    assert_eq!(cpu.step(), Ok(2));
    assert_eq!(cpu.registers[16], 5);
    assert_eq!(cpu.pc, 0);
}

#[test]
fn test_execute_rjmp_wraps_below_zero() {
    let mut cpu = Cpu::new();
    // RJMP -2 at address 0
    cpu.load_program(&[0xCFFE]);
    assert_eq!(cpu.step(), Ok(2));
    assert_eq!(cpu.pc as usize, FLASH_SIZE - 1);
}

#[test]
fn test_execute_rjmp_wraps_past_flash_end() {
    let mut cpu = Cpu::new();
    // RJMP +1 at the last address of the flash
    cpu.flash[FLASH_SIZE - 1] = 0xC001;
    cpu.pc = (FLASH_SIZE - 1) as u16;
    assert_eq!(cpu.step(), Ok(2));
    assert_eq!(cpu.pc, 1);
}

#[test]
fn test_execute_rjmp_does_not_touch_sreg() {
    let mut cpu = Cpu::new();
    cpu.sreg = 0xFF;
    cpu.load_program(&[0xC001]);
    assert_eq!(cpu.step(), Ok(2));
    assert_eq!(cpu.sreg, 0xFF);
}

fn step_n(cpu: &mut Cpu, steps: usize) {
    for _ in 0..steps {
        assert!(cpu.step().is_ok());
    }
}

#[test]
fn test_decode_branch_neighbours_are_not_branches() {
    // SBRC and BLD sit right after the branch group
    assert!(!matches!(
        decode(0xFC00),
        Instruction::BRBS { .. } | Instruction::BRBC { .. }
    ));
    assert!(!matches!(
        decode(0xF800),
        Instruction::BRBS { .. } | Instruction::BRBC { .. }
    ));
}

#[test]
fn test_execute_breq_taken() {
    let mut cpu = Cpu::new();
    // LDI R16, 5 / LDI R17, 5 / CP R16, R17 / BREQ +1
    // LDI R18, 1 (skipped) / LDI R19, 2
    cpu.load_program(&[0xE005, 0xE015, 0x1701, 0xF009, 0xE021, 0xE032]);
    step_n(&mut cpu, 5);
    assert_eq!(cpu.registers[18], 0);
    assert_eq!(cpu.registers[19], 2);
    assert_eq!(cpu.pc, 6);
}

#[test]
fn test_execute_breq_not_taken() {
    let mut cpu = Cpu::new();
    // Same program with R17 = 6, so Z stays off
    cpu.load_program(&[0xE005, 0xE016, 0x1701, 0xF009, 0xE021, 0xE032]);
    step_n(&mut cpu, 6);
    assert_eq!(cpu.registers[18], 1);
    assert_eq!(cpu.registers[19], 2);
    assert_eq!(cpu.pc, 6);
}

#[test]
fn test_execute_brne_countdown_loop() {
    let mut cpu = Cpu::new();
    // LDI R16, 3 / LDI R17, 1 / SUB R16, R17 / BRNE -2
    cpu.load_program(&[0xE003, 0xE011, 0x1B01, 0xF7F1]);
    step_n(&mut cpu, 8);
    assert_eq!(cpu.registers[16], 0);
    assert_eq!(cpu.registers[17], 1);
    assert_eq!(cpu.sreg, FLAG_ZERO);
    assert_eq!(cpu.pc, 4);
}

#[test]
fn test_execute_branch_does_not_touch_sreg() {
    let mut cpu = Cpu::new();
    cpu.sreg = 0xFF;
    // BREQ +1, taken because Z is on
    cpu.load_program(&[0xF009]);
    step_n(&mut cpu, 1);
    assert_eq!(cpu.sreg, 0xFF);
    assert_eq!(cpu.pc, 2);
}

#[test]
fn test_execute_branch_wraps_below_zero() {
    let mut cpu = Cpu::new();
    cpu.sreg = FLAG_CARRY;
    // BRCS -2 at address 0, taken because C is on
    cpu.load_program(&[0xF3F0]);
    step_n(&mut cpu, 1);
    assert_eq!(cpu.pc as usize, FLASH_SIZE - 1);
}

#[test]
fn test_decode_inc_dec_neighbours_are_not_inc_dec() {
    // COM, NEG, SWAP, ASR, LSR, ROR share the 1001 010d dddd xxxx group
    for word in [0x9400, 0x9401, 0x9402, 0x9405, 0x9406, 0x9407] {
        assert!(
            !matches!(
                decode(word),
                Instruction::Inc { .. } | Instruction::Dec { .. }
            ),
            "{word:#06X} must not decode as INC or DEC"
        );
    }
}

#[test]
fn test_execute_inc_basic() {
    let mut cpu = Cpu::new();
    // LDI R16, 5 / INC R16
    cpu.load_program(&[0xE005, 0x9503]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 6);
    assert_eq!(cpu.sreg, 0);
}

#[test]
fn test_execute_inc_wraps_to_zero() {
    let mut cpu = Cpu::new();
    // LDI R16, 255 / INC R16 : Z on, C untouched
    cpu.load_program(&[0xEF0F, 0x9503]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 0);
    assert_eq!(cpu.sreg, FLAG_ZERO);
}

#[test]
fn test_execute_inc_overflow() {
    let mut cpu = Cpu::new();
    // LDI R16, 127 / INC R16 : 127 -> 128, N and V (S = N xor V = 0)
    cpu.load_program(&[0xE70F, 0x9503]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 128);
    assert_eq!(cpu.sreg, FLAG_NEGATIVE | FLAG_OVERFLOW);
}

#[test]
fn test_execute_inc_negative() {
    let mut cpu = Cpu::new();
    // LDI R16, 128 / INC R16 : 128 -> 129, N and S
    cpu.load_program(&[0xE800, 0x9503]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 129);
    assert_eq!(cpu.sreg, FLAG_NEGATIVE | FLAG_SIGN);
}

#[test]
fn test_execute_dec_basic() {
    let mut cpu = Cpu::new();
    // LDI R16, 5 / DEC R16
    cpu.load_program(&[0xE005, 0x950A]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 4);
    assert_eq!(cpu.sreg, 0);
}

#[test]
fn test_execute_dec_to_zero() {
    let mut cpu = Cpu::new();
    // LDI R16, 1 / DEC R16
    cpu.load_program(&[0xE001, 0x950A]);
    step_n(&mut cpu, 1);
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[16], 0);
    assert_eq!(cpu.sreg, FLAG_ZERO);
}

#[test]
fn test_execute_dec_wraps_below_zero() {
    let mut cpu = Cpu::new();
    // DEC R16 with R16 = 0 : 0 -> 255, N and S, C untouched
    cpu.load_program(&[0x950A]);
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[16], 255);
    assert_eq!(cpu.sreg, FLAG_NEGATIVE | FLAG_SIGN);
}

#[test]
fn test_execute_dec_overflow() {
    let mut cpu = Cpu::new();
    // LDI R16, 128 / DEC R16 : 128 -> 127, V and S
    cpu.load_program(&[0xE800, 0x950A]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 127);
    assert_eq!(cpu.sreg, FLAG_OVERFLOW | FLAG_SIGN);
}

#[test]
fn test_execute_inc_keeps_i_t_h_c() {
    let mut cpu = Cpu::new();
    // LDI R16, 5 / INC R16 with every flag on before INC
    cpu.load_program(&[0xE005, 0x9503]);
    step_n(&mut cpu, 1);
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 1);
    // I, T, H, C kept ; Z, N, V, S cleared
    assert_eq!(cpu.sreg, 0xE1);
}

#[test]
fn test_execute_dec_keeps_i_t_h_c() {
    let mut cpu = Cpu::new();
    // LDI R16, 5 / DEC R16 with every flag on before DEC
    cpu.load_program(&[0xE005, 0x950A]);
    step_n(&mut cpu, 1);
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.sreg, 0xE1);
}

#[test]
fn test_execute_dec_brne_countdown_loop() {
    let mut cpu = Cpu::new();
    // LDI R16, 3 / DEC R16 / BRNE -2
    cpu.load_program(&[0xE003, 0x950A, 0xF7F1]);
    step_n(&mut cpu, 7);
    assert_eq!(cpu.registers[16], 0);
    assert_eq!(cpu.sreg, FLAG_ZERO);
    assert_eq!(cpu.pc, 3);
}

#[test]
fn test_decode_logic_neighbours_are_not_logic() {
    // MOV, ADC and CPI sit next to the logic group
    for word in [0x2C00, 0x1C00, 0x3000] {
        assert!(
            !matches!(
                decode(word),
                Instruction::And { .. } | Instruction::Or { .. } | Instruction::Eor { .. }
            ),
            "{word:#06X} must not decode as AND, OR or EOR"
        );
    }
}

#[test]
fn test_execute_and_basic() {
    let mut cpu = Cpu::new();
    // LDI R16, 0xFF / LDI R17, 0x0F / AND R16, R17
    cpu.load_program(&[0xEF0F, 0xE01F, 0x2301]);
    step_n(&mut cpu, 3);
    assert_eq!(cpu.registers[16], 0x0F);
    assert_eq!(cpu.registers[17], 0x0F);
    assert_eq!(cpu.sreg, 0);
}

#[test]
fn test_execute_and_zero() {
    let mut cpu = Cpu::new();
    // LDI R16, 0xF0 / LDI R17, 0x0F / AND R16, R17
    cpu.load_program(&[0xEF00, 0xE01F, 0x2301]);
    step_n(&mut cpu, 3);
    assert_eq!(cpu.registers[16], 0x00);
    assert_eq!(cpu.sreg, FLAG_ZERO);
}

#[test]
fn test_execute_and_negative() {
    let mut cpu = Cpu::new();
    // LDI R16, 0xFF / LDI R17, 0x80 / AND R16, R17 : N and S (V stays off)
    cpu.load_program(&[0xEF0F, 0xE810, 0x2301]);
    step_n(&mut cpu, 3);
    assert_eq!(cpu.registers[16], 0x80);
    assert_eq!(cpu.sreg, FLAG_NEGATIVE | FLAG_SIGN);
}

#[test]
fn test_execute_or_basic() {
    let mut cpu = Cpu::new();
    // LDI R16, 0x0A / LDI R17, 0x05 / OR R16, R17
    cpu.load_program(&[0xE00A, 0xE015, 0x2B01]);
    step_n(&mut cpu, 3);
    assert_eq!(cpu.registers[16], 0x0F);
    assert_eq!(cpu.sreg, 0);
}

#[test]
fn test_execute_or_zero() {
    let mut cpu = Cpu::new();
    // LDI R16, 0 / LDI R17, 0 / OR R16, R17
    cpu.load_program(&[0xE000, 0xE010, 0x2B01]);
    step_n(&mut cpu, 3);
    assert_eq!(cpu.registers[16], 0x00);
    assert_eq!(cpu.sreg, FLAG_ZERO);
}

#[test]
fn test_execute_or_negative() {
    let mut cpu = Cpu::new();
    // LDI R16, 0xF0 / LDI R17, 0x0F / OR R16, R17
    cpu.load_program(&[0xEF00, 0xE01F, 0x2B01]);
    step_n(&mut cpu, 3);
    assert_eq!(cpu.registers[16], 0xFF);
    assert_eq!(cpu.sreg, FLAG_NEGATIVE | FLAG_SIGN);
}

#[test]
fn test_execute_eor_basic() {
    let mut cpu = Cpu::new();
    // LDI R16, 0x55 / LDI R17, 0x0F / EOR R16, R17
    cpu.load_program(&[0xE505, 0xE01F, 0x2701]);
    step_n(&mut cpu, 3);
    assert_eq!(cpu.registers[16], 0x5A);
    assert_eq!(cpu.sreg, 0);
}

#[test]
fn test_execute_eor_clears_register() {
    let mut cpu = Cpu::new();
    // LDI R16, 0xAA / EOR R16, R16 (the CLR idiom)
    cpu.load_program(&[0xEA0A, 0x2700]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 0x00);
    assert_eq!(cpu.sreg, FLAG_ZERO);
}

#[test]
fn test_execute_eor_negative() {
    let mut cpu = Cpu::new();
    // LDI R16, 0xFF / LDI R17, 0x0F / EOR R16, R17
    cpu.load_program(&[0xEF0F, 0xE01F, 0x2701]);
    step_n(&mut cpu, 3);
    assert_eq!(cpu.registers[16], 0xF0);
    assert_eq!(cpu.sreg, FLAG_NEGATIVE | FLAG_SIGN);
}

#[test]
fn test_execute_and_keeps_i_t_h_c() {
    let mut cpu = Cpu::new();
    // LDI R16, 0xFF / LDI R17, 0x0F / AND R16, R17 with every flag on before AND
    cpu.load_program(&[0xEF0F, 0xE01F, 0x2301]);
    step_n(&mut cpu, 2);
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 1);
    // I, T, H, C kept ; Z, N, V, S cleared (V is forced to 0)
    assert_eq!(cpu.sreg, 0xE1);
}

#[test]
fn test_execute_or_keeps_i_t_h_c() {
    let mut cpu = Cpu::new();
    // LDI R16, 0x0A / LDI R17, 0x05 / OR R16, R17
    cpu.load_program(&[0xE00A, 0xE015, 0x2B01]);
    step_n(&mut cpu, 2);
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.sreg, 0xE1);
}

#[test]
fn test_execute_eor_keeps_i_t_h_c() {
    let mut cpu = Cpu::new();
    // LDI R16, 0x55 / LDI R17, 0x0F / EOR R16, R17
    cpu.load_program(&[0xE505, 0xE01F, 0x2701]);
    step_n(&mut cpu, 2);
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.sreg, 0xE1);
}

#[test]
fn test_decode_mov_neighbours_are_not_mov() {
    // OR sits right before MOV, CPI right after the logic group
    for word in [0x2800, 0x2BFF, 0x3000] {
        assert!(
            !matches!(decode(word), Instruction::Mov { .. }),
            "{word:#06X} must not decode as MOV"
        );
    }
}

#[test]
fn test_execute_mov_basic() {
    let mut cpu = Cpu::new();
    // LDI R16, 5 / LDI R17, 7 / MOV R16, R17
    cpu.load_program(&[0xE005, 0xE017, 0x2F01]);
    step_n(&mut cpu, 3);
    assert_eq!(cpu.registers[16], 7);
    // the source is left untouched
    assert_eq!(cpu.registers[17], 7);
    assert_eq!(cpu.sreg, 0);
}

#[test]
fn test_execute_mov_zero_does_not_set_z() {
    let mut cpu = Cpu::new();
    // LDI R16, 5 / LDI R17, 0 / MOV R16, R17
    cpu.load_program(&[0xE005, 0xE010, 0x2F01]);
    step_n(&mut cpu, 3);
    assert_eq!(cpu.registers[16], 0);
    assert_eq!(cpu.sreg, 0);
}

#[test]
fn test_execute_mov_negative_does_not_set_n() {
    let mut cpu = Cpu::new();
    // LDI R16, 5 / LDI R17, 0x80 / MOV R16, R17
    cpu.load_program(&[0xE005, 0xE810, 0x2F01]);
    step_n(&mut cpu, 3);
    assert_eq!(cpu.registers[16], 0x80);
    assert_eq!(cpu.sreg, 0);
}

#[test]
fn test_execute_mov_same_register() {
    let mut cpu = Cpu::new();
    // LDI R16, 5 / MOV R16, R16
    cpu.load_program(&[0xE005, 0x2F00]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 5);
}

#[test]
fn test_execute_mov_to_low_register() {
    let mut cpu = Cpu::new();
    // LDI R17, 7 / MOV R0, R17
    cpu.load_program(&[0xE017, 0x2E01]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[0], 7);
    assert_eq!(cpu.registers[17], 7);
}

#[test]
fn test_execute_mov_does_not_touch_sreg() {
    let mut cpu = Cpu::new();
    // LDI R16, 5 / LDI R17, 7 / MOV R16, R17 with every flag on before MOV
    cpu.load_program(&[0xE005, 0xE017, 0x2F01]);
    step_n(&mut cpu, 2);
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 1);
    // nothing is cleared, unlike INC and AND (0xE1)
    assert_eq!(cpu.sreg, 0xFF);
}

#[test]
fn test_decode_adc_sbc_neighbours() {
    // SUB, CP, CPSE next to ADC ; ADD, CPC next to SBC
    for word in [0x1BFF, 0x17FF, 0x1000] {
        assert!(!matches!(decode(word), Instruction::Adc { .. }));
    }
    for word in [0x0C00, 0x07FF, 0x0400] {
        assert!(!matches!(decode(word), Instruction::Sbc { .. }));
    }
}

#[test]
fn test_execute_adc_carry_off_is_add() {
    let mut cpu = Cpu::new();
    // LDI R16, 5 / LDI R17, 7 / ADC R16, R17 (C = 0)
    cpu.load_program(&[0xE005, 0xE017, 0x1F01]);
    step_n(&mut cpu, 3);
    assert_eq!(cpu.registers[16], 12);
    assert_eq!(cpu.sreg, 0);
}

#[test]
fn test_execute_adc_carry_on_adds_one() {
    let mut cpu = Cpu::new();
    // same program, C = 1 before ADC : 5 + 7 + 1
    cpu.load_program(&[0xE005, 0xE017, 0x1F01]);
    step_n(&mut cpu, 2);
    cpu.sreg = FLAG_CARRY;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[16], 13);
    // the old carry is consumed, not kept
    assert_eq!(cpu.sreg, 0);
}

#[test]
fn test_execute_adc_carry_out_with_carry_off() {
    let mut cpu = Cpu::new();
    // LDI R16, 200 / LDI R17, 100 / ADC R16, R17 : 300 -> 44, C
    cpu.load_program(&[0xEC08, 0xE614, 0x1F01]);
    step_n(&mut cpu, 3);
    assert_eq!(cpu.registers[16], 44);
    assert_eq!(cpu.sreg, FLAG_CARRY);
}

#[test]
fn test_execute_adc_carry_in_makes_carry_out() {
    let mut cpu = Cpu::new();
    // LDI R16, 255 / LDI R17, 0 / ADC R16, R17 with C = 1 : 256 -> 0, C Z H
    cpu.load_program(&[0xEF0F, 0xE010, 0x1F01]);
    step_n(&mut cpu, 2);
    cpu.sreg = FLAG_CARRY;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[16], 0);
    assert_eq!(cpu.sreg, FLAG_CARRY | FLAG_ZERO | FLAG_HALF_CARRY);
}

#[test]
fn test_execute_adc_carry_in_makes_overflow() {
    let mut cpu = Cpu::new();
    // LDI R16, 127 / LDI R17, 0 / ADC R16, R17 with C = 1 : 128, N V H
    cpu.load_program(&[0xE70F, 0xE010, 0x1F01]);
    step_n(&mut cpu, 2);
    cpu.sreg = FLAG_CARRY;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[16], 128);
    assert_eq!(cpu.sreg, FLAG_NEGATIVE | FLAG_OVERFLOW | FLAG_HALF_CARRY);
}

#[test]
fn test_execute_adc_16_bit_addition() {
    let mut cpu = Cpu::new();
    // 0x01FF + 0x0001 : LDI R16, 255 / LDI R17, 1 / LDI R18, 1 / LDI R19, 0
    // ADD R16, R17 / ADC R18, R19
    cpu.load_program(&[0xEF0F, 0xE011, 0xE021, 0xE030, 0x0F01, 0x1F23]);
    step_n(&mut cpu, 6);
    assert_eq!(cpu.registers[16], 0);
    assert_eq!(cpu.registers[18], 2);
}

#[test]
fn test_execute_sbc_carry_off_is_sub() {
    let mut cpu = Cpu::new();
    // LDI R16, 5 / LDI R17, 3 / SBC R16, R17 (C = 0)
    cpu.load_program(&[0xE005, 0xE013, 0x0B01]);
    step_n(&mut cpu, 3);
    assert_eq!(cpu.registers[16], 2);
    assert_eq!(cpu.sreg, 0);
}

#[test]
fn test_execute_sbc_carry_on_subtracts_one() {
    let mut cpu = Cpu::new();
    // same program, C = 1 before SBC : 5 - 3 - 1
    cpu.load_program(&[0xE005, 0xE013, 0x0B01]);
    step_n(&mut cpu, 2);
    cpu.sreg = FLAG_CARRY;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[16], 1);
    assert_eq!(cpu.sreg, 0);
}

#[test]
fn test_execute_sbc_borrow_from_carry_in() {
    let mut cpu = Cpu::new();
    // LDI R16, 5 / LDI R17, 5 / SBC R16, R17 with C = 1 : 255, C N H S
    cpu.load_program(&[0xE005, 0xE015, 0x0B01]);
    step_n(&mut cpu, 2);
    cpu.sreg = FLAG_CARRY;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[16], 255);
    assert_eq!(
        cpu.sreg,
        FLAG_CARRY | FLAG_NEGATIVE | FLAG_HALF_CARRY | FLAG_SIGN
    );
}

#[test]
fn test_execute_sbc_zero_stays_on_if_it_was_on() {
    let mut cpu = Cpu::new();
    // 5 - 5 - 0 = 0 with Z already on : Z stays on
    cpu.load_program(&[0xE005, 0xE015, 0x0B01]);
    step_n(&mut cpu, 2);
    cpu.sreg = FLAG_ZERO;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[16], 0);
    assert_eq!(cpu.sreg, FLAG_ZERO);
}

#[test]
fn test_execute_sbc_zero_is_not_set_if_it_was_off() {
    let mut cpu = Cpu::new();
    // 5 - 5 - 0 = 0 with Z off : Z must NOT be set
    cpu.load_program(&[0xE005, 0xE015, 0x0B01]);
    step_n(&mut cpu, 3);
    assert_eq!(cpu.registers[16], 0);
    assert_eq!(cpu.sreg, 0);
}

#[test]
fn test_execute_sbc_16_bit_subtraction() {
    let mut cpu = Cpu::new();
    // 0x0100 - 0x0001 : LDI R16, 0 / LDI R17, 1 / LDI R18, 1 / LDI R19, 0
    // SUB R16, R18 / SBC R17, R19
    cpu.load_program(&[0xE000, 0xE011, 0xE021, 0xE030, 0x1B02, 0x0B13]);
    step_n(&mut cpu, 6);
    assert_eq!(cpu.registers[16], 255);
    assert_eq!(cpu.registers[17], 0);
    // the result is 0x00FF, not zero : Z must be off
    assert_eq!(cpu.sreg, 0);
}

#[test]
fn test_execute_adc_sbc_keep_i_t() {
    let mut cpu = Cpu::new();
    // ADC 5 + 7 + 1 with every flag on before
    cpu.load_program(&[0xE005, 0xE017, 0x1F01]);
    step_n(&mut cpu, 2);
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.sreg, FLAG_INTERRUPT_ENABLE | FLAG_BIT_COPY);

    // SBC 5 - 3 - 1 with every flag on before
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xE005, 0xE013, 0x0B01]);
    step_n(&mut cpu, 2);
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.sreg, FLAG_INTERRUPT_ENABLE | FLAG_BIT_COPY);
}

#[test]
fn test_decode_subi_cpi_neighbours() {
    // SBCI, ORI, ANDI are not implemented yet : they must not decode as SUBI or CPI
    for word in [0x4000, 0x6000, 0x7000, 0x4FFF] {
        assert!(
            !matches!(
                decode(word),
                Instruction::Subi { .. } | Instruction::Cpi { .. }
            ),
            "{word:#06X} must not decode as SUBI or CPI"
        );
    }
    // SUBI and CPI do not decode as each other
    assert!(!matches!(decode(0x5005), Instruction::Cpi { .. }));
    assert!(!matches!(decode(0x3005), Instruction::Subi { .. }));
}

#[test]
fn test_execute_subi_half_carry() {
    let mut cpu = Cpu::new();
    // LDI R16, 20 / SUBI R16, 5
    cpu.load_program(&[0xE104, 0x5005]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 15);
    assert_eq!(cpu.sreg, FLAG_HALF_CARRY);
}

#[test]
fn test_execute_subi_zero() {
    let mut cpu = Cpu::new();
    // LDI R16, 5 / SUBI R16, 5
    cpu.load_program(&[0xE005, 0x5005]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 0);
    assert_eq!(cpu.sreg, FLAG_ZERO);
}

#[test]
fn test_execute_subi_borrow() {
    let mut cpu = Cpu::new();
    // LDI R16, 5 / SUBI R16, 10 : 251, C N H S
    cpu.load_program(&[0xE005, 0x500A]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 251);
    assert_eq!(
        cpu.sreg,
        FLAG_CARRY | FLAG_NEGATIVE | FLAG_HALF_CARRY | FLAG_SIGN
    );
}

#[test]
fn test_execute_subi_overflow_with_carry() {
    let mut cpu = Cpu::new();
    // LDI R16, 127 / SUBI R16, 255 : 128, C N V
    cpu.load_program(&[0xE70F, 0x5F0F]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 128);
    assert_eq!(cpu.sreg, FLAG_CARRY | FLAG_NEGATIVE | FLAG_OVERFLOW);
}

#[test]
fn test_execute_subi_overflow() {
    let mut cpu = Cpu::new();
    // LDI R16, 128 / SUBI R16, 1 : 127, H V S
    cpu.load_program(&[0xE800, 0x5001]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 127);
    assert_eq!(cpu.sreg, FLAG_HALF_CARRY | FLAG_OVERFLOW | FLAG_SIGN);
}

#[test]
fn test_execute_subi_high_register() {
    let mut cpu = Cpu::new();
    // LDI R31, 5 / SUBI R31, 5
    cpu.load_program(&[0xE0F5, 0x50F5]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[31], 0);
    assert_eq!(cpu.sreg, FLAG_ZERO);
}

#[test]
fn test_execute_subi_adds_with_negative_constant() {
    let mut cpu = Cpu::new();
    // LDI R16, 5 / SUBI R16, 0xFF : subtracting -1 adds 1, the compiler's way of adding
    cpu.load_program(&[0xE005, 0x5F0F]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 6);
}

#[test]
fn test_execute_cpi_equal() {
    let mut cpu = Cpu::new();
    // LDI R16, 5 / CPI R16, 5
    cpu.load_program(&[0xE005, 0x3005]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 5);
    assert_eq!(cpu.sreg, FLAG_ZERO);
}

#[test]
fn test_execute_cpi_lower() {
    let mut cpu = Cpu::new();
    // LDI R16, 5 / CPI R16, 10 : C N H S, R16 untouched
    cpu.load_program(&[0xE005, 0x300A]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 5);
    assert_eq!(
        cpu.sreg,
        FLAG_CARRY | FLAG_NEGATIVE | FLAG_HALF_CARRY | FLAG_SIGN
    );
}

#[test]
fn test_execute_cpi_brne_loop() {
    let mut cpu = Cpu::new();
    // LDI R16, 5 / CPI R16, 5 / BREQ +1 / LDI R17, 1 (skipped) / LDI R18, 2
    cpu.load_program(&[0xE005, 0x3005, 0xF009, 0xE011, 0xE022]);
    step_n(&mut cpu, 4);
    assert_eq!(cpu.registers[17], 0);
    assert_eq!(cpu.registers[18], 2);
    assert_eq!(cpu.pc, 5);
}

#[test]
fn test_execute_subi_cpi_keep_i_t_only() {
    // SUBI R16, 3 on 5 : result 2, no flag, every flag on before
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xE005, 0x5003]);
    step_n(&mut cpu, 1);
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.sreg, FLAG_INTERRUPT_ENABLE | FLAG_BIT_COPY);

    // CPI R16, 3 on 5, same check
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xE005, 0x3003]);
    step_n(&mut cpu, 1);
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.sreg, FLAG_INTERRUPT_ENABLE | FLAG_BIT_COPY);
}

#[test]
fn test_decode_andi_ori_neighbours() {
    // SUBI, SBCI, CPI and the 0x8000 group are not ANDI or ORI
    for word in [0x5000, 0x4000, 0x3000, 0x8000] {
        assert!(
            !matches!(
                decode(word),
                Instruction::Andi { .. } | Instruction::Ori { .. }
            ),
            "{word:#06X} must not decode as ANDI or ORI"
        );
    }
    // ANDI and ORI do not decode as each other
    assert!(!matches!(decode(0x7000), Instruction::Ori { .. }));
    assert!(!matches!(decode(0x6000), Instruction::Andi { .. }));
}

#[test]
fn test_execute_andi_basic() {
    let mut cpu = Cpu::new();
    // LDI R16, 0xFF / ANDI R16, 0x0F
    cpu.load_program(&[0xEF0F, 0x700F]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 0x0F);
    assert_eq!(cpu.sreg, 0);
}

#[test]
fn test_execute_andi_zero() {
    let mut cpu = Cpu::new();
    // LDI R16, 0xF0 / ANDI R16, 0x0F
    cpu.load_program(&[0xEF00, 0x700F]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 0x00);
    assert_eq!(cpu.sreg, FLAG_ZERO);
}

#[test]
fn test_execute_andi_negative() {
    let mut cpu = Cpu::new();
    // LDI R16, 0xFF / ANDI R16, 0x80 : N and S (V stays off)
    cpu.load_program(&[0xEF0F, 0x7800]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 0x80);
    assert_eq!(cpu.sreg, FLAG_NEGATIVE | FLAG_SIGN);
}

#[test]
fn test_execute_andi_clears_one_bit() {
    let mut cpu = Cpu::new();
    // LDI R16, 0xFF / ANDI R16, 0xDF (the compiler's way of clearing bit 5)
    cpu.load_program(&[0xEF0F, 0x7D0F]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 0xDF);
    assert_eq!(cpu.sreg, FLAG_NEGATIVE | FLAG_SIGN);
}

#[test]
fn test_execute_andi_high_register() {
    let mut cpu = Cpu::new();
    // LDI R31, 0xFF / ANDI R31, 0x0F
    cpu.load_program(&[0xEFFF, 0x70FF]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[31], 0x0F);
    assert_eq!(cpu.sreg, 0);
}

#[test]
fn test_execute_ori_basic() {
    let mut cpu = Cpu::new();
    // LDI R16, 0x0A / ORI R16, 0x05
    cpu.load_program(&[0xE00A, 0x6005]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 0x0F);
    assert_eq!(cpu.sreg, 0);
}

#[test]
fn test_execute_ori_zero() {
    let mut cpu = Cpu::new();
    // LDI R16, 0 / ORI R16, 0
    cpu.load_program(&[0xE000, 0x6000]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 0x00);
    assert_eq!(cpu.sreg, FLAG_ZERO);
}

#[test]
fn test_execute_ori_negative() {
    let mut cpu = Cpu::new();
    // LDI R16, 0xF0 / ORI R16, 0x0F
    cpu.load_program(&[0xEF00, 0x600F]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 0xFF);
    assert_eq!(cpu.sreg, FLAG_NEGATIVE | FLAG_SIGN);
}

#[test]
fn test_execute_ori_sets_one_bit() {
    let mut cpu = Cpu::new();
    // LDI R16, 0 / ORI R16, 0x20 (the compiler's way of setting bit 5)
    cpu.load_program(&[0xE000, 0x6200]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 0x20);
    assert_eq!(cpu.sreg, 0);
}

#[test]
fn test_execute_andi_ori_keep_i_t_h_c() {
    // ANDI R16, 0x0F on 0xFF : result 0x0F, every flag on before
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xEF0F, 0x700F]);
    step_n(&mut cpu, 1);
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 1);
    // I, T, H, C kept ; Z, N, V, S cleared
    assert_eq!(cpu.sreg, 0xE1);

    // ORI R16, 0x05 on 0x0A : result 0x0F, same check
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xE00A, 0x6005]);
    step_n(&mut cpu, 1);
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.sreg, 0xE1);
}

#[test]
fn test_decode_sbci_cpc_neighbours() {
    // CPI, SUBI, SBC, ADD and the 0x0300 hole are not SBCI or CPC
    for word in [0x3000, 0x5000, 0x0800, 0x0BFF, 0x0C00, 0x03FF, 0x0300] {
        assert!(
            !matches!(
                decode(word),
                Instruction::Sbci { .. } | Instruction::Cpc { .. }
            ),
            "{word:#06X} must not decode as SBCI or CPC"
        );
    }
    // SBCI and CPC do not decode as each other
    assert!(!matches!(decode(0x4000), Instruction::Cpc { .. }));
    assert!(!matches!(decode(0x0400), Instruction::Sbci { .. }));
    // 0x0000 is still NOP
    assert!(matches!(decode(0x0000), Instruction::Nop));
}

#[test]
fn test_execute_sbci_basic() {
    let mut cpu = Cpu::new();
    // LDI R16, 10 / SBCI R16, 5 (carry off, same as SUBI)
    cpu.load_program(&[0xE00A, 0x4005]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 5);
    assert_eq!(cpu.sreg, 0);
}

#[test]
fn test_execute_sbci_zero_stays_off() {
    let mut cpu = Cpu::new();
    // LDI R16, 5 / SBCI R16, 5 : result 0 but Z was off, so Z stays off
    cpu.load_program(&[0xE005, 0x4005]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 0);
    assert_eq!(cpu.sreg, 0);
}

#[test]
fn test_execute_sbci_zero_kept_on() {
    let mut cpu = Cpu::new();
    // LDI R16, 5 / SBCI R16, 5 with Z on before : Z stays on
    cpu.load_program(&[0xE005, 0x4005]);
    step_n(&mut cpu, 1);
    cpu.sreg = FLAG_ZERO;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[16], 0);
    assert_eq!(cpu.sreg, FLAG_ZERO);
}

#[test]
fn test_execute_sbci_carry_in() {
    let mut cpu = Cpu::new();
    // LDI R16, 5 / SBCI R16, 5 with C on : 5 - 5 - 1 = 0xFF
    cpu.load_program(&[0xE005, 0x4005]);
    step_n(&mut cpu, 1);
    cpu.sreg = FLAG_CARRY;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[16], 0xFF);
    assert_eq!(
        cpu.sreg,
        FLAG_CARRY | FLAG_NEGATIVE | FLAG_HALF_CARRY | FLAG_SIGN
    );
}

#[test]
fn test_execute_sbci_carry_in_overflow() {
    let mut cpu = Cpu::new();
    // LDI R16, 128 / SBCI R16, 0 with C on : 128 - 0 - 1 = 127
    // only the incoming carry makes V (and H) here
    cpu.load_program(&[0xE800, 0x4000]);
    step_n(&mut cpu, 1);
    cpu.sreg = FLAG_CARRY;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[16], 0x7F);
    assert_eq!(cpu.sreg, FLAG_OVERFLOW | FLAG_SIGN | FLAG_HALF_CARRY);
}

#[test]
fn test_execute_sbci_high_register() {
    let mut cpu = Cpu::new();
    // LDI R31, 0xFF / SBCI R31, 0x0F
    cpu.load_program(&[0xEFFF, 0x40FF]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[31], 0xF0);
    assert_eq!(cpu.sreg, FLAG_NEGATIVE | FLAG_SIGN);
}

#[test]
fn test_execute_subi_sbci_chain_16_bits() {
    let mut cpu = Cpu::new();
    // R17:R16 = 0x0100 ; SUBI R16, 1 / SBCI R17, 0 : 0x0100 - 1 = 0x00FF
    // the high byte is 0 but the whole result is not : Z must stay off
    cpu.load_program(&[0xE000, 0xE011, 0x5001, 0x4010]);
    step_n(&mut cpu, 4);
    assert_eq!(cpu.registers[16], 0xFF);
    assert_eq!(cpu.registers[17], 0x00);
    assert_eq!(cpu.sreg, 0);
}

#[test]
fn test_execute_subi_sbci_chain_16_bits_zero() {
    let mut cpu = Cpu::new();
    // R17:R16 = 0x0100 ; SUBI R16, 0 / SBCI R17, 1 : 0x0100 - 0x0100 = 0
    cpu.load_program(&[0xE000, 0xE011, 0x5000, 0x4011]);
    step_n(&mut cpu, 4);
    assert_eq!(cpu.registers[16], 0x00);
    assert_eq!(cpu.registers[17], 0x00);
    assert_eq!(cpu.sreg, FLAG_ZERO);
}

#[test]
fn test_execute_cpc_equal_16_bits() {
    let mut cpu = Cpu::new();
    // R17:R16 = 0x0100, R19:R18 = 0x0100
    // CP R16, R18 / CPC R17, R19
    cpu.load_program(&[0xE000, 0xE011, 0xE020, 0xE031, 0x1702, 0x0713]);
    step_n(&mut cpu, 6);
    assert_eq!(cpu.sreg, FLAG_ZERO);
    // nothing written
    assert_eq!(cpu.registers[16], 0x00);
    assert_eq!(cpu.registers[17], 0x01);
}

#[test]
fn test_execute_cpc_greater_16_bits() {
    let mut cpu = Cpu::new();
    // R17:R16 = 0x0100, R19:R18 = 0x0001 : A > B
    cpu.load_program(&[0xE000, 0xE011, 0xE021, 0xE030, 0x1702, 0x0713]);
    step_n(&mut cpu, 6);
    assert_eq!(cpu.sreg, 0);
    assert_eq!(cpu.registers[17], 0x01);
}

#[test]
fn test_execute_cpc_less_16_bits() {
    let mut cpu = Cpu::new();
    // R17:R16 = 0x0001, R19:R18 = 0x0100 : A < B
    cpu.load_program(&[0xE001, 0xE010, 0xE020, 0xE031, 0x1702, 0x0713]);
    step_n(&mut cpu, 6);
    assert_eq!(
        cpu.sreg,
        FLAG_CARRY | FLAG_NEGATIVE | FLAG_HALF_CARRY | FLAG_SIGN
    );
    // CPC must not write : R17 is still 0 (it would be 0xFF otherwise)
    assert_eq!(cpu.registers[17], 0x00);
}

#[test]
fn test_execute_cpc_low_byte_differs() {
    let mut cpu = Cpu::new();
    // R17:R16 = 0x0001, R19:R18 = 0x0000 : high bytes are equal, but the
    // low bytes are not, so Z must stay off
    cpu.load_program(&[0xE001, 0xE010, 0xE020, 0xE030, 0x1702, 0x0713]);
    step_n(&mut cpu, 6);
    assert_eq!(cpu.sreg, 0);
}

#[test]
fn test_execute_sbci_cpc_keep_i_t() {
    // SBCI R16, 5 on 10 with I and T on : result 5, no flag of its own
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xE00A, 0x4005]);
    step_n(&mut cpu, 1);
    cpu.sreg = FLAG_BIT_COPY | FLAG_INTERRUPT_ENABLE;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.sreg, 0xC0);

    // SBCI R16, 0x0F on 0xFF, every flag on before : 0xFF - 0x0F - 1 = 0xEF
    // I, T kept ; H, N, S on ; C, V, Z off
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xEF0F, 0x400F]);
    step_n(&mut cpu, 1);
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[16], 0xEF);
    assert_eq!(cpu.sreg, 0xF4);

    // CPC R16, R17 on 0xFF and 0x0F, every flag on before : same flags,
    // and R16 untouched
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xEF0F, 0xE01F, 0x0701]);
    step_n(&mut cpu, 2);
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[16], 0xFF);
    assert_eq!(cpu.sreg, 0xF4);
}

#[test]
fn test_decode_com_neg_neighbours() {
    // SWAP, INC, ASR, DEC, SEC and the 0x9000 group are not COM or NEG
    for word in [0x9402, 0x9403, 0x9405, 0x940A, 0x9408, 0x9000] {
        assert!(
            !matches!(
                decode(word),
                Instruction::Com { .. } | Instruction::Neg { .. }
            ),
            "{word:#06X} must not decode as COM or NEG"
        );
    }
    // COM and NEG do not decode as each other
    assert!(!matches!(decode(0x9500), Instruction::Neg { .. }));
    assert!(!matches!(decode(0x9501), Instruction::Com { .. }));
    // INC and DEC are still decoded
    assert_eq!(decode(0x9503), Instruction::Inc { rd: 16 });
    assert_eq!(decode(0x950A), Instruction::Dec { rd: 16 });
}

#[test]
fn test_execute_com_zero() {
    let mut cpu = Cpu::new();
    // LDI R16, 0x00 / COM R16 : C always on, N and S, no V
    cpu.load_program(&[0xE000, 0x9500]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 0xFF);
    assert_eq!(cpu.sreg, FLAG_CARRY | FLAG_NEGATIVE | FLAG_SIGN);
}

#[test]
fn test_execute_com_ff() {
    let mut cpu = Cpu::new();
    // LDI R16, 0xFF / COM R16 : C and Z together
    cpu.load_program(&[0xEF0F, 0x9500]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 0x00);
    assert_eq!(cpu.sreg, FLAG_CARRY | FLAG_ZERO);
}

#[test]
fn test_execute_com_low_nibble() {
    let mut cpu = Cpu::new();
    // LDI R16, 0x0F / COM R16
    cpu.load_program(&[0xE00F, 0x9500]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 0xF0);
    assert_eq!(cpu.sreg, FLAG_CARRY | FLAG_NEGATIVE | FLAG_SIGN);
}

#[test]
fn test_execute_com_high_nibble() {
    let mut cpu = Cpu::new();
    // LDI R16, 0xF0 / COM R16 : only C
    cpu.load_program(&[0xEF00, 0x9500]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 0x0F);
    assert_eq!(cpu.sreg, FLAG_CARRY);
}

#[test]
fn test_execute_com_twice() {
    let mut cpu = Cpu::new();
    // LDI R16, 0xAC is 0xEA0C / COM R16 / COM R16 : back to the start
    cpu.load_program(&[0xEA0C, 0x9500, 0x9500]);
    step_n(&mut cpu, 3);
    assert_eq!(cpu.registers[16], 0xAC);
}

#[test]
fn test_execute_neg_basic() {
    let mut cpu = Cpu::new();
    // LDI R16, 5 / NEG R16 : 0 - 5 = 0xFB
    cpu.load_program(&[0xE005, 0x9501]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 0xFB);
    assert_eq!(
        cpu.sreg,
        FLAG_CARRY | FLAG_NEGATIVE | FLAG_HALF_CARRY | FLAG_SIGN
    );
}

#[test]
fn test_execute_neg_one() {
    let mut cpu = Cpu::new();
    // LDI R16, 1 / NEG R16 : 0 - 1 = 0xFF
    cpu.load_program(&[0xE001, 0x9501]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 0xFF);
    assert_eq!(
        cpu.sreg,
        FLAG_CARRY | FLAG_NEGATIVE | FLAG_HALF_CARRY | FLAG_SIGN
    );
}

#[test]
fn test_execute_neg_zero() {
    let mut cpu = Cpu::new();
    // LDI R16, 0 / NEG R16 : only Z, no carry
    cpu.load_program(&[0xE000, 0x9501]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 0x00);
    assert_eq!(cpu.sreg, FLAG_ZERO);
}

#[test]
fn test_execute_neg_min() {
    let mut cpu = Cpu::new();
    // LDI R16, 0x80 / NEG R16 : no opposite on 8 bits, V on and so S off
    cpu.load_program(&[0xE800, 0x9501]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 0x80);
    assert_eq!(cpu.sreg, FLAG_CARRY | FLAG_NEGATIVE | FLAG_OVERFLOW);
}

#[test]
fn test_execute_neg_no_half_carry() {
    let mut cpu = Cpu::new();
    // LDI R16, 0x10 / NEG R16 : low nibble is 0, so no H
    cpu.load_program(&[0xE100, 0x9501]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 0xF0);
    assert_eq!(cpu.sreg, FLAG_CARRY | FLAG_NEGATIVE | FLAG_SIGN);
}

#[test]
fn test_execute_neg_ff() {
    let mut cpu = Cpu::new();
    // LDI R16, 0xFF / NEG R16 : 0 - 0xFF = 1
    cpu.load_program(&[0xEF0F, 0x9501]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 0x01);
    assert_eq!(cpu.sreg, FLAG_CARRY | FLAG_HALF_CARRY);
}

#[test]
fn test_execute_com_neg_flag_preservation() {
    // COM R16 on 0xF0 (result 0x0F), every flag on before :
    // I, T, H kept ; C on ; Z, N, V, S cleared
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xEF00, 0x9500]);
    step_n(&mut cpu, 1);
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[16], 0x0F);
    assert_eq!(cpu.sreg, 0xE1);

    // NEG R16 on 0x10 (result 0xF0), every flag on before :
    // only I and T kept ; H is recomputed (off here) ; C, N, S on
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xE100, 0x9501]);
    step_n(&mut cpu, 1);
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[16], 0xF0);
    assert_eq!(cpu.sreg, 0xD5);
}

// Loads one register with LDI, sets sreg, then runs one unary instruction.
fn run_unary(ldi: u16, op: u16, sreg_before: u8) -> Cpu {
    let mut cpu = Cpu::new();
    cpu.load_program(&[ldi, op]);
    step_n(&mut cpu, 1);
    cpu.sreg = sreg_before;
    step_n(&mut cpu, 1);
    cpu
}

#[test]
fn test_decode_shifts_neighbours() {
    // COM, NEG, SWAP, INC, SEC, DEC are not shifts
    for word in [0x9400, 0x9401, 0x9402, 0x9403, 0x9408, 0x940A] {
        assert!(
            !matches!(
                decode(word),
                Instruction::Lsr { .. } | Instruction::Asr { .. } | Instruction::Ror { .. }
            ),
            "{word:#06X} must not decode as a shift"
        );
    }
    // 0x9404 is not an instruction at all
    assert!(matches!(decode(0x9404), Instruction::Unknown { .. }));
    // the three shifts do not decode as each other
    assert!(!matches!(
        decode(0x9505),
        Instruction::Lsr { .. } | Instruction::Ror { .. }
    ));
    assert!(!matches!(
        decode(0x9506),
        Instruction::Asr { .. } | Instruction::Ror { .. }
    ));
    assert!(!matches!(
        decode(0x9507),
        Instruction::Asr { .. } | Instruction::Lsr { .. }
    ));
}

#[test]
fn test_execute_lsr() {
    // LSR R16 = 0x9506 ; LDI R16, x : 0xE004 (4), 0xE005 (5), 0xE001 (1), 0xE000 (0),
    // 0xEF0F (0xFF), 0xE800 (0x80), 0xE008 (8)
    let cpu = run_unary(0xE004, 0x9506, 0);
    assert_eq!((cpu.registers[16], cpu.sreg), (0x02, 0), "LSR 4");

    let cpu = run_unary(0xE005, 0x9506, 0);
    assert_eq!(
        (cpu.registers[16], cpu.sreg),
        (0x02, FLAG_CARRY | FLAG_OVERFLOW | FLAG_SIGN),
        "LSR 5"
    );

    let cpu = run_unary(0xE001, 0x9506, 0);
    assert_eq!(
        (cpu.registers[16], cpu.sreg),
        (0x00, FLAG_CARRY | FLAG_ZERO | FLAG_OVERFLOW | FLAG_SIGN),
        "LSR 1"
    );

    let cpu = run_unary(0xE000, 0x9506, 0);
    assert_eq!((cpu.registers[16], cpu.sreg), (0x00, FLAG_ZERO), "LSR 0");

    // N never on, even for a "negative" input : a 0 always enters bit 7
    let cpu = run_unary(0xEF0F, 0x9506, 0);
    assert_eq!(
        (cpu.registers[16], cpu.sreg),
        (0x7F, FLAG_CARRY | FLAG_OVERFLOW | FLAG_SIGN),
        "LSR 0xFF"
    );

    let cpu = run_unary(0xE800, 0x9506, 0);
    assert_eq!((cpu.registers[16], cpu.sreg), (0x40, 0), "LSR 0x80");

    // 0x08 gives 0x04 : bit 2 of the result must not be mistaken for N
    let cpu = run_unary(0xE008, 0x9506, 0);
    assert_eq!((cpu.registers[16], cpu.sreg), (0x04, 0), "LSR 8");
}

#[test]
fn test_execute_asr() {
    // ASR R16 = 0x9505 ; LDI R16, 0x81 = 0xE801 ; LDI R16, 0xF0 = 0xEF00
    let cpu = run_unary(0xE004, 0x9505, 0);
    assert_eq!((cpu.registers[16], cpu.sreg), (0x02, 0), "ASR 4");

    let cpu = run_unary(0xE005, 0x9505, 0);
    assert_eq!(
        (cpu.registers[16], cpu.sreg),
        (0x02, FLAG_CARRY | FLAG_OVERFLOW | FLAG_SIGN),
        "ASR 5"
    );

    let cpu = run_unary(0xE000, 0x9505, 0);
    assert_eq!((cpu.registers[16], cpu.sreg), (0x00, FLAG_ZERO), "ASR 0");

    let cpu = run_unary(0xE001, 0x9505, 0);
    assert_eq!(
        (cpu.registers[16], cpu.sreg),
        (0x00, FLAG_CARRY | FLAG_ZERO | FLAG_OVERFLOW | FLAG_SIGN),
        "ASR 1"
    );

    // -128 / 2 = -64
    let cpu = run_unary(0xE800, 0x9505, 0);
    assert_eq!(
        (cpu.registers[16], cpu.sreg),
        (0xC0, FLAG_NEGATIVE | FLAG_OVERFLOW),
        "ASR 0x80"
    );

    let cpu = run_unary(0xE801, 0x9505, 0);
    assert_eq!(
        (cpu.registers[16], cpu.sreg),
        (0xC0, FLAG_CARRY | FLAG_NEGATIVE | FLAG_SIGN),
        "ASR 0x81"
    );

    // -1 stays -1 : an ASR written as an LSR would give 0x7F
    let cpu = run_unary(0xEF0F, 0x9505, 0);
    assert_eq!(
        (cpu.registers[16], cpu.sreg),
        (0xFF, FLAG_CARRY | FLAG_NEGATIVE | FLAG_SIGN),
        "ASR 0xFF"
    );

    let cpu = run_unary(0xEF00, 0x9505, 0);
    assert_eq!(
        (cpu.registers[16], cpu.sreg),
        (0xF8, FLAG_NEGATIVE | FLAG_OVERFLOW),
        "ASR 0xF0"
    );
}

#[test]
fn test_execute_ror() {
    // ROR R16 = 0x9507 ; LDI R16, 2 = 0xE002
    let cpu = run_unary(0xE002, 0x9507, 0);
    assert_eq!((cpu.registers[16], cpu.sreg), (0x01, 0), "ROR 2, C off");

    // the old carry enters bit 7
    let cpu = run_unary(0xE002, 0x9507, FLAG_CARRY);
    assert_eq!(
        (cpu.registers[16], cpu.sreg),
        (0x81, FLAG_NEGATIVE | FLAG_OVERFLOW),
        "ROR 2, C on"
    );

    let cpu = run_unary(0xE001, 0x9507, 0);
    assert_eq!(
        (cpu.registers[16], cpu.sreg),
        (0x00, FLAG_CARRY | FLAG_ZERO | FLAG_OVERFLOW | FLAG_SIGN),
        "ROR 1, C off"
    );

    let cpu = run_unary(0xE001, 0x9507, FLAG_CARRY);
    assert_eq!(
        (cpu.registers[16], cpu.sreg),
        (0x80, FLAG_CARRY | FLAG_NEGATIVE | FLAG_SIGN),
        "ROR 1, C on"
    );

    let cpu = run_unary(0xE000, 0x9507, FLAG_CARRY);
    assert_eq!(
        (cpu.registers[16], cpu.sreg),
        (0x80, FLAG_NEGATIVE | FLAG_OVERFLOW),
        "ROR 0, C on"
    );

    let cpu = run_unary(0xE000, 0x9507, 0);
    assert_eq!(
        (cpu.registers[16], cpu.sreg),
        (0x00, FLAG_ZERO),
        "ROR 0, C off"
    );
}

#[test]
fn test_execute_lsr_ror_chain_16_bits() {
    let mut cpu = Cpu::new();
    // R17:R16 = 0x0102 ; LSR R17 / ROR R16 : 0x0102 >> 1 = 0x0081
    // the low bit of the high byte goes through C into bit 7 of the low byte
    cpu.load_program(&[0xE002, 0xE011, 0x9516, 0x9507]);
    step_n(&mut cpu, 4);
    assert_eq!(cpu.registers[17], 0x00);
    assert_eq!(cpu.registers[16], 0x81);
    assert_eq!(cpu.sreg, FLAG_NEGATIVE | FLAG_OVERFLOW);
}

#[test]
fn test_execute_shifts_flag_preservation() {
    // R16 = 0x04, every flag on before : I, T, H kept ; C, Z, N, V, S recomputed
    // LSR : result 0x02, C off, nothing else on
    let cpu = run_unary(0xE004, 0x9506, 0xFF);
    assert_eq!(cpu.sreg, 0xE0);

    // ASR : same
    let cpu = run_unary(0xE004, 0x9505, 0xFF);
    assert_eq!(cpu.sreg, 0xE0);

    // ROR : C was on, so the result is 0x82 : N and V on, C off, S off
    let cpu = run_unary(0xE004, 0x9507, 0xFF);
    assert_eq!(cpu.registers[16], 0x82);
    assert_eq!(cpu.sreg, 0xEC);
}

#[test]
fn test_decode_swap_neighbours() {
    // COM, NEG, INC, ASR, SEC are not SWAP
    for word in [0x9400, 0x9401, 0x9403, 0x9405, 0x9408] {
        assert!(
            !matches!(decode(word), Instruction::Swap { .. }),
            "{word:#06X} must not decode as SWAP"
        );
    }
    // 0x9404 is still not an instruction
    assert!(matches!(decode(0x9404), Instruction::Unknown { .. }));
    // the other members of the group are still decoded
    assert_eq!(decode(0x9500), Instruction::Com { rd: 16 });
    assert_eq!(decode(0x9501), Instruction::Neg { rd: 16 });
    assert_eq!(decode(0x9503), Instruction::Inc { rd: 16 });
}

#[test]
fn test_execute_swap() {
    // SWAP R16 = 0x9502 ; LDI R16, x : 0xEA0B (0xAB), 0xE00F (0x0F), 0xEF00 (0xF0),
    // 0xE000 (0x00), 0xEF0F (0xFF), 0xE800 (0x80), 0xE008 (0x08)
    // different nibbles : catches a swap done on one half only
    let cpu = run_unary(0xEA0B, 0x9502, 0);
    assert_eq!((cpu.registers[16], cpu.sreg), (0xBA, 0), "SWAP 0xAB");

    // result has bit 7 on : N must stay off
    let cpu = run_unary(0xE00F, 0x9502, 0);
    assert_eq!((cpu.registers[16], cpu.sreg), (0xF0, 0), "SWAP 0x0F");

    let cpu = run_unary(0xEF00, 0x9502, 0);
    assert_eq!((cpu.registers[16], cpu.sreg), (0x0F, 0), "SWAP 0xF0");

    // zero result : Z must stay off
    let cpu = run_unary(0xE000, 0x9502, 0);
    assert_eq!((cpu.registers[16], cpu.sreg), (0x00, 0), "SWAP 0x00");

    let cpu = run_unary(0xEF0F, 0x9502, 0);
    assert_eq!((cpu.registers[16], cpu.sreg), (0xFF, 0), "SWAP 0xFF");

    let cpu = run_unary(0xE800, 0x9502, 0);
    assert_eq!((cpu.registers[16], cpu.sreg), (0x08, 0), "SWAP 0x80");

    let cpu = run_unary(0xE008, 0x9502, 0);
    assert_eq!((cpu.registers[16], cpu.sreg), (0x80, 0), "SWAP 0x08");
}

#[test]
fn test_execute_swap_twice() {
    let mut cpu = Cpu::new();
    // LDI R16, 0xAB / SWAP R16 / SWAP R16 : back to the start
    cpu.load_program(&[0xEA0B, 0x9502, 0x9502]);
    step_n(&mut cpu, 3);
    assert_eq!(cpu.registers[16], 0xAB);
}

#[test]
fn test_execute_swap_high_register() {
    let mut cpu = Cpu::new();
    // LDI R31, 0xAB / SWAP R31 : the 5 bits of Rd go all the way up
    cpu.load_program(&[0xEAFB, 0x95F2]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[31], 0xBA);
}

#[test]
fn test_execute_swap_other_register_untouched() {
    let mut cpu = Cpu::new();
    // LDI R17, 0x12 / LDI R16, 0xAB / SWAP R16 : R17 must not move
    cpu.load_program(&[0xE112, 0xEA0B, 0x9502]);
    step_n(&mut cpu, 3);
    assert_eq!(cpu.registers[16], 0xBA);
    assert_eq!(cpu.registers[17], 0x12);
}

#[test]
fn test_execute_swap_keeps_all_flags() {
    // every flag on before : SWAP must not touch SREG at all
    let cpu = run_unary(0xEA0B, 0x9502, 0xFF);
    assert_eq!(cpu.registers[16], 0xBA);
    assert_eq!(cpu.sreg, 0xFF);

    // every flag off before : still off
    let cpu = run_unary(0xEA0B, 0x9502, 0x00);
    assert_eq!(cpu.sreg, 0x00);
}

// Loads a program and runs one step per word (no jumps).
fn run_words(program: &[u16]) -> Cpu {
    let mut cpu = Cpu::new();
    cpu.load_program(program);
    step_n(&mut cpu, program.len());
    cpu
}

// Reads the 16-bit value held by registers lo+1 (high byte) and lo (low byte).
fn reg_pair(cpu: &Cpu, lo: usize) -> u16 {
    ((cpu.registers[lo + 1] as u16) << 8) | cpu.registers[lo] as u16
}

#[test]
fn test_decode_adiw_sbiw_neighbours() {
    // COM, RET, the 0x95xx group, CBI, MUL and 0x9400 are not ADIW or SBIW
    for word in [0x9400, 0x9500, 0x9508, 0x95FF, 0x9800, 0x9C00] {
        assert!(
            !matches!(
                decode(word),
                Instruction::Adiw { .. } | Instruction::Sbiw { .. }
            ),
            "{word:#06X} must not decode as ADIW or SBIW"
        );
    }
    // ADIW and SBIW do not decode as each other
    assert!(!matches!(decode(0x9601), Instruction::Sbiw { .. }));
    assert!(!matches!(decode(0x9701), Instruction::Adiw { .. }));
    // the single-register group is still decoded
    assert_eq!(decode(0x9503), Instruction::Inc { rd: 16 });
}

#[test]
fn test_execute_adiw() {
    // LDI R24, x : 0xE085 (5), 0xEF8F (0xFF), 0xE080 (0), 0xE384 (0x34)
    // LDI R25, x : 0xE090 (0), 0xE091 (1), 0xE79F (0x7F), 0xE890 (0x80), 0xEF9F (0xFF), 0xE192 (0x12)
    // ADIW R24, 1 = 0x9601 ; ADIW R24, 0 = 0x9600 ; ADIW R24, 63 = 0x96CF ; ADIW R24, 42 = 0x968A
    let cases: &[(&[u16], u16, u8, &str)] = &[
        (&[0xE085, 0xE090, 0x9601], 0x0006, 0, "0x0005 + 1"),
        // the carry goes from the low byte to the high byte
        (&[0xEF8F, 0xE090, 0x9601], 0x0100, 0, "0x00FF + 1"),
        // non-zero high byte : catches a bad 16-bit assembly
        (&[0xE080, 0xE091, 0x9601], 0x0101, 0, "0x0100 + 1"),
        // all four nibbles are different
        (&[0xE384, 0xE192, 0x968A], 0x125E, 0, "0x1234 + 42"),
        (
            &[0xEF8F, 0xEF9F, 0x9601],
            0x0000,
            FLAG_CARRY | FLAG_ZERO,
            "0xFFFF + 1",
        ),
        // N and V together, so S is off
        (
            &[0xEF8F, 0xE79F, 0x9601],
            0x8000,
            FLAG_NEGATIVE | FLAG_OVERFLOW,
            "0x7FFF + 1",
        ),
        // V must not mean "result == 0x8000"
        (
            &[0xEF8F, 0xE79F, 0x96CF],
            0x803E,
            FLAG_NEGATIVE | FLAG_OVERFLOW,
            "0x7FFF + 63",
        ),
        (
            &[0xE080, 0xE890, 0x9601],
            0x8001,
            FLAG_NEGATIVE | FLAG_SIGN,
            "0x8000 + 1",
        ),
        (&[0xE080, 0xE090, 0x9600], 0x0000, FLAG_ZERO, "0x0000 + 0"),
        (&[0xEF8F, 0xEF9F, 0x96CF], 0x003E, FLAG_CARRY, "0xFFFF + 63"),
        (&[0xE080, 0xE090, 0x968A], 0x002A, 0, "0x0000 + 42"),
    ];
    for (program, expected, flags, label) in cases {
        let cpu = run_words(program);
        assert_eq!(reg_pair(&cpu, 24), *expected, "{label} : value");
        assert_eq!(cpu.sreg, *flags, "{label} : sreg");
    }
}

#[test]
fn test_execute_sbiw() {
    // SBIW R24, 1 = 0x9701 ; SBIW R24, 0 = 0x9700 ; SBIW R24, 63 = 0x97CF ; SBIW R24, 42 = 0x978A
    // LDI R24, x : 0xE086 (6), 0xE081 (1), 0xE180 (0x10)
    let cases: &[(&[u16], u16, u8, &str)] = &[
        (&[0xE086, 0xE090, 0x9701], 0x0005, 0, "0x0006 - 1"),
        // the borrow comes from the high byte
        (&[0xE080, 0xE091, 0x9701], 0x00FF, 0, "0x0100 - 1"),
        // low byte is 0 but the whole value is not : Z must stay off
        (&[0xE081, 0xE091, 0x9701], 0x0100, 0, "0x0101 - 1"),
        (&[0xE384, 0xE192, 0x978A], 0x120A, 0, "0x1234 - 42"),
        (&[0xE081, 0xE090, 0x9701], 0x0000, FLAG_ZERO, "0x0001 - 1"),
        (
            &[0xE080, 0xE090, 0x9701],
            0xFFFF,
            FLAG_CARRY | FLAG_NEGATIVE | FLAG_SIGN,
            "0x0000 - 1",
        ),
        (
            &[0xE080, 0xE890, 0x9701],
            0x7FFF,
            FLAG_OVERFLOW | FLAG_SIGN,
            "0x8000 - 1",
        ),
        // V must not mean "result == 0x7FFF"
        (
            &[0xE080, 0xE890, 0x97CF],
            0x7FC1,
            FLAG_OVERFLOW | FLAG_SIGN,
            "0x8000 - 63",
        ),
        (
            &[0xE081, 0xE890, 0x9701],
            0x8000,
            FLAG_NEGATIVE | FLAG_SIGN,
            "0x8001 - 1",
        ),
        (
            &[0xE180, 0xE090, 0x97CF],
            0xFFD1,
            FLAG_CARRY | FLAG_NEGATIVE | FLAG_SIGN,
            "0x0010 - 63",
        ),
        (&[0xE080, 0xE090, 0x9700], 0x0000, FLAG_ZERO, "0x0000 - 0"),
    ];
    for (program, expected, flags, label) in cases {
        let cpu = run_words(program);
        assert_eq!(reg_pair(&cpu, 24), *expected, "{label} : value");
        assert_eq!(cpu.sreg, *flags, "{label} : sreg");
    }
}

#[test]
fn test_execute_adiw_other_pairs() {
    // R27:R26 (X) : 0x00FF + 1
    let cpu = run_words(&[0xEFAF, 0xE0B0, 0x9611]);
    assert_eq!(reg_pair(&cpu, 26), 0x0100);
    // R29:R28 (Y)
    let cpu = run_words(&[0xEFCF, 0xE0D0, 0x9621]);
    assert_eq!(reg_pair(&cpu, 28), 0x0100);
    // R31:R30 (Z) ; R25:R24 must not move : LDI R24, 0x55 first
    let cpu = run_words(&[0xE585, 0xEFEF, 0xE0F0, 0x9631]);
    assert_eq!(reg_pair(&cpu, 30), 0x0100);
    assert_eq!(cpu.registers[24], 0x55);
    assert_eq!(cpu.registers[25], 0x00);
}

#[test]
fn test_execute_sbiw_other_pairs() {
    // R27:R26 (X) : 0x0100 - 1
    let cpu = run_words(&[0xE0A0, 0xE0B1, 0x9711]);
    assert_eq!(reg_pair(&cpu, 26), 0x00FF);
    // R29:R28 (Y)
    let cpu = run_words(&[0xE0C0, 0xE0D1, 0x9721]);
    assert_eq!(reg_pair(&cpu, 28), 0x00FF);
    // R31:R30 (Z) : 0 - 63
    let cpu = run_words(&[0xE0E0, 0xE0F0, 0x97FF]);
    assert_eq!(reg_pair(&cpu, 30), 0xFFC1);
    assert_eq!(cpu.sreg, FLAG_CARRY | FLAG_NEGATIVE | FLAG_SIGN);
}

#[test]
fn test_execute_sbiw_delay_loop() {
    // LDI R24, 3 / LDI R25, 0 / SBIW R24, 1 / BRNE -2
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xE083, 0xE090, 0x9701, 0xF7F1]);
    step_n(&mut cpu, 8);
    assert_eq!(reg_pair(&cpu, 24), 0);
    assert_eq!(cpu.sreg, FLAG_ZERO);
    assert_eq!(cpu.pc, 4);
}

#[test]
fn test_execute_sbiw_delay_loop_crosses_byte() {
    // 0x0101 : the low byte reaches 0 at 0x0100, the loop must go on
    // until the whole 16-bit value is 0 : 257 turns of 2 steps + 2 LDI
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xE081, 0xE091, 0x9701, 0xF7F1]);
    step_n(&mut cpu, 516);
    assert_eq!(reg_pair(&cpu, 24), 0);
    assert_eq!(cpu.sreg, FLAG_ZERO);
    assert_eq!(cpu.pc, 4);
}

#[test]
fn test_execute_adiw_sbiw_flag_preservation() {
    // ADIW R24, 1 on 0x0005, then SBIW R24, 1 on 0x0006 : result without flags.
    // every flag on before : I, T, H kept ; C, Z, N, V, S recomputed (all off)
    for program in [[0xE085, 0xE090, 0x9601], [0xE086, 0xE090, 0x9701]] {
        let mut cpu = Cpu::new();
        cpu.load_program(&program);
        step_n(&mut cpu, 2);
        cpu.sreg = 0xFF;
        step_n(&mut cpu, 1);
        assert_eq!(cpu.sreg, 0xE0);
    }
}

#[test]
fn test_decode_movw_neighbours() {
    // 0x00FF, MULS, the 0x0300 group, CPC and ADD are not MOVW
    for word in [0x00FF, 0x0200, 0x0300, 0x0400, 0x0BFF] {
        assert!(
            !matches!(decode(word), Instruction::Movw { .. }),
            "{word:#06X} must not decode as MOVW"
        );
    }
    // NOP and CPC are still decoded
    assert!(matches!(decode(0x0000), Instruction::Nop));
    assert_eq!(decode(0x0400), Instruction::Cpc { rd: 0, rr: 0 });
    // 0x00FF is not an instruction
    assert!(matches!(decode(0x00FF), Instruction::Unknown { .. }));
}

#[test]
fn test_execute_movw() {
    // LDI R23, 0x77 = 0xE777 and LDI R28, 0x88 = 0xE8C8 : guards on both sides
    // LDI R26, 0x34 = 0xE3A4 ; LDI R27, 0x12 = 0xE1B2 ; MOVW R24, R26 = 0x01CD
    let cpu = run_words(&[0xE777, 0xE8C8, 0xE3A4, 0xE1B2, 0x01CD]);
    assert_eq!(reg_pair(&cpu, 24), 0x1234, "destination");
    assert_eq!(reg_pair(&cpu, 26), 0x1234, "source untouched");
    assert_eq!(cpu.registers[23], 0x77, "R23 untouched");
    assert_eq!(cpu.registers[28], 0x88, "R28 untouched");
}

#[test]
fn test_execute_movw_low_to_high() {
    // LDI R16, 0xAB = 0xEA0B ; LDI R17, 0xCD = 0xEC1D ; MOVW R0, R16 = 0x0108
    let cpu = run_words(&[0xEA0B, 0xEC1D, 0x0108]);
    assert_eq!(reg_pair(&cpu, 0), 0xCDAB, "destination");
    assert_eq!(reg_pair(&cpu, 16), 0xCDAB, "source untouched");
    assert_eq!(cpu.registers[2], 0x00, "R2 untouched");
}

#[test]
fn test_execute_movw_top_pair() {
    // LDI R28, 0xAB = 0xEACB ; LDI R29, 0xCD = 0xECDD ; MOVW R30, R28 = 0x01FE
    // R31 must receive the high byte of the source
    let cpu = run_words(&[0xEACB, 0xECDD, 0x01FE]);
    assert_eq!(cpu.registers[30], 0xAB);
    assert_eq!(cpu.registers[31], 0xCD);
}

#[test]
fn test_execute_movw_same_pair() {
    // MOVW R24, R24 = 0x01CC : nothing changes
    let cpu = run_words(&[0xE3A4, 0xE1B2, 0x01CC]);
    assert_eq!(reg_pair(&cpu, 24), 0x0000);
    assert_eq!(reg_pair(&cpu, 26), 0x1234);
}

#[test]
fn test_execute_movw_all_pairs() {
    // every destination pair with every source pair, built from the word itself :
    // catches a wrong factor of 2 and any register copied by mistake
    for d in (0..32usize).step_by(2) {
        for s in (0..32usize).step_by(2) {
            let mut cpu = Cpu::new();
            cpu.registers[s] = 0x11;
            cpu.registers[s + 1] = 0x22;
            let word = 0x0100 | (((d / 2) as u16) << 4) | ((s / 2) as u16);
            cpu.load_program(&[word]);
            step_n(&mut cpu, 1);
            assert_eq!(cpu.registers[d], 0x11, "MOVW R{d}, R{s} : low byte");
            assert_eq!(cpu.registers[d + 1], 0x22, "MOVW R{d}, R{s} : high byte");
            // every other register is still 0
            for i in 0..32usize {
                if i != d && i != d + 1 && i != s && i != s + 1 {
                    assert_eq!(cpu.registers[i], 0, "MOVW R{d}, R{s} : R{i} touched");
                }
            }
        }
    }
}

#[test]
fn test_execute_movw_no_flag() {
    // copy of 0x0000 : Z must stay off ; copy of 0x8000 : N must stay off
    // LDI R27, 0x80 = 0xE8B0 ; LDI R26, 0 = 0xE0A0
    let cpu = run_words(&[0xE0A0, 0xE0B0, 0x01CD]);
    assert_eq!(cpu.sreg, 0);
    let cpu = run_words(&[0xE0A0, 0xE8B0, 0x01CD]);
    assert_eq!(reg_pair(&cpu, 24), 0x8000);
    assert_eq!(cpu.sreg, 0);
}

#[test]
fn test_execute_movw_keeps_all_flags() {
    // every flag on before, then every flag off before : SREG must not change
    for sreg in [0xFFu8, 0x00u8] {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0xE3A4, 0xE1B2, 0x01CD]);
        step_n(&mut cpu, 2);
        cpu.sreg = sreg;
        step_n(&mut cpu, 1);
        assert_eq!(reg_pair(&cpu, 24), 0x1234);
        assert_eq!(cpu.sreg, sreg);
    }
}

#[test]
fn test_decode_mul_neighbours() {
    // the group below (CBI, SBIC, SBI, SBIS) and the one above are not MUL
    for word in [0x9800, 0x9BFF, 0xA000] {
        assert!(
            !matches!(decode(word), Instruction::Mul { .. }),
            "{word:#06X} must not decode as MUL"
        );
    }
    // 0x9400 to 0x97FF (COM..., ADIW, SBIW) are not MUL either
    for word in [0x9400, 0x95F3, 0x9600, 0x97FF] {
        assert!(!matches!(decode(word), Instruction::Mul { .. }));
    }
}

#[test]
fn test_decode_muls_mulsu_neighbours() {
    // MOVW, CPC and SBC are not MULS or MULSU
    for word in [0x0100, 0x01FF, 0x0400, 0x0800] {
        assert!(
            !matches!(
                decode(word),
                Instruction::Muls { .. } | Instruction::Mulsu { .. }
            ),
            "{word:#06X} must not decode as MULS or MULSU"
        );
    }
    // MULS and MULSU do not decode as each other
    assert!(!matches!(decode(0x0300), Instruction::Muls { .. }));
    assert!(!matches!(decode(0x0200), Instruction::Mulsu { .. }));
    assert!(!matches!(decode(0x0201), Instruction::Mulsu { .. }));
    assert!(!matches!(decode(0x0301), Instruction::Muls { .. }));
}

#[test]
fn test_decode_fmul_holes_stay_unknown() {
    // the FMUL family shares 0x03xx with MULSU : bits 7 and 3 must be 0
    for word in [0x0308, 0x0380, 0x0388, 0x0390, 0x03FF] {
        assert!(
            !matches!(
                decode(word),
                Instruction::Mul { .. } | Instruction::Muls { .. } | Instruction::Mulsu { .. }
            ),
            "{word:#06X} must not decode as MUL, MULS or MULSU"
        );
    }
}

#[test]
fn test_execute_mul_values() {
    // (program, expected R1:R0, expected SREG, label)
    let cases: [([u16; 3], u16, u8, &str); 6] = [
        ([0xE005, 0xE017, 0x9F01], 0x0023, 0, "MUL 5 x 7"),
        ([0xEC08, 0xE614, 0x9F01], 0x4E20, 0, "MUL 200 x 100"),
        ([0xE800, 0xE012, 0x9F01], 0x0100, 0, "MUL 128 x 2"),
        (
            [0xEF0F, 0xEF1F, 0x9F01],
            0xFE01,
            FLAG_CARRY,
            "MUL 255 x 255",
        ),
        (
            [0xEC08, 0xEC18, 0x9F01],
            0x9C40,
            FLAG_CARRY,
            "MUL 200 x 200",
        ),
        ([0xE000, 0xE017, 0x9F01], 0x0000, FLAG_ZERO, "MUL 0 x 7"),
    ];
    for (program, expected, flags, label) in cases {
        let cpu = run_words(&program);
        assert_eq!(reg_pair(&cpu, 0), expected, "{label}");
        assert_eq!(cpu.sreg, flags, "{label} flags");
    }
}

#[test]
fn test_execute_mul_keeps_operands() {
    // MUL R16, R17 : the result goes to R1:R0, R16 and R17 are untouched
    let cpu = run_words(&[0xEF0F, 0xEF1F, 0x9F01]);
    assert_eq!(cpu.registers[16], 0xFF);
    assert_eq!(cpu.registers[17], 0xFF);
    assert_eq!(cpu.registers[0], 0x01);
    assert_eq!(cpu.registers[1], 0xFE);
}

#[test]
fn test_execute_mul_reads_before_writing() {
    // MUL R0, R1 with R0 = 3 and R1 = 4 : the operands are the registers
    // that receive the result, so they must be read first
    let mut cpu = Cpu::new();
    cpu.registers[0] = 3;
    cpu.registers[1] = 4;
    cpu.load_program(&[0x9C01]);
    step_n(&mut cpu, 1);
    assert_eq!(reg_pair(&cpu, 0), 0x000C);

    // MUL R0, R0 with R0 = 16 : 256 = 0x0100
    let mut cpu = Cpu::new();
    cpu.registers[0] = 16;
    cpu.load_program(&[0x9C00]);
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[0], 0x00);
    assert_eq!(cpu.registers[1], 0x01);
}

#[test]
fn test_execute_mul_then_clr_r1() {
    // avr-gcc follows every MUL with EOR R1, R1 (clr r1) to restore R1 = 0
    let cpu = run_words(&[0xEF0F, 0xEF1F, 0x9F01, 0x2411]);
    assert_eq!(cpu.registers[1], 0x00);
    assert_eq!(cpu.registers[0], 0x01);
}

#[test]
fn test_execute_muls_values() {
    let cases: [([u16; 3], u16, u8, &str); 6] = [
        ([0xEF0F, 0xEF1F, 0x0201], 0x0001, 0, "MULS -1 x -1"),
        ([0xEF0F, 0xE015, 0x0201], 0xFFFB, FLAG_CARRY, "MULS -1 x 5"),
        ([0xE800, 0xE810, 0x0201], 0x4000, 0, "MULS -128 x -128"),
        ([0xE70F, 0xE71F, 0x0201], 0x3F01, 0, "MULS 127 x 127"),
        (
            [0xE800, 0xE71F, 0x0201],
            0xC080,
            FLAG_CARRY,
            "MULS -128 x 127",
        ),
        ([0xE000, 0xEF1F, 0x0201], 0x0000, FLAG_ZERO, "MULS 0 x -1"),
    ];
    for (program, expected, flags, label) in cases {
        let cpu = run_words(&program);
        assert_eq!(reg_pair(&cpu, 0), expected, "{label}");
        assert_eq!(cpu.sreg, flags, "{label} flags");
    }
}

#[test]
fn test_execute_mulsu_values() {
    let cases: [([u16; 3], u16, u8, &str); 6] = [
        (
            [0xEF0F, 0xEF1F, 0x0301],
            0xFF01,
            FLAG_CARRY,
            "MULSU -1 x 255",
        ),
        (
            [0xE800, 0xEF1F, 0x0301],
            0x8080,
            FLAG_CARRY,
            "MULSU -128 x 255",
        ),
        ([0xE70F, 0xEF1F, 0x0301], 0x7E81, 0, "MULSU 127 x 255"),
        ([0xE005, 0xE017, 0x0301], 0x0023, 0, "MULSU 5 x 7"),
        ([0xEF0F, 0xE015, 0x0301], 0xFFFB, FLAG_CARRY, "MULSU -1 x 5"),
        ([0xE000, 0xEF1F, 0x0301], 0x0000, FLAG_ZERO, "MULSU 0 x 255"),
    ];
    for (program, expected, flags, label) in cases {
        let cpu = run_words(&program);
        assert_eq!(reg_pair(&cpu, 0), expected, "{label}");
        assert_eq!(cpu.sreg, flags, "{label} flags");
    }
}

#[test]
fn test_execute_mul_variants_discriminate() {
    // same operands 0xFF and 0xFF, three different results
    let mul = run_words(&[0xEF0F, 0xEF1F, 0x9F01]);
    let muls = run_words(&[0xEF0F, 0xEF1F, 0x0201]);
    let mulsu = run_words(&[0xEF0F, 0xEF1F, 0x0301]);
    assert_eq!(reg_pair(&mul, 0), 0xFE01);
    assert_eq!(reg_pair(&muls, 0), 0x0001);
    assert_eq!(reg_pair(&mulsu, 0), 0xFF01);
}

#[test]
fn test_execute_mul_keeps_other_flags() {
    // MUL 5 x 7 with every flag on before : only C and Z are recomputed
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xE005, 0xE017, 0x9F01]);
    step_n(&mut cpu, 2);
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.sreg, 0xFC);

    // MUL 255 x 255 : C on, Z off
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xEF0F, 0xEF1F, 0x9F01]);
    step_n(&mut cpu, 2);
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.sreg, 0xFD);

    // MUL 0 x 7 : Z on, C off
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xE000, 0xE017, 0x9F01]);
    step_n(&mut cpu, 2);
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.sreg, 0xFE);

    // MULS and MULSU follow the same rule (-1 x 5 : C on, Z off)
    for op in [0x0201, 0x0301] {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0xEF0F, 0xE015, op]);
        step_n(&mut cpu, 2);
        cpu.sreg = 0xFF;
        step_n(&mut cpu, 1);
        assert_eq!(cpu.sreg, 0xFD, "{op:#06X}");
    }
}

#[test]
fn test_decode_bset_bclr_neighbours() {
    // RET, RETI, SLEEP, BREAK, WDR, LPM, SPM : same low nibble, bit 8 set
    // IJMP, DES, JMP, CALL : same group, other low nibble
    for word in [
        0x9508, 0x9518, 0x9588, 0x9598, 0x95A8, 0x95C8, 0x95E8, 0x9409, 0x940B, 0x940C, 0x940E,
    ] {
        assert!(
            !matches!(
                decode(word),
                Instruction::Bset { .. } | Instruction::Bclr { .. }
            ),
            "{word:#06X} must not decode as BSET or BCLR"
        );
    }
    // COM / NEG / SWAP / INC share the 0x940x prefix but not the low nibble
    for word in [0x9400, 0x9401, 0x9402, 0x9403] {
        assert!(
            !matches!(
                decode(word),
                Instruction::Bset { .. } | Instruction::Bclr { .. }
            ),
            "{word:#06X} must not decode as BSET or BCLR"
        );
    }
    // BSET and BCLR do not decode as each other
    assert!(!matches!(decode(0x9408), Instruction::Bclr { .. }));
    assert!(!matches!(decode(0x9488), Instruction::Bset { .. }));
}

#[test]
fn test_execute_bset_bclr_aliases() {
    // (word, sreg before, sreg after, name)
    let cases = [
        (0x9408u16, 0x00u8, 0x01u8, "SEC"),
        (0x9418, 0x00, 0x02, "SEZ"),
        (0x9428, 0x00, 0x04, "SEN"),
        (0x9438, 0x00, 0x08, "SEV"),
        (0x9448, 0x00, 0x10, "SES"),
        (0x9458, 0x00, 0x20, "SEH"),
        (0x9468, 0x00, 0x40, "SET"),
        (0x9478, 0x00, 0x80, "SEI"),
        (0x9488, 0xFF, 0xFE, "CLC"),
        (0x9498, 0xFF, 0xFD, "CLZ"),
        (0x94A8, 0xFF, 0xFB, "CLN"),
        (0x94B8, 0xFF, 0xF7, "CLV"),
        (0x94C8, 0xFF, 0xEF, "CLS"),
        (0x94D8, 0xFF, 0xDF, "CLH"),
        (0x94E8, 0xFF, 0xBF, "CLT"),
        (0x94F8, 0xFF, 0x7F, "CLI"),
    ];
    for (word, before, after, name) in cases {
        let mut cpu = Cpu::new();
        cpu.load_program(&[word]);
        cpu.sreg = before;
        step_n(&mut cpu, 1);
        assert_eq!(cpu.sreg, after, "{name}");
        assert_eq!(cpu.pc, 1, "{name} must advance PC by one word");
    }
}

#[test]
fn test_execute_bset_all_flags() {
    for s in 0u8..8 {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0x9408u16 | ((s as u16) << 4)]);
        cpu.sreg = 0x00;
        step_n(&mut cpu, 1);
        assert_eq!(cpu.sreg, 1u8 << s, "BSET {s} from 0x00");
    }
}

#[test]
fn test_execute_bclr_all_flags() {
    for s in 0u8..8 {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0x9488u16 | ((s as u16) << 4)]);
        cpu.sreg = 0xFF;
        step_n(&mut cpu, 1);
        assert_eq!(cpu.sreg, !(1u8 << s), "BCLR {s} from 0xFF");
    }
}

#[test]
fn test_execute_bset_bclr_keep_other_flags() {
    // a mixed SREG : only the targeted flag may change
    for s in 0u8..8 {
        let mask = 1u8 << s;

        let mut cpu = Cpu::new();
        cpu.load_program(&[0x9408u16 | ((s as u16) << 4)]);
        cpu.sreg = 0xA5;
        step_n(&mut cpu, 1);
        assert_eq!(cpu.sreg, 0xA5 | mask, "BSET {s} on 0xA5");

        let mut cpu = Cpu::new();
        cpu.load_program(&[0x9488u16 | ((s as u16) << 4)]);
        cpu.sreg = 0xA5;
        step_n(&mut cpu, 1);
        assert_eq!(cpu.sreg, 0xA5 & !mask, "BCLR {s} on 0xA5");
    }
}

#[test]
fn test_execute_bset_bclr_idempotent() {
    // SEC twice : still only C
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x9408, 0x9408]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.sreg, FLAG_CARRY);

    // CLC on a cleared SREG : nothing happens
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x9488]);
    step_n(&mut cpu, 1);
    assert_eq!(cpu.sreg, 0x00);
}

#[test]
fn test_execute_bset_bclr_do_not_touch_registers() {
    let mut cpu = Cpu::new();
    // LDI R16, 0x55 / SEC / CLC
    cpu.load_program(&[0xE505, 0x9408, 0x9488]);
    step_n(&mut cpu, 3);
    assert_eq!(cpu.registers[16], 0x55);
    assert_eq!(cpu.sreg, 0x00);
}

#[test]
fn test_execute_sez_then_breq_jumps() {
    let mut cpu = Cpu::new();
    // SEZ / BREQ +1 / LDI R16, 1 (skipped) / LDI R17, 2
    cpu.load_program(&[0x9418, 0xF009, 0xE001, 0xE012]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.pc, 3);
}

#[test]
fn test_execute_clz_then_breq_does_not_jump() {
    let mut cpu = Cpu::new();
    // CLZ / BREQ +1 / LDI R16, 1 / LDI R17, 2
    cpu.load_program(&[0x9498, 0xF009, 0xE001, 0xE012]);
    cpu.sreg = FLAG_ZERO;
    step_n(&mut cpu, 2);
    assert_eq!(cpu.pc, 2);
}

#[test]
fn test_execute_sec_then_ror() {
    let mut cpu = Cpu::new();
    // LDI R16, 0 / SEC / ROR R16 : the old C enters bit 7
    cpu.load_program(&[0xE000, 0x9408, 0x9507]);
    step_n(&mut cpu, 3);
    assert_eq!(cpu.registers[16], 0x80);
    // C = 0 (bit 0 was 0), N = 1, V = N xor C = 1, S = N xor V = 0
    assert_eq!(cpu.sreg, FLAG_NEGATIVE | FLAG_OVERFLOW);
}

#[test]
fn test_execute_clc_then_ror() {
    let mut cpu = Cpu::new();
    // LDI R16, 0 / CLC / ROR R16 : a 0 enters bit 7
    cpu.load_program(&[0xE000, 0x9488, 0x9507]);
    step_n(&mut cpu, 1);
    cpu.sreg = FLAG_CARRY;
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 0x00);
    assert_eq!(cpu.sreg, FLAG_ZERO);
}

#[test]
fn test_decode_bst_bld_neighbours() {
    // bit 3 set (reserved), SBRC, SBRS, BRBS, BRBC
    for word in [
        0xF808, 0xF80F, 0xFA08, 0xFC00, 0xFE00, 0xF000, 0xF400, 0xF7FF,
    ] {
        assert!(
            !matches!(
                decode(word),
                Instruction::Bst { .. } | Instruction::Bld { .. }
            ),
            "{word:#06X} must not decode as BST or BLD"
        );
    }
    // BST and BLD do not decode as each other
    assert!(!matches!(decode(0xF900), Instruction::Bst { .. }));
    assert!(!matches!(decode(0xFB00), Instruction::Bld { .. }));
}

#[test]
fn test_execute_bst_sets_t() {
    // LDI R16, 0xA4 (1010 0100) / BST R16, 7 : bit 7 is 1
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xEA04, 0xFB07]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.sreg, FLAG_BIT_COPY);
    assert_eq!(cpu.registers[16], 0xA4);

    // BST R16, 2 : bit 2 is 1
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xEA04, 0xFB02]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.sreg, FLAG_BIT_COPY);
}

#[test]
fn test_execute_bst_clears_t() {
    // LDI R16, 0xA4 / BST R16, 6 : bit 6 is 0, T was on
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xEA04, 0xFB06]);
    step_n(&mut cpu, 1);
    cpu.sreg = FLAG_BIT_COPY;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.sreg, 0x00);
    assert_eq!(cpu.registers[16], 0xA4);
}

#[test]
fn test_execute_bst_all_bits() {
    for b in 0u8..8 {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0xFB00 | b as u16]);
        cpu.registers[16] = 0xA4;
        step_n(&mut cpu, 1);
        let expected = if (0xA4u8 >> b) & 1 != 0 {
            FLAG_BIT_COPY
        } else {
            0
        };
        assert_eq!(cpu.sreg, expected, "BST R16,{b} on 0xA4");
        assert_eq!(cpu.registers[16], 0xA4, "BST R16,{b} must not change Rd");
    }
}

#[test]
fn test_execute_bst_keeps_other_flags() {
    // bit 7 is 1 : sreg stays 0xFF
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xEA04, 0xFB07]);
    step_n(&mut cpu, 1);
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.sreg, 0xFF);

    // bit 6 is 0 : only T is cleared
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xEA04, 0xFB06]);
    step_n(&mut cpu, 1);
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.sreg, 0xBF);
}

#[test]
fn test_execute_bld_sets_bit() {
    // LDI R17, 0 / BLD R17, 3 with T on : R17 = 0x08
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xE010, 0xF913]);
    step_n(&mut cpu, 1);
    cpu.sreg = FLAG_BIT_COPY;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[17], 0x08);
    assert_eq!(cpu.sreg, FLAG_BIT_COPY);
}

#[test]
fn test_execute_bld_clears_bit() {
    // LDI R17, 0xFF / BLD R17, 3 with T off : R17 = 0xF7
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xEF1F, 0xF913]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[17], 0xF7);
    assert_eq!(cpu.sreg, 0x00);
}

#[test]
fn test_execute_bld_all_bits() {
    for b in 0u8..8 {
        // T on, register cleared : only bit b is set
        let mut cpu = Cpu::new();
        cpu.load_program(&[0xF900 | b as u16]);
        cpu.registers[16] = 0x00;
        cpu.sreg = FLAG_BIT_COPY;
        step_n(&mut cpu, 1);
        assert_eq!(cpu.registers[16], 1u8 << b, "BLD R16,{b} with T on");

        // T off, register full : only bit b is cleared
        let mut cpu = Cpu::new();
        cpu.load_program(&[0xF900 | b as u16]);
        cpu.registers[16] = 0xFF;
        cpu.sreg = 0x00;
        step_n(&mut cpu, 1);
        assert_eq!(cpu.registers[16], !(1u8 << b), "BLD R16,{b} with T off");
    }
}

#[test]
fn test_execute_bld_keeps_sreg() {
    // T on, every flag on : sreg stays 0xFF
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xE010, 0xF913]);
    step_n(&mut cpu, 1);
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.sreg, 0xFF);

    // T off, every other flag on : sreg stays 0xBF
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xEF1F, 0xF913]);
    step_n(&mut cpu, 1);
    cpu.sreg = 0xBF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.sreg, 0xBF);
}

#[test]
fn test_execute_bst_then_bld_copies_a_bit() {
    // LDI R16, 0x80 / LDI R17, 0x02 / BST R16, 7 / BLD R17, 0
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xE800, 0xE012, 0xFB07, 0xF910]);
    step_n(&mut cpu, 4);
    assert_eq!(cpu.registers[17], 0x03);
    assert_eq!(cpu.registers[16], 0x80);
    assert_eq!(cpu.sreg, FLAG_BIT_COPY);
}

#[test]
fn test_execute_bst_then_bld_copies_a_zero() {
    // LDI R16, 0 / LDI R17, 0xFF / BST R16, 0 / BLD R17, 7 : a 0 is copied
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xE000, 0xEF1F, 0xFB00, 0xF917]);
    step_n(&mut cpu, 4);
    assert_eq!(cpu.registers[17], 0x7F);
    assert_eq!(cpu.sreg, 0x00);
}

#[test]
fn test_execute_bst_bld_extreme_registers() {
    // BST R31, 7 then BLD R0, 0
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xFBF7, 0xF800]);
    cpu.registers[31] = 0x80;
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[0], 0x01);
    assert_eq!(cpu.registers[31], 0x80);
}

#[test]
fn test_execute_set_then_bld() {
    // SET / BLD R16, 5 : SET prepares T, BLD pastes it
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x9468, 0xF905]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 0x20);
    assert_eq!(cpu.sreg, FLAG_BIT_COPY);
}

#[test]
fn test_decode_fmul_family_neighbours() {
    // MULSU (bits 7 and 3 off), MULS, CPC, MOVW : none is an FMUL variant
    for word in [0x0300, 0x0301, 0x0307, 0x0370, 0x0200, 0x0400, 0x0100] {
        assert!(
            !matches!(
                decode(word),
                Instruction::Fmul { .. } | Instruction::Fmuls { .. } | Instruction::Fmulsu { .. }
            ),
            "{word:#06X} must not decode as FMUL, FMULS or FMULSU"
        );
    }
    // MULSU keeps 0x0300
    assert!(matches!(decode(0x0300), Instruction::Mulsu { .. }));
    // the three variants do not decode as each other
    assert!(!matches!(
        decode(0x0308),
        Instruction::Fmuls { .. } | Instruction::Fmulsu { .. }
    ));
    assert!(!matches!(
        decode(0x0380),
        Instruction::Fmul { .. } | Instruction::Fmulsu { .. }
    ));
    assert!(!matches!(
        decode(0x0388),
        Instruction::Fmul { .. } | Instruction::Fmuls { .. }
    ));
}

#[test]
fn test_execute_fmul_one_times_one() {
    // FMUL R16,R16 with 0x80 (1.0) : 0x4000 << 1 = 0x8000, no flag
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x0308]);
    cpu.registers[16] = 0x80;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[1], 0x80);
    assert_eq!(cpu.registers[0], 0x00);
    assert_eq!(cpu.sreg, 0);
}

#[test]
fn test_execute_fmul_max() {
    // FMUL R16,R17 with 0xFF x 0xFF : 0xFE01 << 1 = 0xFC02, C
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x0309]);
    cpu.registers[16] = 0xFF;
    cpu.registers[17] = 0xFF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[1], 0xFC);
    assert_eq!(cpu.registers[0], 0x02);
    assert_eq!(cpu.sreg, FLAG_CARRY);
}

#[test]
fn test_execute_fmul_zero() {
    // FMUL R16,R17 with 0 x 0x80 : Z
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x0309]);
    cpu.registers[16] = 0x00;
    cpu.registers[17] = 0x80;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[1], 0x00);
    assert_eq!(cpu.registers[0], 0x00);
    assert_eq!(cpu.sreg, FLAG_ZERO);
}

#[test]
fn test_execute_fmul_half_times_half() {
    // FMUL R16,R16 with 0x40 (0.5) : 0x1000 << 1 = 0x2000 (0.25 in 1.15)
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x0308]);
    cpu.registers[16] = 0x40;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[1], 0x20);
    assert_eq!(cpu.registers[0], 0x00);
    assert_eq!(cpu.sreg, 0);
}

#[test]
fn test_execute_fmul_keeps_operands() {
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x0309]);
    cpu.registers[16] = 0xFF;
    cpu.registers[17] = 0xFF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[16], 0xFF);
    assert_eq!(cpu.registers[17], 0xFF);
}

#[test]
fn test_execute_fmuls_minus_one_times_minus_one() {
    // FMULS R16,R16 with 0x80 : (-128)*(-128) = 0x4000, << 1 = 0x8000, no flag
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x0380]);
    cpu.registers[16] = 0x80;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[1], 0x80);
    assert_eq!(cpu.registers[0], 0x00);
    assert_eq!(cpu.sreg, 0);
}

#[test]
fn test_execute_fmuls_negative() {
    // FMULS R16,R17 with 0x80 x 0x01 : product 0xFF80, result 0xFF00, C
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x0381]);
    cpu.registers[16] = 0x80;
    cpu.registers[17] = 0x01;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[1], 0xFF);
    assert_eq!(cpu.registers[0], 0x00);
    assert_eq!(cpu.sreg, FLAG_CARRY);
}

#[test]
fn test_execute_fmuls_positive() {
    // FMULS R16,R17 with 0x40 x 0x40 : 0x1000 << 1 = 0x2000, no flag
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x0381]);
    cpu.registers[16] = 0x40;
    cpu.registers[17] = 0x40;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[1], 0x20);
    assert_eq!(cpu.registers[0], 0x00);
    assert_eq!(cpu.sreg, 0);
}

#[test]
fn test_execute_fmuls_zero() {
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x0381]);
    cpu.registers[16] = 0x00;
    cpu.registers[17] = 0x80;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[1], 0x00);
    assert_eq!(cpu.registers[0], 0x00);
    assert_eq!(cpu.sreg, FLAG_ZERO);
}

#[test]
fn test_execute_fmulsu_negative_times_unsigned() {
    // FMULSU R16,R17 with 0x80 (signed) x 0xFF (unsigned) :
    // -128 * 255 = 0x8080, << 1 = 0x0100, C
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x0389]);
    cpu.registers[16] = 0x80;
    cpu.registers[17] = 0xFF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[1], 0x01);
    assert_eq!(cpu.registers[0], 0x00);
    assert_eq!(cpu.sreg, FLAG_CARRY);
}

#[test]
fn test_execute_fmulsu_rr_is_unsigned() {
    // FMULSU R16,R17 with 0x01 x 0xFF : 255 (not -1), << 1 = 0x01FE, no flag
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x0389]);
    cpu.registers[16] = 0x01;
    cpu.registers[17] = 0xFF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[1], 0x01);
    assert_eq!(cpu.registers[0], 0xFE);
    assert_eq!(cpu.sreg, 0);
}

#[test]
fn test_execute_fmulsu_zero() {
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x0389]);
    cpu.registers[16] = 0x00;
    cpu.registers[17] = 0xFF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[1], 0x00);
    assert_eq!(cpu.registers[0], 0x00);
    assert_eq!(cpu.sreg, FLAG_ZERO);
}

#[test]
fn test_execute_fmul_family_keep_n_v_s_h_t_i() {
    // sreg = 0xFF : only C and Z are recalculated
    // FMUL 0x80 x 0x80 : C off, Z off -> 0xFC
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x0308]);
    cpu.registers[16] = 0x80;
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.sreg, 0xFC);

    // FMUL 0xFF x 0xFF : C on, Z off -> 0xFD
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x0308]);
    cpu.registers[16] = 0xFF;
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.sreg, 0xFD);

    // FMULS 0 x 0 : C off, Z on -> 0xFE
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x0380]);
    cpu.registers[16] = 0x00;
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.sreg, 0xFE);

    // FMULSU 0x80 x 0x80 on R16,R16 : -128*128 = 0xC000, C on, Z off -> 0xFD
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x0388]);
    cpu.registers[16] = 0x80;
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.sreg, 0xFD);
}

#[test]
fn test_execute_fmul_family_do_not_touch_pc_beyond_one_word() {
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x0308, 0x0380, 0x0388]);
    cpu.registers[16] = 0x40;
    step_n(&mut cpu, 3);
    assert_eq!(cpu.pc, 3);
}

#[test]
fn test_decode_in_out() {
    // IN R16,0x3F / R31,0x3F / R0,0x00 / R16,0x05
    assert_eq!(decode(0xB70F), Instruction::In { rd: 16, a: 0x3F });
    assert_eq!(decode(0xB7FF), Instruction::In { rd: 31, a: 0x3F });
    assert_eq!(decode(0xB000), Instruction::In { rd: 0, a: 0x00 });
    assert_eq!(decode(0xB105), Instruction::In { rd: 16, a: 0x05 });
    // OUT 0x3F,R16 / 0x3F,R31 / 0x00,R0 / 0x3D,R28 / 0x3E,R29
    assert_eq!(decode(0xBF0F), Instruction::Out { a: 0x3F, rr: 16 });
    assert_eq!(decode(0xBFFF), Instruction::Out { a: 0x3F, rr: 31 });
    assert_eq!(decode(0xB800), Instruction::Out { a: 0x00, rr: 0 });
    assert_eq!(decode(0xBFCD), Instruction::Out { a: 0x3D, rr: 28 });
    assert_eq!(decode(0xBFDE), Instruction::Out { a: 0x3E, rr: 29 });
}

#[test]
fn test_decode_in_out_all_addresses_and_registers() {
    // the word is built from a and r, like the BST/BLD test
    for a in 0u8..64 {
        for r in 0u8..32 {
            let low = (((a as u16) & 0x30) << 5) | ((a as u16) & 0x0F) | ((r as u16) << 4);
            assert_eq!(
                decode(0xB000 | low),
                Instruction::In { rd: r, a },
                "IN R{r},{a:#04X}"
            );
            assert_eq!(
                decode(0xB800 | low),
                Instruction::Out { a, rr: r },
                "OUT {a:#04X},R{r}"
            );
        }
    }
}

#[test]
fn test_decode_in_out_neighbours() {
    // STD zone, MUL, RJMP, and the last LDD/STD words
    for word in [0xA000, 0xA800, 0xAFFF, 0x9FFF, 0xC000] {
        assert!(
            !matches!(
                decode(word),
                Instruction::In { .. } | Instruction::Out { .. }
            ),
            "{word:#06X} must not decode as IN or OUT"
        );
    }
    // IN and OUT do not decode as each other
    assert!(!matches!(decode(0xB000), Instruction::Out { .. }));
    assert!(!matches!(decode(0xB800), Instruction::In { .. }));
}

#[test]
fn test_execute_in_reads_io() {
    // IN R16, 0x05 : PORTB is data address 0x25, io[5]
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xB105]);
    cpu.io[0x05] = 0xAA;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[16], 0xAA);
    assert_eq!(cpu.io[0x05], 0xAA);
    assert_eq!(cpu.sreg, 0);
}

#[test]
fn test_execute_out_writes_io() {
    // LDI R16, 0x55 / OUT 0x05, R16
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xE505, 0xB905]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.io[0x05], 0x55);
    assert_eq!(cpu.registers[16], 0x55);
    assert_eq!(cpu.sreg, 0);
}

#[test]
fn test_execute_out_does_not_write_registers() {
    // OUT 0x05 must land in io, not in R5 (data address 0x05)
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xB905]);
    cpu.registers[16] = 0x55;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[5], 0x00);
    assert_eq!(cpu.io[0x05], 0x55);
}

#[test]
fn test_execute_out_in_sreg() {
    // OUT 0x3F, R16 writes sreg
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xEA05, 0xBF0F]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.sreg, 0xA5);

    // IN R16, 0x3F reads sreg
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xB70F]);
    cpu.sreg = 0x5A;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[16], 0x5A);
    assert_eq!(cpu.sreg, 0x5A);
}

#[test]
fn test_execute_out_in_sp() {
    // LDI R16,0xFF / OUT 0x3D,R16 / LDI R16,0x08 / OUT 0x3E,R16
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xEF0F, 0xBF0D, 0xE008, 0xBF0E]);
    step_n(&mut cpu, 4);
    assert_eq!(cpu.sp, 0x08FF);

    // IN R17,0x3D (SPL) / IN R18,0x3E (SPH)
    cpu.load_program(&[0xB71D, 0xB72E]);
    cpu.pc = 0;
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[17], 0xFF);
    assert_eq!(cpu.registers[18], 0x08);
}

#[test]
fn test_execute_avr_gcc_startup_clr_r1_out_sreg() {
    // CLR R1 / OUT 0x3F, R1 : SREG is cleared
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x2411, 0xBE1F]);
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 1);
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.sreg, 0x00);
}

#[test]
fn test_execute_in_out_keep_flags() {
    // IN R16,0x05 and OUT 0x05,R16 never touch sreg
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xB105, 0xB905]);
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 2);
    assert_eq!(cpu.sreg, 0xFF);
}

#[test]
fn test_decode_push_pop() {
    // PUSH R0 / R16 / R28 / R31
    assert_eq!(decode(0x920F), Instruction::Push { rr: 0 });
    assert_eq!(decode(0x930F), Instruction::Push { rr: 16 });
    assert_eq!(decode(0x93CF), Instruction::Push { rr: 28 });
    assert_eq!(decode(0x93FF), Instruction::Push { rr: 31 });
    // POP R0 / R16 / R29 / R31
    assert_eq!(decode(0x900F), Instruction::Pop { rd: 0 });
    assert_eq!(decode(0x910F), Instruction::Pop { rd: 16 });
    assert_eq!(decode(0x91DF), Instruction::Pop { rd: 29 });
    assert_eq!(decode(0x91FF), Instruction::Pop { rd: 31 });
}

#[test]
fn test_decode_push_pop_all_registers() {
    // the word is built from r, like the BST/BLD test
    for r in 0u8..32 {
        let low = (r as u16) << 4;
        assert_eq!(
            decode(0x920F | low),
            Instruction::Push { rr: r },
            "PUSH R{r}"
        );
        assert_eq!(decode(0x900F | low), Instruction::Pop { rd: r }, "POP R{r}");
    }
}

#[test]
fn test_decode_push_pop_neighbours() {
    // STS, LDS, LD Rd,Z+ , LD Rd,-X, ST -X : same group, other low nibble
    for word in [
        0x9200, 0x9000, 0x9001, 0x900E, 0x920E, 0x9010, 0x9100, 0x910E,
    ] {
        assert!(
            !matches!(
                decode(word),
                Instruction::Push { .. } | Instruction::Pop { .. }
            ),
            "{word:#06X} must not decode as PUSH or POP"
        );
    }
    // PUSH and POP do not decode as each other
    assert!(!matches!(decode(0x930F), Instruction::Pop { .. }));
    assert!(!matches!(decode(0x910F), Instruction::Push { .. }));
}

#[test]
fn test_execute_push() {
    // LDI R16, 0x55 / PUSH R16 with sp = 0x08FF
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xE505, 0x930F]);
    cpu.sp = 0x08FF;
    step_n(&mut cpu, 2);
    assert_eq!(cpu.sram[0x08FF - 0x100], 0x55);
    assert_eq!(cpu.sp, 0x08FE);
    assert_eq!(cpu.registers[16], 0x55);
}

#[test]
fn test_execute_pop() {
    // POP R17 with sp = 0x08FE and a byte at 0x08FF
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x911F]);
    cpu.sp = 0x08FE;
    cpu.sram[0x08FF - 0x100] = 0x77;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[17], 0x77);
    assert_eq!(cpu.sp, 0x08FF);
}

#[test]
fn test_execute_push_then_pop_round_trip() {
    // LDI R16, 0x55 / PUSH R16 / POP R17
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xE505, 0x930F, 0x911F]);
    cpu.sp = 0x08FF;
    step_n(&mut cpu, 3);
    assert_eq!(cpu.registers[17], 0x55);
    assert_eq!(cpu.registers[16], 0x55);
    assert_eq!(cpu.sp, 0x08FF);
}

#[test]
fn test_execute_push_pop_lifo_order() {
    // LDI R16,0x55 / LDI R17,0x66 / PUSH R16 / PUSH R17 / POP R18 / POP R19
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xE505, 0xE616, 0x930F, 0x931F, 0x912F, 0x913F]);
    cpu.sp = 0x08FF;
    step_n(&mut cpu, 4);
    assert_eq!(cpu.sp, 0x08FD);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[18], 0x66);
    assert_eq!(cpu.registers[19], 0x55);
    assert_eq!(cpu.sp, 0x08FF);
}

#[test]
fn test_execute_push_pop_extreme_registers() {
    // PUSH R31 / POP R0
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x93FF, 0x900F]);
    cpu.sp = 0x08FF;
    cpu.registers[31] = 0xAB;
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[0], 0xAB);
    assert_eq!(cpu.registers[31], 0xAB);
}

#[test]
fn test_execute_push_pop_keep_flags() {
    // PUSH R16 / POP R17 never touch sreg
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x930F, 0x911F]);
    cpu.sp = 0x08FF;
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 2);
    assert_eq!(cpu.sreg, 0xFF);
}

#[test]
fn test_execute_push_then_in_spl() {
    // PUSH R16 / IN R17, 0x3D (SPL) : the new sp is visible through IN
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x930F, 0xB71D]);
    cpu.sp = 0x08FF;
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[17], 0xFE);
}

#[test]
fn test_execute_push_with_sp_zero_wraps() {
    // sp = 0 : the byte lands in R0 (real hardware behaviour), sp wraps to 0xFFFF
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x930F]);
    cpu.registers[16] = 0x42;
    cpu.sp = 0;
    assert_eq!(cpu.step(), Ok(2));
    assert_eq!(cpu.registers[0], 0x42);
    assert_eq!(cpu.sp, 0xFFFF);
}

#[test]
fn test_execute_pop_beyond_ramend_is_an_error() {
    // sp = 0x08FF : POP reads at 0x0900, past RAMEND
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x910F]);
    cpu.registers[16] = 0x99;
    cpu.sp = 0x08FF;
    assert!(cpu.step().is_err());
    assert_eq!(cpu.registers[16], 0x99);
}

#[test]
fn test_decode_lds_sts() {
    // LDS R0 / R16 / R31
    assert_eq!(decode(0x9000), Instruction::Lds { rd: 0 });
    assert_eq!(decode(0x9100), Instruction::Lds { rd: 16 });
    assert_eq!(decode(0x91F0), Instruction::Lds { rd: 31 });
    // STS R0 / R16 / R31
    assert_eq!(decode(0x9200), Instruction::Sts { rr: 0 });
    assert_eq!(decode(0x9300), Instruction::Sts { rr: 16 });
    assert_eq!(decode(0x93F0), Instruction::Sts { rr: 31 });
}

#[test]
fn test_decode_lds_sts_all_registers() {
    for r in 0u8..32 {
        let low = (r as u16) << 4;
        assert_eq!(decode(0x9000 | low), Instruction::Lds { rd: r }, "LDS R{r}");
        assert_eq!(decode(0x9200 | low), Instruction::Sts { rr: r }, "STS R{r}");
    }
}

#[test]
fn test_decode_lds_sts_neighbours() {
    // POP, PUSH, LD Rd,Z+ , LD Rd,-X, ST -X, ST X+
    for word in [0x900F, 0x920F, 0x9001, 0x900E, 0x920E, 0x920D, 0x9400] {
        assert!(
            !matches!(
                decode(word),
                Instruction::Lds { .. } | Instruction::Sts { .. }
            ),
            "{word:#06X} must not decode as LDS or STS"
        );
    }
    // LDS and STS do not decode as each other
    assert!(!matches!(decode(0x9100), Instruction::Sts { .. }));
    assert!(!matches!(decode(0x9300), Instruction::Lds { .. }));
}

#[test]
fn test_execute_lds_from_sram() {
    // LDS R16, 0x0100
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x9100, 0x0100]);
    cpu.sram[0] = 0x5A;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[16], 0x5A);
    assert_eq!(cpu.pc, 2);
    assert_eq!(cpu.sram[0], 0x5A);
}

#[test]
fn test_execute_lds_from_io() {
    // LDS R16, 0x0025 (PORTB)
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x9100, 0x0025]);
    cpu.io[0x05] = 0xAA;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[16], 0xAA);
}

#[test]
fn test_execute_lds_from_register() {
    // LDS R16, 0x0010 (R16 itself)
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x9110, 0x0011]);
    cpu.registers[17] = 0x33;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[17], 0x33);
    // LDS R17, 0x0010 : copy R16 into R17
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x9110, 0x0010]);
    cpu.registers[16] = 0x77;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[17], 0x77);
}

#[test]
fn test_execute_lds_from_sreg() {
    // LDS R16, 0x005F (SREG)
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x9100, 0x005F]);
    cpu.sreg = 0x5A;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[16], 0x5A);
    assert_eq!(cpu.sreg, 0x5A);
}

#[test]
fn test_execute_sts_to_sram() {
    // STS 0x0100, R16
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x9300, 0x0100]);
    cpu.registers[16] = 0x42;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.sram[0], 0x42);
    assert_eq!(cpu.registers[16], 0x42);
    assert_eq!(cpu.pc, 2);
}

#[test]
fn test_execute_sts_to_io() {
    // STS 0x0025, R16 (PORTB)
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x9300, 0x0025]);
    cpu.registers[16] = 0x20;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.io[0x05], 0x20);
}

#[test]
fn test_execute_sts_to_register() {
    // STS 0x0011, R16 : copy R16 into R17
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x9300, 0x0011]);
    cpu.registers[16] = 0x99;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[17], 0x99);
}

#[test]
fn test_execute_sts_to_sreg_and_sp() {
    // STS 0x005F, R16 writes sreg
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x9300, 0x005F]);
    cpu.registers[16] = 0xA5;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.sreg, 0xA5);

    // STS 0x005D, R16 (SPL) / STS 0x005E, R17 (SPH)
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x9300, 0x005D, 0x9310, 0x005E]);
    cpu.registers[16] = 0xFF;
    cpu.registers[17] = 0x08;
    step_n(&mut cpu, 2);
    assert_eq!(cpu.sp, 0x08FF);
}

#[test]
fn test_execute_sts_then_lds_round_trip() {
    // LDI R16,0x5A / STS 0x0100,R16 / LDS R17,0x0100
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xE50A, 0x9300, 0x0100, 0x9110, 0x0100]);
    step_n(&mut cpu, 3);
    assert_eq!(cpu.registers[17], 0x5A);
    assert_eq!(cpu.pc, 5);
}

#[test]
fn test_execute_lds_sts_second_word_is_not_executed() {
    // k = 0xE0FF looks like LDI R31,0x0F but is only an address : it is out of range here
    // so use a valid look-alike : k = 0x0100 is data, then a real LDI follows
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x9100, 0x0100, 0xE00A]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.pc, 3);
    assert_eq!(cpu.registers[16], 0x0A);
}

#[test]
fn test_execute_lds_sts_keep_flags() {
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x9100, 0x0100, 0x9300, 0x0101]);
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 2);
    assert_eq!(cpu.sreg, 0xFF);
}

#[test]
fn test_execute_lds_sts_beyond_ramend_is_an_error() {
    // LDS R16, 0x0900 : past RAMEND, R16 untouched
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x9100, 0x0900]);
    cpu.registers[16] = 0x11;
    assert!(cpu.step().is_err());
    assert_eq!(cpu.registers[16], 0x11);

    // STS 0x0900, R16 : error
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x9300, 0x0900]);
    assert!(cpu.step().is_err());
}

#[test]
fn test_execute_lds_second_word_outside_flash_is_an_error() {
    // LDS as the very last word of the flash : the second fetch fails
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x9100]);
    cpu.pc = (FLASH_SIZE - 1) as u16;
    cpu.flash[FLASH_SIZE - 1] = 0x9100;
    cpu.registers[16] = 0x11;
    assert!(cpu.step().is_err());
    assert_eq!(cpu.registers[16], 0x11);
}

#[test]
fn test_decode_rcall_ret() {
    assert_eq!(decode(0xD000), Instruction::Rcall { k: 0 });
    assert_eq!(decode(0xD001), Instruction::Rcall { k: 1 });
    assert_eq!(decode(0xDFFF), Instruction::Rcall { k: -1 });
    assert_eq!(decode(0xD7FF), Instruction::Rcall { k: 2047 });
    assert_eq!(decode(0xD800), Instruction::Rcall { k: -2048 });
    assert_eq!(decode(0x9508), Instruction::Ret);
}

#[test]
fn test_decode_rcall_ret_neighbours() {
    // RJMP, LDI, RETI, ICALL, SLEEP, BSET, ROR, a 0x95xx word
    for word in [
        0xC000, 0xE000, 0x9518, 0x9509, 0x9588, 0x9408, 0x9507, 0x9500,
    ] {
        assert!(
            !matches!(decode(word), Instruction::Rcall { .. } | Instruction::Ret),
            "{word:#06X} must not decode as RCALL or RET"
        );
    }
}

#[test]
fn test_execute_rcall_pushes_return_address() {
    // RCALL at pc = 0x122 : return address 0x0123, high byte at the lower address
    let mut cpu = Cpu::new();
    cpu.flash[0x122] = 0xD001;
    cpu.pc = 0x122;
    cpu.sp = 0x08FF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.sram[0x08FE - 0x100], 0x01);
    assert_eq!(cpu.sram[0x08FF - 0x100], 0x23);
    assert_eq!(cpu.sp, 0x08FD);
    assert_eq!(cpu.pc, 0x123 + 1);
}

#[test]
fn test_execute_rcall_destinations() {
    // k = 0 : lands on the next instruction
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xD000]);
    cpu.sp = 0x08FF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.pc, 1);

    // k = +1 : skips one word
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xD001]);
    cpu.sp = 0x08FF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.pc, 2);

    // k = -1 at pc = 5 : lands on itself
    let mut cpu = Cpu::new();
    cpu.flash[5] = 0xDFFF;
    cpu.pc = 5;
    cpu.sp = 0x08FF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.pc, 5);

    // k = +1 on the last word of the flash : wraps around
    let mut cpu = Cpu::new();
    cpu.flash[FLASH_SIZE - 1] = 0xD001;
    cpu.pc = (FLASH_SIZE - 1) as u16;
    cpu.sp = 0x08FF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.pc, 1);
}

#[test]
fn test_execute_ret_pops_return_address() {
    // bytes prepared by hand : high 0x01 at 0x08FE, low 0x23 at 0x08FF
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x9508]);
    cpu.sp = 0x08FD;
    cpu.sram[0x08FE - 0x100] = 0x01;
    cpu.sram[0x08FF - 0x100] = 0x23;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.pc, 0x0123);
    assert_eq!(cpu.sp, 0x08FF);
}

#[test]
fn test_execute_rcall_then_ret_round_trip() {
    // 0: RCALL +1 (dest 2) / 1: LDI R17,9 / 2: RET
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xD001, 0xE019, 0x9508]);
    cpu.sp = 0x08FF;
    step_n(&mut cpu, 2);
    assert_eq!(cpu.pc, 1);
    assert_eq!(cpu.sp, 0x08FF);
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[17], 9);
}

#[test]
fn test_execute_nested_calls() {
    // 0: RCALL +2 (dest 3, ret 1) / 3: RCALL +1 (dest 5, ret 4) / 5: RET / 4: RET
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xD002, 0x0000, 0x0000, 0xD001, 0x9508, 0x9508]);
    cpu.sp = 0x08FF;
    step_n(&mut cpu, 2);
    assert_eq!(cpu.sp, 0x08FB);
    step_n(&mut cpu, 1);
    assert_eq!(cpu.pc, 4);
    step_n(&mut cpu, 1);
    assert_eq!(cpu.pc, 1);
    assert_eq!(cpu.sp, 0x08FF);
}

#[test]
fn test_execute_rcall_ret_keep_flags() {
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xD000, 0x9508]);
    cpu.sp = 0x08FF;
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 2);
    assert_eq!(cpu.sreg, 0xFF);
}

#[test]
fn test_execute_rcall_then_in_spl() {
    // RCALL +0 / IN R16, 0x3D
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xD000, 0xB70D]);
    cpu.sp = 0x08FF;
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 0xFD);
}

#[test]
fn test_execute_ret_beyond_ramend_is_an_error() {
    // sp = 0x08FF : RET reads at 0x0900
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x9508]);
    cpu.sp = 0x08FF;
    assert!(cpu.step().is_err());
}

#[test]
fn test_execute_function_call_integration() {
    // 0: LDI R16,0xFF / 1: OUT 0x3D,R16 / 2: LDI R16,0x08 / 3: OUT 0x3E,R16
    // 4: RCALL +2 (dest 7) / 5: MOV R17,R16 / 6: filler
    // 7: INC R16 / 8: RET
    let mut cpu = Cpu::new();
    cpu.load_program(&[
        0xEF0F, 0xBF0D, 0xE008, 0xBF0E, 0xD002, 0x2F10, 0x0000, 0x9503, 0x9508,
    ]);
    step_n(&mut cpu, 8);
    assert_eq!(cpu.registers[16], 9);
    assert_eq!(cpu.registers[17], 9);
    assert_eq!(cpu.sp, 0x08FF);
    assert_eq!(cpu.pc, 6);
}

#[test]
fn test_decode_jmp_call_family() {
    assert_eq!(decode(0x940C), Instruction::Jmp);
    assert_eq!(decode(0x940E), Instruction::Call);
    assert_eq!(decode(0x9409), Instruction::Ijmp);
    assert_eq!(decode(0x9509), Instruction::Icall);
    assert_eq!(decode(0x9518), Instruction::Reti);
    // the high address bits of the first word are ignored
    assert_eq!(decode(0x940D), Instruction::Jmp);
    assert_eq!(decode(0x95FD), Instruction::Jmp);
    assert_eq!(decode(0x95FE), Instruction::Call);
}

#[test]
fn test_decode_jmp_call_family_neighbours() {
    // BSET, DEC, DES, RET, EIJMP, EICALL, ROR
    for word in [0x9408, 0x940A, 0x940B, 0x9508, 0x9419, 0x9519, 0x9507] {
        assert!(
            !matches!(
                decode(word),
                Instruction::Jmp
                    | Instruction::Call
                    | Instruction::Ijmp
                    | Instruction::Icall
                    | Instruction::Reti
            ),
            "{word:#06X} must not decode as a jump or call"
        );
    }
    // RET and RETI are different words
    assert!(!matches!(decode(0x9508), Instruction::Reti));
    assert!(!matches!(decode(0x9518), Instruction::Ret));
}

#[test]
fn test_execute_jmp() {
    // JMP 3 / LDI R16,10 (skipped) / LDI R17,11
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x940C, 0x0003, 0xE00A, 0xE01B]);
    cpu.sp = 0x08FF;
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 2);
    assert_eq!(cpu.pc, 4);
    assert_eq!(cpu.registers[16], 0);
    assert_eq!(cpu.registers[17], 11);
    assert_eq!(cpu.sp, 0x08FF);
    assert_eq!(cpu.sreg, 0xFF);
}

#[test]
fn test_execute_jmp_is_absolute() {
    // JMP 0 at pc = 5 : lands on 0, not on 5 + 0
    let mut cpu = Cpu::new();
    cpu.flash[5] = 0x940C;
    cpu.flash[6] = 0x0000;
    cpu.pc = 5;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.pc, 0);
}

#[test]
fn test_execute_jmp_second_word_outside_flash_is_an_error() {
    let mut cpu = Cpu::new();
    cpu.flash[FLASH_SIZE - 1] = 0x940C;
    cpu.pc = (FLASH_SIZE - 1) as u16;
    assert!(cpu.step().is_err());
}

#[test]
fn test_execute_call_pushes_return_address() {
    // CALL 0x0200 at pc = 0x121 : return address 0x0123
    let mut cpu = Cpu::new();
    cpu.flash[0x121] = 0x940E;
    cpu.flash[0x122] = 0x0200;
    cpu.pc = 0x121;
    cpu.sp = 0x08FF;
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.sram[0x08FE - 0x100], 0x01);
    assert_eq!(cpu.sram[0x08FF - 0x100], 0x23);
    assert_eq!(cpu.sp, 0x08FD);
    assert_eq!(cpu.pc, 0x0200);
    assert_eq!(cpu.sreg, 0xFF);
}

#[test]
fn test_execute_call_then_ret_round_trip() {
    // 0: CALL 4 / 2: LDI R17,9 / 3: filler / 4: INC R16 / 5: RET
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x940E, 0x0004, 0xE019, 0x0000, 0x9503, 0x9508]);
    cpu.sp = 0x08FF;
    step_n(&mut cpu, 3);
    assert_eq!(cpu.pc, 2);
    assert_eq!(cpu.sp, 0x08FF);
    assert_eq!(cpu.registers[16], 1);
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[17], 9);
}

#[test]
fn test_execute_ijmp() {
    // LDI R30,0x23 / LDI R31,0x01 / IJMP : Z = 0x0123
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xE2E3, 0xE0F1, 0x9409]);
    cpu.sp = 0x08FF;
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 3);
    assert_eq!(cpu.pc, 0x0123);
    assert_eq!(cpu.sp, 0x08FF);
    assert_eq!(cpu.sreg, 0xFF);
}

#[test]
fn test_execute_icall_pushes_pc_plus_one() {
    // ICALL at pc = 0x122 with Z = 0x0200 : return address 0x0123
    let mut cpu = Cpu::new();
    cpu.flash[0x122] = 0x9509;
    cpu.pc = 0x122;
    cpu.registers[30] = 0x00;
    cpu.registers[31] = 0x02;
    cpu.sp = 0x08FF;
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.sram[0x08FE - 0x100], 0x01);
    assert_eq!(cpu.sram[0x08FF - 0x100], 0x23);
    assert_eq!(cpu.sp, 0x08FD);
    assert_eq!(cpu.pc, 0x0200);
    assert_eq!(cpu.sreg, 0xFF);
}

#[test]
fn test_execute_icall_then_ret() {
    // 0: ICALL (Z = 3) / 1: LDI R17,9 / 2: filler / 3: RET
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x9509, 0xE019, 0x0000, 0x9508]);
    cpu.registers[30] = 3;
    cpu.sp = 0x08FF;
    step_n(&mut cpu, 2);
    assert_eq!(cpu.pc, 1);
    assert_eq!(cpu.sp, 0x08FF);
}

#[test]
fn test_execute_reti() {
    // bytes prepared by hand : high 0x01 at 0x08FE, low 0x23 at 0x08FF
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x9518]);
    cpu.sp = 0x08FD;
    cpu.sram[0x08FE - 0x100] = 0x01;
    cpu.sram[0x08FF - 0x100] = 0x23;
    cpu.sreg = 0x00;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.pc, 0x0123);
    assert_eq!(cpu.sp, 0x08FF);
    assert_eq!(cpu.sreg, FLAG_INTERRUPT_ENABLE);
}

#[test]
fn test_execute_reti_keeps_other_flags() {
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x9518]);
    cpu.sp = 0x08FD;
    cpu.sreg = 0x7F;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.sreg, 0xFF);
}

#[test]
fn test_execute_reti_beyond_ramend_is_an_error() {
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x9518]);
    cpu.sp = 0x08FF;
    cpu.sreg = 0x00;
    assert!(cpu.step().is_err());
    assert_eq!(cpu.sreg, 0x00);
}

#[test]
fn test_execute_avr_gcc_startup_integration() {
    // 0: JMP 3 / 2: filler / 3: LDI R16,0xFF / 4: OUT 0x3D,R16
    // 5: LDI R16,0x08 / 6: OUT 0x3E,R16 / 7: CALL 11 / 9: MOV R17,R16
    // 10: filler / 11: INC R16 / 12: RET
    let mut cpu = Cpu::new();
    cpu.load_program(&[
        0x940C, 0x0003, 0x0000, 0xEF0F, 0xBF0D, 0xE008, 0xBF0E, 0x940E, 0x000B, 0x2F10, 0x0000,
        0x9503, 0x9508,
    ]);
    step_n(&mut cpu, 8);
    assert_eq!(cpu.registers[16], 9);
    assert_eq!(cpu.sp, 0x08FF);
    assert_eq!(cpu.pc, 9);
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[17], 9);
}

#[test]
fn test_decode_sbi_cbi() {
    // SBI 0x04,5 / 0x05,5 / 0x00,0 / 0x1F,7
    assert_eq!(decode(0x9A25), Instruction::Sbi { a: 0x04, b: 5 });
    assert_eq!(decode(0x9A2D), Instruction::Sbi { a: 0x05, b: 5 });
    assert_eq!(decode(0x9A00), Instruction::Sbi { a: 0x00, b: 0 });
    assert_eq!(decode(0x9AFF), Instruction::Sbi { a: 0x1F, b: 7 });
    // CBI 0x04,5 / 0x05,5 / 0x00,0 / 0x1F,7
    assert_eq!(decode(0x9825), Instruction::Cbi { a: 0x04, b: 5 });
    assert_eq!(decode(0x982D), Instruction::Cbi { a: 0x05, b: 5 });
    assert_eq!(decode(0x9800), Instruction::Cbi { a: 0x00, b: 0 });
    assert_eq!(decode(0x98FF), Instruction::Cbi { a: 0x1F, b: 7 });
}

#[test]
fn test_decode_sbi_cbi_all_addresses_and_bits() {
    for a in 0u8..32 {
        for b in 0u8..8 {
            let low = ((a as u16) << 3) | b as u16;
            assert_eq!(
                decode(0x9A00 | low),
                Instruction::Sbi { a, b },
                "SBI {a:#04X},{b}"
            );
            assert_eq!(
                decode(0x9800 | low),
                Instruction::Cbi { a, b },
                "CBI {a:#04X},{b}"
            );
        }
    }
}

#[test]
fn test_decode_sbi_cbi_neighbours() {
    // SBIC, SBIS, SBIW, MUL
    for word in [0x9900, 0x9B00, 0x99FF, 0x9BFF, 0x9700, 0x9C00] {
        assert!(
            !matches!(
                decode(word),
                Instruction::Sbi { .. } | Instruction::Cbi { .. }
            ),
            "{word:#06X} must not decode as SBI or CBI"
        );
    }
    assert!(!matches!(decode(0x9A25), Instruction::Cbi { .. }));
    assert!(!matches!(decode(0x9825), Instruction::Sbi { .. }));
}

#[test]
fn test_execute_sbi_sets_bit() {
    // SBI 0x05, 5 on an empty PORTB
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x9A2D]);
    step_n(&mut cpu, 1);
    assert_eq!(cpu.io[0x05], 0x20);

    // other bits are kept : 0x0F -> 0x2F
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x9A2D]);
    cpu.io[0x05] = 0x0F;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.io[0x05], 0x2F);
}

#[test]
fn test_execute_cbi_clears_bit() {
    // CBI 0x05, 5 on a full PORTB
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x982D]);
    cpu.io[0x05] = 0xFF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.io[0x05], 0xDF);
}

#[test]
fn test_execute_sbi_cbi_all_bits() {
    for b in 0u8..8 {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0x9A00 | (5u16 << 3) | b as u16]);
        step_n(&mut cpu, 1);
        assert_eq!(cpu.io[5], 1u8 << b, "SBI 5,{b}");

        let mut cpu = Cpu::new();
        cpu.load_program(&[0x9800 | (5u16 << 3) | b as u16]);
        cpu.io[5] = 0xFF;
        step_n(&mut cpu, 1);
        assert_eq!(cpu.io[5], !(1u8 << b), "CBI 5,{b}");
    }
}

#[test]
fn test_execute_sbi_cbi_idempotent() {
    // SBI twice
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x9A2D, 0x9A2D]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.io[0x05], 0x20);

    // CBI on a bit already off
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x982D]);
    cpu.io[0x05] = 0x01;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.io[0x05], 0x01);
}

#[test]
fn test_execute_sbi_cbi_use_io_and_keep_flags() {
    // the bit lands in io, not in R5, and sreg is untouched
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x9A2D, 0x982D]);
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.registers[5], 0x00);
    assert_eq!(cpu.io[0x05], 0x20);
    step_n(&mut cpu, 1);
    assert_eq!(cpu.io[0x05], 0x00);
    assert_eq!(cpu.registers[5], 0x00);
    assert_eq!(cpu.sreg, 0xFF);
}

#[test]
fn test_execute_blink_start_integration() {
    // SBI 0x04,5 (DDRB) / SBI 0x05,5 (PORTB on) / CBI 0x05,5 (PORTB off)
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x9A25, 0x9A2D, 0x982D]);
    step_n(&mut cpu, 1);
    assert_eq!(cpu.io[0x04], 0x20);
    step_n(&mut cpu, 1);
    assert_eq!(cpu.io[0x05], 0x20);
    step_n(&mut cpu, 1);
    assert_eq!(cpu.io[0x05], 0x00);
    assert_eq!(cpu.io[0x04], 0x20);
}

#[test]
fn test_decode_skips() {
    // CPSE R16,R16 / R16,R17 / R0,R0 / R31,R31
    assert_eq!(decode(0x1300), Instruction::Cpse { rd: 16, rr: 16 });
    assert_eq!(decode(0x1301), Instruction::Cpse { rd: 16, rr: 17 });
    assert_eq!(decode(0x1000), Instruction::Cpse { rd: 0, rr: 0 });
    assert_eq!(decode(0x13FF), Instruction::Cpse { rd: 31, rr: 31 });
    // SBRC R16,0 / R16,7 / R0,0 / R31,7
    assert_eq!(decode(0xFD00), Instruction::Sbrc { rr: 16, b: 0 });
    assert_eq!(decode(0xFD07), Instruction::Sbrc { rr: 16, b: 7 });
    assert_eq!(decode(0xFC00), Instruction::Sbrc { rr: 0, b: 0 });
    assert_eq!(decode(0xFDF7), Instruction::Sbrc { rr: 31, b: 7 });
    // SBRS R16,0 / R16,7 / R0,0 / R31,7
    assert_eq!(decode(0xFF00), Instruction::Sbrs { rr: 16, b: 0 });
    assert_eq!(decode(0xFF07), Instruction::Sbrs { rr: 16, b: 7 });
    assert_eq!(decode(0xFE00), Instruction::Sbrs { rr: 0, b: 0 });
    assert_eq!(decode(0xFFF7), Instruction::Sbrs { rr: 31, b: 7 });
    // SBIC 0x00,0 / 0x05,5 ; SBIS 0x05,5 / 0x1F,7
    assert_eq!(decode(0x9900), Instruction::Sbic { a: 0x00, b: 0 });
    assert_eq!(decode(0x992D), Instruction::Sbic { a: 0x05, b: 5 });
    assert_eq!(decode(0x9B2D), Instruction::Sbis { a: 0x05, b: 5 });
    assert_eq!(decode(0x9BFF), Instruction::Sbis { a: 0x1F, b: 7 });
}

#[test]
fn test_decode_skips_all_operands() {
    for r in 0u8..32 {
        for b in 0u8..8 {
            let low = ((r as u16) << 4) | b as u16;
            assert_eq!(
                decode(0xFC00 | low),
                Instruction::Sbrc { rr: r, b },
                "SBRC R{r},{b}"
            );
            assert_eq!(
                decode(0xFE00 | low),
                Instruction::Sbrs { rr: r, b },
                "SBRS R{r},{b}"
            );
        }
    }
    for a in 0u8..32 {
        for b in 0u8..8 {
            let low = ((a as u16) << 3) | b as u16;
            assert_eq!(
                decode(0x9900 | low),
                Instruction::Sbic { a, b },
                "SBIC {a:#04X},{b}"
            );
            assert_eq!(
                decode(0x9B00 | low),
                Instruction::Sbis { a, b },
                "SBIS {a:#04X},{b}"
            );
        }
    }
}

#[test]
fn test_decode_skips_neighbours() {
    // BLD, BST, reserved bit 3, CP, SUB, ADC, SBC, CBI, SBI
    for word in [
        0xF800, 0xFA00, 0xFC08, 0xFE08, 0x1400, 0x1800, 0x1C00, 0x0800, 0x9800, 0x9A00,
    ] {
        assert!(
            !matches!(
                decode(word),
                Instruction::Cpse { .. }
                    | Instruction::Sbrc { .. }
                    | Instruction::Sbrs { .. }
                    | Instruction::Sbic { .. }
                    | Instruction::Sbis { .. }
            ),
            "{word:#06X} must not decode as a skip"
        );
    }
    // the skips do not decode as each other
    assert!(!matches!(decode(0xFD00), Instruction::Sbrs { .. }));
    assert!(!matches!(decode(0xFF00), Instruction::Sbrc { .. }));
    assert!(!matches!(decode(0x992D), Instruction::Sbis { .. }));
    assert!(!matches!(decode(0x9B2D), Instruction::Sbic { .. }));
}

#[test]
fn test_execute_cpse() {
    // CPSE R16,R17 / LDI R18,5 (skipped when equal) / LDI R19,6
    let prog = [0x1301, 0xE025, 0xE036];
    let mut cpu = Cpu::new();
    cpu.load_program(&prog);
    cpu.registers[16] = 7;
    cpu.registers[17] = 7;
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[18], 0);
    assert_eq!(cpu.registers[19], 6);
    assert_eq!(cpu.pc, 3);
    assert_eq!(cpu.sreg, 0xFF);

    // different : the LDI runs
    let mut cpu = Cpu::new();
    cpu.load_program(&prog);
    cpu.registers[16] = 7;
    cpu.registers[17] = 8;
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[18], 5);
    assert_eq!(cpu.pc, 2);
}

#[test]
fn test_execute_sbrc_sbrs() {
    // SBRC R16,3 / LDI R18,5 / LDI R19,6
    let prog = [0xFD03, 0xE025, 0xE036];
    // bit 3 clear : skip
    let mut cpu = Cpu::new();
    cpu.load_program(&prog);
    cpu.registers[16] = 0x00;
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[18], 0);
    assert_eq!(cpu.registers[19], 6);
    // bit 3 set : no skip
    let mut cpu = Cpu::new();
    cpu.load_program(&prog);
    cpu.registers[16] = 0x08;
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[18], 5);

    // SBRS R16,3 / LDI R18,5 / LDI R19,6
    let prog = [0xFF03, 0xE025, 0xE036];
    // bit 3 set : skip
    let mut cpu = Cpu::new();
    cpu.load_program(&prog);
    cpu.registers[16] = 0x08;
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[18], 0);
    assert_eq!(cpu.registers[19], 6);
    // bit 3 clear : no skip
    let mut cpu = Cpu::new();
    cpu.load_program(&prog);
    cpu.registers[16] = 0x00;
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[18], 5);
}

#[test]
fn test_execute_sbic_sbis() {
    // SBIC 0x05,5 / LDI R18,5 / LDI R19,6
    let prog = [0x992D, 0xE025, 0xE036];
    let mut cpu = Cpu::new();
    cpu.load_program(&prog);
    cpu.io[0x05] = 0x00;
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[18], 0);
    assert_eq!(cpu.registers[19], 6);
    let mut cpu = Cpu::new();
    cpu.load_program(&prog);
    cpu.io[0x05] = 0x20;
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[18], 5);

    // SBIS 0x05,5 / LDI R18,5 / LDI R19,6
    let prog = [0x9B2D, 0xE025, 0xE036];
    let mut cpu = Cpu::new();
    cpu.load_program(&prog);
    cpu.io[0x05] = 0x20;
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[18], 0);
    assert_eq!(cpu.registers[19], 6);
    let mut cpu = Cpu::new();
    cpu.load_program(&prog);
    cpu.io[0x05] = 0x00;
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[18], 5);
}

#[test]
fn test_execute_sbis_reads_io_not_registers() {
    // R5 has bit 5 set but io[5] does not : no skip
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x9B2D, 0xE025]);
    cpu.registers[5] = 0x20;
    cpu.io[0x05] = 0x00;
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[18], 5);
}

#[test]
fn test_execute_skip_over_two_word_instructions() {
    // SBRC R16,0 (R16 = 0 : skip) / LDS R17,0x0100 / LDI R18,5
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xFD00, 0x9110, 0x0100, 0xE025]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.pc, 4);
    assert_eq!(cpu.registers[18], 5);

    // STS
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xFD00, 0x9310, 0x0100, 0xE025]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.pc, 4);
    assert_eq!(cpu.registers[18], 5);

    // JMP : the second word 0xE0FF looks like LDI R31,0x0F and must not run
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xFD00, 0x940C, 0xE0FF, 0xE025]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.pc, 4);
    assert_eq!(cpu.registers[31], 0);
    assert_eq!(cpu.registers[18], 5);

    // CALL : nothing is pushed
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xFD00, 0x940E, 0x0010, 0xE025]);
    cpu.sp = 0x08FF;
    step_n(&mut cpu, 2);
    assert_eq!(cpu.pc, 4);
    assert_eq!(cpu.sp, 0x08FF);
}

#[test]
fn test_execute_skip_one_word_instruction_skips_only_one() {
    // SBRC R16,0 / LDI R18,5 / LDI R19,6 : only the first LDI is skipped
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xFD00, 0xE025, 0xE036]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.pc, 3);
    assert_eq!(cpu.registers[19], 6);
}

#[test]
fn test_execute_skip_a_skip() {
    // SBRC R16,0 (skip) / SBRS R17,0 / LDI R18,5
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xFD00, 0xFF10, 0xE025]);
    step_n(&mut cpu, 2);
    assert_eq!(cpu.pc, 3);
    assert_eq!(cpu.registers[18], 5);
}

#[test]
fn test_execute_skip_at_end_of_flash_is_an_error() {
    let mut cpu = Cpu::new();
    cpu.flash[FLASH_SIZE - 1] = 0xFD00;
    cpu.pc = (FLASH_SIZE - 1) as u16;
    assert!(cpu.step().is_err());
}

#[test]
fn test_execute_skips_keep_flags() {
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xFD00, 0xE025, 0x9B2D, 0xE036]);
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 2);
    assert_eq!(cpu.sreg, 0xFF);
}

#[test]
fn test_execute_wait_loop_integration() {
    // 0: SBIS 0x05,5 / 1: RJMP -2 (back to 0) / 2: LDI R18,5
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x9B2D, 0xCFFE, 0xE025]);
    cpu.io[0x05] = 0x00;
    step_n(&mut cpu, 4); // two turns of the loop
    assert_eq!(cpu.pc, 0);
    cpu.io[0x05] = 0x20;
    step_n(&mut cpu, 2); // SBIS skips the RJMP, LDI runs
    assert_eq!(cpu.registers[18], 5);
    assert_eq!(cpu.pc, 3);
}

#[test]
fn test_decode_ld_st_simple_forms() {
    // (word, pointer, mode) for LD R16 and ST R16
    let forms = [
        (0x0C, Pointer::X, Mode::Direct),
        (0x0D, Pointer::X, Mode::PostInc),
        (0x0E, Pointer::X, Mode::PreDec),
        (0x09, Pointer::Y, Mode::PostInc),
        (0x0A, Pointer::Y, Mode::PreDec),
        (0x01, Pointer::Z, Mode::PostInc),
        (0x02, Pointer::Z, Mode::PreDec),
    ];
    for (low, pointer, mode) in forms {
        assert_eq!(decode(0x9100 | low), Instruction::Ld { rd: 16, pointer, mode });
        assert_eq!(decode(0x9300 | low), Instruction::St { rr: 16, pointer, mode });
    }
}

#[test]
fn test_decode_ld_st_all_registers() {
    for r in 0u8..32 {
        let low = (r as u16) << 4;
        assert_eq!(
            decode(0x900D | low),
            Instruction::Ld { rd: r, pointer: Pointer::X, mode: Mode::PostInc },
            "LD R{r},X+"
        );
        assert_eq!(
            decode(0x920E | low),
            Instruction::St { rr: r, pointer: Pointer::X, mode: Mode::PreDec },
            "ST -X,R{r}"
        );
    }
}

#[test]
fn test_decode_ldd_std_known_words() {
    assert_eq!(decode(0x8108), Instruction::Ldd { rd: 16, pointer: Pointer::Y, q: 0 });
    assert_eq!(decode(0x8109), Instruction::Ldd { rd: 16, pointer: Pointer::Y, q: 1 });
    assert_eq!(decode(0x8101), Instruction::Ldd { rd: 16, pointer: Pointer::Z, q: 1 });
    assert_eq!(decode(0xAD0F), Instruction::Ldd { rd: 16, pointer: Pointer::Y, q: 63 });
    assert_eq!(decode(0xAD07), Instruction::Ldd { rd: 16, pointer: Pointer::Z, q: 63 });
    assert_eq!(decode(0x8309), Instruction::Std { rr: 16, pointer: Pointer::Y, q: 1 });
    assert_eq!(decode(0x8301), Instruction::Std { rr: 16, pointer: Pointer::Z, q: 1 });
    assert_eq!(decode(0xAF0F), Instruction::Std { rr: 16, pointer: Pointer::Y, q: 63 });
    assert_eq!(decode(0xAF07), Instruction::Std { rr: 16, pointer: Pointer::Z, q: 63 });
}

#[test]
fn test_decode_ldd_std_all_q() {
    // the word is built from q : bits 5 / 4-3 / 2-0 go to bit 13 / 11-10 / 2-0
    for q in 0u8..64 {
        let q16 = q as u16;
        let bits = ((q16 & 0x20) << 8) | ((q16 & 0x18) << 7) | (q16 & 0x07);
        assert_eq!(decode(0x8100 | bits), Instruction::Ldd { rd: 16, pointer: Pointer::Z, q }, "LDD Z+{q}");
        assert_eq!(decode(0x8108 | bits), Instruction::Ldd { rd: 16, pointer: Pointer::Y, q }, "LDD Y+{q}");
        assert_eq!(decode(0x8300 | bits), Instruction::Std { rr: 16, pointer: Pointer::Z, q }, "STD Z+{q}");
        assert_eq!(decode(0x8308 | bits), Instruction::Std { rr: 16, pointer: Pointer::Y, q }, "STD Y+{q}");
    }
}

#[test]
fn test_decode_ld_st_neighbours() {
    // LDS, STS, POP, PUSH, LPM, ELPM, unused nibbles, IN, RJMP
    for word in [
        0x9000, 0x9200, 0x900F, 0x920F, 0x9004, 0x9005, 0x9006, 0x9003, 0x9008, 0x900B, 0xB000, 0xC000,
    ] {
        assert!(
            !matches!(
                decode(word),
                Instruction::Ld { .. } | Instruction::St { .. } | Instruction::Ldd { .. } | Instruction::Std { .. }
            ),
            "{word:#06X} must not decode as LD, ST, LDD or STD"
        );
    }
}

fn run_ptr(program: &[u16], setup: impl FnOnce(&mut Cpu)) -> Cpu {
    let mut cpu = Cpu::new();
    cpu.load_program(program);
    setup(&mut cpu);
    step_n(&mut cpu, 1);
    cpu
}

#[test]
fn test_execute_ld_x_modes() {
    // LD R16, X  (X = 0x0100)
    let cpu = run_ptr(&[0x910C], |c| {
        c.registers[26] = 0x00;
        c.registers[27] = 0x01;
        c.sram[0] = 0x5A;
    });
    assert_eq!(cpu.registers[16], 0x5A);
    assert_eq!((cpu.registers[27], cpu.registers[26]), (0x01, 0x00));

    // LD R16, X+ : pointer incremented after
    let cpu = run_ptr(&[0x910D], |c| {
        c.registers[27] = 0x01;
        c.sram[0] = 0x5A;
    });
    assert_eq!(cpu.registers[16], 0x5A);
    assert_eq!((cpu.registers[27], cpu.registers[26]), (0x01, 0x01));

    // LD R16, -X : pointer decremented first, X = 0x0101 -> reads 0x0100
    let cpu = run_ptr(&[0x910E], |c| {
        c.registers[26] = 0x01;
        c.registers[27] = 0x01;
        c.sram[0] = 0x5A;
        c.sram[1] = 0x99;
    });
    assert_eq!(cpu.registers[16], 0x5A);
    assert_eq!((cpu.registers[27], cpu.registers[26]), (0x01, 0x00));
}

#[test]
fn test_execute_ld_y_z_modes() {
    // LD R16, Y+ / -Y / Z+ / -Z, pointer starts at 0x0100 (0x0101 for the -modes)
    for (word, base, start, end) in [
        (0x9109u16, 28usize, 0x0100u16, 0x0101u16),
        (0x910A, 28, 0x0101, 0x0100),
        (0x9101, 30, 0x0100, 0x0101),
        (0x9102, 30, 0x0101, 0x0100),
    ] {
        let cpu = run_ptr(&[word], |c| {
            c.registers[base] = start as u8;
            c.registers[base + 1] = (start >> 8) as u8;
            c.sram[0] = 0x5A;
        });
        assert_eq!(cpu.registers[16], 0x5A, "{word:#06X}");
        let got = ((cpu.registers[base + 1] as u16) << 8) | cpu.registers[base] as u16;
        assert_eq!(got, end, "{word:#06X} pointer");
    }
}

#[test]
fn test_execute_st_modes() {
    // ST X, R16 / X+ / -X
    for (word, start, end, offset) in [
        (0x930Cu16, 0x0100u16, 0x0100u16, 0usize),
        (0x930D, 0x0100, 0x0101, 0),
        (0x930E, 0x0101, 0x0100, 0),
    ] {
        let cpu = run_ptr(&[word], |c| {
            c.registers[16] = 0x42;
            c.registers[26] = start as u8;
            c.registers[27] = (start >> 8) as u8;
        });
        assert_eq!(cpu.sram[offset], 0x42, "{word:#06X}");
        assert_eq!(cpu.registers[16], 0x42);
        let got = ((cpu.registers[27] as u16) << 8) | cpu.registers[26] as u16;
        assert_eq!(got, end, "{word:#06X} pointer");
    }

    // ST Y+, R16 / ST -Z, R16
    let cpu = run_ptr(&[0x9309], |c| {
        c.registers[16] = 0x42;
        c.registers[29] = 0x01;
    });
    assert_eq!(cpu.sram[0], 0x42);
    assert_eq!(cpu.registers[28], 0x01);
    let cpu = run_ptr(&[0x9302], |c| {
        c.registers[16] = 0x42;
        c.registers[30] = 0x01;
        c.registers[31] = 0x01;
    });
    assert_eq!(cpu.sram[0], 0x42);
    assert_eq!((cpu.registers[31], cpu.registers[30]), (0x01, 0x00));
}

#[test]
fn test_execute_ldd_std() {
    // LDD R16, Y+1 (Y = 0x0100) : reads 0x0101, Y unchanged
    let cpu = run_ptr(&[0x8109], |c| {
        c.registers[29] = 0x01;
        c.sram[1] = 0x77;
    });
    assert_eq!(cpu.registers[16], 0x77);
    assert_eq!((cpu.registers[29], cpu.registers[28]), (0x01, 0x00));

    // LDD R16, Z+63
    let cpu = run_ptr(&[0xAD07], |c| {
        c.registers[31] = 0x01;
        c.sram[63] = 0x66;
    });
    assert_eq!(cpu.registers[16], 0x66);

    // LDD R16, Y+0 (this is LD R16, Y)
    let cpu = run_ptr(&[0x8108], |c| {
        c.registers[29] = 0x01;
        c.sram[0] = 0x11;
    });
    assert_eq!(cpu.registers[16], 0x11);

    // STD Y+1, R16 / STD Z+63, R16
    let cpu = run_ptr(&[0x8309], |c| {
        c.registers[16] = 0x42;
        c.registers[29] = 0x01;
    });
    assert_eq!(cpu.sram[1], 0x42);
    let cpu = run_ptr(&[0xAF07], |c| {
        c.registers[16] = 0x42;
        c.registers[31] = 0x01;
    });
    assert_eq!(cpu.sram[63], 0x42);
}

#[test]
fn test_execute_ld_st_zones() {
    // LD R16, X with X = 0x0010 (a register) / 0x0025 (PORTB) / 0x005F (SREG)
    let cpu = run_ptr(&[0x910C], |c| {
        c.registers[26] = 0x10;
        c.registers[17] = 0x33;
        c.registers[16] = 0x00;
    });
    assert_eq!(cpu.registers[16], 0x00);
    let cpu = run_ptr(&[0x910C], |c| {
        c.registers[26] = 0x11;
        c.registers[17] = 0x33;
    });
    assert_eq!(cpu.registers[16], 0x33);
    let cpu = run_ptr(&[0x910C], |c| {
        c.registers[26] = 0x25;
        c.io[0x05] = 0xAA;
    });
    assert_eq!(cpu.registers[16], 0xAA);
    let cpu = run_ptr(&[0x910C], |c| {
        c.registers[26] = 0x5F;
        c.sreg = 0x5A;
    });
    assert_eq!(cpu.registers[16], 0x5A);

    // ST X, R16 to SREG
    let cpu = run_ptr(&[0x930C], |c| {
        c.registers[16] = 0xA5;
        c.registers[26] = 0x5F;
    });
    assert_eq!(cpu.sreg, 0xA5);
}

#[test]
fn test_execute_ld_st_wrap_and_errors() {
    // -X from 0x0000 : the pointer wraps to 0xFFFF, the access fails, the pointer is untouched
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x910E]);
    cpu.registers[16] = 0x11;
    assert!(cpu.step().is_err());
    assert_eq!(cpu.registers[16], 0x11);
    assert_eq!((cpu.registers[27], cpu.registers[26]), (0x00, 0x00));

    // LD R16, X+ at 0x0900 : error, pointer not moved
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x910D]);
    cpu.registers[26] = 0x00;
    cpu.registers[27] = 0x09;
    assert!(cpu.step().is_err());
    assert_eq!((cpu.registers[27], cpu.registers[26]), (0x09, 0x00));

    // ST X+ to 0x08FF then the pointer wraps nowhere : 0x08FF -> 0x0900
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x930D]);
    cpu.registers[16] = 0x42;
    cpu.registers[26] = 0xFF;
    cpu.registers[27] = 0x08;
    assert_eq!(cpu.step(), Ok(2));
    assert_eq!(cpu.sram[0x08FF - 0x100], 0x42);
    assert_eq!((cpu.registers[27], cpu.registers[26]), (0x09, 0x00));

    // LDD R16, Y+1 with Y = 0xFFFF : no overflow panic, address wraps to 0x0000 (R0)
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x8109]);
    cpu.registers[28] = 0xFF;
    cpu.registers[29] = 0xFF;
    cpu.registers[0] = 0x21;
    assert_eq!(cpu.step(), Ok(2));
    assert_eq!(cpu.registers[16], 0x21);
}

#[test]
fn test_execute_ld_st_keep_flags() {
    let cpu = run_ptr(&[0x910D], |c| {
        c.registers[27] = 0x01;
        c.sreg = 0xFF;
    });
    assert_eq!(cpu.sreg, 0xFF);
}

#[test]
fn test_execute_clear_bss_loop_integration() {
    // 0: LDI R26,0 / 1: LDI R27,1 / 2: LDI R17,1
    // 3: ST X+,R1 / 4: CPI R26,4 / 5: CPC R27,R17 / 6: BRNE -4 (back to 3)
    // clears four bytes of SRAM at 0x0100..0x0103, the end is 0x0104
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xE0A0, 0xE0B1, 0xE011, 0x921D, 0x30A4, 0x07B1, 0xF7E1]);
    for i in 0..8 {
        cpu.sram[i] = 0xFF;
    }
    step_n(&mut cpu, 3 + 4 * 4);
    assert_eq!(&cpu.sram[0..4], &[0, 0, 0, 0]);
    assert_eq!(cpu.sram[4], 0xFF);
    assert_eq!(cpu.pc, 7);
}

#[test]
fn test_execute_copy_ld_x_plus_st_y_plus() {
    // LD R16,X+ / ST Y+,R16 twice : copy sram[0..2] to sram[8..10]
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x910D, 0x9309, 0x910D, 0x9309]);
    cpu.registers[27] = 0x01;
    cpu.registers[28] = 0x08;
    cpu.registers[29] = 0x01;
    cpu.sram[0] = 0xAA;
    cpu.sram[1] = 0xBB;
    step_n(&mut cpu, 4);
    assert_eq!(cpu.sram[8], 0xAA);
    assert_eq!(cpu.sram[9], 0xBB);
}

#[test]
fn test_decode_lpm() {
    assert_eq!(decode(0x95C8), Instruction::Lpm);
    assert_eq!(decode(0x9104), Instruction::Lpmz { rd: 16 });
    assert_eq!(decode(0x9004), Instruction::Lpmz { rd: 0 });
    assert_eq!(decode(0x91F4), Instruction::Lpmz { rd: 31 });
    assert_eq!(decode(0x9105), Instruction::Lpmzplus { rd: 16 });
    assert_eq!(decode(0x9005), Instruction::Lpmzplus { rd: 0 });
    for r in 0u8..32 {
        let low = (r as u16) << 4;
        assert_eq!(decode(0x9004 | low), Instruction::Lpmz { rd: r }, "LPM R{r},Z");
        assert_eq!(decode(0x9005 | low), Instruction::Lpmzplus { rd: r }, "LPM R{r},Z+");
    }
}

#[test]
fn test_decode_lpm_neighbours() {
    // ELPM, ELPM, WDR, SPM, XCH, LAS, LD Z+, LD X
    for word in [0x9006, 0x9007, 0x95D8, 0x95E8, 0x95A8, 0x9204, 0x9205, 0x9001, 0x900C] {
        assert!(
            !matches!(
                decode(word),
                Instruction::Lpm | Instruction::Lpmz { .. } | Instruction::Lpmzplus { .. }
            ),
            "{word:#06X} must not decode as LPM"
        );
    }
}

fn run_lpm(word: u16, z: u16) -> Cpu {
    let mut cpu = Cpu::new();
    cpu.load_program(&[word]);
    cpu.flash[0x10] = 0xBEEF;
    cpu.registers[30] = z as u8;
    cpu.registers[31] = (z >> 8) as u8;
    step_n(&mut cpu, 1);
    cpu
}

#[test]
fn test_execute_lpm_low_and_high_byte() {
    // Z = 0x0020 : low byte of word 0x10 ; Z = 0x0021 : high byte
    assert_eq!(run_lpm(0x9104, 0x0020).registers[16], 0xEF);
    assert_eq!(run_lpm(0x9104, 0x0021).registers[16], 0xBE);
    assert_eq!(run_lpm(0x9105, 0x0020).registers[16], 0xEF);
    assert_eq!(run_lpm(0x9105, 0x0021).registers[16], 0xBE);
}

#[test]
fn test_execute_lpm_r0() {
    let cpu = run_lpm(0x95C8, 0x0021);
    assert_eq!(cpu.registers[0], 0xBE);
    assert_eq!(cpu.registers[16], 0x00);
    let cpu = run_lpm(0x95C8, 0x0020);
    assert_eq!(cpu.registers[0], 0xEF);
}

#[test]
fn test_execute_lpm_z_unchanged_and_post_increment() {
    let cpu = run_lpm(0x9104, 0x0020);
    assert_eq!((cpu.registers[31], cpu.registers[30]), (0x00, 0x20));
    let cpu = run_lpm(0x9105, 0x0020);
    assert_eq!((cpu.registers[31], cpu.registers[30]), (0x00, 0x21));
    // carry into the high byte : 0x00FF -> 0x0100
    let cpu = run_lpm(0x9105, 0x00FF);
    assert_eq!((cpu.registers[31], cpu.registers[30]), (0x01, 0x00));
}

#[test]
fn test_execute_lpm_z_plus_reads_low_then_high() {
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x9105, 0x9115]);
    cpu.flash[0x10] = 0xBEEF;
    cpu.registers[30] = 0x20;
    step_n(&mut cpu, 2);
    assert_eq!(cpu.registers[16], 0xEF);
    assert_eq!(cpu.registers[17], 0xBE);
}

#[test]
fn test_execute_lpm_outside_flash_is_an_error() {
    for word in [0x95C8u16, 0x9104, 0x9105] {
        let mut cpu = Cpu::new();
        cpu.load_program(&[word]);
        cpu.registers[16] = 0x11;
        cpu.registers[31] = 0x80; // Z = 0x8000 -> word index 16384
        assert!(cpu.step().is_err(), "{word:#06X}");
        assert_eq!(cpu.registers[16], 0x11);
        assert_eq!((cpu.registers[31], cpu.registers[30]), (0x80, 0x00));
    }
}

#[test]
fn test_execute_lpm_keeps_flags() {
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x9105]);
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 1);
    assert_eq!(cpu.sreg, 0xFF);
}

#[test]
fn test_execute_copy_data_loop_integration() {
    // 0: LDI R30,0x10 / 1: LDI R31,0 / 2: LDI R26,0 / 3: LDI R27,1
    // 4: LPM R0,Z+ / 5: ST X+,R0 / 6: LPM R0,Z+ / 7: ST X+,R0
    // data in flash word 8 (byte address 0x10) = 0xBEEF
    let mut cpu = Cpu::new();
    cpu.load_program(&[
        0xE1E0, 0xE0F0, 0xE0A0, 0xE0B1, 0x9005, 0x920D, 0x9005, 0x920D,
    ]);
    cpu.flash[8] = 0xBEEF;
    step_n(&mut cpu, 8);
    assert_eq!(cpu.sram[0], 0xEF);
    assert_eq!(cpu.sram[1], 0xBE);
    assert_eq!((cpu.registers[27], cpu.registers[26]), (0x01, 0x02));
}

#[test]
fn test_decode_sleep_wdr_break() {
    assert_eq!(decode(0x9588), Instruction::Sleep);
    assert_eq!(decode(0x95A8), Instruction::Wdr);
    assert_eq!(decode(0x9598), Instruction::Break);
}

#[test]
fn test_decode_sleep_wdr_break_neighbours() {
    // RET, RETI, LPM, SPM, unused words, SLEEP + 1
    for word in [0x9508, 0x9518, 0x95C8, 0x95E8, 0x95B8, 0x95D8, 0x95F8, 0x9589] {
        assert!(
            !matches!(
                decode(word),
                Instruction::Sleep | Instruction::Wdr | Instruction::Break
            ),
            "{word:#06X} must not decode as SLEEP, WDR or BREAK"
        );
    }
    // they do not decode as each other
    assert!(!matches!(decode(0x9588), Instruction::Wdr | Instruction::Break));
    assert!(!matches!(decode(0x95A8), Instruction::Sleep | Instruction::Break));
    assert!(!matches!(decode(0x9598), Instruction::Sleep | Instruction::Wdr));
}

#[test]
fn test_execute_sleep_wdr_break_do_nothing() {
    for word in [0x9588u16, 0x95A8, 0x9598] {
        let mut cpu = Cpu::new();
        cpu.load_program(&[word]);
        cpu.sreg = 0xFF;
        cpu.sp = 0x08FF;
        cpu.registers[16] = 0x55;
        cpu.registers[31] = 0xAA;
        cpu.io[5] = 0x20;
        step_n(&mut cpu, 1);
        assert_eq!(cpu.pc, 1, "{word:#06X}");
        assert_eq!(cpu.sreg, 0xFF, "{word:#06X}");
        assert_eq!(cpu.sp, 0x08FF, "{word:#06X}");
        assert_eq!(cpu.registers[16], 0x55, "{word:#06X}");
        assert_eq!(cpu.registers[31], 0xAA, "{word:#06X}");
        assert_eq!(cpu.io[5], 0x20, "{word:#06X}");
    }
}

#[test]
fn test_execute_sleep_wdr_break_in_a_row() {
    let mut cpu = Cpu::new();
    cpu.load_program(&[0x9588, 0x95A8, 0x9598]);
    cpu.sreg = 0xFF;
    step_n(&mut cpu, 3);
    assert_eq!(cpu.pc, 3);
    assert_eq!(cpu.sreg, 0xFF);
}

// Words that must stay Unknown on the ATmega328P (reserved or absent encodings).
fn expected_unknown(w: u16) -> bool {
    let lo4 = w & 0x000F;
    match w {
        // only NOP (0x0000) exists below 0x0100
        0x0000..=0x00FF => w != 0x0000,
        // LD / LDS / LPM / POP group : 3, 6 and 7 (ELPM), 8 and B are unused
        0x9000..=0x91FF => matches!(lo4, 0x3 | 0x6 | 0x7 | 0x8 | 0xB),
        // ST / STS / PUSH group : 3, 4 (XCH), 5 (LAS), 6 (LAC), 7 (LAT), 8, B
        0x9200..=0x93FF => matches!(lo4, 0x3 | 0x4 | 0x5 | 0x6 | 0x7 | 0x8 | 0xB),
        // 0x94xx and 0x95xx
        0x9400..=0x95FF => match lo4 {
            0x4 | 0xB => true, // 0x9404 hole, DES
            0x9 => w != 0x9409 && w != 0x9509, // only IJMP and ICALL (EIJMP, EICALL absent)
            0x8 => {
                // 0x94x8 is BSET/BCLR for every s, 0x95x8 is a mix
                w >> 8 == 0x95
                    && !matches!(w, 0x9508 | 0x9518 | 0x9588 | 0x9598 | 0x95A8 | 0x95C8)
            }
            _ => false,
        },
        // BLD, BST, SBRC, SBRS : bit 3 must be 0
        0xF800..=0xFFFF => w & 0x0008 != 0,
        _ => false,
    }
}

#[test]
fn test_decode_coverage_all_words() {
    let mut unknown = 0u32;
    for w in 0..=u16::MAX {
        let is_unknown = matches!(decode(w), Instruction::Unknown(..));
        assert_eq!(
            is_unknown,
            expected_unknown(w),
            "{w:#06X} : decode says unknown = {is_unknown}, expected {}",
            expected_unknown(w)
        );
        if is_unknown {
            unknown += 1;
        }
    }
    // 255 + 160 + 224 + 104 + 1024
    assert_eq!(unknown, 1767);
}

#[test]
fn test_execute_never_panics_on_any_word() {
    // every word that decodes to a real instruction is run once on a cpu whose
    // pointers and stack are in valid places ; errors are fine, panics are not
    for w in 0..=u16::MAX {
        let mut cpu = Cpu::new();
        cpu.load_program(&[w, 0x0000]);
        cpu.sp = 0x08FF;
        for r in [26, 28, 30] {
            cpu.registers[r] = 0x00;
            cpu.registers[r + 1] = 0x01;
        }
        cpu.sreg = 0xFF;
        let _ = cpu.step();
    }
}

//Loader test
#[test]
fn load_hex_puts_words_in_flash() {
    let mut cpu = Cpu::new();
    // octets 0x0C 0x94 à l'adresse 0 → mot 0x940C (little-endian)
    let hex = ":020000000C945E\n:00000001FF\n";
    load_hex(hex, &mut cpu).unwrap();
    assert_eq!(cpu.flash[0], 0x940C);
}

#[test]
fn load_hex_rejects_bad_checksum() {
    let mut cpu = Cpu::new();
    assert!(load_hex(":0100000042BE\n", &mut cpu).is_err());
}