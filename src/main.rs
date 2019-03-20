
mod instruction;
pub use instruction::{Instruction, ExtendedInstruction};

fn main() {
    for code in 0x00..=0xFF {
        println!("0x{:02X} = {:?}", code, Instruction::decode(code));
    }

    println!();

    for code in 0x00..=0xFF {
        println!("0xCB 0x{:02X} = {:?}", code, ExtendedInstruction::decode(code));
    }
}
