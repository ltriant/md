#[macro_use] extern crate log;

mod bus;
mod console;
mod cpu;
mod rom;

use std::env;
use std::io;
use std::process;

use crate::console::Console;

fn main() -> io::Result<()> {
    env_logger::init();

    if let Some(rom_path) = env::args().skip(1).next() {
        let cart = rom::load_cartridge(&rom_path)?;
        let mut console = Console::new_mega_drive();
        if let Err(e) = console.power_up(&cart) {
            println!("Unable to power up the Mega Drive: {:?}", e);
            process::exit(1);
        }
    }

    Ok(())
}
