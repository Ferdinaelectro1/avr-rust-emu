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
