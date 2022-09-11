extern crate sdl2;

mod chip_8;
mod console;
mod display;
mod keyboard;
mod sound;

use chip_8::Chip8;
use clap::Parser;
use display::Display;
use keyboard::Keyboard;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sound::Sound;
use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::rc::Rc;

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

    let sdl_context = sdl2::init().unwrap();

    let event_pump = Rc::new(RefCell::new(sdl_context.event_pump().unwrap()));

    let keyboard = Keyboard::new(Rc::clone(&event_pump));
    let display = Display::new(&sdl_context);
    let sound = Sound::new(&sdl_context);
    let mut c8 = Chip8::new(keyboard, display, sound);

    if let Ok(rom) = fs::read(config.file) {
        c8.load(&rom);
    }

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
