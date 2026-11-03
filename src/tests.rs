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
