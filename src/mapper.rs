use crate::constants::*;
use crate::io::{Bus, Device};

pub enum Mapper {
    Rom
}

impl Mapper {
    pub fn peek_u8(&self, bus: &Bus, rom: &[u8], ram: &[u8], addr: u16) -> u8 {
        match &self {
            Mapper::Rom => read_u8_rom(bus, rom, ram, addr)
        }
    }

    pub fn read_u8(&mut self, bus: &Bus, rom: &[u8], ram: &[u8], addr: u16) -> u8 {
        match &self {
            Mapper::Rom => read_u8_rom(bus, rom, ram, addr)
        }
    }

    pub fn write_u8(&mut self, bus: &Bus, ram: &mut [u8], addr: u16, value: u8) {
        match &self {
            Mapper::Rom => write_u8_rom(bus, ram, addr, value)
        }
    }
}

fn read_u8_rom(bus: &Bus, rom: &[u8], ram: &[u8], addr: u16) -> u8 {
    match addr {
        ROM_START ..= ROM_END => rom.get(addr as usize).cloned().unwrap_or(0xFF),
        VRAM_START ..= VRAM_END => bus.video.read_u8(addr),
        LO_RAM_START ..= LO_RAM_END => {
            let addr = addr - LO_RAM_START;
            ram[addr as usize]
        },
        IO_REG_LY => bus.video.read_u8(addr),
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

fn write_u8_rom(bus: &Bus, ram: &mut [u8], addr: u16, value: u8) {
    match addr {
        ROM_START ..= ROM_END => {},
        VRAM_START ..= VRAM_END => bus.video.write_u8(addr, value),
        LO_RAM_START ..= LO_RAM_END => {
            let addr = addr - LO_RAM_START;
            ram[addr as usize] = value;
        },
        IO_REG_LY => bus.video.write_u8(addr, value),
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