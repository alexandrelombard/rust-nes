mod rom_file;
mod cpu;
mod ppu;
mod memory;
mod controller;

use crate::memory::{Memory, PPU_CTRL};
use crate::cpu::Cpu;
use crate::ppu::Ppu;

extern crate sdl2;

use sdl2::pixels::{Color, PixelFormat};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use log::{LevelFilter, Level, log_enabled, debug, error};
use sdl2::EventPump;
use crate::rom_file::RomFile;
use sdl2::render::{Canvas, Texture, TextureAccess, TextureCreator};
use std::any::Any;

pub fn main() {
    // Initialize logger
    env_logger::init();

    // Initialize SDL and canvas
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window =
        video_subsystem
            .window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()
            .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut framebuffer: [u8; 1024 * 256 * 3];
    let texture_creator = canvas.texture_creator();
    let mut texture =
        texture_creator
            .create_texture(None, TextureAccess::Static, 1024, 256);
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Load the ROM file
    let rom_result = RomFile::new("roms/Donkey Kong (World) (Rev A).nes");
    // let rom_result = RomFile::new("roms/Super Mario Bros. (World).nes");

    if rom_result.is_ok() {
        let rom_file = rom_result.unwrap();

        if rom_file.is_nes() {
            println!("ROM file opened: {p} ({s})", p=rom_file.file_path, s=rom_file.data.len());
            println!("\tPRG size: {0} x 16kB", rom_file.raw_prg_size());
            println!("\tCHR size: {0} x 8kB", rom_file.raw_chr_size());
            println!("\tTrainer: {0}", rom_file.has_trainer());

            // Initialize the NES emulation system
            let mut cpu_mem = Memory::new();
            cpu_mem.load(&rom_file);

            let mut cpu = Cpu::new(&cpu_mem);

            let mut ppu = Ppu::new();
            ppu.load(&rom_file);

            // Run
            'running: loop {
                canvas.set_draw_color(Color::RGB(0, 0, 0));
                canvas.clear();

                handle_user_input(&mut cpu_mem, &mut event_pump);
                cpu.step(&mut cpu_mem);
                ppu.step(&mut cpu_mem);

                canvas.present();

                ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            }

            // Run test
            for i in 0..20 {
                // Run a predefined amount of steps for debug
                cpu.step(&mut cpu_mem);
            }

            // Set VBLANK to true and run for another set of steps
            ppu.set_vblank(&mut cpu_mem, true);
            debug!("VBLANK set to true");

            // Run test
            for i in 0..300 {
                // Run a predefined amount of steps for debug
                cpu.step(&mut cpu_mem);
            }
        } else {
            error!("ROM file is not a NES ROM file: {p} ({s})", p=rom_file.file_path, s=rom_file.data.len());
        }
    }
}

pub fn handle_user_input(memory: &mut Memory, event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                std::process::exit(0)
            },
            Event::KeyDown { keycode: Some(Keycode::K), .. } => {    // A
                // memory.write(CONTROLLER_1_ADDRESS, JOY_BUTTON_A);
            },
            Event::KeyDown { keycode: Some(Keycode::L), .. } => {    // B
                // memory.write(CONTROLLER_1_ADDRESS, JOY_BUTTON_B);
            },
            Event::KeyDown { keycode: Some(Keycode::Space), .. } => {    // START
                // memory.write(CONTROLLER_1_ADDRESS, JOY_START);
            },
            Event::KeyDown { keycode: Some(Keycode::Return), .. } => {    // SELECT
                // memory.write(CONTROLLER_1_ADDRESS, JOY_SELECT);
            },
            Event::KeyDown { keycode: Some(Keycode::Z), .. } => {   // UP
                // memory.write(CONTROLLER_1_ADDRESS, JOY_UP);
            },
            Event::KeyDown { keycode: Some(Keycode::S), .. } => {   // DOWN
                // memory.write(CONTROLLER_1_ADDRESS, JOY_DOWN);
            },
            Event::KeyDown { keycode: Some(Keycode::Q), .. } => {   // LEFT
                // memory.write(CONTROLLER_1_ADDRESS, JOY_LEFT);
            },
            Event::KeyDown { keycode: Some(Keycode::D), .. } => {   // RIGHT
                // memory.write(CONTROLLER_1_ADDRESS, JOY_RIGHT);
            }
            _ => {/* do nothing */}
        }
    }
}

fn draw<T>(ppu: &Ppu, memory: &Memory, framebuffer: &mut [u8; 1024 * 256 * 3], texture: &mut Texture) {
    for r in 0..1024 {
        for col in 0..256 {
            let tile_nr = ppu.read_vram(0x2000 + (r / 8 * 32) + (col / 8));
            let tile_attr = ppu.read_vram(0);

            let adr = memory.get_background_pattern_table_address_value() + (tile_nr as u16 * 0x10) + (r % 8) as u16;
            let pixel = ((ppu.read_vram(adr) >> (7 - (col % 8))) & 1) + (((ppu.read_vram(adr + 8) >> (7 - (col % 8))) & 1) * 2);
            // framebuffer[(r * 256 * 3) + (col * 3)] = COLORS[pixel];
            // framebuffer[(r * 256 * 3) + (col * 3) + 1] = COLORS[pixel];
            // framebuffer[(r * 256 * 3) + (col * 3) + 2] = COLORS[pixel];
            framebuffer[((r * 256 * 3) + (col * 3)) as usize] = 255;
            framebuffer[((r * 256 * 3) + (col * 3) + 1) as usize] = 255;
            framebuffer[((r * 256 * 3) + (col * 3) + 2) as usize] = 255;
        }
    }

    texture.update(None, framebuffer, 256 * 3);
}

// pub fn main() {
//     let sdl_context = sdl2::init().unwrap();
//     let video_subsystem = sdl_context.video().unwrap();
//
//     let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
//         .position_centered()
//         .build()
//         .unwrap();
//
//     let mut canvas = window.into_canvas().build().unwrap();
//
//     canvas.set_draw_color(Color::RGB(0, 255, 255));
//     canvas.clear();
//     canvas.present();
//     let mut event_pump = sdl_context.event_pump().unwrap();
//     let mut i = 0;
//     'running: loop {
//         i = (i + 1) % 255;
//         canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
//         canvas.clear();
//         for event in event_pump.poll_iter() {
//             match event {
//                 Event::Quit {..} |
//                 Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
//                     break 'running
//                 },
//                 _ => {}
//             }
//         }
//         // The rest of the game loop goes here...
//
//         canvas.present();
//         ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
//     }
// }
