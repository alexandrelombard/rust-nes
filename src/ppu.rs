use crate::memory::{Memory, PPU_STATUS};
use crate::rom_file::RomFile;

// https://wiki.nesdev.com/w/index.php/PPU_registers#Status_.28.242002.29_.3C_read
// https://emudev.de/nes-emulator/cartridge-loading-pattern-tables-and-ppu-registers/

pub struct Ppu {
    cycles: u32,
    scanline: u32,
    vram: [u8; 0x4000],
    nmi_occurred: bool,
}

impl Ppu {
    pub fn new() -> Ppu {
        return Ppu {
            cycles: 0,
            scanline: 0,
            vram: [0; 0x4000],
            nmi_occurred: false,
        };
    }

    pub fn load(&mut self, rom_file: &RomFile) {
        let chr_data = rom_file.chr_data();
        self.vram[0x0000..(0x0000 + rom_file.chr_size() as usize)].clone_from_slice(&chr_data);
    }

    pub fn vram_data(&self) -> &[u8;0x4000] {
        return &self.vram;
    }

    pub fn read_vram(&self, address: u16) -> u8 {
        return self.vram[address as usize];
    }

    pub fn write_vram(&mut self, address: u16, val: u8) {
        self.vram[address as usize] = val;
    }

    pub fn get_chr_tile(&self, x: u8, y:u8) -> [[u8;8]; 8] {
        let mut result = [[0;8]; 8];
        let offset: usize = (x as usize * 8 * 2) + (y as usize * 16 * 8 * 2);

        for i in 0..8 {
            for j in 0..8 {
                let lb = ((self.vram[i + offset] & (0b10000000 >> j)) >> (7-j));
                let hb = ((self.vram[i + 8 + offset] & (0b10000000 >> j)) >> (7-j)) << 1;

                result[i][j] = hb | lb;
            }
        }

        return result;
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
            self.nmi_occurred = true;
        } else if self.scanline == 261 && self.cycles == 1 {
            // VBlank off / pre-render line
            self.set_vblank(memory, false);
            self.nmi_occurred = false;
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
