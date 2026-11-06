// Not every example uses every helper
#![allow(dead_code)]

use avr_rust_emu::{Cpu,decode};

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

/// Run until `pc` leaves the program, printing one trace line per step.
/// Returns the number of steps executed.
pub fn run_until_end(cpu: &mut Cpu, program: &[u16], max_steps: usize) -> Result<usize, String> {
    cpu.load_program(program);
    let mut steps = 0;
    while (cpu.pc() as usize) < program.len() {
        if steps == max_steps {
            return Err(format!("stopped after {max_steps} steps (infinite loop?)"));
        }
        println!(
            "step {:>2}: pc={:#06X}  word={:#06X}",
            steps + 1,
            cpu.pc(),
            program[cpu.pc() as usize]
        );
        cpu.step()?;
        steps += 1;
    }
    Ok(steps)
}

pub fn run_steps(cpu: &mut Cpu, max_steps: usize) -> Result<(), String> {
    for n in 0..max_steps {
        let word = cpu.flash[cpu.pc() as usize];
        let instr = decode(word);
        println!("step {:>3}: pc={:#06X}  word={:#06X}  {:?}", n + 1, cpu.pc(), word, instr);
        cpu.step()?;
    }
    Ok(())
}
