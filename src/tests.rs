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

#[test]
fn read_regs_test() {
    let mut cpu = Cpu::new();
    cpu.registers[0] = 0xFF;
    assert_eq!(cpu.read_data(REGISTERS_START),Ok(0xFF));
    cpu.registers[3] = 0xFF;
    assert_eq!(cpu.read_data(REGISTERS_START+3),Ok(0xFF));
    cpu.registers[CPU_REGISTERS_SIZE-1] = 0xFF;
    assert_eq!(cpu.read_data(REGISTERS_END),Ok(0xFF));
}

#[test]
fn read_io_test() {
    let mut cpu = Cpu::new();
    cpu.io[0] = 0xFF;
    assert_eq!(cpu.read_data(IO_START),Ok(0xFF));
    cpu.io[3] = 0xFF;
    assert_eq!(cpu.read_data(IO_START+3),Ok(0xFF));
    cpu.io[IO_SIZE-1] = 0xFF;
    assert_eq!(cpu.read_data(IO_END),Ok(0xFF));
}

#[test]
fn read_sram_test() {
    let mut cpu = Cpu::new();
    cpu.sram[0] = 0xFF;
    assert_eq!(cpu.read_data(SRAM_START),Ok(0xFF));
    cpu.sram[3] = 0xFF;
    assert_eq!(cpu.read_data(SRAM_START + 3),Ok(0xFF));
    cpu.sram[SRAM_SIZE-1] = 0xFF;
    assert_eq!(cpu.read_data(RAMEND),Ok(0xFF));
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
    assert_eq!(cpu.read_data(SREG_ADDR),Ok(0xF2));
}

#[test]
fn spl_read_returns_low_byte() {
    let mut cpu = Cpu::new();
    cpu.sp = 0x1234;
    assert_eq!(cpu.read_data(SPL_ADDR),Ok(0x34));
}

#[test]
fn sph_read_keeps_high_byte() {
    let mut cpu = Cpu::new();
    cpu.sp = 0x1234;
    assert_eq!(cpu.read_data(SPH_ADDR),Ok(0x12));
}
