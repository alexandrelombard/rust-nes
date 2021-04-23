use crate::memory::Memory;

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
}

impl Ppu {
    pub fn new() -> Ppu {
        return Ppu {
            cycles: 0,
            scanline: 0,
        };
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
