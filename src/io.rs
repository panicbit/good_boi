use std::sync::{Mutex, Arc};
use crate::constants::{
    VRAM_START, VRAM_END,
    IO_REG_LY,
};

pub struct Bus {
    pub video: Arc<Device>,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            video: Arc::new(Mutex::new(DummyVideo::new())),
        }
    }
}

pub trait Device {
    fn peek_u8(&self, addr: u16) -> u8;
    fn read_u8(&self, addr: u16) -> u8;
    fn write_u8(&self, addr: u16, value: u8);
}

impl<T: Device + ?Sized> Device for Arc<T> {
    fn peek_u8(&self, addr: u16) -> u8 {
        self.as_ref().peek_u8(addr)
    }

    fn read_u8(&self, addr: u16) -> u8 {
        self.as_ref().read_u8(addr)
    }

    fn write_u8(&self, addr: u16, value: u8) {
        self.as_ref().write_u8(addr, value)
    }
}

struct DummyVideo {
    ram: Vec<u8>,
}

impl DummyVideo {
    pub fn new() -> Self {
        Self {
            ram: vec![0; 0x2000],
        }
    }
}

impl Device for Mutex<DummyVideo> {
    fn peek_u8(&self, addr: u16) -> u8 {
        let this = self.lock().unwrap();
        match addr {
            VRAM_START ..= VRAM_END => this.ram[addr as usize],
            IO_REG_LY => {
                println!("TODO: IO_REG_LY peek");
                0x99
            },
            _ => {
                println!("TODO: video peek @ {:04X}", addr);
                0
            },
        }
    }

    fn read_u8(&self, addr: u16) -> u8 {
        let this = self.lock().unwrap();
        match addr {
            VRAM_START ..= VRAM_END => this.ram[addr as usize],
            IO_REG_LY => {
                println!("TODO: IO_REG_LY read");
                0x94
            },
            _ => {
                println!("TODO: video read @ {:04X}", addr);
                0
            },
        }
    }

    fn write_u8(&self, addr: u16, value: u8) {
        let mut this = self.lock().unwrap();
        match addr {
            VRAM_START ..= VRAM_END => this.ram[addr as usize] = value,
            IO_REG_LY => println!("Invalid write to IO_REG_LY"),
            _ => {
                println!("TODO: video write @ {:04X}", addr);
            },
        }
    }
}
