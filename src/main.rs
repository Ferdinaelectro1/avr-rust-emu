use std::println;

use avr_rust_emu::Cpu;


fn main() {
    let mut cpu  = Cpu::new();
    /*
    LDI R16, 1      → 0xE001
    LDI R17, 1      → 0xE011
    ADD R16, R17    → 0x0F01   (R16 = R16 + R17)
    ADD R17, R16    → 0x0F10   (R17 = R17 + R16)
    ... twice add repeat 6 times
    */
    let program = [
        0xE001,
        0xE011,
        0x0F01,
        0x0F10,
        0x0F01,
        0x0F10,
        0x0F01,
        0x0F10,
        0x0F01,
        0x0F10,
        0x0F01,
        0x0F10,
        0x0F01,
        0x0F10,
    ];
    cpu.load_program(&program);
    for _ in 0..program.len() {
        match cpu.step() {
            Ok(()) => {
                println!("next instruction..");
            }   ,
            Err(msg) => {
                println!("{msg}");
            }
        }
    }
    cpu.print_cpu();
}
