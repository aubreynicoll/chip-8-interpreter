extern crate sdl2;

mod chip_8;

use chip_8::{Chip8, Key};
use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use std::process;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn get_input_from_scancode(code: Scancode) -> Option<Key> {
    match code {
        Scancode::Num1 => Some(Key::new(0x1)),
        Scancode::Num2 => Some(Key::new(0x2)),
        Scancode::Num3 => Some(Key::new(0x3)),
        Scancode::Num4 => Some(Key::new(0xC)),
        Scancode::Q => Some(Key::new(0x4)),
        Scancode::W => Some(Key::new(0x5)),
        Scancode::E => Some(Key::new(0x6)),
        Scancode::R => Some(Key::new(0xD)),
        Scancode::A => Some(Key::new(0x7)),
        Scancode::S => Some(Key::new(0x8)),
        Scancode::D => Some(Key::new(0x9)),
        Scancode::F => Some(Key::new(0xE)),
        Scancode::Z => Some(Key::new(0xA)),
        Scancode::X => Some(Key::new(0x0)),
        Scancode::C => Some(Key::new(0xB)),
        Scancode::V => Some(Key::new(0xF)),
        _ => None,
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Chip-8", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut c8 = Chip8::new();
    c8.load(&[0x12, 0x00]);

    canvas.set_draw_color(Color::RGB(0, 0, 0));

    loop {
        canvas.clear();

        let mut key_input = None;
        for event in event_pump.poll_iter() {
            key_input = match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    scancode: Some(Scancode::Escape),
                    ..
                } => process::exit(0),
                Event::KeyDown {
                    scancode: Some(code),
                    ..
                } => get_input_from_scancode(code),
                _ => None,
            };
        }

        c8.execute(key_input);

        canvas.present();

        Chip8::sleep();
    }
}
