mod macros;
mod tetris;

extern crate sdl2;

use crate::tetris::{Piece, Tetris, BOARD_HEIGHT, BOARD_WIDTH};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use std::collections::HashMap;

const TILE_SIZE: u32 = 32;

const T_COLOR: Color = Color::RGBA(162, 50, 240, 255);
const I_COLOR: Color = Color::RGBA(0, 183, 235, 255);
const L_COLOR: Color = Color::RGBA(255, 117, 24, 255);
const J_COLOR: Color = Color::RGBA(0, 0, 205, 255);
const Z_COLOR: Color = Color::RGBA(220, 20, 60, 255);
const S_COLOR: Color = Color::RGBA(50, 205, 50, 255);
const O_COLOR: Color = Color::RGBA(255, 223, 0, 255);

fn render(
    canvas: &mut Canvas<Window>,
    texture_creator: &TextureCreator<WindowContext>,
    board_background_texture: &Texture,
    game_boards: &Vec<Tetris>,
) -> Result<(), String> {
    // create textures for the boards
    let mut board_textures: Vec<Texture> = Vec::new();
    for _ in game_boards.iter() {
        let mut b = texture_creator
            .create_texture_target(
                PixelFormatEnum::RGBA32,
                4 + tetris::BOARD_WIDTH as u32 * TILE_SIZE,
                4 + tetris::BOARD_HEIGHT as u32 * TILE_SIZE,
            )
            .map_err(|e| e.to_string())?;
        b.set_blend_mode(sdl2::render::BlendMode::Blend);
        board_textures.push(b);
    }
    let mut iter_vec: Vec<(&mut Texture, &Tetris)> = Vec::new();
    for t in board_textures.iter_mut().zip(game_boards.iter()) {
        iter_vec.push(t);
    }
    canvas
        .with_multiple_texture_canvas(iter_vec.iter(), |board_canvas, game| {
            game.draw_board_texture(board_canvas, 2, 2)
                .expect("couldn't draw board");
        })
        .map_err(|e| e.to_string())?;

    // copy boards onto canvas
    let mut i = 0;
    let mut dst = Rect::new(
        10,
        10,
        BOARD_WIDTH as u32 * TILE_SIZE + 4,
        BOARD_HEIGHT as u32 * TILE_SIZE + 4,
    );
    for board_texture in board_textures.iter() {
        // if i > 0 {
        //     dst = Rect::new(0, 0, 1, 1);
        // }
        canvas.copy(board_background_texture, None, dst)?;
        canvas.copy(board_texture, None, dst)?;

        // i += 1;
    }
    Ok(())
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Perfect Clear Trainer", 800, 600)
        .resizable()
        .maximized()
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

    // create board background texture
    let mut board_background_texture = texture_creator
        .create_texture_target(
            PixelFormatEnum::RGBA32,
            4 + tetris::BOARD_WIDTH as u32 * TILE_SIZE,
            4 + tetris::BOARD_HEIGHT as u32 * TILE_SIZE,
        )
        .map_err(|e| e.to_string())?;
    board_background_texture.set_blend_mode(sdl2::render::BlendMode::Blend);
    canvas
        .with_texture_canvas(&mut board_background_texture, |tex| {
            tex.set_draw_color(Color::RGBA(0, 0, 0, 0));
            tex.clear();
            tex.set_draw_color(Color::RGBA(80, 80, 80, 255));
            let mut start = 0;
            for line in 0..=BOARD_HEIGHT {
                start = match line {
                    0 => 0,
                    BOARD_HEIGHT => 2,
                    _ => 1,
                };
                tex.fill_rect(Rect::new(
                    0,
                    (line as u32 * TILE_SIZE) as i32 + start,
                    BOARD_WIDTH as u32 * TILE_SIZE + 4,
                    2,
                ))
                .expect("could not draw board bg rect");
            }
            for col in 0..=BOARD_WIDTH {
                start = match col {
                    0 => 0,
                    BOARD_WIDTH => 2,
                    _ => 1,
                };
                tex.fill_rect(Rect::new(
                    (col as u32 * TILE_SIZE) as i32 + start,
                    0,
                    2,
                    BOARD_HEIGHT as u32 * TILE_SIZE + 4,
                ))
                .expect("could not draw board bg rect");
            }
        })
        .map_err(|e| e.to_string())?;
    // let texture_map = generate_texture_map(&mut canvas, &texture_creator)?;

    let mut boards = vec![Tetris::new()];

    let mut event_pump = sdl_context.event_pump()?;
    let mut frame: u32 = 0;
    let mut scale: f32 = 3.0;
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
                    boards[0].spawn_next();
                    // boards[0].toggle_state();
                }

                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    repeat: false,
                    ..
                } => {
                    scale -= 0.1;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    repeat: false,
                    ..
                } => {
                    scale += 0.1;
                }

                _ => {}
            }
        }

        render(
            &mut canvas,
            &texture_creator,
            &board_background_texture,
            &boards,
        )?;
        canvas.present();
    }

    Ok(())
}
