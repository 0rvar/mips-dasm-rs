#[derive(Debug, PartialEq)]
pub enum MipsInstructionType { R, J, I }
use MipsInstructionType::*;

#[derive(Debug, PartialEq)]
pub enum MipsInstructionArgument {
    Register(String),
    RegisterWithOffset(i16, String),
    Value(i16),
    Address(i32)
}
use MipsInstructionArgument::*;

fn register(index: u8) -> MipsInstructionArgument {
    Register(get_register_name(index)) 
}

fn get_register_name(index: u8) -> String {
    MIPS_REGISTER_NAMES[index as usize].into()
}

#[derive(Debug)]
pub struct MipsInstruction {
  pub name: String,
  pub instruction_type: MipsInstructionType,
  pub arguments: Vec<MipsInstructionArgument>
}

macro_rules! BITMASK(
    ($n:expr) => (
        ((1 << $n) - 1)
    );
);

pub fn disassemble(encoded_instruction: u32) -> Result<MipsInstruction, String> {
    if encoded_instruction == 0 {
        return Ok(MipsInstruction {
            name: "nop".into(),
            instruction_type: R,
            arguments: vec!()
        });
    }

    let op: u8 = (encoded_instruction >> 26) as u8;
    let op_upper: u8 = (op >> 3) & BITMASK!(3);
    let op_lower: u8 = op & BITMASK!(3);

    let rs: u8 = ((encoded_instruction >> 21) & BITMASK!(5)) as u8;

    let rt: u8 = ((encoded_instruction >> 16) & BITMASK!(5)) as u8;
    let rt_upper: u8 = rt >> 3;
    let rt_lower: u8 = rt & BITMASK!(3);

    let rd: u8 = ((encoded_instruction >> 11) & BITMASK!(5)) as u8;
    let sa: u8 = ((encoded_instruction >> 6) & BITMASK!(5)) as u8;

    let funct: u8 = (encoded_instruction & BITMASK!(6)) as u8;
    let funct_upper: u8 = (funct >> 3) & BITMASK!(3);
    let funct_lower: u8 = funct & BITMASK!(3);

    let imm: i16 = (encoded_instruction & BITMASK!(16)) as i16;

    // Tricky business: two's complement 26-bit
    let mut target: i32 = (encoded_instruction & BITMASK!(26)) as i32;
    if target > BITMASK!(25) {
        target |= 0xfc000000;
    }

    let name: Option<String>;
    let i_type: Option<MipsInstructionType>;

    if op == 0 {
        name = Some(MIPS_REGISTER_INSTRUCTION_NAMES[funct_upper as usize][funct_lower as usize].into());
        i_type = Some(MipsInstructionType::R);
    } else if op == 0x1c {
        name = Some(MIPS_REGISTER_C_INSTRUCTION_NAMES[funct_upper as usize][funct_lower as usize].into());
        i_type = Some(MipsInstructionType::R);
    } else if op == 1 {
        name = Some(MIPS_REGISTER_RT_INSTRUCTION_NAMES[rt_upper as usize][rt_lower as usize].into());
        i_type = Some(MipsInstructionType::I);
    } else {
        name = Some(MIPS_ROOT_INSTRUCTION_NAMES[op_upper as usize][op_lower as usize].into());
        i_type = Some(MipsInstructionType::I);
    }

    let instruction_name: String = match name {
        Some(n) => n,
        None => return Err(format!("Could not deduce name for instruction 0x{:x}", encoded_instruction))
    };

    let instruction_type: MipsInstructionType = match i_type {
        Some(t) => t,
        None => return Err(format!("Could not deduce type for instruction 0x{:x}", encoded_instruction))
    };

    let instruction_name_copy = instruction_name.clone();
    let with_args = |args| Ok(MipsInstruction {
        name: instruction_name_copy,
        instruction_type: instruction_type,
        arguments: args
    });
    

    if op == 0 {
        match funct_upper {
            MIPS_REG_TYPE_SHIFT_OR_SHIFTV => {
                if funct_lower < 4 {
                    //Shift
                    return with_args(vec!(
                        register(rd),
                        register(rt),
                        Value(sa as i16)
                    ));
                } else {
                    //ShiftV
                    return with_args(vec!(
                        register(rd),
                        register(rt),
                        register(rs)
                    ));
                }
            },
            MIPS_REG_TYPE_JUMPR => {
                if funct_lower < 1 {
                    return with_args(vec!(register(rs)));
                } else {
                    return with_args(vec!(
                        register(rd),
                        register(rs)
                    ));
                }
            },
            MIPS_REG_TYPE_MOVE => {
                if funct_lower % 2 == 0 {
                    return with_args(vec!(register(rd)));
                } else {
                    return with_args(vec!(register(rs)));
                }
            },
            MIPS_REG_TYPE_DIVMULT => {
                return with_args(vec!(
                    register(rs),
                    register(rt)
                ));
            },
            i if i == MIPS_REG_TYPE_ARITHLOG_GTE || i == MIPS_REG_TYPE_ARITHLOG_GTE + 1 => {
                return with_args(vec!(
                    register(rd),
                    register(rs),
                    register(rt)
                ));
            },
            _ => {
                return Err("Unknown function index".into());
            }
        }
    } else if op == 0x1c {
        match funct_upper {
            MIPS_REG_C_TYPE_MULTIPLY => {
                if funct_lower == 2 {
                    return with_args(vec!(
                        register(rd),
                        register(rs),
                        register(rt)
                    ));
                } else {
                    return with_args(vec!(
                        register(rs),
                        register(rt)
                    ));
                }
            },
            MIPS_REG_C_TYPE_COUNT => {
                return with_args(vec!(
                    register(rd),
                    register(rs)
                ));
            },
            _ => {}
        }

    } else if op == 1 {
        return with_args(vec!(
            register(rs),
            Value(imm)
        ));
    } else {
        match op_upper {
            MIPS_ROOT_TYPE_JUMP_OR_BRANCH => {
                if op_lower < 4 {
                    // Jump
                    return Ok(MipsInstruction {
                        name: instruction_name,
                        instruction_type: J,
                        arguments: vec!(Address(target))
                    });
                } else {
                    if op_lower < 6 {
                        //Branch
                        //Shift
                        return with_args(vec!(
                            register(rs),
                            register(rt),
                            Value(imm)
                        ));
                    } else { 
                        //BranchZ
                        return with_args(vec!(
                            register(rs),
                            Value(imm)
                        ));
                    }
                }
            },
            MIPS_ROOT_TYPE_ARITHLOGI => {
                if op_lower < 7 {
                    return with_args(vec!(
                        register(rt),
                        register(rs),
                        Value(imm)
                    ));
                } else {
                    return with_args(vec!(
                        register(rt),
                        Value(imm)
                    ));
                }
            },
            i if i >= MIPS_ROOT_TYPE_LOADSTORE_GTE && i <= MIPS_ROOT_TYPE_LOADSTORE_GTE + 3 => {
                return with_args(vec!(
                    register(rt),
                    RegisterWithOffset(imm, get_register_name(rs))
                ));
            },
            _ => {}
        }
    }

    return Err("Unknown instruction".into());
}

