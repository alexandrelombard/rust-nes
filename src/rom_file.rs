use std::fs;
use std::io;

const HEADER_SIZE: u16 = 16;
const TRAINER_SIZE: u16 = 512;

const FLAG_MIRRORING: u8 = 0b00000001;
const FLAG_CARTRIDGE_BATTERY: u8 = 0b00000010;
const FLAG_TRAINER: u8 = 0b00000100;
const FLAG_MIRRORING_CONTROL: u8 = 0b00001000;

/// ROM file (INES format)
pub struct RomFile {
    pub file_path: String,
    pub data: Vec<u8>
}

impl RomFile {
    /// True if the header contains the NES signature
    pub fn is_nes(&self) -> bool {
        self.data[0] == 0x4E &&
            self.data[1] == 0x45 &&
            self.data[2] == 0x53 &&
            self.data[3] == 0x1A
    }

    /// Gets the PRG ROM size in 16kB units
    pub fn raw_prg_size(&self) -> u8 {
        self.data[4]
    }

    /// Gets the PRG ROM size in byte units
    pub fn prg_size(&self) -> u16 {
        (self.raw_prg_size() as u16) * (16 * 1024)
    }

    /// Gets the PRG ROM data
    pub fn prg_data(&self) -> Vec<u8> {
        let prg_address = self.prg_data_address();
        let prg_size = self.prg_size();
        return self.data[prg_address as usize .. (prg_address + prg_size) as usize].to_vec()
    }

    /// Gets the CHR ROM size in 8kB units
    pub fn raw_chr_size(&self) -> u8 {
        self.data[5]
    }

    /// Gets the CHR ROM size in 8kB units
    pub fn chr_size(&self) -> u16 {
        self.raw_chr_size() as u16 * (8 * 1024)
    }

    /// Gets the CHR ROM data
    pub fn chr_data(&self) -> Vec<u8> {
        let chr_address = self.chr_data_address();
        let chr_size = self.chr_size();
        return self.data[chr_address as usize .. (chr_address + chr_size) as usize].to_vec()
    }

    /// Gets whether a trainer is present or no in the ROM file
    pub fn has_trainer(&self) -> bool {
        (self.data[6] & FLAG_TRAINER) != 0
    }

    pub fn prg_data_address(&self) -> u16 {
        if self.has_trainer() {
            HEADER_SIZE + TRAINER_SIZE
        } else {
            HEADER_SIZE
        }
    }

    pub fn chr_data_address(&self) -> u16 {
        self.prg_data_address() + self.prg_size()
    }
}

/// Reads a ROM file
pub fn read(path: &str) -> io::Result<RomFile> {
    let result = fs::read(path);

    return if result.is_ok() {
        let file_data = result.unwrap();

        Ok(RomFile {
            file_path: path.to_string(),
            data: file_data
        })
    } else {
        Err(result.unwrap_err())
    }
}


