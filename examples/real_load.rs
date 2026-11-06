mod common;
use std::{fs, println};

use avr_rust_emu::{Cpu, hex::load_hex};

fn main() {
    let hex = fs::read_to_string("code.hex").expect("cannot read test.hex");
    let mut cpu = Cpu::new();
    let total = load_hex(&hex, &mut cpu).expect("hex load failed");
    println!("{} total octets write : ", total);
    println!("{:04X?}", &cpu.flash[..8]);
    println!("Running program ...");
    if let Err(e) = common::run_steps(&mut cpu, 30) {
        println!("Arrêt : {}", e);
    }
    cpu.print_cpu();
}