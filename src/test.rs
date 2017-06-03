use mips_disassemble::{disassemble, MipsInstructionType, MipsInstructionArgument};
use MipsInstructionType::*;
use MipsInstructionArgument::*;


#[test]
fn disassemble_r_nop() {
    assert_disassembles_to(0x00000000, R, "nop", vec!());
}

#[test]
fn disassemble_r_add() {
    assert_disassembles_to(0x01398820, R, "add", vec!(r("$s1"), r("$t1"), r("$t9")));
}

#[test]
fn disassemble_r_addu() {
    assert_disassembles_to(0x01398821, R, "addu", vec!(r("$s1"), r("$t1"), r("$t9")));
}

#[test]
fn disassemble_r_and() {
    assert_disassembles_to(0x01398824, R, "and", vec!(r("$s1"), r("$t1"), r("$t9")));
}

#[test]
fn disassemble_r_clo() {
    assert_disassembles_to(0x71208821, R, "clo", vec!(r("$s1"), r("$t1")));
}

#[test]
fn disassemble_r_clz() {
    assert_disassembles_to(0x71208820, R, "clz", vec!(r("$s1"), r("$t1")));
}

#[test]
fn disassemble_r_div() {
    assert_disassembles_to(0x0229001a, R, "div", vec!(r("$s1"), r("$t1")));
}

#[test]
fn disassemble_r_divu() {
    assert_disassembles_to(0x0229001b, R, "divu", vec!(r("$s1"), r("$t1")));
}

#[test]
fn disassemble_r_mult() {
    assert_disassembles_to(0x02290018, R, "mult", vec!(r("$s1"), r("$t1")));
}

#[test]
fn disassemble_r_multu() {
    assert_disassembles_to(0x02290019, R, "multu", vec!(r("$s1"), r("$t1")));
}

#[test]
fn disassemble_r_mul() {
    assert_disassembles_to(0x71398802, R, "mul", vec!(r("$s1"), r("$t1"), r("$t9")));
}

#[test]
fn disassemble_r_madd() {
    assert_disassembles_to(0x72290000, R, "madd", vec!(r("$s1"), r("$t1")));
}

#[test]
fn disassemble_r_maddu() {
    assert_disassembles_to(0x72290001, R, "maddu", vec!(r("$s1"), r("$t1")));
}

#[test]
fn disassemble_r_msub() {
    assert_disassembles_to(0x72290004, R, "msub", vec!(r("$s1"), r("$t1")));
}

#[test]
fn disassemble_r_msubu() {
    assert_disassembles_to(0x72290005, R, "msubu", vec!(r("$s1"), r("$t1")));
}

#[test]
fn disassemble_r_nor() {
    assert_disassembles_to(0x01398827, R, "nor", vec!(r("$s1"), r("$t1"), r("$t9")));
}

#[test]
fn disassemble_r_or() {
    assert_disassembles_to(0x01398825, R, "or", vec!(r("$s1"), r("$t1"), r("$t9")));
}

#[test]
fn disassemble_r_sll() {
    assert_disassembles_to(0x00098cc0, R, "sll", vec!(r("$s1"), r("$t1"), Value(19)));
}

#[test]
fn disassemble_r_sllv() {
    assert_disassembles_to(0x03298804, R, "sllv", vec!(r("$s1"), r("$t1"), r("$t9")));
}

#[test]
fn disassemble_r_sra() {
    assert_disassembles_to(0x00098cc3, R, "sra", vec!(r("$s1"), r("$t1"), Value(19)));
}

#[test]
fn disassemble_r_srav() {
    assert_disassembles_to(0x03298807, R, "srav", vec!(r("$s1"), r("$t1"), r("$t9")));
}

#[test]
fn disassemble_r_srl() {
    assert_disassembles_to(0x00098cc2, R, "srl", vec!(r("$s1"), r("$t1"), Value(19)));
}

#[test]
fn disassemble_r_srlv() {
    assert_disassembles_to(0x03298806, R, "srlv", vec!(r("$s1"), r("$t1"), r("$t9")));
}

#[test]
fn disassemble_r_sub() {
    assert_disassembles_to(0x01398822, R, "sub", vec!(r("$s1"), r("$t1"), r("$t9")));
}

