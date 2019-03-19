
mod instruction;
pub use instruction::Instruction;

fn main() {
    for code in 0x00..=0xFF {
        println!("0x{:02X} = {:?}", code, Instruction::decode(code));
    }
}
