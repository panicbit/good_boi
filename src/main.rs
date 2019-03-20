
mod instruction;
pub use instruction::{Instruction, ExtendedInstruction, Cond, Operand, Reg8, Reg16};

pub struct Core {
    ip: u16,
    sp: u16,
    reg_a: u8,
    reg_b: u8,
    reg_c: u8,
    reg_hl: u16,
    reg_f: u8,
    rom: Vec<u8>,
    interrupts_enabled: bool,
    trace_instructions: bool,
    trace_state: bool,
}

impl Core {
    pub fn new(rom: Vec<u8>) -> Self {
        Self {
            ip: 0x100,
            sp: 0xFFFE,
            reg_a: 0,
            reg_b: 0,
            reg_c: 0,
            reg_hl: 0,
            reg_f: 0,
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

    pub fn reg_h(&self) -> u8 {
        (self.reg_hl >> 8) as u8
    }

    pub fn reg_l(&self) -> u8 {
        self.reg_hl as u8
    }

    pub fn reg_af(&self) -> u16 {
        (self.reg_a as u16) << 8 | self.reg_f as u16
    }

    pub fn reg_bc(&self) -> u16 {
        (self.reg_b as u16) << 8 | self.reg_c as u16
    }

    pub fn execute(&mut self, instr: Instruction) {
        if self.trace_instructions {
            println!(">> {:?}", instr);
        }

        self.ip += 1;

        match instr {
            Instruction::Nop => {},
            Instruction::Jr(cond, offset) => self.execute_jr(cond, offset),
            Instruction::Jp(cond, addr) => self.execute_jp(cond, addr),
            Instruction::Call(cond, addr) => self.execute_call(cond, addr),
            Instruction::Di => self.execute_di(),
            Instruction::Ld(target, source) => self.execute_ld(target, source),
            Instruction::Ldh(target, source) => self.execute_ldh(target, source),
            Instruction::Push(source) => self.execute_push(source),
            Instruction::Cp(source) => self.execute_cp(source),
            _ => unimplemented!("execute: {:?}", instr),
        }

        if self.trace_state {
            println!("|| ip@{:02X} sp@{:02X}", self.ip, self.sp);
        }
    }

    pub fn execute_cp(&mut self, source: Operand) {
        let value = self.load_u8_operand(source);
        let comp = self.reg_a - value;

        // TODO: set flags correctly
    }

    pub fn execute_jr(&mut self, cond: Cond, offset: Operand) {
        let cond = self.evaluate_cond(cond);
        let offset = self.load_u8_operand(offset);

        if cond {
            self.ip += offset as u16;
        }
    }

    pub fn execute_jp(&mut self, cond: Cond, addr: Operand) {
        let cond = self.evaluate_cond(cond);
        let addr = self.load_u16_operand(addr);

        if cond {
            self.ip = addr;
        }
    }

    pub fn execute_call(&mut self, cond: Cond, addr: Operand) {
        let cond = self.evaluate_cond(cond);
        let addr = self.load_u16_operand(addr);

        if cond {
            self.push_u16(self.ip);
            self.ip = addr;
        }
    }

    pub fn execute_di(&mut self) {
        self.interrupts_enabled = false;
    }

    pub fn execute_ld(&mut self, target: Operand, source: Operand) {
        let value = self.load_operand(source);

        self.store_operand(target, value);
    }

    pub fn execute_ldh(&mut self, target: Operand, source: Operand) {
        match (target, source) {
            (Operand::Imm8Ref, _) => {
                let ptr = self.decode_imm8() as usize + 0xFF00;
                let value = self.load_u8_operand(source);

                self.rom[ptr] = value;
            },
            _ => unimplemented!("execute_ldh: {:?} <- {:?}", target, source),
        }
    }

    pub fn execute_push(&mut self, source: Reg16) {
        match source {
            Reg16::AF => self.push_u16(self.reg_af()),
            Reg16::BC => self.push_u16(self.reg_bc()),
            _ => unimplemented!("execute_push: {:?}", source),
        }
    }

    pub fn push_u16(&mut self, value: u16) {
        let lo = value as u8;
        let hi = (value >> 8) as u8;

        self.sp -= 2;
        self.rom[self.sp as usize] = lo;
        self.rom[self.sp as usize + 1] = hi;
    }

    pub fn evaluate_cond(&self, cond: Cond) -> bool {
        match cond {
            Cond::Always => true,
            Cond::ZSet => (self.reg_f >> 7) & 1 == 1,
            Cond::ZReset => (self.reg_f >> 7) & 1 == 0,
            _ => unimplemented!("Cond::{:?}", cond),
        }
    }

    pub fn load_operand(&mut self, source: Operand) -> Value {
        match source {
            Operand::Imm8 => Value::U8(self.decode_imm8()),
            Operand::Imm16 => Value::U16(self.decode_imm16()),
            Operand::Reg8(Reg8::H) => Value::U8(self.reg_h()),
            Operand::Reg8(Reg8::L) => Value::U8(self.reg_l()),
            Operand::Reg8(Reg8::A) => Value::U8(self.reg_a),
            _ => unimplemented!("load_operand: {:?}", source),
        }
    }

    pub fn store_operand(&mut self, target: Operand, value: Value) {
        match value {
            Value::U8(value) => self.store_operand_u8(target, value),
            Value::U16(value) => self.store_operand_u16(target, value),
        }
    }

    pub fn store_operand_u8(&mut self, target: Operand, value: u8) {
        match target {
            Operand::Reg8(Reg8::A) => self.reg_a = value,
            Operand::Imm16Ref => {
                let ptr = self.decode_imm16() as usize;
                self.rom[ptr] = value;
            }
            _ => unimplemented!("store_operand_u8: {:?} <- {:?}", target, value)
        }
    }

    pub fn store_operand_u16(&mut self, target: Operand, value: u16) {
        match target {
            Operand::Reg16(Reg16::HL) => self.reg_hl = value,
            Operand::Reg16(Reg16::SP) => self.sp = value,
            _ => unimplemented!("store_operand_u16: {:?} <- {:?}", target, value)
        }
    }

    pub fn load_u8_operand(&mut self, operand: Operand) -> u8 {
        match operand {
            Operand::Imm8 => self.decode_imm8(),
            Operand::Reg8(Reg8::A) => self.reg_a,
            _ => unimplemented!("load_u8_operand: {:?}", operand),
        }
    }

    pub fn load_u16_operand(&mut self, operand: Operand) -> u16 {
        match operand {
            Operand::Imm16 => self.decode_imm16(),
            _ => unimplemented!("load_u16_operand: {:?}", operand),
        }
    }

    pub fn decode_imm8(&mut self) -> u8 {
        let value = self.rom[self.ip as usize];

        self.ip += 1;

        value
    }

    pub fn decode_imm16(&mut self) -> u16 {
        let lo = self.rom[self.ip as usize] as u16;
        let hi = self.rom[self.ip as usize + 1] as u16;
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
