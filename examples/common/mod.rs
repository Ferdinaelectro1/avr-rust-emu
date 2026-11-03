// Not every example uses every helper
#![allow(dead_code)]

use avr_rust_emu::Cpu;

/// Load `program` into flash and execute it one word at a time,
/// printing a trace line after each step. Stops at the first error.
pub fn run_program(cpu: &mut Cpu, program: &[u16]) -> Result<(), String> {
    cpu.load_program(program);
    for (i, word) in program.iter().enumerate() {
        cpu.step()?;
        println!("step {:>2}: {:#06X}", i + 1, word);
    }
    Ok(())
}
