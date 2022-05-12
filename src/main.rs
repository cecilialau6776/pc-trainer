mod input_manager;
mod macros;
mod render;
mod tetris;

extern crate sdl2;

use crate::tetris::Tetris;
use input_manager::InputManager;
use render::Renderer;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::pixels::Color;
use std::time::SystemTime;

const TILE_SIZE: u32 = 32;

const T_COLOR: Color = Color::RGBA(162, 50, 240, 255);
const I_COLOR: Color = Color::RGBA(0, 183, 235, 255);
const L_COLOR: Color = Color::RGBA(255, 117, 24, 255);
const J_COLOR: Color = Color::RGBA(0, 0, 205, 255);
const Z_COLOR: Color = Color::RGBA(220, 20, 60, 255);
const S_COLOR: Color = Color::RGBA(50, 205, 50, 255);
const O_COLOR: Color = Color::RGBA(255, 223, 0, 255);

pub fn main() -> Result<(), String> {
    let mut boards = [Tetris::new()];

    let (texture_creator, mut renderer) = Renderer::new()?;
    renderer.init(&texture_creator)?;

    let mut event_pump = renderer.sdl_context().event_pump()?;

    let event_subsystem = renderer.sdl_context().event().map_err(|e| e.to_string())?;
    let mut input_manager = InputManager::new();
    // game loop
    let mut current_time = SystemTime::now();
    'game_loop: loop {
        // calculate frame time and fps
        let new_time = SystemTime::now();
        let frame_time = new_time
            .duration_since(current_time)
            .map_err(|e| e.to_string())?;
        current_time = new_time;
        let fps = (1.0 / frame_time.as_secs_f64()) as u32;

        // handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'game_loop,
                Event::KeyDown {
                    scancode: Some(Scancode::Space),
                    ..
                } => boards[0].spawn_next(),
                _ => input_manager.process_input(event, &mut boards[0]),
            }
        }

        // update playing board
        // boards[0].update(&current_time);

        renderer.render(&boards)?;
    }

    Ok(())
}
