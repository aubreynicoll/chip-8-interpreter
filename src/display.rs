use crate::chip_8::{BitMap, DisplayInterface};
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};
use std::{cell::RefCell, convert::TryInto, rc::Rc};

pub const SCALE_FACTOR: u32 = 20;
pub const WINDOW_WIDTH: u32 = 64 * SCALE_FACTOR;
pub const WINDOW_HEIGHT: u32 = 32 * SCALE_FACTOR;

pub struct Display {
    canvas: Rc<RefCell<Canvas<Window>>>,
}

impl Display {
    pub fn new(canvas: Rc<RefCell<Canvas<Window>>>) -> Self {
        Display { canvas }
    }
}

impl DisplayInterface for Display {
    fn draw(&self, bitmap: &BitMap) {
        let mut canvas = self.canvas.borrow_mut();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 255, 255));

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
                    canvas.draw_rect(r).unwrap();
                    canvas.fill_rect(r).unwrap();
                }
            }
        }

        canvas.present();
    }
}
