extern crate capstone;

use capstone::prelude::*;

const X86_CODE: &'static [u8] = 
    b"\x55\x48\x8b\x05\xb8\x13\x00\x00\xe9\x14\x9e\x08\x00\x45\x31\xe4";

fn main() {
    let mut cs = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode64)
        .detail(true)
        .build()
        .expect("Failed to create Capstone object");

    let insns = cs.disasm_all(X86_CODE, 0x1000)
        .expect("Failed to dissasemble");

    println!("count = {}", insns.len());
    for insn in insns.iter() {
        println!("0x{:x}: {:6} {}",
                 insn.address(),
                 insn.mnemonic().unwrap_or(""),
                 insn.op_str().unwrap_or("")
                 );
    }
}
