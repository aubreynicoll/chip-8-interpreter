extern crate sdl2;

mod chip_8;
mod keyboard;

use chip_8::Chip8;
use keyboard::Keyboard;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use std::cell::RefCell;
use std::process;
use std::rc::Rc;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Chip-8", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let event_pump = Rc::new(RefCell::new(sdl_context.event_pump().unwrap()));

    let keyboard = Keyboard::new(Rc::clone(&event_pump));
    let mut c8 = Chip8::new(keyboard);

    c8.load(&[0x12, 0x00]);

    canvas.set_draw_color(Color::RGB(0, 0, 0));

    loop {
        canvas.clear();

        for event in event_pump.borrow_mut().poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    scancode: Some(Scancode::Escape),
                    ..
                } => process::exit(0),
                _ => (),
            };
        }

        c8.execute();

        canvas.present();

        c8.sleep();
    }
}
