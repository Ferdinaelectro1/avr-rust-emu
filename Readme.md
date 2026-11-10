# avr-rust-emu

An **ATmega328P** (the microcontroller of the Arduino Uno) emulator written in Rust, instruction by instruction. The goal: load a `.hex` file compiled by `avr-gcc` (for example a Blink) and run it.

The project is written to understand the hardware from the inside: every instruction is decoded from its 16-bit encoding, its flags are computed by hand, and each one comes with its tests.

## Table of contents

- [Features](#features)
- [Getting started](#getting-started)
- [Emulated architecture](#emulated-architecture)
- [Repository layout](#repository-layout)
- [How it works](#how-it-works)
- [Instruction set](#instruction-set)
- [Examples](#examples)
- [Tests](#tests)
- [Project conventions](#project-conventions)
- [Pitfalls met along the way](#pitfalls-met-along-the-way)
- [Roadmap](#roadmap)

## Features

- 8-bit AVR CPU: 32 registers, `PC`, `SP`, `SREG` with the eight flags `I T H S V N Z C`
- Harvard memory: 16,384 16-bit words of flash, 2,048 bytes of SRAM, 224 I/O registers
- Clear `fetch` / `decode` / `execute` cycle, one arm per instruction
- Faithful flag computation (incoming carry, half-carry, overflow, the "sticky" Z of `SBC`)
- Extended instruction set: arithmetic, multiplication, jumps, calls, stack, memory access, bit manipulation
- Intel HEX loader (`hex::load_hex`) to run a program built with `avr-gcc`
- CPU state display (`print_cpu`)
- Example with a graphical interface (`gui`)
- A large unit test suite and runnable examples

## Getting started

### Requirements

- Stable Rust (install via [rustup](https://rustup.rs))
- Optional, to build your own programs: `avr-gcc` and `avr-libc`

### Build and test

```bash
git clone https://github.com/Ferdinaelectro1/avr-rust-emu.git
cd avr-rust-emu
cargo build
cargo test
```

### Run an example

```bash
cargo run --example fibonacci
cargo run --example countdown
```

Note: the two dashes in `--example` are required.

### Load and run a `.hex` file

Build a program for the ATmega328P and convert it to Intel HEX:

```bash
avr-gcc -mmcu=atmega328p -Os -DF_CPU=16000000UL blink.c -o blink.elf
avr-objcopy -O ihex -R .eeprom blink.elf blink.hex
```

Then load it into the CPU with `hex::load_hex`, which returns the number of bytes written to flash:

```rust
mod common;
use std::fs;

use avr_rust_emu::{hex::load_hex, Cpu};

fn main() {
    let hex = fs::read_to_string("examples/code.hex").expect("cannot read code.hex");
    let mut cpu = Cpu::new();
    let total = load_hex(&hex, &mut cpu).expect("hex load failed");
    println!("{total} bytes written");
    println!("Running program ...");
    if let Err(e) = common::run_realtime(&mut cpu) {
        println!("Stopped: {e}");
    }
    cpu.print_cpu();
}
```

<!-- TO COMPLETE: file name of this example in examples/ and the matching cargo run --example command. -->

### Use the CPU as a library

```rust
use avr_rust_emu::Cpu;

fn main() {
    let mut cpu = Cpu::new();
    // LDI R16, 5 / LDI R17, 7 / ADD R16, R17
    cpu.load_program(&[0xE005, 0xE017, 0x0F01]);
    for _ in 0..3 {
        cpu.step().unwrap();
    }
    cpu.print_cpu(); // R16 = 12
}
```

## Emulated architecture

| Element | Value |
|---|---|
| Core | 8-bit AVR, Harvard architecture |
| Program flash | 16,384 16-bit words (32 KB) |
| SRAM | 2,048 bytes |
| General-purpose registers | R0 to R31 |
| I/O registers | 224 bytes |
| `PC` | counts **words**, 14 useful bits (wraps around) |

Data address map (`0x0000` to `0x08FF`):

| Range | Content |
|---|---|
| `0x0000..=0x001F` | registers R0 to R31 |
| `0x0020..=0x00FF` | I/O registers |
| `0x0100..=0x08FF` | SRAM |

`SPL` (`0x5D`), `SPH` (`0x5E`) and `SREG` (`0x5F`) sit in the I/O range: they are redirected to the CPU's `sp` and `sreg` fields, which are their only source of truth.

The `IN` and `OUT` instructions use I/O addresses (0 to 63), offset by `0x20` from data addresses: `SREG` is `0x3F` as an I/O address and `0x5F` as a data address.

### Status register `SREG`

| Bit | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|
| Flag | I | T | H | S | V | N | Z | C |

## Repository layout

```
avr-rust-emu/
├── Cargo.toml
├── src/
│   ├── lib.rs          Cpu, fetch, decode, execute, step, print_cpu
│   ├── main.rs         entry point (.hex loader)
│   ├── constants.rs    sizes, addresses, SREG masks
│   ├── hex.rs          Intel HEX loader (load_hex)
│   └── tests.rs        unit tests (module of lib.rs)
└── examples/
    ├── common/mod.rs   run_program, run_until_end and run_realtime
    ├── fibonacci.rs
    ├── countdown.rs
    ├── gui.rs          example with a graphical interface
    └── code.hex        program built with avr-gcc, loaded by the loader example
```

## How it works

1. **`fetch`** reads the flash word at `pc` and advances `pc` by 1. 32-bit instructions (`JMP`, `CALL`, `LDS`, `STS`) read a second word with a second `fetch`.
2. **`decode(word)`** is a free function: a `match` with mask guards (`w & mask == pattern`) that produces a variant of the `Instruction` enum. 6-bit masks are tested before 4-bit ones, and each mask covers all the fixed bits of the instruction.
3. **`execute(instruction)`** has one arm per variant. An unknown instruction returns an error.
4. **`step`** chains the three.

Flags are factored into dedicated methods by instruction family:

| Method | Instructions | Particularity |
|---|---|---|
| `update_flags_add` / `update_flags_sub` | `ADD`, `ADC`, `SUB`, `SBC`, `CP`, `CPC`, `SUBI`, `CPI`, `SBCI`, `NEG` | incoming carry taken into account |
| `update_flags_incdec` | `INC`, `DEC` | C and H unchanged |
| `update_flags_logic` | `AND`, `OR`, `EOR`, `ANDI`, `ORI`, `COM` | V cleared, so S = N |
| `update_flags_shift` | `LSR`, `ASR`, `ROR` | V = N xor C, so S = C |
| `update_flags_word` | `ADIW`, `SBIW` | 16-bit Z, C and V from bit 15 |
| `update_flags_mul` | `MUL`, `MULS`, `MULSU` | only C (bit 15) and Z change |

## Instruction set

### Arithmetic and logic

| Instruction | 16-bit encoding | Flags |
|---|---|---|
| `ADD Rd, Rr` | `0000 11rd dddd rrrr` | H S V N Z C |
| `ADC Rd, Rr` | `0001 11rd dddd rrrr` | H S V N Z C |
| `SUB Rd, Rr` | `0001 10rd dddd rrrr` | H S V N Z C |
| `SBC Rd, Rr` | `0000 10rd dddd rrrr` | H S V N Z C (sticky Z) |
| `SUBI Rd, K` | `0101 KKKK dddd KKKK` | H S V N Z C |
| `SBCI Rd, K` | `0100 KKKK dddd KKKK` | H S V N Z C (sticky Z) |
| `CP Rd, Rr` | `0001 01rd dddd rrrr` | like `SUB`, no write-back |
| `CPC Rd, Rr` | `0000 01rd dddd rrrr` | like `SBC`, no write-back |
| `CPI Rd, K` | `0011 KKKK dddd KKKK` | like `SUBI`, no write-back |
| `INC Rd` | `1001 010d dddd 0011` | S V N Z |
| `DEC Rd` | `1001 010d dddd 1010` | S V N Z |
| `AND Rd, Rr` | `0010 00rd dddd rrrr` | S V N Z |
| `OR Rd, Rr` | `0010 10rd dddd rrrr` | S V N Z |
| `EOR Rd, Rr` | `0010 01rd dddd rrrr` | S V N Z |
| `ANDI Rd, K` | `0111 KKKK dddd KKKK` | S V N Z |
| `ORI Rd, K` | `0110 KKKK dddd KKKK` | S V N Z |
| `COM Rd` | `1001 010d dddd 0000` | S V N Z C (C = 1) |
| `NEG Rd` | `1001 010d dddd 0001` | H S V N Z C |
| `SWAP Rd` | `1001 010d dddd 0010` | none |
| `LSR Rd` | `1001 010d dddd 0110` | S V N Z C |
| `ASR Rd` | `1001 010d dddd 0101` | S V N Z C |
| `ROR Rd` | `1001 010d dddd 0111` | S V N Z C |
| `ADIW Rd+1:Rd, K` | `1001 0110 KKdd KKKK` | S V N Z C |
| `SBIW Rd+1:Rd, K` | `1001 0111 KKdd KKKK` | S V N Z C |
| `MUL Rd, Rr` | `1001 11rd dddd rrrr` | Z C |
| `MULS Rd, Rr` | `0000 0010 dddd rrrr` | Z C |
| `MULSU Rd, Rr` | `0000 0011 0ddd 0rrr` | Z C |
| `FMUL Rd, Rr` | `0000 0011 0ddd 1rrr` | Z C |
| `FMULS Rd, Rr` | `0000 0011 1ddd 0rrr` | Z C |
| `FMULSU Rd, Rr` | `0000 0011 1ddd 1rrr` | Z C |

Aliases recognised without a dedicated variant: `CLR Rd` (`EOR Rd, Rd`), `TST Rd` (`AND Rd, Rd`), `SBR` (`ORI`), `CBR` (`ANDI` with `~K`).

### Data transfer and memory

| Instruction | 16-bit encoding | Flags |
|---|---|---|
| `LDI Rd, K` | `1110 KKKK dddd KKKK` | none |
| `MOV Rd, Rr` | `0010 11rd dddd rrrr` | none |
| `MOVW Rd+1:Rd, Rr+1:Rr` | `0000 0001 dddd rrrr` | none |
| `IN Rd, A` | `1011 0AAd dddd AAAA` | none |
| `OUT A, Rr` | `1011 1AAr rrrr AAAA` | none (except when `A` is `SREG`) |
| `PUSH Rr` | `1001 001r rrrr 1111` | none |
| `POP Rd` | `1001 000d dddd 1111` | none |
| `LDS Rd, k` | `1001 000d dddd 0000` + 16-bit address | none |
| `STS k, Rr` | `1001 001r rrrr 0000` + 16-bit address | none |
| `LD Rd, X/Y/Z` (and `+`, `-`) | `1001 000d dddd xxxx` | none |
| `ST X/Y/Z, Rr` (and `+`, `-`) | `1001 001r rrrr xxxx` | none |
| `LDD Rd, Y/Z+q` | `10q0 qq0d dddd yqqq` | none |
| `STD Y/Z+q, Rr` | `10q0 qq1r rrrr yqqq` | none |
| `LPM` | `1001 0101 1100 1000` | none |
| `LPM Rd, Z` | `1001 000d dddd 0100` | none |
| `LPM Rd, Z+` | `1001 000d dddd 0101` | none |

For `LD`/`ST`, the low four bits (`xxxx`) select the pointer (X, Y or Z) and the mode: unchanged, post-increment (`+`) or pre-decrement (`-`).

### Jumps, calls and branches

| Instruction | 16-bit encoding | Note |
|---|---|---|
| `RJMP k` | `1100 kkkk kkkk kkkk` | `k` is 12-bit signed |
| `RCALL k` | `1101 kkkk kkkk kkkk` | pushes the return address (2 bytes) |
| `JMP k` | `1001 010k kkkk 110k` + 16 bits | absolute jump, 2 words |
| `CALL k` | `1001 010k kkkk 111k` + 16 bits | absolute call, 2 words |
| `IJMP` | `1001 0100 0000 1001` | jumps to the address held in Z |
| `ICALL` | `1001 0101 0000 1001` | calls the address held in Z |
| `RET` | `1001 0101 0000 1000` | pops the return address |
| `RETI` | `1001 0101 0001 1000` | return from interrupt |
| `BRBS s, k` | `1111 00kk kkkk ksss` | branches if flag `s` is 1 |
| `BRBC s, k` | `1111 01kk kkkk ksss` | branches if flag `s` is 0 |
| `CPSE Rd, Rr` | `0001 00rd dddd rrrr` | skips the next instruction if `Rd == Rr` |
| `SBRC Rr, b` | `1111 110r rrrr 0bbb` | skips if bit `b` of `Rr` is 0 |
| `SBRS Rr, b` | `1111 111r rrrr 0bbb` | skips if bit `b` of `Rr` is 1 |
| `SBIC A, b` | `1001 1001 AAAA Abbb` | skips if bit `b` of I/O register `A` is 0 |
| `SBIS A, b` | `1001 1011 AAAA Abbb` | skips if bit `b` of I/O register `A` is 1 |

`BRBS` and `BRBC` cover all the aliases `BREQ`, `BRNE`, `BRCS`, `BRCC`, `BRMI`, `BRPL`, `BRLT`, `BRGE`, `BRIE`, etc., depending on the flag number `s`. The conditional skip instructions (`CPSE`, `SBRC`, `SBRS`, `SBIC`, `SBIS`) skip an instruction of 1 or 2 words.

### Bits and flags

| Instruction | 16-bit encoding | Effect |
|---|---|---|
| `BSET s` | `1001 0100 0sss 1000` | sets flag `s` of `SREG` to 1 |
| `BCLR s` | `1001 0100 1sss 1000` | clears flag `s` of `SREG` to 0 |
| `BST Rd, b` | `1111 101d dddd 0bbb` | copies bit `b` of `Rd` into T |
| `BLD Rd, b` | `1111 100d dddd 0bbb` | copies T into bit `b` of `Rd` |
| `SBI A, b` | `1001 1010 AAAA Abbb` | sets bit `b` of I/O register `A` |
| `CBI A, b` | `1001 1000 AAAA Abbb` | clears bit `b` of I/O register `A` |

`BSET` and `BCLR` cover the aliases `SEC`, `CLC`, `SEZ`, `CLZ`, `SEN`, `CLN`, `SEV`, `CLV`, `SES`, `CLS`, `SEH`, `CLH`, `SET`, `CLT`, `SEI` and `CLI`.

### System

| Instruction | 16-bit encoding |
|---|---|
| `NOP` | `0000 0000 0000 0000` |
| `SLEEP` | `1001 0101 1000 1000` |
| `WDR` | `1001 0101 1010 1000` |
| `BREAK` | `1001 0101 1001 1000` |

Any word that matches no instruction is decoded as `Unknown(word)`, and `execute` then returns an error.

## Examples

### `fibonacci`

Computes a Fibonacci sequence in the registers with `LDI`, `ADD` and `MOV`, then prints the CPU state.

### `countdown`

Countdown loop, program `[0xE003, 0xE011, 0x1B01, 0xF7F1]`:

```asm
    LDI  R16, 3
    LDI  R17, 1
loop:
    SUB  R16, R17
    BRNE loop
```

Ends after 8 steps with `R16 = 0`, `SREG = 0x02` (Z flag) and `PC = 4`.

### `gui`

Example with a graphical interface.

```bash
cargo run --example gui
```

### `.hex` loader

Loads `examples/code.hex` into flash with `hex::load_hex`, then runs the program continuously with `common::run_realtime` until an error (see [Load and run a `.hex` file](#load-and-run-a-hex-file)).

## Tests

```bash
cargo test
```

Tests live in `src/tests.rs` and can see the CPU's private fields. For each instruction you will find:

- **decoding** tests (R0 and R31 bounds, constants with different nibbles);
- **neighbour** tests: nearby words must not be mistaken for the instruction (holes such as `0x9404` stay `Unknown`);
- **flag** tests on the classic cases (`255 + 1`, `127 + 1`, `128 - 1`...);
- **preservation** tests: `sreg = 0xFF` before the instruction, to check that only the right flags change;
- **integration** tests (16-bit chains `SUBI`/`SBCI`, `CP`/`CPC` comparisons, delay loops with `SBIW`, `MUL` followed by `clr r1`).

Assertions carry messages (`"MUL 255 x 255"`) so the failing case is immediately visible.

## Project conventions

- One commit per step, in Conventional Commits: `feat(cpu): ...`, `refactor(cpu): ...`, `test(cpu): ...`, `docs: ...`
- Every instruction ships with its tests
- Intermediate computations in `u16` to avoid overflow panics in debug builds

Example:

```
feat(cpu): add ADIW and SBIW instructions

* Add ADIW and SBIW (R25:R24, X, Y and Z pairs, K: 0 to 63)
* Add update_flags_word (16-bit Z, N ; C and V from bit 15 before and after ; S = N xor V ; I, T, H kept)
* Add decode, 16-bit flag, register pair, delay loop and preservation tests
```

## Pitfalls met along the way

A few classic mistakes made while writing the emulator, useful to anyone writing one:

- `+=` and `-=` on `u8` panic in debug builds: compute in `u16`.
- `A..B` excludes `B`: use `A..=B` for memory zone bounds.
- `|` binds tighter than `&&`, and `*` tighter than `&`.
- Read the carry (`C`) **before** calling the flag methods, which clear it.
- The `FLAG_*` masks apply to `sreg`, never to a register's value.
- A `u8` cast straight to `i16` stays positive: go through `i8` for the signed value.
- Widen to `u16` **before** shifting (`(hi as u16) << 8`).
- A decode mask must cover all the fixed bits, including those at the bottom of the word.

## Roadmap

- [x] Arithmetic, logic, shift and multiply instructions
- [x] Relative jumps and conditional branches
- [x] Memory and stack instructions
- [x] Intel HEX file loader
- [x] Running a Blink built with `avr-gcc`
- [ ] Peripherals: GPIO, timers, UART
- [ ] Interrupts

## License
