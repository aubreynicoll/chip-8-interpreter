use rand;
use std::thread;
use std::time;

const SPRITE_POINTER: usize = 0x20;
const SPRITE_SIZE: usize = 5;
const SPRITE_DATA: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Key {
    value: u8,
}

impl Key {
    pub fn new(value: u8) -> Self {
        if value > 0xF {
            panic!("Expected value between 0x0 and 0xF, got {}", value);
        }

        Key { value }
    }

    pub fn value(&self) -> &u8 {
        &self.value
    }
}

pub trait KeyboardInterface {
    fn is_key_pressed(&self, key: Key) -> bool;
    fn get_pressed_key(&self) -> Option<Key>;
}

// Chip-8 Display rendered 64x32 monochrome
// x values are packed descending for easy shifting, MSB = x[63] and LSB = x[0]
// y values are packed ascending, array[0] = y[0] and array[31] = y[31]
pub type BitMap = [u64; 0x20];

pub trait DisplayInterface {
    fn draw(&self, bitmap: &BitMap);
}

pub struct Chip8<K, D>
where
    K: KeyboardInterface,
    D: DisplayInterface,
{
    v: [u8; 0x10],
    i: usize,
    dt: u8,
    st: u8,
    pc: usize,
    sp: usize,
    ram: [u8; 0x1000],
    vram: BitMap,
    keyboard: K,
    display: D,
}

