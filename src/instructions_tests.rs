#[cfg(test)]
mod tests {
    use crate::{
        FL_NEG, FL_POS, FL_ZRO, MEMORY_MAX, N_REGS, R_COND, R_P7, R_PC, instructions::Instruction,
    };

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

    #[test]
    fn test_jmp() {
        // opcode: 1100, base: 010
        let op_body = 0b1100_000_110_000000;
        let instruction = Instruction::from(op_body);
        match instruction {
            Instruction::JMP { base } => {
                assert_eq!(base, 6);
            }
            _ => panic!("Expected JMP instruction"),
        }
        let mut regs = [0u16; N_REGS];
        let mut mem = [0u16; MEMORY_MAX];
        regs[6] = 1234;
        instruction.eval(&mut regs, &mut mem);
        assert_eq!(regs[R_PC], 1234);
    }

    #[test]
    fn test_jump_subroutine() {
        // opcode: 0100, offset flag: 1, offset: 00000001011
        let op_body = 0b0100_1_00000001011;
        let instruction = Instruction::from(op_body);
        match instruction {
            Instruction::JSR { offset } => {
                assert_eq!(offset, 11);
            }
            _ => panic!("Expected JSR instruction"),
        }
        let mut regs = [0u16; N_REGS];
        let mut mem = [0u16; MEMORY_MAX];
        regs[R_PC] = 1200;
        instruction.eval(&mut regs, &mut mem);
        assert_eq!(regs[R_PC], 1211);
        assert_eq!(regs[R_P7], 1200);
    }

    #[test]
    fn test_jump_subroutine_register() {
        // opcode: 0100, offset flag: 0, empty: 00, register: 011, empty: 000000
        let op_body = 0b0100_0_00_011_000000;
        let instruction = Instruction::from(op_body);
        match instruction {
            Instruction::JSRr { base } => {
                assert_eq!(base, 3);
            }
            _ => panic!("Expected JSRr instruction"),
        }
        let mut regs = [0u16; N_REGS];
        let mut mem = [0u16; MEMORY_MAX];
        regs[R_PC] = 8900;
        regs[3] = 67;
        instruction.eval(&mut regs, &mut mem);
        assert_eq!(regs[R_PC], 67);
        assert_eq!(regs[R_P7], 8900);
    }

    #[test]
    fn test_load() {
        // opcode: 0010, dr: 100, offset: 001001000
        let op_body = 0b0010_100_001001000;
        let instruction = Instruction::from(op_body);
        match instruction {
            Instruction::LD { dr, offset } => {
                assert_eq!(dr, 4);
                assert_eq!(offset, 72);
            }
            _ => panic!("Expected LD instruction"),
        }
        let mut regs = [0u16; N_REGS];
        let mut mem = [0u16; MEMORY_MAX];
        regs[R_PC] = 3000;
        regs[4] = 123;
        mem[3072] = 1100;
        instruction.eval(&mut regs, &mut mem);
        assert_eq!(regs[4], 1100);
        assert!(regs[R_COND] & FL_POS != 0);
    }

    #[test]
    fn test_load_indirect() {
        // opcode: 1010, dr: 010, offset: 001001011
        let op_body = 0b1010_010_001001011;
        let instruction = Instruction::from(op_body);
        match instruction {
            Instruction::LDI { dr, offset } => {
                assert_eq!(dr, 2);
                assert_eq!(offset, 75);
            }
            _ => panic!("Expected LDI instruction"),
        }
        let mut regs = [0u16; N_REGS];
        let mut mem = [0u16; MEMORY_MAX];
        regs[R_PC] = 100;
        regs[2] = 0xFFFF;
        mem[175] = 9999;
        mem[9999] = 0xF0CA;
        instruction.eval(&mut regs, &mut mem);
        assert_eq!(regs[2], 0xF0CA);
        assert_eq!(regs[R_PC], 100);
        assert!(regs[R_COND] == FL_NEG);
    }

    #[test]
    fn test_load_register() {
        // opcode: 0110, dr: 000, base: 001, offset: 011011
        let op_body = 0b0110_000_001_011011;
        let instruction = Instruction::from(op_body);
        match instruction {
            Instruction::LDR { dr, base, offset } => {
                assert_eq!(dr, 0);
                assert_eq!(base, 1);
                assert_eq!(offset, 27);
            }
            _ => panic!("Expected LDR instruction"),
        }
        let mut regs = [0u16; N_REGS];
        let mut mem = [0u16; MEMORY_MAX];
        regs[0] = 100;
        regs[1] = 1203;
        mem[1230] = 0xABCD;
        instruction.eval(&mut regs, &mut mem);
        assert_eq!(regs[0], 0xABCD);
        assert!(regs[R_COND] == FL_NEG);
    }

    #[test]
    fn test_load_effective_address() {
        // opcode: 1110, dr: 100, offset: 001001000
        let op_body = 0b1110_100_001001000;
        let instruction = Instruction::from(op_body);
        match instruction {
            Instruction::LEA { dr, offset } => {
                assert_eq!(dr, 4);
                assert_eq!(offset, 72);
            }
            _ => panic!("Expected LEA instruction"),
        }
        let mut regs = [0u16; N_REGS];
        let mut mem = [0u16; MEMORY_MAX];
        regs[R_PC] = 3000;
        regs[4] = 123;
        instruction.eval(&mut regs, &mut mem);
        assert_eq!(regs[4], 3072);
        assert!(regs[R_COND] == FL_POS);
    }

    #[test]
    fn test_not() {
        // opcode: 1001, dr: 000, base: 001, empty: 111111
        let op_body = 0b1001_010_010_111111;
        let instruction = Instruction::from(op_body);
        match instruction {
            Instruction::NOT { dr, sr } => {
                assert_eq!(dr, 2);
                assert_eq!(sr, 2);
            }
            _ => panic!("Expected NOT instruction"),
        }
        let mut regs = [0u16; N_REGS];
        let mut mem = [0u16; MEMORY_MAX];
        regs[2] = 0xF0FF;
        instruction.eval(&mut regs, &mut mem);
        assert_eq!(regs[2], 0x0F00);
        assert!(regs[R_COND] == FL_POS);
    }

    #[test]
    fn test_res() {
        // opcode: 1101
        let op_body = 0b1101_000000000000;
        let instruction = Instruction::from(op_body);
        match instruction {
            Instruction::RES => return,
            _ => panic!("Expected RES instruction"),
        }
    }

    #[test]
    fn test_rti() {
        // opcode: 1000
        let op_body = 0b1000_000000000000;
        let instruction = Instruction::from(op_body);
        match instruction {
            Instruction::RTI => return,
            _ => panic!("Expected RTI instruction"),
        }
    }
}
