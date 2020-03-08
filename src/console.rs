use std::thread;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::rom::Cartridge;
use crate::cpu::CPU;

const SCREEN_WIDTH: u32 = 320;
const SCREEN_HEIGHT: u32 = 224;

#[derive(Debug)]
pub enum MegaDriveError {
    SDLIntegerError(sdl2::IntegerOrSdlError),
    SDLVideo(sdl2::video::WindowBuildError),
    SDLOther(String),
}

pub struct Console {
    cpu: CPU,
}

impl Console {
    pub fn new_mega_drive() -> Self {
        let cpu = CPU::new_68k();

        Self {
            cpu: cpu,
        }
    }

    pub fn power_up(&mut self, cart: &Cartridge) -> Result<(), MegaDriveError> {
        info!("Powering up");

        let sdl_context = sdl2::init()
            .map_err(MegaDriveError::SDLOther)?;

        let video_sys = sdl_context.video()
            .map_err(MegaDriveError::SDLOther)?;

        let window = video_sys.window("mega drive", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .build()
            .map_err(MegaDriveError::SDLVideo)?;

        let mut canvas = window.into_canvas()
            .build()
            .map_err(MegaDriveError::SDLIntegerError)?;

        let mut event_pump = sdl_context.event_pump()
            .map_err(MegaDriveError::SDLOther)?;

        'running: loop {
            let cpu_cycles = self.cpu.step();

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => { break 'running },
                    Event::KeyDown { keycode: Some(key), .. } => {
                        match key {
                            Keycode::Return => { break 'running },
                            _ => { },
                        }
                    },
                    _ => { },
                }
            }

            thread::sleep(Duration::from_millis(200));
        }

        info!("Powering down");
        Ok(())
    }
}
