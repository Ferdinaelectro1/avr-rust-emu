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