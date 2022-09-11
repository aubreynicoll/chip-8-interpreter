use std::env;

pub fn debug(get_msg: impl Fn() -> String) {
    if env::var("CHIP_8_DEBUG_MODE").is_ok() {
        println!("{}", get_msg());
    }
}
