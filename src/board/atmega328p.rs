use super::{Machine, PinMode, PinState};
use crate::hex::load_hex;
use crate::{Cpu, IO_START};

const CLOCK_HZ: u64 = 16_000_000;

pub struct Atmega328pBoard {
    cpu: Cpu,
    /// Levels imposed from outside on port B input pins (buttons...)
    external_b: u8,
}

impl Atmega328pBoard {
    pub fn new(cpu: Cpu) -> Self {
        Self { cpu, external_b: 0 }
    }

    /// Build a board and load an Intel HEX program into its flash.
    pub fn from_hex(hex: &str) -> Result<Self, String> {
        let mut cpu = Cpu::new();
        load_hex(hex, &mut cpu)?;
        Ok(Self::new(cpu))
    }
}

impl Machine for Atmega328pBoard {
    fn clock_hz(&self) -> u64 {
        CLOCK_HZ
    }

    fn cycles(&self) -> u64 {
        self.cpu.cycles
    }

    fn run_until(&mut self, target: u64) -> Result<(), String> {
        while self.cpu.cycles < target {
            self.cpu.step()?;
        }
        Ok(())
    }

    fn pins(&self) -> Vec<PinState> {
        // Same style as your GUI code: adapt to your read_data signature
        let ddrb = self.cpu.read_data(0x04 + IO_START).unwrap_or(0);
        let portb = self.cpu.read_data(0x05 + IO_START).unwrap_or(0);

        (0..8)
            .map(|bit| {
                let ddr = ddrb & (1 << bit) != 0;
                let port = portb & (1 << bit) != 0;
                let external = self.external_b & (1 << bit) != 0;

                let (mode, high) = match (ddr, port) {
                    (true, _) => (PinMode::Output, port),
                    (false, true) => (PinMode::InputPullup, true),
                    (false, false) => (PinMode::Input, external),
                };

                let label = match bit {
                    0..=5 => format!("D{}", bit + 8), // PB0..PB5 = D8..D13
                    6 => "XTAL1".to_string(),
                    _ => "XTAL2".to_string(),
                };

                PinState { name: format!("PB{bit}"), label, mode, high }
            })
            .collect()
    }

    fn set_input(&mut self, pin: &str, high: bool) {
        // Only port B for now: "PB0".."PB7"
        if let Some(bit) = pin.strip_prefix("PB").and_then(|n| n.parse::<u8>().ok()) {
            if bit < 8 {
                if high {
                    self.external_b |= 1 << bit;
                } else {
                    self.external_b &= !(1 << bit);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blink_sets_pb5_as_output() {
        let hex = include_str!("../../examples/code.hex");
        let mut board = Atmega328pBoard::from_hex(hex).unwrap();
        board.run_until(100).unwrap(); // enough to run the `sbi`

        let pins = board.pins();
        let pb5 = pins.iter().find(|p| p.name == "PB5").unwrap();
        assert_eq!(pb5.mode, PinMode::Output);
        assert_eq!(pb5.label, "D13");
    }
}