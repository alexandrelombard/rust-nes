use sdl2::render::{Texture, Canvas, RenderTarget, TextureAccess};
use crate::ppu::Ppu;
use sdl2::pixels::PixelFormatEnum;

use log::{LevelFilter, Level, log_enabled, debug, error};

pub fn debug_palette(val: u8) -> [u8; 3] {
    return match val {
        0 => {
            [11, 72, 0]     // Dark green
        },
        1 => {
            [0, 42, 136]    // Dark blue
        },
        2 => {
            [102, 102, 102] // Light grey
        },
        _ => {
            [0, 0, 0]
        }
    }
}

pub fn fill_texture_chr_data<P>(texture: &mut Texture, ppu: &Ppu, palette: P) where P: Fn(u8)->[u8;3] {
    let tile_width = 8 * 3;
    let line_width = 8 * 256 * 3;
    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        for x in 0..16 {
            for y in 0..16 {
                let offset: usize = x as usize * tile_width + y as usize * line_width;
                let tile = ppu.get_chr_tile(x, y);
                for i in 0..8 {
                    for j in 0..8 {
                        let color = palette(tile[i][j]);
                        buffer[i * (256 * 3) + (3 * j) % (256 * 3) + offset] = color[0];
                        buffer[i * (256 * 3) + (3 * j + 1) % (256 * 3) + offset] = color[1];
                        buffer[i * (256 * 3) + (3 * j + 2) % (256 * 3) + offset] = color[2];
                    }
                }
            }
        }

    });
}
