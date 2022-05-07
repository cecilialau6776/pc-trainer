mod macros;
mod tetris;

extern crate sdl2;

use crate::tetris::{Piece, Tetris};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use std::collections::HashMap;

const TILE_SIZE: u32 = 16;

const T_COLOR: Color = Color::RGB(161, 50, 240);
const I_COLOR: Color = Color::RGB(0, 183, 235);
const L_COLOR: Color = Color::RGB(255, 117, 24);
const J_COLOR: Color = Color::RGB(0, 0, 205);
const Z_COLOR: Color = Color::RGB(220, 20, 60);
const S_COLOR: Color = Color::RGB(50, 205, 50);
const O_COLOR: Color = Color::RGB(255, 223, 0);

fn generate_texture_map<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Result<HashMap<Piece, Texture<'a>>, String> {
    let mut t_texture = texture_creator
        .create_texture_target(None, TILE_SIZE * 3, TILE_SIZE * 2)
        .map_err(|e| e.to_string())?;
    let mut i_texture = texture_creator
        .create_texture_target(None, TILE_SIZE * 4, TILE_SIZE * 1)
        .map_err(|e| e.to_string())?;
    let mut l_texture = texture_creator
        .create_texture_target(None, TILE_SIZE * 3, TILE_SIZE * 2)
        .map_err(|e| e.to_string())?;
    let mut j_texture = texture_creator
        .create_texture_target(None, TILE_SIZE * 3, TILE_SIZE * 2)
        .map_err(|e| e.to_string())?;
    let mut z_texture = texture_creator
        .create_texture_target(None, TILE_SIZE * 3, TILE_SIZE * 2)
        .map_err(|e| e.to_string())?;
    let mut s_texture = texture_creator
        .create_texture_target(None, TILE_SIZE * 3, TILE_SIZE * 2)
        .map_err(|e| e.to_string())?;
    let mut o_texture = texture_creator
        .create_texture_target(None, TILE_SIZE * 2, TILE_SIZE * 2)
        .map_err(|e| e.to_string())?;

    {
        let textures = vec![
            (&mut t_texture, Piece::T),
            (&mut i_texture, Piece::I),
            (&mut l_texture, Piece::L),
            (&mut j_texture, Piece::J),
            (&mut z_texture, Piece::Z),
            (&mut s_texture, Piece::S),
            (&mut o_texture, Piece::O),
        ];
        match canvas.with_multiple_texture_canvas(textures.iter(), |texture_canvas, piece| {
            texture_canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
            texture_canvas.clear();
            match *piece {
                Piece::T => texture!(texture_canvas, T_COLOR, TILE_SIZE),
                Piece::I => texture!(texture_canvas, I_COLOR, TILE_SIZE),
                Piece::J => texture!(texture_canvas, J_COLOR, TILE_SIZE),
                Piece::L => texture!(texture_canvas, L_COLOR, TILE_SIZE),
                Piece::S => texture!(texture_canvas, S_COLOR, TILE_SIZE),
                Piece::Z => texture!(texture_canvas, Z_COLOR, TILE_SIZE),
                Piece::O => texture!(texture_canvas, O_COLOR, TILE_SIZE),
                Piece::None => {}
            }
        }) {
            Ok(()) => {}
            Err(e) => return Err(e.to_string()),
        };
    }

    let mut texture_map = HashMap::new();
    texture_map.insert(Piece::T, t_texture);
    texture_map.insert(Piece::I, i_texture);
    texture_map.insert(Piece::L, l_texture);
    texture_map.insert(Piece::J, j_texture);
    texture_map.insert(Piece::Z, z_texture);
    texture_map.insert(Piece::S, s_texture);
    texture_map.insert(Piece::O, o_texture);
    Ok(texture_map)
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Perfect Clear Trainer", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    println!(" Using SDL_Renderer \"{}\"", canvas.info().name);
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let textureMap = generate_texture_map(&mut canvas, &texture_creator)?;
    let mut game = Tetris::new();

    let mut event_pump = sdl_context.event_pump()?;
    let mut frame: u32 = 0;
    'running: loop {
        // handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    repeat: false,
                    ..
                } => {
                    game.toggle_state();
                }

                _ => {}
            }
        }

        // draw board
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for (x, row) in game.rows_iter().enumerate() {
            for (y, piece) in row.enumerate() {
                if let Some(texture) = textureMap.get(piece) {
                    canvas.copy(
                        texture,
                        None,
                        Rect::new(
                            (x as u32 * TILE_SIZE) as i32,
                            (y as u32 * TILE_SIZE) as i32,
                            TILE_SIZE,
                            TILE_SIZE,
                        ),
                    )?;
                }
            }
        }
        canvas.present();
    }

    Ok(())
}
