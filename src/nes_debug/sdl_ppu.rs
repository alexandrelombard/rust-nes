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
            [102, 102, 102] // Light grey
        },
        1 => {
            [0, 42, 136]    // Dark blue
        },
        2 => {
            [11, 72, 0]     // Dark green
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
        for i in 0x0 as u32..0x2000 {
            let val = ppu.read_vram(i as u16);
            // let color = val_to_color(val);
            let colors = get_colors_2bpp(val);
            buffer[(12 * i) as usize]      = colors[0];
            buffer[(12 * i + 1) as usize]  = colors[1];
            buffer[(12 * i + 2) as usize]  = colors[2];

            buffer[(12 * i + 3) as usize]  = colors[3];
            buffer[(12 * i + 4) as usize]  = colors[4];
            buffer[(12 * i + 5) as usize]  = colors[5];

            buffer[(12 * i + 6) as usize]  = colors[6];
            buffer[(12 * i + 7) as usize]  = colors[7];
            buffer[(12 * i + 8) as usize]  = colors[8];

            buffer[(12 * i + 9) as usize]  = colors[9];
            buffer[(12 * i + 10) as usize] = colors[10];
            buffer[(12 * i + 11) as usize] = colors[11];
        }
    });
}
