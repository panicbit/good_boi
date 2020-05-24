use failure::Fallible;
use crate::bus::Device;

const MBC1_BANK_SIZE_16K: usize = 16 * 1024;

pub struct Cartridge {
    mapper: Mapper,
}

impl Cartridge {
    pub fn load(rom: impl Into<Vec<u8>>) -> Fallible<Self> {
        eprintln!("TODO: detect correct cartridge type");

        Ok(Self {
            mapper: Mapper::MBC1(MBC1::new(rom.into())),
        })
    }
}

impl Device for Cartridge {
    fn read(&self, addr: u16) -> u8 {
        match &self.mapper {
            Mapper::MBC1(m) => m.read(addr),
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match &mut self.mapper {
            Mapper::MBC1(m) => m.write(addr, value),
        }
    }
}

enum Mapper {
    MBC1(MBC1),
}

struct MBC1 {
    rom_bank: usize,
    rom: Vec<u8>,
}

impl MBC1 {
    pub fn new(rom: Vec<u8>) -> Self {
        Self {
            rom_bank: 1,
            rom,
        }
    }

    pub fn select_rom_bank(&mut self, mut bank: u8) {
        // MBC1 only cares for the lowest 1 bytes
        bank &= 0b1_1111;

        self.rom_bank = match bank {
            0 => 1,
            20 => 21,
            40 => 41,
            60 => 61,
            _ => bank as usize,
        };
    }

    /// Read banked rom at base address 0x4000
    pub fn read_rom(&self, addr: u16) -> u8 {
        let addr = addr as usize - 0x4000;
        let start = self.rom_bank * MBC1_BANK_SIZE_16K;
        let index = start + addr;

        self.rom.get(index).copied().unwrap_or(0)
    }
}

impl Device for MBC1 {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3FFF => self.rom.get(addr as usize).copied().unwrap_or(0),
            0x4000..=0x7FFF => self.read_rom(addr),
            _ => unimplemented!("MBC1 read @ 0x{:02X}", addr),
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            _ => unimplemented!("MBC1 write 0x{:02X} @ 0x{:02X}", value, addr),
        }
    }
}
