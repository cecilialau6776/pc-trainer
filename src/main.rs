mod macros;
mod render;
mod tetris;

extern crate sdl2;

use crate::tetris::Tetris;
use render::Renderer;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
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

// fn render(
//     canvas: &mut Canvas<Window>,
//     texture_creator: &TextureCreator<WindowContext>,
//     board_background_texture: &Texture,
//     game_boards: &Vec<Tetris>,
// ) -> Result<(), String> {
//     // create textures for the boards
//     let mut board_textures: Vec<Texture> = Vec::new();
//     for _ in game_boards.iter() {
//         let mut b = texture_creator
//             .create_texture_target(
//                 PixelFormatEnum::RGBA32,
//                 4 + tetris::BOARD_WIDTH as u32 * TILE_SIZE,
//                 4 + tetris::BOARD_HEIGHT as u32 * TILE_SIZE,
//             )
//             .map_err(|e| e.to_string())?;
//         b.set_blend_mode(sdl2::render::BlendMode::Blend);
//         board_textures.push(b);
//     }
//     let mut iter_vec: Vec<(&mut Texture, &Tetris)> = Vec::new();
//     for t in board_textures.iter_mut().zip(game_boards.iter()) {
//         iter_vec.push(t);
//     }
//     canvas
//         .with_multiple_texture_canvas(iter_vec.iter(), |board_canvas, game| {
//             game.draw_board_texture(board_canvas, 2, 2)
//                 .expect("couldn't draw board");
//         })
//         .map_err(|e| e.to_string())?;

//     // copy boards onto canvas
//     let mut i = 0;
//     let mut dst = Rect::new(
//         10,
//         10,
//         BOARD_WIDTH as u32 * TILE_SIZE + 4,
//         BOARD_HEIGHT as u32 * TILE_SIZE + 4,
//     );
//     for board_texture in board_textures.iter() {
//         // if i > 0 {
//         //     dst = Rect::new(0, 0, 1, 1);
//         // }
//         canvas.copy(board_background_texture, None, dst)?;
//         canvas.copy(board_texture, None, dst)?;

//         // i += 1;
//     }
//     Ok(())
// }

pub fn main() -> Result<(), String> {
    // TODO: move rendering to seperate module
    let mut boards = [Tetris::new()];

    let (texture_creator, mut renderer) = Renderer::new()?;
    renderer.init(&texture_creator)?;

    let mut event_pump = renderer.sdl_context().event_pump()?;

    let event_subsystem = renderer.sdl_context().event().map_err(|e| e.to_string())?;
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
                // TODO: process in event handler
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    repeat: false,
                    ..
                } => boards[0].spawn_next(),
                _ => {}
            }
        }

        // update playing board
        // boards[0].update(&current_time);

        renderer.render(&boards)?;
    }

    Ok(())
}
