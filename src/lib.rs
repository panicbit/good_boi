pub use self::core::Core;
pub use self::cartridge::Cartridge;
pub use self::bus::Bus;

mod instruction;
mod core;
mod bus;
mod cartridge;

pub mod constants {
    pub const LO_RAM_SIZE: usize = 8 * 1024;
    pub const HI_RAM_SIZE: usize = 0x7F;
    pub const TOTAL_RAM_SIZE: usize = LO_RAM_SIZE + HI_RAM_SIZE;
    pub const ROM_START: u16 = 0x0000;
    pub const ROM_END: u16 = 0x7FFF;
    pub const VRAM_START: u16 = 0x8000;
    pub const VRAM_END: u16 = 0x9FFF;
    pub const VRAM_SIZE: usize = 0x2000;
    pub const LO_RAM_START: u16 = 0xC000;
    pub const LO_RAM_END: u16 = 0xDFFF;
    pub const IO_START: u16 = 0xFF00;
    pub const IO_REG_LY: u16 = 0xFF44;
    pub const IO_END: u16 = 0xFF4B;
    pub const HI_RAM_START: u16 = 0xFF80;
    pub const HI_RAM_END: u16 = 0xFFFE;
    pub const INTERRUPT_ENABLE_REGISTER: u16 = 0xFFFF;
}
