mod chip_8;

use chip_8::Chip8;
use std::fs;

fn main() {
    let mut c8 = Chip8::new();
    c8.load(&[0x12, 0x00]);
    c8.run();
}
