use crate::chip_8::{BitMap, DisplayInterface};
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window, Sdl};

pub const SCALE_FACTOR: u32 = 20;
pub const WINDOW_WIDTH: u32 = 64 * SCALE_FACTOR;
pub const WINDOW_HEIGHT: u32 = 32 * SCALE_FACTOR;

pub struct Display {
    canvas: Canvas<Window>,
}

impl Display {
    pub fn new(sdl_context: &Sdl) -> Self {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Chip-8", WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        Display { canvas }
    }
}

impl DisplayInterface for Display {
    fn draw(&mut self, bitmap: &BitMap) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGB(255, 255, 255));

        for row in 0..32 {
            let data = bitmap[row as usize];
            for col in 0..64 {
                if data & (1 << col) != 0 {
                    let r = Rect::new(
                        col * SCALE_FACTOR as i32,
                        row * SCALE_FACTOR as i32,
                        SCALE_FACTOR,
                        SCALE_FACTOR,
                    );
                    self.canvas.draw_rect(r).unwrap();
                    self.canvas.fill_rect(r).unwrap();
                }
            }
        }

        self.canvas.present();
    }
}
