use crate::{FL_NEG, FL_POS, FL_ZRO, MEMORY_MAX, N_REGS, R_COND, R_PC};

pub enum Instruction {
    BR { offset: u16, n: u16, z: u16, p: u16 },   // branch
    ADDReg { dr: usize, sr1: usize, sr2: usize }, // add register
    ADDImm { dr: usize, sr1: usize, imm: u16 },   // add immediate
    LD,                                           // load
    ST,                                           // store
    JSR,                                          // jump to subroutine pc offset
    JSRr,                                         // jump to subroutine register
    ANDReg { dr: usize, sr1: usize, sr2: usize }, // and register
    ANDImm { dr: usize, sr1: usize, imm: u16 },   // and immediate
    LDR,                                          // load register
    STR,                                          // store register
    RTI,                                          // unused
    NOT,                                          // bitwise not
    LDI,                                          // load indirect
    STI,                                          // store indirect
    JMP,                                          // jump
    RET,                                          // return
    RES,                                          // reserved (unused)
    LEA,                                          // load effective address
    TRAP,                                         // execute trap
}

impl Instruction {
    pub fn from(value: u16) -> Self {
        let opcode = (value >> 12) & 0b1111;
        let body = value & 0x0FFF;
        match opcode {
            0 => Self::build_br(body),
            1 => Self::build_add(body),
            2 => Self::build_ld(body),
            3 => Self::build_st(body),
            4 => Self::build_jsr(body),
            5 => Self::build_and(body),
            6 => Self::build_ldr(body),
            7 => Self::build_str(body),
            8 => Self::build_rti(body),
            9 => Self::build_not(body),
            10 => Self::build_ldi(body),
            11 => Self::build_sti(body),
            12 => Self::build_jmp(body),
            13 => Self::build_res(body), // unused
            14 => Self::build_lea(body),
            15 => Self::build_trap(body),
            _ => panic!("Invalid opcode: {}", opcode),
        }
    }

    pub fn eval(self, regs: &mut [u16; N_REGS], mem: &mut [u16; MEMORY_MAX]) {
        match self {
            Instruction::BR { offset, n, z, p } => {
                if (n & regs[R_COND] == FL_NEG)
                    || (z & regs[R_COND] == FL_ZRO)
                    || (p & regs[R_COND] == FL_POS)
                {
                    regs[R_PC] = regs[R_PC].wrapping_add(offset);
                }
            }
            Instruction::ADDReg { dr, sr1, sr2 } => {
                regs[dr] = regs[sr1].wrapping_add(regs[sr2]);
                Self::update_flags(regs, dr);
            }
            Instruction::ADDImm { dr, sr1, imm } => {
                regs[dr] = regs[sr1].wrapping_add(imm as u16);
                Self::update_flags(regs, dr);
            }
            Instruction::LD => todo!(),
            Instruction::ST => todo!(),
            Instruction::JSR => todo!(),
            Instruction::JSRr => todo!(),
            Instruction::ANDReg { dr, sr1, sr2 } => {
                regs[dr] = regs[sr1] & regs[sr2];
                Self::update_flags(regs, dr);
            }
            Instruction::ANDImm { dr, sr1, imm } => {
                regs[dr] = regs[sr1] & imm;
                Self::update_flags(regs, dr);
            }
            Instruction::LDR => todo!(),
            Instruction::STR => todo!(),
            Instruction::RTI => todo!(),
            Instruction::NOT => todo!(),
            Instruction::LDI => todo!(),
            Instruction::STI => todo!(),
            Instruction::JMP => todo!(),
            Instruction::RET => return, // unused, noop
            Instruction::RES => todo!(),
            Instruction::LEA => todo!(),
            Instruction::TRAP => todo!(),
        }
    }

    fn build_br(body: u16) -> Instruction {
        let offset = Self::sign_extend(body & 0b0000_0001_1111_1111, 9);
        let n = Self::bool_extend((body & 0b0000_1000_0000_0000) != 0);
        let z = Self::bool_extend((body & 0b0000_0100_0000_0000) != 0);
        let p = Self::bool_extend((body & 0b0000_0010_0000_0000) != 0);
        Self::BR { offset, n, z, p }
    }

    fn build_add(body: u16) -> Instruction {
        let dr = ((body & 0b0000_1110_0000_0000) >> 9) as usize;
        let sr1 = ((body & 0b0000_0001_1100_0000) >> 6) as usize;
        let is_imm = (body & 0b0000_0000_0010_0000) != 0;
        if is_imm {
            // extend sign bit for immediate value
            let imm = Self::sign_extend(body & 0b0000_0000_0001_1111, 5);
            Self::ADDImm { dr, sr1, imm }
        } else {
            let sr2 = (body & 0b0000_0000_0000_0111) as usize;
            Self::ADDReg { dr, sr1, sr2 }
        }
    }

    fn build_ld(body: u16) -> Instruction {
        todo!()
    }

    fn build_st(body: u16) -> Instruction {
        todo!()
    }

    fn build_jsr(body: u16) -> Instruction {
        todo!()
    }

    fn build_and(body: u16) -> Instruction {
        let dr = ((body & 0b0000_1110_0000_0000) >> 9) as usize;
        let sr1 = ((body & 0b0000_0001_1100_0000) >> 6) as usize;
        let is_imm = (body & 0b0000_0000_0010_0000) != 0;
        if is_imm {
            // extend sign bit for immediate value
            let imm = Self::sign_extend(body & 0b0000_0000_0001_1111, 5);
            Self::ANDImm { dr, sr1, imm }
        } else {
            let sr2 = (body & 0b0000_0000_0000_0111) as usize;
            Self::ANDReg { dr, sr1, sr2 }
        }
    }

