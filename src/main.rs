extern crate sdl2;

mod chip_8;
mod display;
mod keyboard;

use chip_8::Chip8;
use clap::Parser;
use display::Display;
use keyboard::Keyboard;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::rc::Rc;

const WINDOW_WIDTH: u32 = 64;
const WINDOW_HEIGHT: u32 = 32;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Config {
    #[clap(
        short,
        long,
        value_parser,
        value_name = "FILE",
        help = "ROM file to be loaded into Chip-8"
    )]
    file: PathBuf,
}

fn main() {
    let config = Config::parse();

    let rom = fs::read(config.file).unwrap();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Chip-8", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let canvas = Rc::new(RefCell::new(window.into_canvas().build().unwrap()));
    let event_pump = Rc::new(RefCell::new(sdl_context.event_pump().unwrap()));

    let keyboard = Keyboard::new(Rc::clone(&event_pump));
    let display = Display::new(Rc::clone(&canvas));
    let mut c8 = Chip8::new(keyboard, display);

    c8.load(&rom);

    loop {
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

        c8.sleep();
    }
}
