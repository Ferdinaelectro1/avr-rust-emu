//! Hardware-agnostic view of a board: pins and time.
//! The GUI only knows about this module, never about registers.

mod atmega328p;
pub use atmega328p::Atmega328pBoard;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PinMode {
    Input,
    InputPullup,
    Output,
}

#[derive(Clone, Debug)]
pub struct PinState {
    pub name: String,  // "PB5"
    pub label: String, // "D13" (Arduino name, for display)
    pub mode: PinMode,
    pub high: bool,    // current logic level
}

pub trait Machine {
    fn clock_hz(&self) -> u64;
    fn cycles(&self) -> u64;
    /// Run until at least `target` cycles have elapsed.
    fn run_until(&mut self, target: u64) -> Result<(), String>;
    /// Snapshot of every pin, for the UI.
    fn pins(&self) -> Vec<PinState>;
    /// Drive an input pin from outside (a button, for example).
    fn set_input(&mut self, pin: &str, high: bool);
}