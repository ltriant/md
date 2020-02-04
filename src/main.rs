#[macro_use] extern crate log;

use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom};
use std::str;
use std::u32;

const OFFSET_CONSOLE: usize = 0;
const OFFSET_COPYRIGHT: usize = 16;
const OFFSET_DOMESTIC_NAME: usize = 32;
const OFFSET_OVERSEAS_NAME: usize = 80;
const OFFSET_SERIAL_NO: usize = 128;
const OFFSET_CHECKSUM: usize = 142;
//const OFFSET_IO_SUPPORT: usize = 144;
const OFFSET_ROM_START: usize = 160;
const OFFSET_ROM_END: usize = 164;
const OFFSET_RAM_START: usize = 168;
const OFFSET_RAM_END: usize = 172;
const OFFSET_BACKUP_RAM_ID: usize = 176;
//const OFFSET_BACKUP_RAM_START: usize = 186;
//const OFFSET_BACKUP_RAM_END: usize = 190;
const OFFSET_MODEM_SUPPORT: usize = 194;
//const OFFSET_MEMO: usize = 206;
const OFFSET_COUNTRY: usize = 240;
const OFFSET_END: usize = 256;

#[derive(Debug, Hash, Eq, PartialEq)]
enum CountrySupport {
    Japan,
    USA,
    Europe,
}

#[derive(Debug)]
struct Cartridge {
    ram_start: u32,
    ram_end:   u32,
    rom_start: u32,
    rom_end:   u32,
    countries: HashSet<CountrySupport>,
    rom:       Vec<u8>,
}

fn u32_from_slice(data: &[u8]) -> u32 {
      ((data[0] as u32) << 24)
    | ((data[1] as u32) << 16)
    | ((data[2] as u32) << 8)
    |  (data[3] as u32)
}

fn load_cartridge(rom_path: &String) -> Result<Cartridge, io::Error> {
    // Resources:
    //   https://wiki.megadrive.org/index.php?title=MD_Rom_Header
    //   https://en.wikibooks.org/wiki/Genesis_Programming#ROM_header
    //
    // Note that there's a discrepancy between these two links regarding the
    // backup RAM fields, so I kinda figured it out from these aswell as
    // manually inspecting a Sonic 2 ROM.

    info!("Loading ROM: {}", rom_path);

    let mut fh = File::open(rom_path.as_str())?;

    // The ROM header starts at $000100
    fh.seek(SeekFrom::Start(0x100))?;

    let mut header = [0; 256];
    fh.read(&mut header)?;

    debug!("Console: {}", str::from_utf8(&header[OFFSET_CONSOLE .. OFFSET_COPYRIGHT]).unwrap());
    debug!("Domestic Name: {}", str::from_utf8(&header[OFFSET_DOMESTIC_NAME .. OFFSET_OVERSEAS_NAME]).unwrap());
    debug!("Overseas Name: {}", str::from_utf8(&header[OFFSET_OVERSEAS_NAME .. OFFSET_SERIAL_NO]).unwrap());
    info!("Serial No: {}", str::from_utf8(&header[OFFSET_SERIAL_NO .. OFFSET_CHECKSUM]).unwrap());

    let countries = str::from_utf8(&header[OFFSET_COUNTRY .. OFFSET_END]).unwrap();
    info!("Country: {}", countries);

    let rom_start = u32_from_slice(&header[OFFSET_ROM_START .. OFFSET_ROM_END]);
    let rom_end   = u32_from_slice(&header[OFFSET_ROM_END .. OFFSET_RAM_START]);
    info!("ROM: 0x{:08X} - 0x{:08X}", rom_start, rom_end);

    let ram_start = u32_from_slice(&header[OFFSET_RAM_START .. OFFSET_RAM_END]);
    let ram_end   = u32_from_slice(&header[OFFSET_RAM_END .. OFFSET_BACKUP_RAM_ID]);
    info!("RAM: 0x{:08X} - 0x{:08X}", ram_start, ram_end);

    if header[OFFSET_BACKUP_RAM_ID] as char == 'R' && header[OFFSET_BACKUP_RAM_ID + 1] as char == 'A' {
        // TODO backup RAM is enabled, wat do?
        warn!("Backup RAM is enabled: {:?}", &header[OFFSET_BACKUP_RAM_ID .. OFFSET_MODEM_SUPPORT]);
    }

    let mut country_support = HashSet::new();

    if countries.contains("J") {
        country_support.insert(CountrySupport::Japan);
    }

    if countries.contains("U") {
        country_support.insert(CountrySupport::USA);
    }

    if countries.contains("E") {
        country_support.insert(CountrySupport::Europe);
    }

    // The ROM data starts at $000200
    fh.seek(SeekFrom::Start(0x200))?;

    let mut rom = Vec::new();
    let bytes = fh.read_to_end(&mut rom)?;
    debug!("Read {} bytes of ROM data", bytes);

    Ok(Cartridge {
        rom_start: rom_start,
        rom_end:   rom_end,
        ram_start: ram_start,
        ram_end:   ram_end,
        countries: country_support,
        rom:       rom,
    })
}

fn main() -> io::Result<()> {
    env_logger::init();

    if let Some(rom_path) = env::args().skip(1).next() {
        let cart = load_cartridge(&rom_path)?;
        println!("Hello, Mega Drive!");
    }

    Ok(())
}
