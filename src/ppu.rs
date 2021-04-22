use crate::memory::Memory;

// https://wiki.nesdev.com/w/index.php/PPU_registers#Status_.28.242002.29_.3C_read

const PPU_CTRL: u16     = 0x2000;
const PPU_MASK: u16     = 0x2001;
const PPU_STATUS: u16   = 0x2002;
const OAM_ADDR: u16     = 0x2003;
const OAM_DATA: u16     = 0x2004;
const PPU_SCROLL: u16   = 0x2005;
const PPU_ADDR: u16     = 0x2006;
const PPU_DATA: u16     = 0x2007;
const OAM_DMA: u16      = 0x4014;


pub trait Ppu {
    /// Gets the VBLANK status
    fn get_vblank(&self) -> bool;

    /// Sets the VBLANK status
    fn set_vblank(&mut self, status: bool);
}

impl Ppu for Memory {
    // pub fn new(mem: &mut Memory) -> Ppu {
    //     let mut ppu = Ppu {
    //         memory: mem
    //     };
    //
    //     return ppu;
    // }

    fn get_vblank(&self) -> bool {
        self.read(PPU_STATUS) & 0b10000000 != 0
    }

    fn set_vblank(&mut self, status: bool) {
        let current_status = self.read(PPU_STATUS);
        let updated_status =
            if status {
                current_status | 0b10000000
            } else {
                current_status & 0b01111111
            };
        self.write(PPU_STATUS, updated_status)
    }
}
