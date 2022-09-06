use std::env;

pub fn debug(msg: &str) {
    if env::var("CHIP_8_DEBUG_MODE").is_ok() {
        println!("{}", msg);
    }
}