    fn build_ldr(body: u16) -> Instruction {
        todo!()
    }

    fn build_str(body: u16) -> Instruction {
        todo!()
    }

    fn build_rti(body: u16) -> Instruction {
        todo!()
    }

    fn build_not(body: u16) -> Instruction {
        todo!()
    }

    fn build_ldi(body: u16) -> Instruction {
        todo!()
    }

    fn build_sti(body: u16) -> Instruction {
        todo!()
    }

    fn build_jmp(body: u16) -> Instruction {
        todo!()
    }

    fn build_res(body: u16) -> Instruction {
        Self::RES
    }

    fn build_lea(body: u16) -> Instruction {
        todo!()
    }

    fn build_trap(body: u16) -> Instruction {
        todo!()
    }

    fn update_flags(regs: &mut [u16; N_REGS], dr: usize) {
        if regs[dr] == 0 {
            regs[R_COND] = FL_ZRO;
        } else if (regs[dr] & 0b1000_0000_0000_0000) == 1 {
            regs[R_COND] = FL_NEG;
        } else {
            regs[R_COND] = FL_POS;
        }
    }

    fn sign_extend(value: u16, bit_count: u16) -> u16 {
        if (value >> (bit_count - 1)) & 1 == 1 {
            value | (0xFFFF << bit_count)
        } else {
            value
        }
    }

    fn bool_extend(value: bool) -> u16 {
        if value { 0xFFFF } else { 0x0000 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_branch() {
        // opcode: 0000, n: 1, z: 0, p: 1, offset: 000000001
        let op_body = 0b0000_101_000000001;
        let instruction = Instruction::from(op_body);
        match instruction {
            Instruction::BR { offset, n, z, p } => {
                assert_eq!(offset, 1);
                assert_eq!(n, 0xFFFF);
                assert_eq!(z, 0x0000);
                assert_eq!(p, 0xFFFF);
            }
            _ => panic!("Expected BR instruction"),
        }
        let mut regs = [0u16; N_REGS];
        let mut mem = [0u16; MEMORY_MAX];
        regs[R_PC] = 100;
        regs[R_COND] = FL_NEG; // Set condition to negative
        instruction.eval(&mut regs, &mut mem);
        assert_eq!(regs[R_PC], 101); // PC should increment by offset
    }

    #[test]
    fn test_add_register() {
        // opcode: 0001, dr: 001, sr1: 010, empty: 000 sr2: 011
        let op_body = 0b0001_001_010_000_011;
        let instruction = Instruction::from(op_body);
        match instruction {
            Instruction::ADDReg { dr, sr1, sr2 } => {
                assert_eq!(dr, 1);
                assert_eq!(sr1, 2);
                assert_eq!(sr2, 3);
            }
            _ => panic!("Expected ADDReg instruction"),
        }
        let mut regs = [0u16; N_REGS];
        let mut mem = [0u16; MEMORY_MAX];
        regs[2] = 5;
        regs[3] = 10;
        instruction.eval(&mut regs, &mut mem);
        assert_eq!(regs[1], 15);
        assert!(regs[R_COND] & FL_POS != 0);
    }

    #[test]
    fn test_add_immediate() {
        // opcode: 0001, dr: 001, sr1: 010, immediate flag: 1, imm: 11011
        let op_body = 0b0001_001_010_1_11011;
        let instruction = Instruction::from(op_body);
        match instruction {
            Instruction::ADDImm { dr, sr1, imm } => {
                assert_eq!(dr, 1);
                assert_eq!(sr1, 2);
                assert_eq!(imm, (-5 as i16) as u16); // 2's complement of 5 is 0xFFFB
            }
            _ => panic!("Expected ADDImm instruction"),
        }
        let mut regs = [0u16; N_REGS];
        let mut mem = [0u16; MEMORY_MAX];
        regs[2] = 5;
        instruction.eval(&mut regs, &mut mem);
        assert_eq!(regs[1], 0);
        assert!(regs[R_COND] & FL_ZRO != 0);
    }

    #[test]
    fn test_and_register() {
        // opcode: 0101, dr: 001, sr1: 010, empty: 000 sr2: 011
        let op_body = 0b0101_001_010_000_011;
        let instruction = Instruction::from(op_body);
        match instruction {
            Instruction::ANDReg { dr, sr1, sr2 } => {
                assert_eq!(dr, 1);
                assert_eq!(sr1, 2);
                assert_eq!(sr2, 3);
            }
            _ => panic!("Expected ANDReg instruction"),
        }
        let mut regs = [0u16; N_REGS];
        let mut mem = [0u16; MEMORY_MAX];
        regs[2] = 0xF0F0;
        regs[3] = 0x0F1F;
        instruction.eval(&mut regs, &mut mem);
        assert_eq!(regs[1], 0x0010);
        assert!(regs[R_COND] & FL_POS != 0);
    }

    #[test]
    fn test_and_immediate() {
        // opcode: 0101, dr: 001, sr1: 010, immediate flag: 1, imm: 01111
        let op_body = 0b0101_001_010_1_01111;
        let instruction = Instruction::from(op_body);
        match instruction {
            Instruction::ANDImm { dr, sr1, imm } => {
                assert_eq!(dr, 1);
                assert_eq!(sr1, 2);
                assert_eq!(imm, 0x000F);
            }
            _ => panic!("Expected ANDImm instruction"),
        }
        let mut regs = [0u16; N_REGS];
        let mut mem = [0u16; MEMORY_MAX];
        regs[2] = 0xF0F0;
        instruction.eval(&mut regs, &mut mem);
        assert_eq!(regs[1], 0xF0F0 & 0x000F);
        assert!(regs[R_COND] & FL_ZRO != 0);
    }
}
