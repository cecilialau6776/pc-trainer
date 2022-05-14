use std::collections::HashMap;

use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::Sdl;

use crate::tetris::{Piece, Rotation, Tetris, BOARD_HEIGHT, BOARD_WIDTH};
use crate::{get_deltas, TILE_SIZE};

const BOARD_BACKGROUND: &str = "board_bg";

const DEFAULT_BOARD_TEXTURE_PARAMS: (PixelFormatEnum, u32, u32) = (
    PixelFormatEnum::RGBA32,
    4 + crate::tetris::BOARD_WIDTH as u32 * crate::TILE_SIZE,
    4 + crate::tetris::BOARD_HEIGHT as u32 * crate::TILE_SIZE,
);

pub struct Renderer<'a> {
    sdl_context: Sdl,
    // window: Window,
    texture_creator: Option<&'a TextureCreator<WindowContext>>,
    textures: HashMap<String, Texture<'a>>,
    // background_texture: Option<Texture<'a>>,
    canvas: Canvas<Window>,
}

impl<'a> Renderer<'a> {
    pub fn new() -> Result<(TextureCreator<WindowContext>, Self), String> {
        let sdl_context = sdl2::init()?;
        let window = sdl_context
            .video()?
            .window("Perfect Clear Trainer", 800, 600)
            .resizable()
            .maximized()
            .build()
            .map_err(|e| e.to_string())?;
        let canvas = window
            .into_canvas()
            .target_texture()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())?;
        let texture_creator = canvas.texture_creator();
        // let mut tm = TextureManager::new(&texture_creator);
        Ok((
            texture_creator,
            Renderer {
                sdl_context,
                canvas,
                textures: HashMap::new(),
                texture_creator: None,
                // background_texture: None,
            },
        ))
    }

    pub fn init(
        &mut self,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Result<(), String> {
        // let mut renderer = Renderer::new()?;

        // clear canvas
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.present();

        // create textures
        let piece_options = Some((PixelFormatEnum::RGBA32, 4 * TILE_SIZE, 3 * TILE_SIZE));
        let mut textures = vec![
            (Renderer::make_texture(texture_creator, None)?),
            (Renderer::make_texture(texture_creator, piece_options)?),
            (Renderer::make_texture(texture_creator, piece_options)?),
            (Renderer::make_texture(texture_creator, piece_options)?),
            (Renderer::make_texture(texture_creator, piece_options)?),
            (Renderer::make_texture(texture_creator, piece_options)?),
            (Renderer::make_texture(texture_creator, piece_options)?),
            (Renderer::make_texture(texture_creator, piece_options)?),
        ];
        let details = vec![BOARD_BACKGROUND, "t", "i", "j", "l", "s", "z", "o"];
        let mut iter_vec = Vec::new();
        for (texture, str) in textures.iter_mut().zip(details.iter()) {
            iter_vec.push((texture, *str));
        }

        self.canvas
            .with_multiple_texture_canvas(iter_vec.iter(), |tex, details| {
                tex.set_draw_color(Color::RGBA(0, 0, 0, 0));
                tex.clear();
                if *details == BOARD_BACKGROUND {
                    tex.set_draw_color(Color::RGBA(80, 80, 80, 255));
                    let mut start;
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
                } else {
                    if let Some((deltas, x_offset, y_offset, color)) = match *details {
                        "t" => Some((
                            get_deltas!(Piece::T, Rotation::Spawn),
                            3 * TILE_SIZE / 2,
                            3 * TILE_SIZE / 2,
                            crate::T_COLOR,
                        )),
                        "i" => Some((
                            get_deltas!(Piece::I, Rotation::Spawn),
                            TILE_SIZE,
                            TILE_SIZE,
                            crate::I_COLOR,
                        )),
                        "j" => Some((
                            get_deltas!(Piece::J, Rotation::Spawn),
                            3 * TILE_SIZE / 2,
                            3 * TILE_SIZE / 2,
                            crate::J_COLOR,
                        )),
                        "l" => Some((
                            get_deltas!(Piece::L, Rotation::Spawn),
                            3 * TILE_SIZE / 2,
                            3 * TILE_SIZE / 2,
                            crate::L_COLOR,
                        )),
                        "s" => Some((
                            get_deltas!(Piece::S, Rotation::Spawn),
                            3 * TILE_SIZE / 2,
                            3 * TILE_SIZE / 2,
                            crate::S_COLOR,
                        )),
                        "z" => Some((
                            get_deltas!(Piece::Z, Rotation::Spawn),
                            3 * TILE_SIZE / 2,
                            3 * TILE_SIZE / 2,
                            crate::Z_COLOR,
                        )),
                        "o" => Some((
                            get_deltas!(Piece::O, Rotation::Spawn),
                            TILE_SIZE,
                            TILE_SIZE / 2,
                            crate::O_COLOR,
                        )),
                        _ => None,
                    } {
                        tex.set_draw_color(color);
                        tex.fill_rect(Rect::new(
                            x_offset as i32,
                            y_offset as i32,
                            TILE_SIZE,
                            TILE_SIZE,
                        ))
                        .expect("could not draw piece texture");
                        tex.fill_rect(Rect::new(
                            x_offset as i32 + TILE_SIZE as i32 * deltas.0 .1,
                            y_offset as i32 + TILE_SIZE as i32 * deltas.0 .0,
                            TILE_SIZE,
                            TILE_SIZE,
                        ))
                        .expect("could not draw piece texture");
                        tex.fill_rect(Rect::new(
                            x_offset as i32 + TILE_SIZE as i32 * deltas.1 .1,
                            y_offset as i32 + TILE_SIZE as i32 * deltas.1 .0,
                            TILE_SIZE,
                            TILE_SIZE,
                        ))
                        .expect("could not draw piece texture");
                        tex.fill_rect(Rect::new(
                            x_offset as i32 + TILE_SIZE as i32 * deltas.2 .1,
                            y_offset as i32 + TILE_SIZE as i32 * deltas.2 .0,
                            TILE_SIZE,
                            TILE_SIZE,
                        ))
                        .expect("could not draw piece texture");
                    }
                }
            })
            .map_err(|e| e.to_string())?;

        for (texture, name) in textures.into_iter().zip(details.into_iter()) {
            self.textures.insert(name.to_string(), texture);
        }
        self.texture_creator = Some(texture_creator);
        Ok(())
    }

    pub fn sdl_context(&self) -> &Sdl {
        &self.sdl_context
    }

    pub fn render(&mut self, game_boards: &[Tetris]) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        let mut main_board_texture: Texture = Renderer::make_texture(
            self.texture_creator.expect("tex creator not initalized"),
            None,
        )?;
        let iter_vec = vec![(&mut main_board_texture, &game_boards[0])];
        self.canvas
            .with_multiple_texture_canvas(iter_vec.iter(), |c, game| {
                game.draw_board_texture(c, 2, 2, true)
                    .expect("couldn't draw board");
            })
            .map_err(|e| e.to_string())?;

        // render main board
        let (ww, wh) = self.canvas.window().size();
        let w = BOARD_WIDTH as u32 * TILE_SIZE + 4;
        let h = BOARD_HEIGHT as u32 * TILE_SIZE + 4;
        let main_board_dst = Rect::new(ww as i32 / 2 - w as i32 / 2, 10, w, h);
        self.canvas.copy(
            self.textures
                .get(BOARD_BACKGROUND)
                .as_ref()
                .expect("bg tex not initialized"),
            None,
            main_board_dst,
        )?;
        self.canvas
            .copy(&main_board_texture, None, main_board_dst)?;

        // render hold piece
        // TODO: render box around hold piece
        if let Some(piece) = game_boards[0].get_hold() {
            let hold_dst = Rect::new(
                main_board_dst.x() - (5 * TILE_SIZE as i32),
                main_board_dst.y(),
                4 * TILE_SIZE,
                3 * TILE_SIZE,
            );
            let texture = match piece {
                Piece::T => self.textures.get("t").expect("no piece texture"),
                Piece::I => self.textures.get("i").expect("no piece texture"),
                Piece::J => self.textures.get("j").expect("no piece texture"),
                Piece::L => self.textures.get("l").expect("no piece texture"),
                Piece::S => self.textures.get("s").expect("no piece texture"),
                Piece::Z => self.textures.get("z").expect("no piece texture"),
                Piece::O => self.textures.get("o").expect("no piece texture"),
                Piece::None => {
                    panic!("get_hold() returned Piece::None");
                }
            };
            self.canvas.copy(texture, None, hold_dst)?;
        }

        // render queue
        let mut piece_dst = Rect::new(
            main_board_dst.right() + TILE_SIZE as i32,
            main_board_dst.y(),
            4 * TILE_SIZE,
            3 * TILE_SIZE,
        );
        for piece in game_boards[0].get_queue() {
            let texture = match piece {
                Piece::T => self.textures.get("t").expect("no piece texture"),
                Piece::I => self.textures.get("i").expect("no piece texture"),
                Piece::J => self.textures.get("j").expect("no piece texture"),
                Piece::L => self.textures.get("l").expect("no piece texture"),
                Piece::S => self.textures.get("s").expect("no piece texture"),
                Piece::Z => self.textures.get("z").expect("no piece texture"),
                Piece::O => self.textures.get("o").expect("no piece texture"),
                Piece::None => {
                    panic!("get_hold() returned Piece::None");
                }
            };
            self.canvas.copy(texture, None, piece_dst)?;
            piece_dst.offset(0, 3 * TILE_SIZE as i32);
        }
        self.canvas.present();
        Ok(())
    }

    fn make_texture(
        texture_creator: &'a TextureCreator<WindowContext>,
        details: Option<(PixelFormatEnum, u32, u32)>,
    ) -> Result<Texture, String> {
        let params = details.unwrap_or(DEFAULT_BOARD_TEXTURE_PARAMS);
        let mut tex = texture_creator
            .create_texture_target(params.0, params.1, params.2)
            .map_err(|e| e.to_string())?;
        tex.set_blend_mode(sdl2::render::BlendMode::Blend);
        Ok(tex)
    }
}
