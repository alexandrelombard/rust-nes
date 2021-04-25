use sdl2::render::{Texture, Canvas, RenderTarget, TextureAccess};
use crate::ppu::Ppu;
use sdl2::pixels::PixelFormatEnum;

use log::{LevelFilter, Level, log_enabled, debug, error};

fn val_to_color(val: u8) -> [u8; 3] {
    return match val {
        0 => {
            [102, 102, 102] // Light grey
        },
        1 => {
            [0, 42, 136]    // Dark blue
        },
        9 => {
            [11, 72, 0]     // Dark green
        },
        _ => {
            [255, 255, 255]
        }
    }
}

pub fn fill_texture_chr_data(texture: &mut Texture, ppu: &Ppu) {
    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        for i in 0x0..0x1000 {
            let val = ppu.read_vram(i);
            let color = val_to_color(val);
            buffer[(3 * i) as usize]      = color[0];
            buffer[(3 * i + 1) as usize]  = color[1];
            buffer[(3 * i + 2) as usize]  = color[2];
        }
    });
}
