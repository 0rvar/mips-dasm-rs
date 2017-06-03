#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate mips_dasm;

use mips_dasm::disassemble;

fuzz_target!(|data: u32| {
    disassemble(data);
});
