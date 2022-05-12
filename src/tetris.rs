use rand::{prelude::*, seq::SliceRandom};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::collections::VecDeque;
use std::time::SystemTime;

use crate::{get_at, get_deltas, set_at, transmute_active};

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

#[derive(Copy, Clone, PartialEq, Eq)]
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Rotation {
    Spawn = 0,
    Left = 1,
    Right = 3,
    Flip = 2,
}

impl std::ops::Add for Rotation {
    type Output = Rotation;

    fn add(self, other: Rotation) -> Rotation {
        match (self as u8 + other as u8) % 4 {
            0 => Rotation::Spawn,
            1 => Rotation::Left,
            2 => Rotation::Flip,
            3 => Rotation::Right,
            _ => {
                panic!("error adding rotations");
            }
        }
    }
}

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
pub struct Tetris {
    board: [[Piece; BOARD_WIDTH]; BOARD_HEIGHT],
    row_counts: [u8; BOARD_HEIGHT],
    line_map: [usize; BOARD_HEIGHT],
    queue: VecDeque<Piece>,
    piece_active: Piece,
    col_active: usize,
    line_active: usize,
    state: State,
    rng: StdRng,
    rot_active: Rotation,
    lock_timestamp: SystemTime, // starting timestamp to calculate when a lock should occur, if not harddropped.
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
            piece_active: Piece::None,
            col_active: 0,
            line_active: 0,
            rot_active: Rotation::Spawn,
            lock_timestamp: SystemTime::UNIX_EPOCH,
        }
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn move_active(&mut self, dir: Direction) {
        // TODO: check if move is valid

        // perform move
        let (line, col) = match dir {
            Direction::Up => (self.line_active - 1, self.col_active),
            Direction::Down => (self.line_active + 1, self.col_active),
            Direction::Left => (self.line_active, self.col_active - 1),
            Direction::Right => (self.line_active, self.col_active + 1),
        };
        transmute_active!(self, line, col, self.rot_active);
    }

    pub fn rot_active(&mut self, rot: Rotation) {
        let rot_final = self.rot_active + rot;
        if let Some(tests) = match self.piece_active {
            Piece::T | Piece::J | Piece::L | Piece::S | Piece::Z => {
                match (self.rot_active, rot_final) {
                    (Rotation::Right, Rotation::Spawn) | (Rotation::Right, Rotation::Flip) => {
                        Some([(1, 0), (1, -1), (0, 2), (1, 2)])
                    }
                    (Rotation::Spawn, Rotation::Right) | (Rotation::Flip, Rotation::Right) => {
                        Some([(-1, 0), (-1, 1), (0, -2), (-1, -2)])
                    }
                    (Rotation::Flip, Rotation::Left) | (Rotation::Spawn, Rotation::Left) => {
                        Some([(1, 0), (1, 1), (0, -2), (1, -2)])
                    }
                    (Rotation::Left, Rotation::Flip) | (Rotation::Left, Rotation::Spawn) => {
                        Some([(-1, 0), (-1, -1), (0, 2), (-1, 2)])
                    }
                    _ => None,
                }
            }
            // TODO: fix I piece rotation; something's off
            Piece::I => match (self.rot_active, rot_final) {
                (Rotation::Spawn, Rotation::Right) | (Rotation::Left, Rotation::Flip) => {
                    Some([(-2, 0), (1, 0), (-2, -1), (1, 2)])
                }
                (Rotation::Right, Rotation::Spawn) | (Rotation::Flip, Rotation::Left) => {
                    Some([(2, 0), (-1, 0), (2, 1), (-1, -2)])
                }
                (Rotation::Right, Rotation::Flip) | (Rotation::Spawn, Rotation::Left) => {
                    Some([(-1, 0), (2, 0), (-1, 2), (2, -1)])
                }
                (Rotation::Flip, Rotation::Right) | (Rotation::Left, Rotation::Spawn) => {
                    Some([(1, 0), (-2, 0), (1, -2), (-2, 1)])
                }
                _ => None,
            },
            Piece::O => None,
            Piece::None => None,
        } {
            // Remove current piece from board for checks
            self.set_piece_at(
                self.piece_active,
                self.rot_active,
                Piece::None,
                self.line_active,
                self.col_active,
            );
            // Run tests
            let (a, b, c): ((i32, i32), (i32, i32), (i32, i32)) =
                get_deltas!(self.piece_active, rot_final);
            for test in [(0, 0)].into_iter().chain(tests.into_iter()) {
                if get_at!(
                    self,
                    (self.line_active as i32 + a.0 + test.0) as usize,
                    (self.col_active as i32 + a.1 + test.1) as usize
                ) == Piece::None
                    && get_at!(
                        self,
                        (self.line_active as i32 + b.0 + test.0) as usize,
                        (self.col_active as i32 + b.1 + test.1) as usize
                    ) == Piece::None
                    && get_at!(
                        self,
                        (self.line_active as i32 + c.0 + test.0) as usize,
                        (self.col_active as i32 + c.1 + test.1) as usize
                    ) == Piece::None
                {
                    // put piece in place
                    self.set_piece_at(
                        self.piece_active,
                        rot_final,
                        self.piece_active,
                        self.line_active,
                        self.col_active,
                    );
                    self.rot_active = rot_final;
                    return;
                }
            }

            // rotation unsuccessful; put piece back
            self.set_piece_at(
                self.piece_active,
                self.rot_active,
                self.piece_active,
                self.line_active,
                self.col_active,
            );
        }
    }

    pub fn update(&self, timestamp: SystemTime) {
        // TODO: lock in piece if time is correct
        // TODO: check for cleared lines
    }

    fn set_piece_at(
        &mut self,
        shape: Piece,
        shape_rot: Rotation,
        fill: Piece,
        line: usize,
        col: usize,
    ) {
        if shape == Piece::None {
            return;
        }
        set_at!(self, line, col, fill);
        let (a, b, c): ((i32, i32), (i32, i32), (i32, i32)) = get_deltas!(shape, shape_rot);
        set_at!(
            self,
            (line as i32 + a.0) as usize,
            (col as i32 + a.1) as usize,
            fill
        );
        set_at!(
            self,
            (line as i32 + b.0) as usize,
            (col as i32 + b.1) as usize,
            fill
        );
        set_at!(
            self,
            (line as i32 + c.0) as usize,
            (col as i32 + c.1) as usize,
            fill
        );
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
        self.piece_active = self.queue.pop_front().unwrap_or(Piece::None);
        if self.queue.len() < 7 {
            self.queue_add_bag();
        }
        self.col_active = 4;
        self.line_active = 3;
        self.set_piece_at(
            self.piece_active,
            Rotation::Spawn,
            self.piece_active,
            self.line_active,
            self.col_active,
        );
        println!("spawn piece {:?}", self.piece_active);
    }
}