#[test]
fn disassemble_r_subu() {
    assert_disassembles_to(0x01398823, R, "subu", vec!(r("$s1"), r("$t1"), r("$t9")));
}

#[test]
fn disassemble_r_xor() {
    assert_disassembles_to(0x01398826, R, "xor", vec!(r("$s1"), r("$t1"), r("$t9")));
}

#[test]
fn disassemble_r_slt() {
    assert_disassembles_to(0x0139882a, R, "slt", vec!(r("$s1"), r("$t1"), r("$t9")));
}

#[test]
fn disassemble_r_sltu() {
    assert_disassembles_to(0x0139882b, R, "sltu", vec!(r("$s1"), r("$t1"), r("$t9")));
}

#[test]
fn disassemble_r_jalr() {
    assert_disassembles_to(0x01208809, R, "jalr", vec!(r("$s1"), r("$t1")));
}


#[test]    
fn disassemble_i_addi() {
    assert_disassembles_to(0x213104d2, I, "addi", vec!(r("$s1"), r("$t1"), Value(1234)));
    assert_disassembles_to(0x2131fb2e, I, "addi", vec!(r("$s1"), r("$t1"), Value(-1234)));
}

#[test]
fn disassemble_i_addiu() {
    assert_disassembles_to(0x253104d2, I, "addiu", vec!(r("$s1"), r("$t1"), Value(1234)));
    assert_disassembles_to(0x2531fb2e, I, "addiu", vec!(r("$s1"), r("$t1"), Value(-1234)));
}

#[test]
fn disassemble_i_slti() {
    assert_disassembles_to(0x293104d2, I, "slti", vec!(r("$s1"), r("$t1"), Value(1234)));
    assert_disassembles_to(0x2931fb2e, I, "slti", vec!(r("$s1"), r("$t1"), Value(-1234)));
}

#[test]
fn disassemble_i_sltiu() {
    assert_disassembles_to(0x2d3104d2, I, "sltiu", vec!(r("$s1"), r("$t1"), Value(1234)));
    assert_disassembles_to(0x2d31fb2e, I, "sltiu", vec!(r("$s1"), r("$t1"), Value(-1234)));
}

#[test]
fn disassemble_i_andi() {
    assert_disassembles_to(0x313104d2, I, "andi", vec!(r("$s1"), r("$t1"), Value(1234)));
}

#[test]
fn disassemble_i_ori() {
    assert_disassembles_to(0x353104d2, I, "ori", vec!(r("$s1"), r("$t1"), Value(1234)));
}

#[test]
fn disassemble_i_xori() {
    assert_disassembles_to(0x393104d2, I, "xori", vec!(r("$s1"), r("$t1"), Value(1234)));
}

#[test]
fn disassemble_i_lui() {
    assert_disassembles_to(0x3c1104d2, I, "lui", vec!(r("$s1"), Value(1234)));
}

#[test]
fn disassemble_i_teqi() {
    assert_disassembles_to(0x062c04d2, I, "teqi", vec!(r("$s1"), Value(1234)));
    assert_disassembles_to(0x062cfb2e, I, "teqi", vec!(r("$s1"), Value(-1234)));
}

#[test]
fn disassemble_i_tnei() {
    assert_disassembles_to(0x062e04d2, I, "tnei", vec!(r("$s1"), Value(1234)));
    assert_disassembles_to(0x062efb2e, I, "tnei", vec!(r("$s1"), Value(-1234)));
}

#[test]
fn disassemble_i_tgei() {
    assert_disassembles_to(0x062804d2, I, "tgei", vec!(r("$s1"), Value(1234)));
    assert_disassembles_to(0x0628fb2e, I, "tgei", vec!(r("$s1"), Value(-1234)));
}

#[test]
fn disassemble_i_tgeiu() {
    assert_disassembles_to(0x062904d2, I, "tgeiu", vec!(r("$s1"), Value(1234)));
    assert_disassembles_to(0x0629fb2e, I, "tgeiu", vec!(r("$s1"), Value(-1234)));
}

#[test]
fn disassemble_i_tlti() {
    assert_disassembles_to(0x062a04d2, I, "tlti", vec!(r("$s1"), Value(1234)));
    assert_disassembles_to(0x062afb2e, I, "tlti", vec!(r("$s1"), Value(-1234)));
}