impl<K, D> Chip8<K, D>
where
    K: KeyboardInterface,
    D: DisplayInterface,
{
    pub fn new(keyboard: K, display: D) -> Self {
        let mut new_c8 = Chip8 {
            v: [0x0; 0x10],
            i: 0x0,
            dt: 0x0,
            st: 0x0,
            pc: 0x200,
            sp: 0x0,
            ram: [0x0; 0x1000],
            vram: [0x0; 0x20],
            keyboard,
            display,
        };

        // initialize sprite data to system memory
        for (i, &byte) in SPRITE_DATA.iter().enumerate() {
            new_c8.ram[SPRITE_POINTER + i] = byte;
        }

        // draw initial blank state
        new_c8.display.draw(&new_c8.vram);

        new_c8
    }

    pub fn load(&mut self, rom: &[u8]) {
        for (i, &byte) in rom.iter().enumerate() {
            self.ram[self.pc + i] = byte;
        }
    }

    fn get_next_opcode(&mut self) -> u16 {
        let msb = self.ram[self.pc];
        let lsb = self.ram[self.pc + 1];
        self.pc += 2;

        let opcode = ((msb as u16) << 8) + lsb as u16;
        return opcode;
    }

    fn push_stack(&mut self, addr: usize) {
        if self.sp >= 0x20 {
            panic!("stack overflow");
        }

        let msb = (addr >> 8) as u8;
        let lsb = addr as u8;

        self.ram[self.sp] = msb;
        self.ram[self.sp + 1] = lsb;
        self.sp += 2;
    }

    fn pop_stack(&mut self) -> usize {
        if self.sp == 0 {
            panic!("stack empty");
        }

        self.sp -= 2;
        let msb = self.ram[self.sp];
        let lsb = self.ram[self.sp + 1];

        let addr = ((msb as usize) << 8) + lsb as usize;
        return addr;
    }

    pub fn execute(&mut self) {
        let opcode = self.get_next_opcode();
        let addr = (opcode & 0xFFF) as usize;
        let x = ((opcode >> 8) & 0xF) as usize;
        let y = ((opcode >> 4) & 0xF) as usize;
        let val = opcode as u8;

        match opcode >> 12 {
            0x0 => {
                match opcode & 0xFFF {
                    0x0E0 => {
                        // clear display
                        println!("{}: clear display", opcode);
                        todo!();
                    }
                    0x0EE => {
                        // return
                        let addr = self.pop_stack();

                        println!("{}: return (to addr {:#06x})", opcode, addr);

                        self.pc = addr;
                    }
                    _ => panic!("invalid opcode: {}", opcode),
                }
            }
            0x1 => {
                // jump to addr
                println!("{}: jump to {:#06x}", opcode, addr);

                if addr < 0x200 {
                    panic!("jump to reserved address");
                }
                self.pc = addr;
            }
            0x2 => {
                // call subroutine
                println!("{}: call subroutine at {:#06x}", opcode, addr);

                if addr < 0x200 {
                    panic!("call to reserved address");
                }
                self.push_stack(self.pc);
                self.pc = addr;
            }
            0x3 => {
                // skip next op if v[x] == val
                println!("{}: skip next op if v[{}] == {}", opcode, x, val);

                if self.v[x] == val {
                    self.pc += 2;
                }
            }
            0x4 => {
                // skip next op if v[x] != val
                println!("{}: skip next op if v[{}] != {}", opcode, x, val);

                if self.v[x] != val {
                    self.pc += 2;
                }
            }
            0x5 => {
                match opcode & 0xF {
                    0x0 => {
                        // skip next op if v[x] == v[y]
                        println!("{}: skip next op if v[{}] == v[{}]", opcode, x, y);

                        if self.v[x] == self.v[y] {
                            self.pc += 2;
                        }
                    }
                    _ => panic!("invalid opcode: {}", opcode),
                }
            }
            0x6 => {
                // v[x] = val
                println!("{}: set v[{}] to {}", opcode, x, val);

                self.v[x] = val;
            }
            0x7 => {
                // v[x] += val
                println!("{}: add {} to v[{}]", opcode, val, x);

                self.v[x] += val;
            }
            0x8 => {
                match opcode & 0xF {
                    0x0 => {
                        // v[x] = v[y]
                        println!("{}: set v[{}] to value of v[{}]", opcode, x, y);

                        self.v[x] = self.v[y];
                    }
                    0x1 => {
                        // v[x] |= v[y]
                        println!("{}: set v[{}] to value of v[{}] | v[{}]", opcode, x, x, y);

                        self.v[x] |= self.v[y];
                    }
                    0x2 => {
                        // v[x] &= v[y]
                        println!("{}: set v[{}] to value of v[{}] & v[{}]", opcode, x, x, y);

                        self.v[x] &= self.v[y];
                    }
                    0x3 => {
                        // v[x] ^= v[y]
                        println!("{}: set v[{}] to value of v[{}] ^ v[{}]", opcode, x, x, y);

                        self.v[x] ^= self.v[y];
                    }
                    0x4 => {
                        // v[x] += v[y]
                        // set v[F] to 1 if overflow occurs, else 0
                        println!("{}: set v[{}] to v[{}] + v[{}]", opcode, x, x, y);

                        let sum = self.v[x].wrapping_add(self.v[y]);
                        self.v[0xF] = if sum < self.v[x] { 1 } else { 0 };
                        self.v[x] = sum;
                    }
                    0x5 => {
                        // v[x] -= v[y]
                        // set v[F] to 1 if overflow DOES NOT occur, else 0
                        println!("{}: set v[{}] to v[{}] - v[{}]", opcode, x, x, y);

                        let diff = self.v[x].wrapping_sub(self.v[y]);
                        self.v[0xF] = if diff > self.v[x] { 0 } else { 1 };
                        self.v[x] = diff;
                    }
                    0x6 => {
                        // set v[F] to value of v[y] & 1 (value of lsb)
                        // v[x] = v[y] >> 1
                        println!(
                            "{}: set v[{}] to value of v[{}] shifted right by 1 bit (div by 2)",
                            opcode, x, y
                        );

                        self.v[0xF] = self.v[y] & 1;
                        self.v[x] = self.v[y] >> 1;
                    }
                    0x7 => {
                        // v[x] = v[y] - v[x]
                        // set v[F] to 1 if overflow DOES NOT occur, else 0
                        println!("{}: set[{}] to v[{}] - v[{}]", opcode, x, y, x);

                        let diff = self.v[y].wrapping_sub(self.v[x]);
                        self.v[0xF] = if diff > self.v[y] { 0 } else { 1 };
                        self.v[x] = diff;
                    }
                    0xE => {
                        // set v[F] to v[y] >> 7 (value of msb)
                        // v[x] = v[y] << 1
                        println!(
                            "{}: set v[{}] to value of v[{}] shifted left by 1 bit (mult by 2)",
                            opcode, x, y
                        );

                        self.v[0xF] = self.v[y] >> 7;
                        self.v[x] = self.v[y] << 1;
                    }
                    _ => panic!("invalid opcode: {}", opcode),
                }
            }
            0x9 => {
                match opcode & 0xF {
                    0x0 => {
                        // skip next op if v[x] != v[y]
                        println!("{}: skip next op if v[{}] != v[{}]", opcode, x, y);

                        if self.v[x] != self.v[y] {
                            self.pc += 2;
                        }
                    }
                    _ => panic!("invalid opcode: {}", opcode),
                }
            }
            0xA => {
                // set i to addr
                println!("{}: set i to {:#06x}", opcode, addr);

                self.i = addr;
            }
            0xB => {
                // jump to addr + v[0]
                println!("{}: jump to {:#06x}", opcode, addr);

                if addr < 0x200 {
                    panic!("jump to reserved address");
                }
                self.pc = addr;
            }
            0xC => {
                // v[x] = random_byte & val
                let random_byte = rand::random::<u8>();

                println!("{}: set v[{}] to random & val", opcode, x);

                self.v[x] = random_byte & val;
            }
            0xD => {
                // display n-byte sprite on screen at point (x, y)
                let n = (val & 0xF) as usize;
                self.v[0xF] = 0;

                for row in 0..n {
                    let byte = self.ram[self.i + row];
                    let sprite_row = byte.reverse_bits() as u64;

                    let previous = self.vram[y + row] & 0xFF << x;

                    self.vram[y + row] ^= sprite_row << x;

                    if self.vram[y + row] & previous < previous {
                        self.v[0xF] = 1;
                    }
                }
            }
            0xE => {
                match opcode & 0xFF {
                    0x9E => {
                        // skip next op if key with value of v[x] is pressed
                        if self.keyboard.is_key_pressed(Key::new(self.v[x])) {
                            self.pc += 2;
                        }
                    }
                    0xA1 => {
                        // skip next op if key with value of v[x] is NOT pressed
                        if !self.keyboard.is_key_pressed(Key::new(self.v[x])) {
                            self.pc += 2;
                        }
                    }
                    _ => panic!("invalid opcode: {}", opcode),
                }
            }
            0xF => {
                match opcode & 0xFF {
                    0x07 => {
                        // set v[x] to value of dt
                        println!("{}: set v[{}] to value of dt", opcode, x);

                        self.v[x] = self.dt;
                    }
                    0x0A => {
                        // wait for key press, store value in v[x]
                        match self.keyboard.get_pressed_key() {
                            Some(key) => self.v[x] = key.value,
                            None => self.pc -= 2,
                        }
                    }
                    0x15 => {
                        // set dt to value of v[x]
                        println!("{}: set dt to value of v[{}]", opcode, x);

                        self.dt = self.v[x];
                    }
                    0x18 => {
                        // set st to value of v[x]
                        println!("{}: set st to value of v[{}]", opcode, x);

                        self.st = self.v[x];
                    }
                    0x1E => {
                        // i += v[x]
                        println!("{}: set i to i + v[{}]", opcode, x);

                        self.i += self.v[x] as usize;
                    }
                    0x29 => {
                        // set i to sprite for value in v[x]
                        self.i = SPRITE_POINTER + self.v[x] as usize * SPRITE_SIZE;
                    }
                    0x33 => {
                        // BCD of v[x] in i, i+1, i+2
                        // according to sources, this does not mutate the value of i
                        println!("{}: store BCD of v[{}] starting at i", opcode, x);

                        if self.i < 0x200 {
                            panic!("write to reserved memory");
                        }

                        let val = self.v[x];

                        self.ram[self.i] = val / 100;
                        self.ram[self.i + 1] = (val / 10) % 10;
                        self.ram[self.i + 2] = val % 10;
                    }
                    0x55 => {
                        // store v[0]..=v[x] in memory starting at i
                        // sets i to i + n + 1
                        println!("{}: store v[0] through v[{}] starting at i", opcode, x);

                        if self.i < 0x200 {
                            panic!("write to reserved memory");
                        }

                        for n in 0..=x {
                            self.ram[self.i] = self.v[n];
                            self.i += 1;
                        }
                    }
                    0x65 => {
                        // read memory into v[0] through v[x] starting at i
                        // sets i to i + n + 1
                        println!("{}: read into v[0] through v[{}] starting at i", opcode, x);

                        for n in 0..=x {
                            self.v[n] = self.ram[self.i];
                            self.i += 1;
                        }
                    }
                    _ => panic!("invalid opcode: {}", opcode),
                }
            }
            _ => panic!("invalid opcode: {}", opcode),
        }
    }

    pub fn sleep(&self) {
        let dur = time::Duration::from_secs(1);
        thread::sleep(dur);
    }

    pub fn print_registers(&self) {
        println!("---Registers---");
        for (i, v) in self.v.iter().enumerate() {
            println!("v[{:x}]: {:#04x}", i, v);
        }
        println!("i: {:#05x}", self.i);
        println!("st: {:#04x}", self.st);
        println!("dt: {:#04x}", self.dt);
        println!("pc: {:#05x}", self.pc);
        println!("sp: {:#05x}", self.sp);
    }

    pub fn print_memory(&self) {
        println!("---Memory---");
        for (i, byte) in self.ram.iter().enumerate() {
            println!("{:#05x}: {:#04x}", i, byte);
        }
    }
}
