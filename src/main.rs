pub use crate::core::Core;

mod instruction;
mod core;

fn main() {
    // let data = include_bytes!("../gb-test-roms/cpu_instrs/cpu_instrs.gb");
    let data = include_bytes!("../gb-test-roms/cpu_instrs/individual/06-ld r,r.gb");

    let mut core = Core::new(data.to_vec());

    core.run();

    // for (addr, code) in data.iter().cloned().enumerate() {
    //     let instr = Instruction::decode(code);

    //     println!("{:04X} = 0x{:02X} {:?}", addr, code, instr);
    // }
}
