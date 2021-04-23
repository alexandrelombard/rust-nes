mod rom_file;
mod cpu;
mod ppu;
mod memory;

use crate::memory::Memory;
use crate::cpu::Cpu;
use crate::ppu::Ppu;

extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use log::{LevelFilter, Level, log_enabled, debug, error};

pub fn main() {
    // Initialize logger
    env_logger::init();

    let result = rom_file::read("roms/Super Mario Bros. (World).nes");

    if result.is_ok() {
        let rom_file = result.unwrap();

        if rom_file.is_nes() {
            println!("ROM file opened: {p} ({s})", p=rom_file.file_path, s=rom_file.data.len());
            println!("\tPRG size: {0} x 16kB", rom_file.raw_prg_size());
            println!("\tCHR size: {0} x 8kB", rom_file.raw_chr_size());
            println!("\tTrainer: {0}", rom_file.has_trainer());

            // Initialize the NES emulation system
            let mut mem = Memory::new();
            mem.load(rom_file);

            let mut cpu = Cpu::new(&mem);

            // Run test
            for i in 0..20 {
                // Run a predefined amount of steps for debug
                cpu.step(&mut mem);
            }

            // Set VBLANK to true and run for another set of steps
            mem.set_vblank(true);
            debug!("VBLANK set to true");

            // Run test
            for i in 0..300 {
                // Run a predefined amount of steps for debug
                cpu.step(&mut mem);
            }
        } else {
            error!("ROM file is not a NES ROM file: {p} ({s})", p=rom_file.file_path, s=rom_file.data.len());
        }
    }
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
