mod rom;

#[macro_use] extern crate log;

use std::env;
use std::io;

fn main() -> io::Result<()> {
    env_logger::init();

    if let Some(rom_path) = env::args().skip(1).next() {
        let _cart = rom::load_cartridge(&rom_path)?;
        println!("Hello, Mega Drive!");
    }

    Ok(())
}
