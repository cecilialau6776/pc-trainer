use rand::{prelude::*, seq::SliceRandom};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::collections::VecDeque;

pub const BOARD_HEIGHT: usize = 24;
pub const BOARD_WIDTH: usize = 10;
const PIECES: [Piece; 7] = [
    Piece::T,
    Piece::I,
    Piece::J,
    Piece::L,
    Piece::S,
    Piece::Z,
    Piece::O,
];

#[derive(Copy, Clone)]
pub enum State {
    Paused,
    Playing,
}

#[derive(Clone, PartialEq, Eq, Hash, Copy, Debug)]
pub enum Piece {
    T,
    I,
    J,
    L,
    S,
    Z,
    O,
    None,
}

#[derive(Clone)]
pub struct Tetris {
    board: [[Piece; BOARD_WIDTH]; BOARD_HEIGHT],
    row_counts: [u8; BOARD_HEIGHT],
    line_map: [usize; BOARD_HEIGHT],
    queue: VecDeque<Piece>,
    active_piece: Piece,
    col_active: usize,
    line_active: usize,
    state: State,
    rng: StdRng,
}

impl Tetris {
    pub fn new() -> Tetris {
        // init line map
        let mut line_map = [0; BOARD_HEIGHT];
        for i in 0..BOARD_HEIGHT {
            line_map[i] = i;
        }

        // Fill initial queue
        let mut queue: VecDeque<Piece> = VecDeque::new();
        let mut rng: StdRng = SeedableRng::from_entropy();
        let mut pieces_clone = PIECES.clone();
        pieces_clone.shuffle(&mut rng);
        queue.append(&mut VecDeque::from_iter(pieces_clone));
        let mut pieces_clone = PIECES.clone();
        pieces_clone.shuffle(&mut rng);
        queue.append(&mut VecDeque::from_iter(pieces_clone));

        Tetris {
            board: [[Piece::None; BOARD_WIDTH]; BOARD_HEIGHT],
            row_counts: [0; BOARD_HEIGHT],
            line_map,
            queue,
            state: State::Playing,
            rng,
            active_piece: Piece::None,
            col_active: 0,
            line_active: 0,
        }
    }

    pub fn toggle_state(&mut self) {
        match self.state {
            State::Playing => self.state = State::Paused,
            State::Paused => self.state = State::Playing,
        }
    }

    pub fn get(&self, line: usize, col: usize) -> Piece {
        self.board[self.line_map[line]][col]
    }

    fn queue_add_bag(&mut self) {
        let mut pieces_clone = PIECES.clone();
        pieces_clone.shuffle(&mut self.rng);
        self.queue.append(&mut VecDeque::from_iter(pieces_clone));
    }

    pub fn draw_board_texture(
        &self,
        texture_canvas: &mut Canvas<Window>,
        x_offset: i32,
        y_offset: i32,
    ) -> Result<(), String> {
        texture_canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
        texture_canvas.clear();
        for line in 0..BOARD_HEIGHT {
            for col in 0..BOARD_WIDTH {
                match self.board[self.line_map[line]][col] {
                    Piece::T => texture_canvas.set_draw_color(crate::T_COLOR),
                    Piece::I => texture_canvas.set_draw_color(crate::I_COLOR),
                    Piece::J => texture_canvas.set_draw_color(crate::J_COLOR),
                    Piece::L => texture_canvas.set_draw_color(crate::L_COLOR),
                    Piece::S => texture_canvas.set_draw_color(crate::S_COLOR),
                    Piece::Z => texture_canvas.set_draw_color(crate::Z_COLOR),
                    Piece::O => texture_canvas.set_draw_color(crate::O_COLOR),
                    Piece::None => continue,
                }
                texture_canvas.fill_rect(Rect::new(
                    x_offset + (col as u32 * crate::TILE_SIZE) as i32,
                    y_offset + (line as u32 * crate::TILE_SIZE) as i32,
                    crate::TILE_SIZE,
                    crate::TILE_SIZE,
                ))?;
            }
        }
        Ok(())
    }

    // TODO: remove pub
    pub fn spawn_next(&mut self) {
        // TODO: check for top-out
        self.active_piece = self.queue.pop_front().unwrap_or(Piece::None);
        if self.queue.len() < 7 {
            self.queue_add_bag();
        }
        self.col_active = 4;
        self.line_active = 1;
        match self.active_piece {
            Piece::T => {
                self.board[self.line_map[2]][4] = self.active_piece;
                self.board[self.line_map[3]][3] = self.active_piece;
                self.board[self.line_map[3]][4] = self.active_piece;
                self.board[self.line_map[3]][5] = self.active_piece;
            }
            Piece::I => {
                self.board[self.line_map[3]][3] = self.active_piece;
                self.board[self.line_map[3]][4] = self.active_piece;
                self.board[self.line_map[3]][5] = self.active_piece;
                self.board[self.line_map[3]][6] = self.active_piece;
            }
            Piece::J => {
                self.board[self.line_map[2]][3] = self.active_piece;
                self.board[self.line_map[3]][3] = self.active_piece;
                self.board[self.line_map[3]][4] = self.active_piece;
                self.board[self.line_map[3]][5] = self.active_piece;
            }
            Piece::L => {
                self.board[self.line_map[2]][5] = self.active_piece;
                self.board[self.line_map[3]][3] = self.active_piece;
                self.board[self.line_map[3]][4] = self.active_piece;
                self.board[self.line_map[3]][5] = self.active_piece;
            }
            Piece::S => {
                self.board[self.line_map[2]][4] = self.active_piece;
                self.board[self.line_map[2]][5] = self.active_piece;
                self.board[self.line_map[3]][3] = self.active_piece;
                self.board[self.line_map[3]][4] = self.active_piece;
            }
            Piece::Z => {
                self.board[self.line_map[2]][3] = self.active_piece;
                self.board[self.line_map[2]][4] = self.active_piece;
                self.board[self.line_map[3]][4] = self.active_piece;
                self.board[self.line_map[3]][5] = self.active_piece;
            }
            Piece::O => {
                self.board[self.line_map[2]][4] = self.active_piece;
                self.board[self.line_map[2]][5] = self.active_piece;
                self.board[self.line_map[3]][4] = self.active_piece;
                self.board[self.line_map[3]][5] = self.active_piece;
            }
            Piece::None => {}
        }
        println!("spawn piece {:?}", self.active_piece);
    }
}
