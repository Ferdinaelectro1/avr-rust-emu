use std::assert_eq;

use super::*;

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
    assert_eq!(cpu.step(), Ok(()));
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
    assert_eq!(cpu.step(), Ok(()));
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
    assert_eq!(cpu.step(), Ok(()));
    assert_eq!(cpu.registers[31], 255);
    assert_eq!(cpu.pc, 1);
}

#[test]
fn test_execute_two_ldi() {
    let mut cpu = Cpu::new();
    // LDI R16, 5 / LDI R23, 23
    cpu.load_program(&[0xE005, 0xE177]);
    assert_eq!(cpu.step(), Ok(()));
    assert_eq!(cpu.step(), Ok(()));
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
        assert_eq!(cpu.step(), Ok(()));
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
    assert_eq!(cpu.step(), Ok(()));
    assert_eq!(cpu.pc, 1);
}

#[test]
fn test_execute_rjmp_forward_skips_instruction() {
    let mut cpu = Cpu::new();
    // RJMP +1 / LDI R16, 5 (skipped) / LDI R17, 7
    cpu.load_program(&[0xC001, 0xE005, 0xE017]);
    assert_eq!(cpu.step(), Ok(()));
    assert_eq!(cpu.step(), Ok(()));
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
        assert_eq!(cpu.step(), Ok(()));
        assert_eq!(cpu.pc, 0);
    }
}

#[test]
fn test_execute_rjmp_backward() {
    let mut cpu = Cpu::new();
    // LDI R16, 5 / RJMP -2
    cpu.load_program(&[0xE005, 0xCFFE]);
    assert_eq!(cpu.step(), Ok(()));
    assert_eq!(cpu.step(), Ok(()));
    assert_eq!(cpu.registers[16], 5);
    assert_eq!(cpu.pc, 0);
}

#[test]
fn test_execute_rjmp_wraps_below_zero() {
    let mut cpu = Cpu::new();
    // RJMP -2 at address 0
    cpu.load_program(&[0xCFFE]);
    assert_eq!(cpu.step(), Ok(()));
    assert_eq!(cpu.pc as usize, FLASH_SIZE - 1);
}

#[test]
fn test_execute_rjmp_wraps_past_flash_end() {
    let mut cpu = Cpu::new();
    // RJMP +1 at the last address of the flash
    cpu.flash[FLASH_SIZE - 1] = 0xC001;
    cpu.pc = (FLASH_SIZE - 1) as u16;
    assert_eq!(cpu.step(), Ok(()));
    assert_eq!(cpu.pc, 1);
}

#[test]
fn test_execute_rjmp_does_not_touch_sreg() {
    let mut cpu = Cpu::new();
    cpu.sreg = 0xFF;
    cpu.load_program(&[0xC001]);
    assert_eq!(cpu.step(), Ok(()));
    assert_eq!(cpu.sreg, 0xFF);
}

fn step_n(cpu: &mut Cpu, steps: usize) {
    for _ in 0..steps {
        assert_eq!(cpu.step(), Ok(()));
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
