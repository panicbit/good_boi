use crate::instruction::{Instruction, ExtendedInstruction, Cond, Operand, Reg8, Reg16};

pub struct Core {
    ip: u16,
    sp: u16,
    reg_a: u8,
    reg_b: u8,
    reg_c: u8,
    reg_d: u8,
    reg_e: u8,
    reg_f: u8,
    reg_h: u8,
    reg_l: u8,
    ram: Vec<u8>,
    interrupts_enabled: bool,
    trace_instructions: bool,
    trace_state: bool,
}

impl Core {
    pub fn new(rom: Vec<u8>) -> Self {
        let mut ram = vec![0; 0x10000];

        ram[..rom.len()].copy_from_slice(&rom);

        Self {
            ip: 0x100,
            sp: 0xFFFE,
            reg_a: 0,
            reg_b: 0,
            reg_c: 0,
            reg_d: 0,
            reg_e: 0,
            reg_h: 0,
            reg_l: 0,
            reg_f: 0,
            ram,
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
        let code = self.read_mem_u8(self.ip);
        let instr = Instruction::decode(code);

        self.execute(instr);
    }

    pub fn reg_hl(&self) -> u16 {
        (self.reg_h as u16) << 8 | self.reg_l as u16
    }

    pub fn set_reg_hl(&mut self, value: u16) {
        self.reg_h = (value >> 8) as u8;
        self.reg_l = value as u8;
    }

    pub fn reg_af(&self) -> u16 {
        (self.reg_a as u16) << 8 | self.reg_f as u16
    }

    pub fn set_reg_af(&mut self, value: u16) {
        self.reg_a = (value >> 8) as u8;
        self.reg_f = value as u8;
    }

    pub fn reg_bc(&self) -> u16 {
        (self.reg_b as u16) << 8 | self.reg_c as u16
    }

    pub fn set_reg_bc(&mut self, value: u16) {
        self.reg_b = (value >> 8) as u8;
        self.reg_c = value as u8;
    }

    pub fn reg_de(&self) -> u16 {
        (self.reg_d as u16) << 8 | self.reg_e as u16
    }

    pub fn set_reg_de(&mut self, value: u16) {
        self.reg_d = (value >> 8) as u8;
        self.reg_e = value as u8;
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
            Instruction::Inc(target) => self.execute_inc(target),
            Instruction::Dec(target) => self.execute_dec(target),
            Instruction::Xor(value) => self.execute_xor(value),
            _ => unimplemented!("execute: {:?}", instr),
        }

        if self.trace_state {
            println!("|| ip@{:02X} sp@{:02X}", self.ip, self.sp);
        }
    }

    fn write_mem_u8(&mut self, addr: u16, value: u8) {
        println!("${:04X} = {:02X}", addr, value);

        if addr == 0xFF01 && value == 0x81 {
            println!("\n {} \n", self.ram[0xFF01] as char);
        }

        self.ram[addr as usize] = value;
    }

    pub fn peek_mem_u8(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    fn read_mem_u8(&mut self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    pub fn peek_mem_u16(&self, addr: u16) -> u16 {
        let lo = self.peek_mem_u8(addr) as u16;
        let hi = self.peek_mem_u8(addr + 1) as u16;
        hi << 8 | lo
    }

    fn read_mem_u16(&mut self, addr: u16) -> u16 {
        let lo = self.read_mem_u8(addr) as u16;
        let hi = self.read_mem_u8(addr + 1) as u16;
        hi << 8 | lo
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
                let ptr = self.decode_imm8() as u16 + 0xFF00;
                let value = self.load_u8_operand(source);

                self.write_mem_u8(ptr, value);
            },
            _ => unimplemented!("execute_ldh: {:?} <- {:?}", target, source),
        }
    }

    pub fn execute_push(&mut self, source: Reg16) {
        match source {
            Reg16::AF => self.push_u16(self.reg_af()),
            Reg16::BC => self.push_u16(self.reg_bc()),
            Reg16::DE => self.push_u16(self.reg_de()),
            Reg16::HL => self.push_u16(self.reg_hl()),
            _ => unimplemented!("execute_push: {:?}", source),
        }
    }

    pub fn execute_inc(&mut self, target: Operand) {
        match self.load_operand(target) {
            Value::U8(value) => self.store_operand_u8(target, value + 1),
            Value::U16(value) => self.store_operand_u16(target, value + 1),
        }
    }

    pub fn execute_dec(&mut self, target: Operand) {
        match self.load_operand(target) {
            Value::U8(value) => self.store_operand_u8(target, value - 1),
            Value::U16(value) => self.store_operand_u16(target, value - 1),
        }
    }

    pub fn execute_xor(&mut self, value: Operand) {
        // TODO
        self.load_operand(value);
    }

    pub fn push_u16(&mut self, value: u16) {
        let lo = value as u8;
        let hi = (value >> 8) as u8;

        self.sp -= 2;
        self.write_mem_u8(self.sp, lo);
        self.write_mem_u8(self.sp + 1, hi);
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
            Operand::Reg8(Reg8::A) => Value::U8(self.reg_a),
            Operand::Reg8(Reg8::B) => Value::U8(self.reg_b),
            Operand::Reg8(Reg8::C) => Value::U8(self.reg_c),
            Operand::Reg8(Reg8::D) => Value::U8(self.reg_d),
            Operand::Reg8(Reg8::E) => Value::U8(self.reg_e),
            Operand::Reg8(Reg8::H) => Value::U8(self.reg_h),
            Operand::Reg8(Reg8::L) => Value::U8(self.reg_l),
            Operand::RegRef16(Reg16::HL) => Value::U8(self.read_mem_u8(self.reg_hl())),
            Operand::RegRef16(Reg16::HLInc) => {
                let addr = self.reg_hl();
                self.set_reg_hl(addr + 1);
                Value::U8(self.read_mem_u8(addr))
            },
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
            Operand::Reg8(Reg8::B) => self.reg_b = value,
            Operand::Reg8(Reg8::C) => self.reg_c = value,
            Operand::Reg8(Reg8::D) => self.reg_d = value,
            Operand::Reg8(Reg8::E) => self.reg_e = value,
            Operand::Reg8(Reg8::H) => self.reg_h = value,
            Operand::Reg8(Reg8::L) => self.reg_l = value,
            Operand::RegRef16(Reg16::DE) => self.write_mem_u8(self.reg_de(), value),
            Operand::Imm16Ref => {
                let ptr = self.decode_imm16();
                self.write_mem_u8(ptr, value);
            }
            _ => unimplemented!("store_operand_u8: {:?} <- {:?}", target, value)
        }
    }

    pub fn store_operand_u16(&mut self, target: Operand, value: u16) {
        match target {
            Operand::Reg16(Reg16::DE) => self.set_reg_de(value),
            Operand::Reg16(Reg16::HL) => self.set_reg_hl(value),
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
        let value = self.read_mem_u8(self.ip);

        self.ip += 1;

        value
    }

    pub fn decode_imm16(&mut self) -> u16 {
        let value = self.read_mem_u16(self.ip);

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
