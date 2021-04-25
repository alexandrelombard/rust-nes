use crate::memory::Memory;
use crate::rom_file::RomFile;

// https://wiki.nesdev.com/w/index.php/PPU_registers#Status_.28.242002.29_.3C_read
// https://emudev.de/nes-emulator/cartridge-loading-pattern-tables-and-ppu-registers/

const PPU_CTRL: u16     = 0x2000;
const PPU_MASK: u16     = 0x2001;
const PPU_STATUS: u16   = 0x2002;
const OAM_ADDR: u16     = 0x2003;
const OAM_DATA: u16     = 0x2004;
const PPU_SCROLL: u16   = 0x2005;
const PPU_ADDR: u16     = 0x2006;
const PPU_DATA: u16     = 0x2007;
const OAM_DMA: u16      = 0x4014;


pub struct Ppu {
    cycles: u32,
    scanline: u32,
    vram: [u8; 0x4000]
}

impl Ppu {
    pub fn new() -> Ppu {
        return Ppu {
            cycles: 0,
            scanline: 0,
            vram: [0; 0x4000]
        };
    }

    pub fn load(&mut self, rom_file: &RomFile) {
        let chr_data = rom_file.chr_data();
        self.vram[0x0000..0x2000].clone_from_slice(&chr_data);
    }

    pub fn read_vram(&self, address: u16) -> u8 {
        return self.vram[address as usize];
    }

    pub fn write_vram(&mut self, address: u16, val: u8) {
        self.vram[address as usize] = val;
    }

    pub fn step(&mut self, memory: &mut Memory) {
        self.cycles += 1;

        if self.cycles > 340 {
            self.cycles -= 341;
            self.scanline += 1
        }

        if 0 <= self.scanline && self.scanline <= 239 {
            // Drawing
        } else if self.scanline == 241 && self.cycles == 1 {
            // VBlank
            self.set_vblank(memory, true);
            // TODO
        } else if self.scanline == 261 && self.cycles == 1 {
            // VBlank off / pre-render line
            self.set_vblank(memory, false);
            // TODO
        }
    }

    /// Gets the VBLANK status
    pub fn get_vblank(&self, memory: &Memory) -> bool {
        memory.read(PPU_STATUS) & 0b10000000 != 0
    }

    /// Sets the VBLANK status
    pub fn set_vblank(&self, memory: &mut Memory, status: bool) {
        let current_status = memory.read(PPU_STATUS);
        let updated_status =
            if status {
                current_status | 0b10000000
            } else {
                current_status & 0b01111111
            };
        memory.write(PPU_STATUS, updated_status)
    }
}
