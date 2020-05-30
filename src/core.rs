use crate::instruction::{Instruction, ExtendedInstruction, Cond, Operand, Reg8, Reg16};
use crate::bus::Bus;

pub struct Core {
    pc: u16,
    sp: u16,
    reg_a: u8,
    reg_b: u8,
    reg_c: u8,
    reg_d: u8,
    reg_e: u8,
    reg_f: u8,
    reg_h: u8,
    reg_l: u8,
    // rom: Vec<u8>,
    // ram: Vec<u8>,
    interrupts_enabled: bool,
    // mapper: Mapper,
    bus: Bus,
}

impl Core {
    pub fn new(bus: Bus) -> Self {
        Self {
            pc: 0x100,
            sp: 0xFFFE,
            reg_a: 0x11,
            reg_b: 0x00,
            reg_c: 0x00,
            reg_d: 0xFF,
            reg_e: 0x56,
            reg_h: 0x00,
            reg_l: 0x0D,
            reg_f: 0x80,
            // rom,
            // ram: vec![0; TOTAL_RAM_SIZE as usize],
            interrupts_enabled: true,
            // mapper: Mapper::Rom,
            bus,
        }
    }

    pub fn pc(&self) -> u16 {
        self.pc
    }

    pub fn current_instruction(&self) -> Instruction {
        let code = self.peek_mem_u8(self.pc);
        Instruction::decode(code)
    }

    pub fn current_extended_instruction(&self) -> ExtendedInstruction {
        let code = self.peek_mem_u8(self.pc + 1);
        ExtendedInstruction::decode(code)
    }

