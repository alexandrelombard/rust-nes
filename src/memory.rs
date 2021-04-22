use crate::rom_file::RomFile;

pub const NES_INTERNAL_RAM: u16 = 0x0000;
pub const NES_PPU_REGISTERS: u16 = 0x2000;
pub const NES_APU_IO_REGISTERS: u16 = 0x4000;
pub const NES_CARTRIDGE_SPACE: u16 = 0x4020;

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
    data: [u8; 0xFFFF + 1]
}

impl Memory {
    pub fn new() -> Memory {
        let mem = Memory {
            data: [0; 0xFFFF + 1]
        };

        return mem;
    }

    /// Load the given ROM into the virtual memory
    pub fn load(&mut self, rom_file: RomFile) {
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
