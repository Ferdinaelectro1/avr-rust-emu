//! Countdown from 3 to 0 with a conditional branch.
//!
//! ```text
//! LDI R16, 3      0xE003
//! LDI R17, 1      0xE011
//! SUB R16, R17    0x1B01   <- loop start (address 2)
//! BRNE -2         0xF7F1   jump back to the SUB while Z is 0
//! ```
//!
//! R16 goes 3, 2, 1, 0. The last SUB gives 0 and turns Z on, so BRNE
//! is not taken and the program ends.
//!
//! Expected: 8 steps, R16 = 0x00, R17 = 0x01, SREG = 0x02 (Z), PC = 0x0004.
//!
//! Run with: `cargo run --example countdown`

mod common;

use avr_rust_emu::Cpu;

fn main() {
    let mut cpu = Cpu::new();
    let program = [0xE003, 0xE011, 0x1B01, 0xF7F1];

    match common::run_until_end(&mut cpu, &program, 100) {
        Ok(steps) => println!("done in {steps} steps"),
        Err(msg) => eprintln!("{msg}"),
    }
    cpu.print_cpu();
}