    pub fn step(&mut self) {
        let instruction = self.current_instruction();
        self.execute(instruction);
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

    pub fn flag_z(&self) -> bool {
        self.reg_f & (1 << 7) != 0
    }

    pub fn set_flag_z(&mut self, set: bool) {
        if set {
            self.reg_f |=   1 << 7;
        } else {
            self.reg_f &= !(1 << 7);
        }
    }

    pub fn flag_n(&self) -> bool {
        self.reg_f & (1 << 6) != 0
    }

    pub fn set_flag_n(&mut self, set: bool) {
        if set {
            self.reg_f |=   1 << 6;
        } else {
            self.reg_f &= !(1 << 6);
        }
    }

    pub fn flag_h(&self) -> bool {
        self.reg_f & (1 << 5) != 0
    }

    pub fn set_flag_h(&mut self, set: bool) {
        if set {
            self.reg_f |=   1 << 5;
        } else {
            self.reg_f &= !(1 << 5);
        }
    }

    pub fn flag_c(&self) -> bool {
        self.reg_f & (1 << 4) != 0
    }

    pub fn set_flag_c(&mut self, set: bool) {
        if set {
            self.reg_f |=   1 << 4;
        } else {
            self.reg_f &= !(1 << 4);
        }
    }

    fn reg_hl_postincrement(&mut self) -> u16 {
        let value = self.reg_hl();
        self.set_reg_hl(value.wrapping_add(1));
        value
    }

    fn reg_hl_postdecrement(&mut self) -> u16 {
        let value = self.reg_hl();
        self.set_reg_hl(value.wrapping_sub(1));
        value
    }

    pub fn print_state(&self) {
        eprintln!("af= {af:04X}", af = self.reg_af());
        eprintln!("bc= {bc:04X}", bc = self.reg_bc());
        eprintln!("de= {de:04X}", de = self.reg_de());
        eprintln!("hl= {hl:04X}", hl = self.reg_hl());
        eprintln!("sp= {sp:04X}", sp = self.sp);
        eprintln!("pc= {sp:04X}", sp = self.pc);
        eprintln!("nn= {nn:04X}", nn = self.peek_mem_u16(self.pc+1));
        eprintln!("ZNHC");
        eprintln!("{:04b}", self.reg_f >> 4);

        let instruction = self.current_instruction();

        eprintln!("→ {:?}", instruction);

        if instruction == Instruction::Extended {
            eprintln!("→ {:?}", self.current_extended_instruction());
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        self.pc += 1;

        match instruction {
            Instruction::Nop => {},
            Instruction::Jr(cond, offset) => self.execute_jr(cond, offset),
            Instruction::Jp(cond, addr) => self.execute_jp(cond, addr),
            Instruction::Call(cond, addr) => self.execute_call(cond, addr),
            Instruction::Di => self.execute_di(),
            Instruction::Ld(target, source) => self.execute_ld(target, source),
            Instruction::Ldh(target, source) => self.execute_ldh(target, source),
            Instruction::Push(source) => self.execute_push(source),
            Instruction::Pop(target) => self.execute_pop(target),
            Instruction::Cp(source) => self.execute_cp(source),
            Instruction::Add(target, value) => self.execute_add(target, value),
            Instruction::Adc(value) => self.execute_add_carry(value),
            Instruction::Sub(value) => self.execute_sub(value),
            Instruction::Inc(target) => self.execute_inc(target),
            Instruction::Dec(target) => self.execute_dec(target),
            Instruction::Or(value) => self.execute_or(value),
            Instruction::Xor(value) => self.execute_xor(value),
            Instruction::And(value) => self.execute_and(value),
            Instruction::Rra => self.execute_rotate_right_a(),
            Instruction::Rla => self.execute_rotate_left_a(),
            Instruction::Ret(cond) => self.execute_ret(cond),
            Instruction::Extended => {
                let code = self.read_mem_u8(self.pc);
                let instruction = ExtendedInstruction::decode(code);
                self.execute_extended(instruction);
            }
            _ => {
                self.pc -= 1;
                self.print_state();
                unimplemented!("execute: {:?}", instruction)
            },
        }
    }

    fn execute_add(&mut self, target: Operand, value: Operand) {
        let a = self.load_operand(target);
        let b = self.load_operand(value);

        match (a, b) {
            (Value::U8(a), Value::U8(b)) => {
                let (value, carry) = a.overflowing_add(b);
                let (_, half_carry) = (a << 4).overflowing_add(b << 4);

                self.set_flag_z(value == 0);
                self.set_flag_n(false);
                self.set_flag_h(half_carry);
                self.set_flag_c(carry);

                self.store_operand_u8(target, value);
            },
            (Value::U16(a), Value::U16(b)) => {
                let (value, carry) = a.overflowing_add(b);
                let (_, half_carry) = (a << 8).overflowing_add(b << 8);

                self.set_flag_z(value == 0);
                self.set_flag_n(false);
                self.set_flag_h(half_carry);
                self.set_flag_c(carry);

                self.store_operand_u16(target, value);
            },
            _ => {
                unimplemented!("execute_add: {:?} + {:?}", target, value);
            }
        }
    }

    fn execute_add_carry(&mut self, operand: Operand) {
        let operand = self.load_u8_operand(operand);
        let mut total_carry = false;
        let mut total_half_carry = false;

        // Add operand
        {
            let (result, carry) = self.reg_a.overflowing_add(operand);
            total_carry |= carry;

            let (_, half_carry) = (self.reg_a << 4).overflowing_add(operand << 4);
            total_half_carry |= half_carry;

            self.reg_a = result;
        }

        // Add 1
        {
            let (result, carry) = self.reg_a.overflowing_add(1);
            total_carry |= carry;

            let (_, half_carry) = (self.reg_a << 4).overflowing_add(1 << 4);
            total_half_carry |= half_carry;

            self.reg_a = result;
        }

        self.set_flag_z(self.reg_a == 0);
        self.set_flag_n(false);
        self.set_flag_h(total_half_carry);
        self.set_flag_c(total_carry);
    }

    fn execute_sub(&mut self, value: Operand) {
        let a = self.reg_a;
        let b = self.load_u8_operand(value);
        let (value, carry) = a.overflowing_sub(b);
        let (_, half_carry) = (a << 4).overflowing_sub(b << 4);

        self.set_flag_z(value == 0);
        self.set_flag_n(false);
        self.set_flag_h(half_carry);
        self.set_flag_c(carry);

        self.reg_a = value;
    }

    fn execute_or(&mut self, value: Operand) {
        let value = self.load_u8_operand(value);
        self.reg_a |= value;

        self.set_flag_z(self.reg_a == 0);
        self.set_flag_n(false);
        self.set_flag_h(false);
        self.set_flag_c(false);
    }

    fn execute_xor(&mut self, value: Operand) {
        let value = self.load_u8_operand(value);
        self.reg_a ^= value;

        self.set_flag_z(self.reg_a == 0);
        self.set_flag_n(false);
        self.set_flag_h(false);
        self.set_flag_c(false);
    }

    fn execute_and(&mut self, value: Operand) {
        let value = self.load_u8_operand(value);
        self.reg_a &= value;

        self.set_flag_z(self.reg_a == 0);
        self.set_flag_n(false);
        self.set_flag_h(true);
        self.set_flag_c(false);
    }

    fn execute_ret(&mut self, cond: Cond) {
        let cond = self.evaluate_cond(cond);

        if cond {
            let addr = self.pop_u16();
            self.pc = addr;
        }
    }

    fn execute_rotate_right_a(&mut self) {
        self.execute_rotate_right(Operand::Reg8(Reg8::A));
        self.set_flag_z(false);
    }

    fn execute_rotate_left_a(&mut self) {
        self.execute_rotate_left(Operand::Reg8(Reg8::A));
        self.set_flag_z(false);
    }

    fn execute_extended(&mut self, instruction: ExtendedInstruction) {
        self.pc += 1;

        match instruction {
            ExtendedInstruction::Srl(target) => self.execute_shift_right_logical(target),
            ExtendedInstruction::Rr(target) => self.execute_rotate_right(target),
            ExtendedInstruction::Bit(bit, target) => self.execute_bit(bit, target),
            ExtendedInstruction::Rl(target) => self.execute_rotate_left(target),
            _ => {
                self.pc -= 2;
                self.print_state();
                unimplemented!("execute_extended: {:?}", instruction)
            },
        }
    }

    fn execute_shift_right_logical(&mut self, target: Operand) {
        let value = self.load_u8_operand(target);
        let carry = value & 1 == 1;
        let value = value >> 1;

        self.set_flag_z(value == 0);
        self.set_flag_n(false);
        self.set_flag_h(false);
        self.set_flag_c(carry);
        self.store_operand_u8(target, value);
    }

    fn execute_rotate_right(&mut self, target: Operand) {
        let value = self.load_u8_operand(target);
        let carry = value & 1 == 1;
        let value = value | self.flag_c() as u8;
        let value = value.rotate_right(1);

        self.set_flag_z(value == 0);
        self.set_flag_n(false);
        self.set_flag_h(false);
        self.set_flag_c(carry);
        self.store_operand_u8(target, value);
    }

    pub fn write_mem_u8(&mut self, addr: u16, value: u8) {
        // println!("${:04X} = {:02X}", addr, value);

        self.bus.write(addr, value);
    }

    pub fn write_mem_u16(&mut self, addr: u16, value: u16) {
        // println!("${:04X} = {:04X}", addr, value);

        let lo = value as u8;
        let hi = (value >> 8) as u8;

        self.write_mem_u8(addr    , lo);
        self.write_mem_u8(addr + 1, hi);
    }

    pub fn peek_mem_u8(&self, addr: u16) -> u8 {
        // TODO: either remove peeking or add peeking to bus
        // self.mapper.peek_u8(&self.bus, &self.rom, &self.ram, addr)
        self.bus.read(addr)
    }

    fn read_mem_u8(&self, addr: u16) -> u8 {
        // self.mapper.read_u8(&self.bus, &self.rom, &self.ram, addr)
        self.bus.read(addr)
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

        self.set_flag_z(self.reg_a == value);
        self.set_flag_n(true);
        self.set_flag_h(self.reg_a & 0xF > value & 0xF);
        self.set_flag_c(self.reg_a < value);
    }

    pub fn execute_jr(&mut self, cond: Cond, offset: Operand) {
        let cond = self.evaluate_cond(cond);
        let offset = self.load_u8_operand(offset) as i8;

        if cond {
            if offset.is_positive() {
                self.pc += offset as u16;
            } else {
                self.pc -= offset.abs() as u16;
            }
        }
    }

    pub fn execute_jp(&mut self, cond: Cond, addr: Operand) {
        let cond = self.evaluate_cond(cond);
        let addr = self.load_u16_operand(addr);

        if cond {
            self.pc = addr;
        }
    }

    pub fn execute_call(&mut self, cond: Cond, addr: Operand) {
        let cond = self.evaluate_cond(cond);
        let addr = self.load_u16_operand(addr);

        if cond {
            self.push_u16(self.pc);
            self.pc = addr;
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
            (Operand::Reg8(Reg8::A), Operand::Imm8Ref) => {
                let ptr = self.decode_imm8() as u16 + 0xFF00;
                let value = self.read_mem_u8(ptr);
                self.reg_a = value;
            }
            (Operand::Imm8Ref, Operand::Reg8(Reg8::A)) => {
                let value = self.load_u8_operand(source);
                let ptr = self.decode_imm8() as u16 + 0xFF00;

                self.write_mem_u8(ptr, value);
            },
            _ => {
                self.pc -= 1;
                self.print_state();
                unimplemented!("execute_ldh: {:?} <- {:?}", target, source)
            },
        }
    }

    pub fn execute_push(&mut self, source: Reg16) {
        let value = match source {
            Reg16::AF => self.reg_af(),
            Reg16::BC => self.reg_bc(),
            Reg16::DE => self.reg_de(),
            Reg16::HL => self.reg_hl(),
            _ => unimplemented!("execute_push: {:?}", source),
        };

        self.push_u16(value);
    }

    pub fn execute_pop(&mut self, target: Reg16) {
        let value = self.pop_u16();

        match target {
            Reg16::AF => self.set_reg_af(value),
            Reg16::BC => self.set_reg_bc(value),
            Reg16::DE => self.set_reg_de(value),
            Reg16::HL => self.set_reg_hl(value),
            _ => unimplemented!("execute_pop: {:?}", target),
        }
    }

    pub fn execute_inc(&mut self, target: Operand) {
        match self.load_operand(target) {
            Value::U8(value) => {
                let half_carry = value & 0xF == 0xF;
                let value = value.wrapping_add(1);

                self.set_flag_z(value == 0);
                self.set_flag_n(false);
                self.set_flag_h(half_carry);
                self.store_operand_u8(target, value)
            },
            Value::U16(value) => {
                let half_carry = value & 0xFF == 0xFF;
                let value = value.wrapping_add(1);

                self.set_flag_z(value == 0);
                self.set_flag_n(false);
                self.set_flag_h(half_carry);
                self.store_operand_u16(target, value)
            },
        }
    }

    pub fn execute_dec(&mut self, target: Operand) {
        match self.load_operand(target) {
            Value::U8(value) => {
                let value = value.wrapping_sub(1);
                let half_carry = value & 0xF == 0xF;
                self.set_flag_z(value == 0);
                self.set_flag_n(true);
                self.set_flag_h(half_carry);
                self.store_operand_u8(target, value)
            },
            Value::U16(value) => self.store_operand_u16(target, value - 1),
        }
    }

    pub fn execute_bit(&mut self, bit: u8, target: Operand) {
        let value = self.load_u8_operand(target);
        let res = value & (1 << bit);
        self.set_flag_z(res == 0);
        self.set_flag_n(false);
        self.set_flag_h(true);
    }

    pub fn execute_rotate_left(&mut self, target: Operand) {
        let value = self.load_u8_operand(target);
        let carry = (value & 0x80) != 0;

        let res = value << 1 | self.flag_c() as u8;
        self.store_operand_u8(target, res);
        self.set_flag_z(res == 0);
        self.set_flag_n(false);
        self.set_flag_h(false);
        self.set_flag_c(carry);
    }

    pub fn push_u16(&mut self, value: u16) {
        self.write_mem_u8(self.sp, (value >> 8) as u8);
        self.sp -= 1;
        self.write_mem_u8(self.sp, value as u8);
        self.sp -= 1;
    }

    pub fn pop_u16(&mut self) -> u16 {
        self.sp += 1;
        let vh = self.read_mem_u8(self.sp) as u16;
        self.sp += 1;
        let vl = (self.read_mem_u8(self.sp) as u16) << 8;

        vl | vh
    }

    pub fn evaluate_cond(&self, cond: Cond) -> bool {
        match cond {
            Cond::Always => true,
            Cond::ZSet => self.flag_z(),
            Cond::ZReset => !self.flag_z(),
            Cond::CSet => self.flag_c(),
            Cond::CReset => !self.flag_c(),
        }
    }

    pub fn load_operand(&mut self, source: Operand) -> Value {
        match source {
            Operand::Imm8 => Value::U8(self.decode_imm8()),
            Operand::Imm16 => Value::U16(self.decode_imm16()),
            Operand::Reg8(reg8) => Value::U8(self.load_u8_register(reg8)),
            Operand::Reg16(reg16) => Value::U16(self.load_u16_register(reg16)),
            Operand::RegRef16(reg16) => {
                let addr = self.load_u16_register(reg16);
                Value::U8(self.read_mem_u8(addr))
            },
            Operand::Imm16Ref => {
                let addr = self.decode_imm16();
                Value::U8(self.read_mem_u8(addr))
            },
            _ => unimplemented!("load_operand: {:?}", source),
        }
    }

    fn load_u8_register(&mut self, reg8: Reg8) -> u8 {
        match reg8 {
            Reg8::A => self.reg_a,
            Reg8::B => self.reg_b,
            Reg8::C => self.reg_c,
            Reg8::D => self.reg_d,
            Reg8::E => self.reg_e,
            Reg8::H => self.reg_h,
            Reg8::L => self.reg_l,
        }
    }

    fn load_u16_register(&mut self, reg16: Reg16) -> u16 {
        match reg16 {
            Reg16::AF => self.reg_af(),
            Reg16::BC => self.reg_bc(),
            Reg16::DE => self.reg_de(),
            Reg16::HL => self.reg_hl(),
            Reg16::HLInc => self.reg_hl_postincrement(),
            Reg16::HLDec => self.reg_hl_postdecrement(),
            Reg16::SP => self.sp,
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
            Operand::RegRef16(reg16) => {
                let addr = self.load_u16_register(reg16);
                self.write_mem_u8(addr, value);
            },
            Operand::Imm16Ref => {
                let ptr = self.decode_imm16();
                self.write_mem_u8(ptr, value);
            },
            Operand::RegRef8(reg) => {
                let addr = 0xFF00 + self.load_u8_register(reg) as u16;
                self.write_mem_u8(addr, value)
            }
            _ => unimplemented!("store_operand_u8: {:?} <- {:?}", target, value)
        }
    }

    pub fn store_operand_u16(&mut self, target: Operand, value: u16) {
        match target {
            Operand::Reg16(Reg16::BC) => self.set_reg_bc(value),
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
            Operand::Reg8(Reg8::B) => self.reg_b,
            Operand::Reg8(Reg8::C) => self.reg_c,
            Operand::Reg8(Reg8::D) => self.reg_d,
            Operand::Reg8(Reg8::E) => self.reg_e,
            Operand::Reg8(Reg8::H) => self.reg_h,
            Operand::Reg8(Reg8::L) => self.reg_l,
            Operand::RegRef16(reg16) => {
                let addr = self.load_u16_register(reg16);
                self.read_mem_u8(addr)
            },
            _ => unimplemented!("load_u8_operand: {:?}", operand),
        }
    }

    pub fn load_u16_operand(&mut self, operand: Operand) -> u16 {
        match operand {
            Operand::Imm16 => self.decode_imm16(),
            Operand::RegRef16(reg16) => {
                let addr = self.load_u16_register(reg16);
                self.read_mem_u16(addr)
            },
            _ => unimplemented!("load_u16_operand: {:?}", operand),
        }
    }

    pub fn decode_imm8(&mut self) -> u8 {
        let value = self.read_mem_u8(self.pc);

        self.pc += 1;

        value
    }

    pub fn decode_imm16(&mut self) -> u16 {
        let value = self.read_mem_u16(self.pc);

        self.pc += 2;

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
