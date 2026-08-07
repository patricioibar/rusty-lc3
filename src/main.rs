use crate::instructions::Instruction;

mod instructions;
mod instructions_tests;

// Constants
const MEMORY_MAX: usize = 1 << 16;
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

fn main() {
    let mut regs: [u16; N_REGS] = [0; N_REGS];
    let mut mem: [u16; MEMORY_MAX] = [0; MEMORY_MAX];
    /* since exactly one condition flag should be set at any given time, set the Z flag */
    regs[R_COND] = FL_NEG;

    /* set the PC to starting position */
    /* 0x3000 is the default */
    regs[R_PC] = PC_START;

    loop {
        /* FETCH */

        // construct instruction from memory at PC
        let instr = Instruction::from(mem[regs[R_PC] as usize]);
        // then evaluate the instruction, passing in the registers and memory
        instr.eval(&mut regs, &mut mem);
        // this two steps could be combined into one
        // but as this is a didactic project, I prefered to keep them separate for clarity
    }
}
