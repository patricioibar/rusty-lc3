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
        let dr = (body & 0b0000_1110_0000_0000 >> 9) as usize;
        let sr1 = (body & 0b0000_0001_1100_0000 >> 6) as usize;
        let is_imm = body & 0b0000_0000_0010_0000 != 0;
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
        let dr = (body & 0b0000_1110_0000_0000 >> 9) as usize;
        let sr1 = (body & 0b0000_0001_1100_0000 >> 6) as usize;
        let is_imm = body & 0b0000_0000_0010_0000 != 0;
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
