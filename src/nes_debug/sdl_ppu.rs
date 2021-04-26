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

fn val_to_color_2bpp(val: u8) -> [u8; 3] {
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

fn get_colors_2bpp(val: u8) -> [u8; 4 * 3] {
    let c3 = (val & 0b00000011);
    let c2 = (val & 0b00001100) >> 2;
    let c1 = (val & 0b00110000) >> 4;
    let c0 = (val & 0b11000000) >> 6;

    let color0 = val_to_color_2bpp(c0);
    let color1 = val_to_color_2bpp(c1);
    let color2 = val_to_color_2bpp(c2);
    let color3 = val_to_color_2bpp(c3);

    return [color0[0], color0[1], color0[2],
        color1[0], color1[1], color1[2],
        color2[0], color2[1], color2[2],
        color3[0], color3[1], color3[2]];
}

pub fn fill_texture_chr_data(texture: &mut Texture, ppu: &Ppu) {
    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        let tile0 = ppu.get_chr_tile(1, 1);
        for i in 0..8 {
            for j in 0..8 {
                let color = val_to_color_2bpp(tile0[i][j]);
                buffer[i * (256 * 3) + (3 * j) % (256 * 3)] = color[0];
                buffer[i * (256 * 3) + (3 * j + 1) % (256 * 3)] = color[1];
                buffer[i * (256 * 3) + (3 * j + 2) % (256 * 3)] = color[2];
            }
        }
    });
}
