use crate::chip_8::{BitMap, DisplayInterface};
use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};
use std::{cell::RefCell, rc::Rc};

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
                    canvas.draw_point(Point::new(col, row)).unwrap();
                }
            }
        }
        canvas.present();
    }
}