#[test]
fn disassemble_i_tltiu() {
    assert_disassembles_to(0x062b04d2, I, "tltiu", vec!(r("$s1"), Value(1234)));
    assert_disassembles_to(0x062bfb2e, I, "tltiu", vec!(r("$s1"), Value(-1234)));
}

#[test]
fn disassemble_i_lb() {
    assert_disassembles_to(0x813104d2, I, "lb", vec!(r("$s1"), ro(1234, "$t1")));
    assert_disassembles_to(0x8131fb2e, I, "lb", vec!(r("$s1"), ro(-1234, "$t1")));
}

#[test]
fn disassemble_i_lbu() {
    assert_disassembles_to(0x913104d2, I, "lbu", vec!(r("$s1"), ro(1234, "$t1")));
    assert_disassembles_to(0x9131fb2e, I, "lbu", vec!(r("$s1"), ro(-1234, "$t1")));
}

#[test]
fn disassemble_i_lh() {
    assert_disassembles_to(0x853104d2, I, "lh", vec!(r("$s1"), ro(1234, "$t1")));
    assert_disassembles_to(0x8531fb2e, I, "lh", vec!(r("$s1"), ro(-1234, "$t1")));
}

#[test]
fn disassemble_i_lhu() {
    assert_disassembles_to(0x953104d2, I, "lhu", vec!(r("$s1"), ro(1234, "$t1")));
    assert_disassembles_to(0x9531fb2e, I, "lhu", vec!(r("$s1"), ro(-1234, "$t1")));
}

#[test]
fn disassemble_i_lw() {
    assert_disassembles_to(0x8d3104d2, I, "lw", vec!(r("$s1"), ro(1234, "$t1")));
    assert_disassembles_to(0x8d31fb2e, I, "lw", vec!(r("$s1"), ro(-1234, "$t1")));
}

#[test]
fn disassemble_i_lwl() {
    assert_disassembles_to(0x893104d2, I, "lwl", vec!(r("$s1"), ro(1234, "$t1")));
    assert_disassembles_to(0x8931fb2e, I, "lwl", vec!(r("$s1"), ro(-1234, "$t1")));
}

#[test]
fn disassemble_i_lwr() {
    assert_disassembles_to(0x993104d2, I, "lwr", vec!(r("$s1"), ro(1234, "$t1")));
    assert_disassembles_to(0x9931fb2e, I, "lwr", vec!(r("$s1"), ro(-1234, "$t1")));
}

#[test]
fn disassemble_i_ll() {
    assert_disassembles_to(0xc13104d2, I, "ll", vec!(r("$s1"), ro(1234, "$t1")));
    assert_disassembles_to(0xc131fb2e, I, "ll", vec!(r("$s1"), ro(-1234, "$t1")));
}

#[test]
fn disassemble_i_sb() {
    assert_disassembles_to(0xa13104d2, I, "sb", vec!(r("$s1"), ro(1234, "$t1")));
    assert_disassembles_to(0xa131fb2e, I, "sb", vec!(r("$s1"), ro(-1234, "$t1")));
}

#[test]
fn disassemble_i_sh() {
    assert_disassembles_to(0xa53104d2, I, "sh", vec!(r("$s1"), ro(1234, "$t1")));
    assert_disassembles_to(0xa531fb2e, I, "sh", vec!(r("$s1"), ro(-1234, "$t1")));
}

#[test]
fn disassemble_i_sw() {
    assert_disassembles_to(0xad3104d2, I, "sw", vec!(r("$s1"), ro(1234, "$t1")));
    assert_disassembles_to(0xad31fb2e, I, "sw", vec!(r("$s1"), ro(-1234, "$t1")));
}

#[test]
fn disassemble_i_swl() {
    assert_disassembles_to(0xa93104d2, I, "swl", vec!(r("$s1"), ro(1234, "$t1")));
    assert_disassembles_to(0xa931fb2e, I, "swl", vec!(r("$s1"), ro(-1234, "$t1")));
}

#[test]
fn disassemble_i_swr() {
    assert_disassembles_to(0xb93104d2, I, "swr", vec!(r("$s1"), ro(1234, "$t1")));
    assert_disassembles_to(0xb931fb2e, I, "swr", vec!(r("$s1"), ro(-1234, "$t1")));
}

