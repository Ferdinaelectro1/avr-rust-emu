//! Fibonacci with two registers.
//!
//! ```text
//! LDI R16, 1      0xE001
//! LDI R17, 1      0xE011
//! ADD R16, R17    0x0F01   R16 = R16 + R17
//! ADD R17, R16    0x0F10   R17 = R17 + R16
//! ... the two ADD repeated 6 times
//! ```
//!
//! Registers and flags after each step:
//!
//! ```text
//! step  R16  R17  flags
//!    1    1    0
//!    2    1    1
//!    3    2    1
//!    4    2    3
//!    5    5    3
//!    6    5    8
//!    7   13    8
//!    8   13   21  H
//!    9   34   21  H
//!   10   34   55
//!   11   89   55
//!   12   89  144  H V N
//!   13  233  144  S N
//!   14  233  121  S V C   (144 + 233 = 377 overflows 8 bits)
//! ```
//!
//! Run with: `cargo run --example fibonacci`

mod common;

use avr_rust_emu::Cpu;

fn main() {
    let mut cpu = Cpu::new();
    let program = [
        0xE001, 0xE011, 0x0F01, 0x0F10, 0x0F01, 0x0F10, 0x0F01, 0x0F10, 0x0F01, 0x0F10, 0x0F01,
        0x0F10, 0x0F01, 0x0F10,
    ];

    if let Err(msg) = common::run_program(&mut cpu, &program) {
        eprintln!("{msg}");
    }
    cpu.print_cpu();
}
