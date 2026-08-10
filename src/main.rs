use crate::{
    instructions::{Instruction, ProgramState},
    memory::Memory,
};

mod image;
mod instructions;
mod instructions_tests;
mod memory;
mod traps;

// Constants
const N_REGS: usize = 10;
const PC_START: u16 = 0x3000;

// Registers
pub const R_P0: usize = 0;
pub const R_P1: usize = 1;
pub const R_P2: usize = 2;
pub const R_P3: usize = 3;
pub const R_P4: usize = 4;
pub const R_P5: usize = 5;
pub const R_P6: usize = 6;
pub const R_P7: usize = 7;
pub const R_PC: usize = 8;
pub const R_COND: usize = 9;

// Flags
pub const FL_POS: u16 = 1 << 0;
pub const FL_ZRO: u16 = 1 << 1;
pub const FL_NEG: u16 = 1 << 2;

fn main() -> Result<(), i32> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Usage: {} <image-file>", args[0]);
        return Err(1);
    }

    let mut regs: [u16; N_REGS] = [0; N_REGS];
    let mut mem: Memory = Memory::new();

    for arg in &args[1..] {
        if let Err(e) = image::load_image_file(arg, &mut mem) {
            eprintln!("Failed to load image file '{}': {}", arg, e);
            return Err(1);
        }
    }

    /* since exactly one condition flag should be set at any given time, set the Z flag */
    regs[R_COND] = FL_ZRO;
    /* set the PC to starting position */
    /* 0x3000 is the default */
    regs[R_PC] = PC_START;

    // crossterm to poll keyboard events
    crossterm::terminal::enable_raw_mode().map_err(|_| 2)?;
    loop {
        // construct instruction from memory at PC
        let instr = Instruction::from(mem.get(regs[R_PC] as usize));
        regs[R_PC] = regs[R_PC].wrapping_add(1);
        // then evaluate the instruction, passing in the registers and memory
        if instr.eval(&mut regs, &mut mem) == ProgramState::Halted {
            break;
        }
        // this two steps could be combined into one
        // but as this is a didactic project, I prefered to keep them separate for clarity
    }
    crossterm::terminal::disable_raw_mode().map_err(|_| 2)?;

    Ok(())
}