const MIPS_REGISTER_RT_INSTRUCTION_NAMES: &'static [&'static [&'static str]] = &[
    &["bltz", "bgez"],
    &["tgei", "tgeiu", "tlti", "tltiu", "teqi", "", "tnei"],
    &["bltzal", "bgezal"]
];

const MIPS_REGISTER_C_INSTRUCTION_NAMES: &'static [&'static [&'static str]] = &[
    &["madd", "maddu", "mul", "", "msub", "msubu"],
    &[],
    &[],
    &[],
    &["clz", "clo"]
];

const MIPS_REGISTER_INSTRUCTION_NAMES: &'static [&'static [&'static str]] = &[
    &["sll", "", "srl", "sra", "sllv", "", "srlv", "srav"],
    &["jr", "jalr"],
    &["mfhi", "mthi", "mflo", "mtlo"],
    &["mult", "multu", "div", "divu"],
    &["add", "addu", "sub", "subu", "and", "or", "xor", "nor"],
    &["", "", "slt", "sltu"]
];

const MIPS_ROOT_INSTRUCTION_NAMES: &'static [&'static [&'static str]] = &[
	&["", "", "j", "jal", "beq", "bne", "blez", "bgtz"],
	&["addi", "addiu", "slti", "sltiu", "andi", "ori", "xori", "lui"],
	&[],
	&["llo", "lhi", "trap"],
	&["lb", "lh", "lwl", "lw", "lbu", "lhu", "lwr"],
	&["sb", "sh", "swl", "sw", "", "", "swr"],
    &["ll"],
    &["sc"]
];

const MIPS_REG_C_TYPE_MULTIPLY: u8 = 0;
const MIPS_REG_C_TYPE_COUNT: u8 = 4;

const MIPS_REG_TYPE_SHIFT_OR_SHIFTV: u8 = 0;
const MIPS_REG_TYPE_JUMPR: u8 = 1;
const MIPS_REG_TYPE_MOVE: u8 = 2;
const MIPS_REG_TYPE_DIVMULT: u8 = 3;
const MIPS_REG_TYPE_ARITHLOG_GTE: u8 = 4;

const MIPS_ROOT_TYPE_JUMP_OR_BRANCH: u8 = 0;
const MIPS_ROOT_TYPE_ARITHLOGI: u8 = 1;
#[allow(dead_code)]
const MIPS_ROOT_TYPE_LOADI_OR_TRAP: u8 = 3;
const MIPS_ROOT_TYPE_LOADSTORE_GTE: u8 = 4;

const MIPS_REGISTER_NAMES: &'static [&'static str] = &[
    "$zero",    // Hardware constant 0
    "$at",      // Reserved for assembler
    "$v0",      // Return values
    "$v1",
    "$a0",      // Arguments
    "$a1",
    "$a2",
    "$a3",
    "$t0",      // Temporaries
    "$t1",
    "$t2",
    "$t3",
    "$t4",
    "$t5",
    "$t6",
    "$t7",
    "$s0",      // Saved values
    "$s1",
    "$s2",
    "$s3",
    "$s4",
    "$s5",
    "$s6",
    "$s7",
    "$t8",      // Cont. Saved values
    "$t9",
    "$k0",      // Reserved for OS
    "$k1",
    "$gp",      // Global pointer
    "$sp",      // Stack Pointer
    "$fp",      // Frame Pointer
    "$ra"       // Return Adress
];