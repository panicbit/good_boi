
mod instruction;
pub use instruction::{Instruction, ExtendedInstruction, Cond, Operand, Reg8, Reg16};

pub struct Core {
    ip: usize,
    sp: u16,
    reg_a: u8,
    rom: Vec<u8>,
    interrupts_enabled: bool,
    trace_instructions: bool,
    trace_state: bool,
}

impl Core {
    pub fn new(rom: Vec<u8>) -> Self {
        Self {
            ip: 0x100,
            sp: 0xFFFF,
            reg_a: 0,
            rom,
            interrupts_enabled: true,
            trace_instructions: true,
            trace_state: true,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.step();
        }
    }

    pub fn step(&mut self) {
        let code = self.rom[self.ip as usize];
        let instr = Instruction::decode(code);

        self.execute(instr);
    }

    pub fn execute(&mut self, instr: Instruction) {
        if self.trace_instructions {
            println!(">> {:?}", instr);
        }

        self.ip += 1;

        match instr {
            Instruction::Nop => {},
            Instruction::Jp(cond, addr) => self.execute_jp(cond, addr),
            Instruction::Di => self.execute_di(),
            Instruction::Ld(target, source) => self.execute_ld(target, source),
            _ => unimplemented!("Instruction::{:?}", instr),
        }

        if self.trace_state {
            println!("|| ip@{:02X} sp@{:02X}", self.ip, self.sp);
        }
    }

    pub fn execute_jp(&mut self, cond: Cond, addr: Operand) {
        let cond = self.evaluate_cond(cond);
        let addr = self.load_u16_operand(addr);

        if cond {
            self.ip = addr as usize;
        }
    }

    pub fn execute_di(&mut self) {
        self.interrupts_enabled = false;
    }

    pub fn execute_ld(&mut self, target: Operand, source: Operand) {
        let value = self.load_operand(source);

        self.store_operand(target, value);
    }

    pub fn evaluate_cond(&self, cond: Cond) -> bool {
        match cond {
            Cond::Always => true,
            _ => unimplemented!("Cond::{:?}", cond),
        }
    }

    pub fn load_operand(&mut self, source: Operand) -> Value {
        match source {
            Operand::Imm8 => Value::U8(self.decode_imm8()),
            Operand::Imm16 => Value::U16(self.decode_imm16()),
            Operand::Reg8(Reg8::A) => Value::U8(self.reg_a),
            _ => unimplemented!("load_operand: {:?}", source),
        }
    }

    pub fn store_operand(&mut self, target: Operand, value: Value) {
        match (target, value) {
            (Operand::Reg8(Reg8::A), Value::U8(value)) => self.reg_a = value,
            (Operand::Reg16(Reg16::SP), _) => self.sp = value.as_u16(),
            (Operand::Imm16Ref, Value::U8(value)) => {
                let ptr = self.decode_imm16() as usize;
                self.rom[ptr] = value;
            },
            _ => unimplemented!("store_operand: {:?} <- {:?}", target, value),
        }
    }

    pub fn load_u16_operand(&mut self, operand: Operand) -> u16 {
        match operand {
            Operand::Imm16 => self.decode_imm16(),
            _ => unimplemented!("load_u16_operand: {:?}", operand),
        }
    }

    pub fn decode_imm8(&mut self) -> u8 {
        let value = self.rom[self.ip];

        self.ip += 1;

        value
    }

    pub fn decode_imm16(&mut self) -> u16 {
        let lo = self.rom[self.ip] as u16;
        let hi = self.rom[self.ip + 1] as u16;
        let value = hi << 8 | lo;

        self.ip += 2;

        value
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Value {
    U8(u8),
    U16(u16),
}

impl Value {
    fn as_u16(self) -> u16 {
        match self {
            Value::U8(value) => value as u16,
            Value::U16(value) => value,
        }
    }
}

fn main() {
    let data = include_bytes!("../gb-test-roms/cpu_instrs/cpu_instrs.gb");
    let mut core = Core::new(data.to_vec());

    core.run();

    // for (addr, code) in data.iter().cloned().enumerate() {
    //     let instr = Instruction::decode(code);

    //     println!("{:04X} = 0x{:02X} {:?}", addr, code, instr);
    // }
}
