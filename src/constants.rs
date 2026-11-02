//constants.rs
// Memory sizes (flash in 16-bit words, others in bytes).
pub const FLASH_SIZE: usize = 16_384;
pub const SRAM_SIZE: usize = 2_048;
pub const CPU_REGISTERS_SIZE: usize = 32;
pub const IO_SIZE: usize = 224;

// SREG flag masks (bit order is fixed by the hardware)
pub const FLAG_CARRY: u8 = 0b0000_0001; // Bit 0 (C): Carry on ADD, borrow on SUB
pub const FLAG_ZERO: u8 = 0b0000_0010; // Bit 1 (Z): Set if the result is zero
pub const FLAG_NEGATIVE: u8 = 0b0000_0100; // Bit 2 (N): Copy of bit 7 of the result
pub const FLAG_OVERFLOW: u8 = 0b0000_1000; // Bit 3 (V): Two's complement (signed) overflow
pub const FLAG_SIGN: u8 = 0b0001_0000; // Bit 4 (S): True sign of the result, S = N xor V
pub const FLAG_HALF_CARRY: u8 = 0b0010_0000; // Bit 5 (H): Carry from bit 3 to bit 4 (BCD)
pub const FLAG_BIT_COPY: u8 = 0b0100_0000; // Bit 6 (T): Bit storage used by BST / BLD
pub const FLAG_INTERRUPT_ENABLE: u8 = 0b1000_0000; // Bit 7 (I): Global interrupt enable

// Data address space layout (0x0000 - 0x08FF)
pub const REGISTERS_START: u16 = 0x0000; // R0..R31 (0x0000 - 0x001F)
pub const REGISTERS_END: u16 = 0x001F;
pub const IO_START: u16 = 0x0020; // I/O registers (0x0020 - 0x00FF)
pub const IO_END: u16 = 0x00FF;
pub const SRAM_START: u16 = 0x0100; // Internal SRAM (0x0100 - 0x08FF)
pub const RAMEND: u16 = SRAM_START + SRAM_SIZE as u16 - 1; // Last SRAM address = 0x08FF

// Special registers (data addresses, as seen by LD/ST/LDS/STS)
pub const SPL_ADDR: u16 = 0x005D; // Stack pointer, low byte
pub const SPH_ADDR: u16 = 0x005E; // Stack pointer, high byte
pub const SREG_ADDR: u16 = 0x005F; // Status register
