
pub enum Mapper {
    Rom
}

impl Mapper {
    pub fn peek_u8(&self, rom: &[u8], ram: &[u8], addr: u16) -> u8 {
        match &self {
            Mapper::Rom => read_u8_rom(rom, ram, addr)
        }
    }

    pub fn read_u8(&mut self, rom: &[u8], ram: &[u8], addr: u16) -> u8 {
        match &self {
            Mapper::Rom => read_u8_rom(rom, ram, addr)
        }
    }

    pub fn write_u8(&mut self, ram: &mut [u8], addr: u16, value: u8) {
        match &self {
            Mapper::Rom => write_u8_rom(ram, addr, value)
        }
    }
}

const LO_RAM_SIZE: u16 = 8 * 1024;
const HI_RAM_SIZE: u16 = 0x7F;
pub const TOTAL_RAM_SIZE: u16 = LO_RAM_SIZE + HI_RAM_SIZE;
const ROM_START: u16 = 0x0000;
const ROM_END: u16 = 0x7FFF;
const LO_RAM_START: u16 = 0xC000;
const LO_RAM_END: u16 = 0xDFFF;
const IO_START: u16 = 0xFF00;
const IO_END: u16 = 0xFF4B;
const HI_RAM_START: u16 = 0xFF80;
const HI_RAM_END: u16 = 0xFFFE;
const INTERRUPT_ENABLE_REGISTER: u16 = 0xFFFF;

fn read_u8_rom(rom: &[u8], ram: &[u8], addr: u16) -> u8 {
    match addr {
        ROM_START ..= ROM_END => rom.get(addr as usize).cloned().unwrap_or(0xFF),
        LO_RAM_START ..= LO_RAM_END => {
            let addr = addr - LO_RAM_START;
            ram[addr as usize]
        },
        IO_START ..= IO_END => {
            println!("TODO: I/O read @ {:04X}", addr);
            0
        },
        HI_RAM_START ..= HI_RAM_END => {
            let addr = addr - HI_RAM_START + LO_RAM_SIZE;
            ram[addr as usize]
        },
        INTERRUPT_ENABLE_REGISTER => {
            println!("TODO: IF register read @ {:04X}", addr);
            0
        },
        _ => unimplemented!("Mapper::read_u8 @ {:04X}", addr),
    }
}

fn write_u8_rom(ram: &mut [u8], addr: u16, value: u8) {
    match addr {
        ROM_START ..= ROM_END => {},
        LO_RAM_START ..= LO_RAM_END => {
            let addr = addr - LO_RAM_START;
            ram[addr as usize] = value;
        },
        IO_START ..= IO_END => {
            println!("TODO: I/O write @ {:04X}", addr);
        },
        HI_RAM_START ..= HI_RAM_END => {
            let addr = addr - HI_RAM_START + LO_RAM_SIZE;
            ram[addr as usize] = value;
        },
        INTERRUPT_ENABLE_REGISTER => {
            println!("TODO: IF register read @ {:04X}", addr);
        },
        _ => unimplemented!("Mapper::write_u8 @ {:04X}", addr),
    }
}