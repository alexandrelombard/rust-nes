use crate::rom_file::RomFile;

use log::{debug, info, error, warn};

pub const NES_INTERNAL_RAM: u16 = 0x0000;
pub const NES_PPU_REGISTERS: u16 = 0x2000;
pub const NES_APU_IO_REGISTERS: u16 = 0x4000;
pub const NES_CARTRIDGE_SPACE: u16 = 0x4020;

pub const PPU_CTRL: u16     = 0x2000;
pub const PPU_MASK: u16     = 0x2001;
pub const PPU_STATUS: u16   = 0x2002;
pub const OAM_ADDR: u16     = 0x2003;
pub const OAM_DATA: u16     = 0x2004;
pub const PPU_SCROLL: u16   = 0x2005;
pub const PPU_ADDR: u16     = 0x2006;
pub const PPU_DATA: u16     = 0x2007;
pub const OAM_DMA: u16      = 0x4014;

pub const FLAG_NMI_ENABLE: u8               = 0b10000000;
pub const FLAG_PPU_MASTER_SLAVE: u8         = 0b01000000;
pub const FLAG_SPRITE_HEIGHT: u8            = 0b00100000;
pub const FLAG_BACKGROUND_TILE_SELECT: u8   = 0b00010000;
pub const FLAG_SPRITE_TILE_SELECT: u8       = 0b00001000;
pub const FLAG_INCREMENT_MODE: u8           = 0b00000100;
pub const FLAG_NAMETABLE_SELECT: u8         = 0b00000011;

pub enum AddressingMode {
    None,
    Immediate,
    Absolute,
    Implied,
    Accumulator,
    AbsoluteX,
    AbsoluteY,
    ZeroPageIndexed,
    ZeroPageX,
    ZeroPageY,
    Indirect,
    IndexedIndirect,
    IndirectIndexed,
    Relative,
}

pub struct Memory {
    data: [u8; 0xFFFF + 1],
}

impl Memory {
    pub fn new() -> Memory {
        let mem = Memory {
            data: [0; 0xFFFF + 1]
        };

        return mem;
    }

    /// Load the given ROM into the virtual memory
    pub fn load(&mut self, rom_file: &RomFile) {
        if rom_file.get_mapper_type() != 0 {
            error!("Unsupported mapper");   // For now on, we only support the mapper 0
        }

        self.data[0xc000..0xc000+0x4000].clone_from_slice(&rom_file.data[0x10..0x10+0x4000]);

        let prg_data = rom_file.prg_data();
        self.data[0x8000..0x8000+prg_data.len()].clone_from_slice(&prg_data);
    }

    /// Read the data at the given address
    pub fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    pub fn write(&mut self, address: u16, val: u8) {
        self.data[address as usize] = val;
    }

    // region Specific reading functions
    pub fn get_nmi_enable(&self) -> bool {
        self.read(PPU_CTRL) & FLAG_NMI_ENABLE != 0
    }

    pub fn set_nmi_enable(&mut self, nmi_enable: bool) {
        let val = self.read(PPU_CTRL);
        if nmi_enable {
            self.write(PPU_CTRL, val | FLAG_NMI_ENABLE);
        } else {
            self.write(PPU_CTRL, val & !FLAG_NMI_ENABLE);
        }
    }

    pub fn get_background_tile_select(&self) -> bool {
        self.read(PPU_CTRL) & FLAG_BACKGROUND_TILE_SELECT != 0
    }

    pub fn set_background_tile_select(&mut self, nmi_enable: bool) {
        let val = self.read(PPU_CTRL);
        if nmi_enable {
            self.write(PPU_CTRL, val | FLAG_BACKGROUND_TILE_SELECT);
        } else {
            self.write(PPU_CTRL, val & !FLAG_BACKGROUND_TILE_SELECT);
        }
    }

    pub fn get_background_pattern_table_address_value(&self) -> u16 {
        // TODO
        return 0
    }
    // endregion

    // region Memory addressing
    pub fn get_immediate(&self, address: u16) -> u16 {
        address + 1
    }

    pub fn get_zeropage(&self, address: u16) -> u16 {
        (address + 1) & 0x00ff
    }

    pub fn get_zeropage_x(&self, address: u16, x: u8) -> u16 {
        (address + x as u16) & 0x00ff
    }

    pub fn get_zeropage_y(&self, address: u16, y: u8) -> u16 {
        (address + y as u16) & 0x00ff
    }

    pub fn get_absolute(&self, address: u16) -> u16 {
        self.read(address + 1) as u16 + ((self.read(address + 2) as u16) << 8)
    }

    pub fn get_absolute_x(&self, address: u16, x: u8) -> u16 {
        self.read(address + 1 + x as u16) as u16 + ((self.read(address + 2 + x as u16) as u16) << 8)
    }

    pub fn get_absolute_y(&self, address: u16, y: u8) -> u16 {
        self.read(address + 1 + y as u16) as u16 + ((self.read(address + 2 + y as u16) as u16) << 8)
    }

    pub fn get_relative(&self, address: u16) -> u16 {
        let offset = self.read(address + 1) as i8 as i16;
        let address_with_offset = address as i16 + offset;
        address_with_offset as u16
    }

    pub fn get_indirect_x(&self, address: u16, x: u8) -> u16 {
        self.read((address + x as u16) % 256) as u16 +
            (self.read( (address + x as u16 + 1) % 256) as u16 * 256)
    }

    pub fn get_indirect_y(&self, address: u16, y: u8) -> u16 {
        self.read(address) as u16 +
            (self.read((address + 1) % 256) as u16 * 256 + y as u16)
    }
    // endregion
}