#[test]
fn disassemble_i_sc() {
    assert_disassembles_to(0xe13104d2, I, "sc", vec!(r("$s1"), ro(1234, "$t1")));
    assert_disassembles_to(0xe131fb2e, I, "sc", vec!(r("$s1"), ro(-1234, "$t1")));
}

#[test]
fn disassemble_i_beq() {
    assert_disassembles_to(0x1229ffff, I, "beq", vec!(r("$s1"), r("$t1"), Value(-1)));
    assert_disassembles_to(0x1229000e, I, "beq", vec!(r("$s1"), r("$t1"), Value(14)));
}

#[test]
fn disassemble_i_bgez() {
    assert_disassembles_to(0x0621fffd, I, "bgez", vec!(r("$s1"), Value(-3)));
    assert_disassembles_to(0x0621000c, I, "bgez", vec!(r("$s1"), Value(12)));
}

#[test]
fn disassemble_i_bgezal() {
    assert_disassembles_to(0x0631fffb, I, "bgezal", vec!(r("$s1"), Value(-5)));
    assert_disassembles_to(0x0631000a, I, "bgezal", vec!(r("$s1"), Value(10)));
}

#[test]
fn disassemble_i_bgtz() {
    assert_disassembles_to(0x1e20fff9, I, "bgtz", vec!(r("$s1"), Value(-7)));
    assert_disassembles_to(0x1e200008, I, "bgtz", vec!(r("$s1"), Value(8)));
}

#[test]
fn disassemble_i_blez() {
    assert_disassembles_to(0x1a20fff7, I, "blez", vec!(r("$s1"), Value(-9)));
    assert_disassembles_to(0x1a200006, I, "blez", vec!(r("$s1"), Value(6)));
}

#[test]
fn disassemble_i_bltzal() {
    assert_disassembles_to(0x0630fff5, I, "bltzal", vec!(r("$s1"), Value(-11)));
    assert_disassembles_to(0x06300004, I, "bltzal", vec!(r("$s1"), Value(4)));
}

#[test]
fn disassemble_i_bltz() {
    assert_disassembles_to(0x0620fff3, I, "bltz", vec!(r("$s1"), Value(-13)));
    assert_disassembles_to(0x06200002, I, "bltz", vec!(r("$s1"), Value(2)));
}

#[test]
fn disassemble_i_bne() {
    assert_disassembles_to(0x1629fff1, I, "bne", vec!(r("$s1"), r("$t1"), Value(-15)));
    assert_disassembles_to(0x16290000, I, "bne", vec!(r("$s1"), r("$t1"), Value(0)));
}


#[test]
fn disassemble_j_j() {
    assert_disassembles_to(0xbfffb2e, J, "j", vec!(Address(-1234)));
    assert_disassembles_to(0x80004d2, J, "j", vec!(Address(1234)));
}

#[test]
fn disassemble_j_jal() {
    assert_disassembles_to(0xffffb2e, J, "jal", vec!(Address(-1234)));
    assert_disassembles_to(0xc0004d2, J, "jal", vec!(Address(1234)));
}


fn r(register_name: &str) -> MipsInstructionArgument {
    Register(register_name.into())
}

fn ro(offset: i16, register_name: &str) -> MipsInstructionArgument {
    RegisterWithOffset(offset, register_name.into())
}

fn assert_disassembles_to(instruction_number: u32, instruction_type: MipsInstructionType, name: &str, arguments: Vec<MipsInstructionArgument>) {
    let instruction = match disassemble(instruction_number) {
        Ok(i) => i,
        Err(e) => panic!(format!("Test failed for 0x{:x}: invalid instruction (expected {}): {}", instruction_number, name, e))
    };
    
    if instruction.instruction_type != instruction_type {
        panic!(format!("Test failed for 0x{:x} wrong type (got {:?}, expected {:?})",
                instruction_number, instruction.instruction_type, instruction_type));
    }

    if name != instruction.name {
        panic!(format!("Test failed for 0x{:x} wrong name (got {}, expected {})",
                instruction_number, instruction.name, name));
    }

    assert_eq!(instruction.arguments, arguments);
}
