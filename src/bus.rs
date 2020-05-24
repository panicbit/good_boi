use parking_lot::Mutex;
use crate::cartridge::Cartridge;
use crate::constants::*;

pub struct Bus {
    serial: Mutex<Serial>,
    cartridge: Mutex<Cartridge>,
    low_ram: Mutex<Ram>,
    unimplemented_warning: Mutex<UnimplementedWarning>,
}

impl Bus {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            serial: Mutex::new(Serial::default()),
            cartridge: Mutex::new(cartridge),
            low_ram: Mutex::new(Ram::new(LO_RAM_SIZE)),
            unimplemented_warning: Mutex::new(UnimplementedWarning),
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            // TODO: map to boot rom initially
            0x0000..=0x7FFF => self.cartridge.read(addr),
            0xC000..=0xDFFF => self.low_ram.read(addr - 0xC000),
            0xE000..=0xFDFF => self.low_ram.read(addr - 0xE000),
            VRAM_START..=VRAM_END => self.unimplemented_warning.read(addr),
            0xFF01..=0xFF02 => self.serial.read(addr),
            0xFF07..=0xFFFF => self.unimplemented_warning.read(addr),
            _ => panic!("Invalid read @ 0x{:02X}", addr),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x7FFF => self.cartridge.write(addr, value),
            0xC000..=0xDFFF => self.low_ram.write(addr - 0xC000, value),
            0xE000..=0xFDFF => self.low_ram.write(addr - 0xE000, value),
            VRAM_START..=VRAM_END => self.unimplemented_warning.write(addr, value),
            0xFF01..=0xFF02 => self.serial.write(addr, value),
            0xFF07..=0xFFFF => self.unimplemented_warning.write(addr, value),
            _ => panic!("Invalid write @ 0x{:02X}", addr),
        }
    }
}

pub trait Device: 'static {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, value: u8);
}

impl<T: Device> Device for Mutex<T> {
    fn read(&self, addr: u16) -> u8 {
        self.lock().read(addr)
    }

    fn write(&mut self, addr: u16, value: u8) {
        self.lock().write(addr, value)
    }
}

pub struct Ram {
    data: Vec<u8>,
}

impl Ram {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0x11; size],
        }
    }
}

impl Device for Ram {
    fn read(&self, addr: u16) -> u8 {
        self.data.get(addr as usize).copied().unwrap()
    }

    fn write(&mut self, addr: u16, value: u8) {
        if let Some(byte) = self.data.get_mut(addr as usize) {
            *byte = value;
        } else {
            panic!("Invalid write past end of RAM");
        }
    }
}

#[derive(Default)]
struct Serial {
    value: u8,
}

impl Device for Serial {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0xFF01 => self.value,
            0xFF02 => unimplemented!("Read from SC"),
            _ => panic!(),
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0xFF01 => self.value = value,
            0xFF02 => {
                println!("SERIAL DEBUG OUTPUT: {:?}", self.value as char);
            },
            _ => panic!(),
        }
    }
}

struct UnimplementedWarning;

impl Device for UnimplementedWarning {
    fn read(&self, addr: u16) -> u8 {
        eprintln!("Unimplemented read @ 0x{:02X}", addr);
        0
    }

    fn write(&mut self, addr: u16, value: u8) {
        eprintln!("Unimplemented write of 0x{:02X} @ 0x{:02X}", value, addr)
    }
}
