pub enum Instruction {
    BR,     // branch
    ADDReg, // add register
    ADDImm, // add immediate
    LD,     // load
    ST,     // store
    JSR,    // jump to subroutine pc offset
    JSRr,   // jump to subroutine register
    ANDReg, // and register
    ANDImm, // and immediate
    LDR,    // load register
    STR,    // store register
    RTI,    // unused
    NOT,    // bitwise not
    LDI,    // load indirect
    STI,    // store indirect
    JMP,    // jump
    RET,    // return
    RES,    // reserved (unused)
    LEA,    // load effective address
    TRAP,   // execute trap
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

    pub fn eval(&self, regs: &mut [u16; 10], mem: &mut [u16; 65536]) {
        match &self {
            Instruction::BR => todo!(),
            Instruction::ADDReg => todo!(),
            Instruction::ADDImm => todo!(),
            Instruction::LD => todo!(),
            Instruction::ST => todo!(),
            Instruction::JSR => todo!(),
            Instruction::JSRr => todo!(),
            Instruction::ANDReg => todo!(),
            Instruction::ANDImm => todo!(),
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
        todo!()
    }

    fn build_add(body: u16) -> Instruction {
        todo!()
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
        todo!()
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
}
