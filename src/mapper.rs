
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

fn read_u8_rom(rom: &[u8], ram: &[u8], addr: u16) -> u8 {
    match addr {
        0x0000..=0x7FFF => rom.get(addr as usize).cloned().unwrap_or(0xFF),
        0xC000..=0xDFFF => {
            let addr = addr - 0xC000;
            ram[addr as usize]
        },
        0xFF00..=0xFF7F => {
            println!("TODO: I/O read @ {:04X}", addr);
            0
        },
        0xFF80..=0xFFFE => {
            let addr = addr - 0xFF80 + 0xE000;
            ram[addr as usize]
        },
        0xFFFF => {
            println!("TODO: IF register read @ {:04X}", addr);
            0
        },
        _ => unimplemented!("Mapper::read_u8 @ {:04X}", addr),
    }
}

fn write_u8_rom(ram: &mut [u8], addr: u16, value: u8) {
    match addr {
        0x0000..=0x7FFF => {},
        0xC000..=0xDFFF => {
            let addr = addr - 0xC000;
            ram[addr as usize] = value;
        },
        0xFF00..=0xFF7F => {
            println!("TODO: I/O write @ {:04X}", addr);
        },
        0xFF80..=0xFFFE => {
            let addr = addr - 0xFF80 + 8 * 1024;
            ram[addr as usize] = value;
        },
        0xFFFF => {
            println!("TODO: IF register read @ {:04X}", addr);
        },
        _ => unimplemented!("Mapper::write_u8 @ {:04X}", addr),
    }
}