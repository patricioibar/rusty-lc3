use std::io::{Read, Write, stdout};

use crate::{MEMORY_MAX, N_REGS, R_P0};

pub enum Trap {
    GETC,  // get character from keyboard, not echoed onto the terminal
    OUT,   // output a character
    PUTS,  // output a word string
    IN,    // get character from keyboard, echoed onto the terminal
    PUTSP, // output a byte string
    HALT,  // halt the program
}

impl Trap {
    pub fn from(trapvect: u8) -> Self {
        match trapvect {
            0x20 => Self::GETC,
            0x21 => Self::OUT,
            0x22 => Self::PUTS,
            0x23 => Self::IN,
            0x24 => Self::PUTSP,
            0x25 => Self::HALT,
            _ => panic!("Invalid trapvect: {}", trapvect),
        }
    }

    pub fn exec(self, regs: &mut [u16; N_REGS], mem: &mut [u16; MEMORY_MAX]) {
        match self {
            Trap::GETC => {
                let mut buf = [0u8];
                let _ = std::io::stdin().read_exact(&mut buf);
                regs[R_P0] = buf[0] as u16;
            }
            Trap::OUT => {
                let _ = stdout().write(&[regs[R_P0] as u8]);
                let _ = stdout().flush();
            }
            Trap::PUTS => {
                let mut i = regs[R_P0] as usize;
                while let c = mem[i]
                    && c != 0x00
                {
                    let _ = stdout().write(&[c as u8]);
                    i = i + 1;
                }
                let _ = stdout().flush();
            }
            Trap::IN => todo!(),
            Trap::PUTSP => todo!(),
            Trap::HALT => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{MEMORY_MAX, N_REGS, instructions::Instruction, traps::Trap};
    use gag::BufferRedirect;
    use std::io::Read;

    #[test]
    fn test_puts() {
        // opcode: 1111, empty: 000, trapvect: 00100010
        let op_body = 0b1111_0000_00100010;
        let instruction = Instruction::from(op_body);
        assert!(matches!(
            &instruction,
            Instruction::TRAP { trap: Trap::PUTS }
        ));

        // intercept stdout
        let mut buf = BufferRedirect::stdout().unwrap();
        let mut regs = [0u16; N_REGS];
        let mut mem = [0u16; MEMORY_MAX];
        regs[0] = 100;
        for (i, c) in "hola".bytes().enumerate() {
            mem[100 + i] = c as u16;
        }
        instruction.eval(&mut regs, &mut mem);

        let mut output = String::new();
        buf.read_to_string(&mut output).unwrap();
        assert_eq!(&output, "hola");
    }

    #[test]
    fn test_out() {
        // opcode: 1111, empty: 000, trapvect: 00100001
        let op_body = 0b1111_0000_00100001;
        let instruction = Instruction::from(op_body);
        assert!(matches!(
            &instruction,
            Instruction::TRAP { trap: Trap::OUT }
        ));

        // intercept stdout
        let mut buf = BufferRedirect::stdout().unwrap();
        let mut regs = [0u16; N_REGS];
        let mut mem = [0u16; MEMORY_MAX];
        regs[0] = 'X' as u16;
        instruction.eval(&mut regs, &mut mem);

        let mut output = String::new();
        buf.read_to_string(&mut output).unwrap();
        assert_eq!(&output, "X");
    }
}
