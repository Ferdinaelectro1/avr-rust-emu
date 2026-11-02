use std::assert_eq;

use super::*;

#[test]
fn write_regs_test() {
    let mut cpu = Cpu::new();
    assert_eq!(cpu.write_data(REGISTERS_START, 0xFF),Ok(()));
    assert_eq!(cpu.registers[0],0xFF);
    assert_eq!(cpu.write_data(REGISTERS_START + 3, 0xFF),Ok(()));
    assert_eq!(cpu.registers[3],0xFF);
    assert_eq!(cpu.write_data(REGISTERS_END, 0xFF),Ok(()));
    assert_eq!(cpu.registers[CPU_REGISTERS_SIZE - 1],0xFF);
}

#[test]
fn write_io_test() {
    let mut cpu = Cpu::new();
    assert_eq!(cpu.write_data(IO_START, 0xFF),Ok(()));
    assert_eq!(cpu.io[0],0xFF);
    assert_eq!(cpu.write_data(IO_START + 3, 0xFF),Ok(()));
    assert_eq!(cpu.io[3],0xFF);
    assert_eq!(cpu.write_data(IO_END, 0xFF),Ok(()));
    assert_eq!(cpu.io[IO_SIZE - 1],0xFF);
}

#[test]
fn write_sram_test() {
    let mut cpu = Cpu::new();
    assert_eq!(cpu.write_data(SRAM_START, 0xFF),Ok(()));
    assert_eq!(cpu.sram[0],0xFF);
    assert_eq!(cpu.write_data(SRAM_START + 3, 0xFF),Ok(()));
    assert_eq!(cpu.sram[3],0xFF);
    assert_eq!(cpu.write_data(RAMEND, 0xFF),Ok(()));
    assert_eq!(cpu.sram[SRAM_SIZE - 1],0xFF);
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