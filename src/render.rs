use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::Sdl;

use crate::tetris::{Tetris, BOARD_HEIGHT, BOARD_WIDTH};
use crate::TILE_SIZE;

const DEFAULT_BOARD_TEXTURE_PARAMS: (PixelFormatEnum, u32, u32) = (
    PixelFormatEnum::RGBA32,
    4 + crate::tetris::BOARD_WIDTH as u32 * crate::TILE_SIZE,
    4 + crate::tetris::BOARD_HEIGHT as u32 * crate::TILE_SIZE,
);

pub struct Renderer<'a> {
    sdl_context: Sdl,
    // window: Window,
    texture_creator: Option<&'a TextureCreator<WindowContext>>,
    background_texture: Option<Texture<'a>>,
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
                texture_creator: None,
                background_texture: None,
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

        // create board background texture
        let mut board_background_texture = Renderer::make_texture(texture_creator, None)?;
        self.canvas
            .with_texture_canvas(&mut board_background_texture, |tex| {
                tex.set_draw_color(Color::RGBA(0, 0, 0, 0));
                tex.clear();
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
            })
            .map_err(|e| e.to_string())?;

        self.background_texture = Some(board_background_texture);
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
        let (ww, wh) = self.canvas.window().size();
        let w = BOARD_WIDTH as u32 * TILE_SIZE + 4;
        let h = BOARD_HEIGHT as u32 * TILE_SIZE + 4;
        let main_board_dst = Rect::new(ww as i32 / 2 - w as i32 / 2, 10, w, h);
        self.canvas.copy(
            self.background_texture
                .as_ref()
                .expect("bg tex not initialized"),
            None,
            main_board_dst,
        )?;
        self.canvas
            .copy(&main_board_texture, None, main_board_dst)?;
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
