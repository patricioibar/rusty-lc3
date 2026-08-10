use std::io::{Read, Write, stdin, stdout};

use crate::{N_REGS, R_P0, instructions::ProgramState, memory::Memory};

#[derive(Debug, PartialEq, Eq, Clone)]
#[allow(clippy::upper_case_acronyms)]
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
            _ => panic!("Invalid trapvect: 0x{:04X}", trapvect),
        }
    }

    pub fn exec(self, regs: &mut [u16; N_REGS], mem: &mut Memory) -> ProgramState {
        match self {
            Trap::GETC => {
                let mut buf = [0u8];
                let _ = stdin().read_exact(&mut buf);
                regs[R_P0] = buf[0] as u16;
            }
            Trap::OUT => {
                let ch = regs[R_P0] as u8;
                write_char(ch);
                let _ = stdout().flush();
            }
            Trap::PUTS => {
                let mut i = regs[R_P0] as usize;
                loop {
                    let c = mem.get(i);
                    if c == 0x00 {
                        break;
                    }
                    // just print low bytes
                    let ch = c as u8;
                    write_char(ch);
                    i += 1;
                }
                let _ = stdout().flush();
            }
            Trap::IN => {
                let mut buf = [0u8];
                let _ = write!(&mut stdout(), "Enter a character: ");
                let _ = stdin().read_exact(&mut buf);
                regs[R_P0] = buf[0] as u16;
                let _ = stdout().write(&buf);
                let _ = stdout().flush();
            }
            Trap::PUTSP => {
                let mut i = regs[R_P0] as usize;
                loop {
                    let c = mem.get(i);
                    if c == 0x0000 {
                        break;
                    }
                    let lo = (c & 0x00FF) as u8;
                    write_char(lo);
                    let hi = ((c >> 8) & 0x00FF) as u8;
                    write_char(hi);
                    i += 1;
                }
                let _ = stdout().flush();
            }
            Trap::HALT => {
                let _ = write!(stdout(), "HALT\r\n");
                let _ = stdout().flush();
                return ProgramState::Halted;
            }
        }
        ProgramState::Running
    }
}

fn write_char(ch: u8) {
    if ch == b'\n' {
        let _ = write!(stdout(), "\r\n");
    } else {
        let _ = write!(stdout(), "{}", ch as char);
    }
}

#[cfg(test)]
mod tests {
    use crate::{N_REGS, instructions::Instruction, memory::Memory, traps::Trap};
    use gag::BufferRedirect;
    use std::io::Read;

    #[test]
    #[ignore = "not passing concurrently with other tests"]
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
        let mut mem = Memory::new();
        regs[0] = 100;
        for (i, c) in "hola".bytes().enumerate() {
            mem.set(100 + i, c as u16);
        }
        instruction.eval(&mut regs, &mut mem);

        let mut output = String::new();
        buf.read_to_string(&mut output).unwrap();
        assert_eq!(&output, "hola");
    }

    #[test]
    #[ignore = "not passing concurrently with other tests"]
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
        let mut mem = Memory::new();
        regs[0] = 'X' as u16;
        instruction.eval(&mut regs, &mut mem);

        let mut output = String::new();
        buf.read_to_string(&mut output).unwrap();
        assert_eq!(&output, "X");
    }

    #[test]
    #[ignore = "not passing concurrently with other tests"]
    fn test_putsp() {
        // opcode: 1111, empty: 000, trapvect: 00100100
        let op_body = 0b1111_0000_00100100;
        let instruction = Instruction::from(op_body);
        assert!(matches!(
            &instruction,
            Instruction::TRAP { trap: Trap::PUTSP }
        ));

        // intercept stdout
        let mut buf = BufferRedirect::stdout().unwrap();
        let mut regs = [0u16; N_REGS];
        let mut mem = Memory::new();
        regs[0] = 100;
        let mut iter = "holaa".bytes();
        let mut i = 0;
        while let Some(c) = iter.next() {
            let mut value = c as u16;
            if let Some(c) = iter.next() {
                value |= (c as u16) << 8;
            }
            mem.set(100 + i, value);
            i = i + 1
        }
        instruction.eval(&mut regs, &mut mem);

        let mut output = String::new();
        buf.read_to_string(&mut output).unwrap();
        assert_eq!(&output, "holaa");
    }
}
