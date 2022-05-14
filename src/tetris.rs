use rand::{prelude::*, seq::SliceRandom};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::collections::VecDeque;
use std::time::SystemTime;

use crate::{get_at, get_color, get_deltas, set_at};

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
    current_gravity: u32,       // time in ms between the active piece moving down a cell
    gravity: u32,               // time in ms between the active piece moving down a cell
    gravity_timestamp: SystemTime,
    swap_piece: Piece,
    swapped: bool,
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
        // queue.push_back(Piece::I);
        // queue.push_back(Piece::S);
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
            current_gravity: 0,
            gravity: 0,
            gravity_timestamp: SystemTime::UNIX_EPOCH,
            swap_piece: Piece::None,
            swapped: false,
        }
    }

    pub fn start(&mut self) {
        self.state = State::Playing;
        self.spawn_next(None);
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn softdrop_instant(&mut self, timestamp: SystemTime) {
        self.lock_timestamp = timestamp;
        while self.move_active(Direction::Down) {}
        // self.move_active(Direction::Down);
    }

    pub fn softdrop_start(&mut self, sdf: u32) {
        self.current_gravity /= sdf;
    }

    pub fn softdrop_stop(&mut self) {
        self.current_gravity = self.gravity;
    }

    pub fn harddrop(&mut self) {
        while self.move_active(Direction::Down) {}
        self.lock_active();
    }

    pub fn swap(&mut self) {
        if !self.swapped {
            self.swapped = true;
            self.set_piece_at(
                self.piece_active,
                self.rot_active,
                Piece::None,
                self.line_active,
                self.col_active,
            );
            let temp = self.swap_piece;
            self.swap_piece = self.piece_active;
            self.piece_active = temp;
            if self.piece_active == Piece::None {
                self.spawn_next(None);
            } else {
                self.spawn_next(Some(self.piece_active));
            }
        }
    }

    fn lock_active(&mut self) {
        self.swapped = false;
        let ((a, _), (b, _), (c, _)) = get_deltas!(self.piece_active, self.rot_active);
        // update row counts
        self.row_counts[self.line_map[self.line_active]] += 1;
        self.row_counts[self.line_map[(self.line_active as i32 + a) as usize]] += 1;
        self.row_counts[self.line_map[(self.line_active as i32 + b) as usize]] += 1;
        self.row_counts[self.line_map[(self.line_active as i32 + c) as usize]] += 1;
        // println!("{:?}", self.row_counts);
        // TODO: do something with lines cleared
        let mut lines_cleared = 0;
        let mut i = BOARD_HEIGHT - 1;
        while i > BOARD_HEIGHT - 20 {
            if self.row_counts[self.line_map[i]] == 10 {
                lines_cleared += 1;
                // println!("{:?}", (1..=i).rev());
                for j in (1..=i).rev() {
                    self.row_counts[self.line_map[j]] = self.row_counts[self.line_map[j - 1]];
                    self.board[self.line_map[j]] = self.board[self.line_map[j - 1]];
                }
                self.row_counts[self.line_map[0]] = 0;
                self.board[self.line_map[0]] = [Piece::None; BOARD_WIDTH];
                continue;
            }
            i -= 1;
        }
        // spawn piece
        self.spawn_next(None);
    }

    pub fn move_active(&mut self, dir: Direction) -> bool {
        let (a, b, c) = get_deltas!(self.piece_active, self.rot_active);
        let la = self.line_active as i32;
        let ca = self.col_active as i32;
        self.set_piece_at(
            self.piece_active,
            self.rot_active,
            Piece::None,
            self.line_active,
            self.col_active,
        );
        if match dir {
            Direction::Down => {
                get_at!(self, (la + 1), ca) != Piece::None
                    || get_at!(self, (la + 1 + a.0), (ca + a.1)) != Piece::None
                    || get_at!(self, (la + 1 + b.0), (ca + b.1)) != Piece::None
                    || get_at!(self, (la + 1 + c.0), (ca + c.1)) != Piece::None
            }
            Direction::Left => {
                get_at!(self, (la), (ca - 1)) != Piece::None
                    || get_at!(self, (la + a.0), (ca - 1 + a.1)) != Piece::None
                    || get_at!(self, (la + b.0), (ca - 1 + b.1)) != Piece::None
                    || get_at!(self, (la + c.0), (ca - 1 + c.1)) != Piece::None
            }
            Direction::Right => {
                get_at!(self, (la), (ca + 1)) != Piece::None
                    || get_at!(self, (la + a.0), (ca + 1 + a.1)) != Piece::None
                    || get_at!(self, (la + b.0), (ca + 1 + b.1)) != Piece::None
                    || get_at!(self, (la + c.0), (ca + 1 + c.1)) != Piece::None
            }
        } {
            self.set_piece_at(
                self.piece_active,
                self.rot_active,
                self.piece_active,
                self.line_active,
                self.col_active,
            );
            return false;
        }
        // perform move
        let (line, col) = match dir {
            Direction::Down => (self.line_active + 1, self.col_active),
            Direction::Left => (self.line_active, self.col_active - 1),
            Direction::Right => (self.line_active, self.col_active + 1),
        };
        self.line_active = line;
        self.col_active = col;
        self.set_piece_at(
            self.piece_active,
            self.rot_active,
            self.piece_active,
            self.line_active,
            self.col_active,
        );
        true
    }

    pub fn rot_active(&mut self, rot: Rotation) {
        let rot_final = self.rot_active + rot;
        if rot == Rotation::Flip {
            // remove active from board for tests
            self.set_piece_at(
                self.piece_active,
                self.rot_active,
                Piece::None,
                self.line_active,
                self.col_active,
            );

            let la = self.line_active as i32;
            let ca = self.col_active as i32;
            let (a, b, c) = get_deltas!(self.piece_active, rot_final);
            self.rot_active = if get_at!(self, (la), (ca)) == Piece::None
                && get_at!(self, (la + a.0), (ca + a.1)) == Piece::None
                && get_at!(self, (la + b.0), (ca + b.1)) == Piece::None
                && get_at!(self, (la + c.0), (ca + c.1)) == Piece::None
            {
                rot_final
            } else {
                self.rot_active
            };
            self.set_piece_at(
                self.piece_active,
                self.rot_active,
                self.piece_active,
                self.line_active,
                self.col_active,
            );

            return;
        }
        if let Some(tests) = match self.piece_active {
            Piece::T | Piece::J | Piece::L | Piece::S | Piece::Z => {
                match (self.rot_active, rot_final) {
                    (Rotation::Right, Rotation::Spawn) | (Rotation::Right, Rotation::Flip) => {
                        Some([(0, 1), (1, 1), (-2, 0), (-2, 1)])
                    }
                    (Rotation::Spawn, Rotation::Right) | (Rotation::Flip, Rotation::Right) => {
                        Some([(0, -1), (-1, -1), (2, 0), (2, -1)])
                    }
                    (Rotation::Flip, Rotation::Left) | (Rotation::Spawn, Rotation::Left) => {
                        Some([(0, 1), (-1, 1), (2, 0), (2, 1)])
                    }
                    (Rotation::Left, Rotation::Flip) | (Rotation::Left, Rotation::Spawn) => {
                        Some([(0, -1), (1, -1), (-2, 0), (-2, -1)])
                    }
                    _ => None,
                }
            }

            // TODO: fix I piece rotation; something's off
            Piece::I => match (self.rot_active, rot_final) {
                (Rotation::Spawn, Rotation::Right) | (Rotation::Left, Rotation::Flip) => {
                    Some([(0, -2), (0, 1), (1, -2), (-2, 1)])
                }
                (Rotation::Right, Rotation::Spawn) | (Rotation::Flip, Rotation::Left) => {
                    Some([(0, 2), (0, -1), (-1, 2), (2, -1)])
                }
                (Rotation::Right, Rotation::Flip) | (Rotation::Spawn, Rotation::Left) => {
                    Some([(0, -1), (0, 2), (-2, -1), (1, 2)])
                }
                (Rotation::Flip, Rotation::Right) | (Rotation::Left, Rotation::Spawn) => {
                    Some([(0, 1), (0, -2), (2, 1), (-1, -2)])
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
            let la = self.line_active as i32;
            let ca = self.col_active as i32;
            for test in std::iter::once((0, 0)).chain(tests.into_iter()) {
                if get_at!(self, (la + test.0), (ca + test.1)) == Piece::None
                    && get_at!(self, (la + a.0 + test.0), (ca + a.1 + test.1)) == Piece::None
                    && get_at!(self, (la + b.0 + test.0), (ca + b.1 + test.1)) == Piece::None
                    && get_at!(self, (la + c.0 + test.0), (ca + c.1 + test.1)) == Piece::None
                {
                    // put piece in place
                    self.set_piece_at(
                        self.piece_active,
                        rot_final,
                        self.piece_active,
                        (la + test.0) as usize,
                        (ca + test.1) as usize,
                    );
                    self.line_active = (la + test.0) as usize;
                    self.col_active = (ca + test.1) as usize;
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

    pub fn get_hold(&self) -> Option<Piece> {
        match self.swap_piece {
            Piece::None => None,
            _ => Some(self.swap_piece),
        }
    }

    pub fn get_queue(&self) -> [Piece; 5] {
        [
            *self.queue.get(0).unwrap(),
            *self.queue.get(1).unwrap(),
            *self.queue.get(2).unwrap(),
            *self.queue.get(3).unwrap(),
            *self.queue.get(4).unwrap(),
        ]
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
        project: bool,
    ) -> Result<(), String> {
        texture_canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
        texture_canvas.clear();
        if project {
            let (a, b, c) = get_deltas!(self.piece_active, self.rot_active);
            let la = self.line_active as i32;
            let ca = self.col_active as i32;
            let orig = vec![
                (la, ca),
                (la + a.0, ca + a.1),
                (la + b.0, ca + b.1),
                (la + c.0, ca + c.1),
            ];
            let mut piece = orig.clone();
            while (orig.contains(&piece[0]) || get_at!(self, piece[0].0, piece[0].1) == Piece::None)
                && (orig.contains(&piece[1])
                    || get_at!(self, piece[1].0, piece[1].1) == Piece::None)
                && (orig.contains(&piece[2])
                    || get_at!(self, piece[2].0, piece[2].1) == Piece::None)
                && (orig.contains(&piece[3])
                    || get_at!(self, piece[3].0, piece[3].1) == Piece::None)
            {
                piece[0] = (piece[0].0 + 1, piece[0].1);
                piece[1] = (piece[1].0 + 1, piece[1].1);
                piece[2] = (piece[2].0 + 1, piece[2].1);
                piece[3] = (piece[3].0 + 1, piece[3].1);
            }
            piece[0] = (piece[0].0 - 1, piece[0].1);
            piece[1] = (piece[1].0 - 1, piece[1].1);
            piece[2] = (piece[2].0 - 1, piece[2].1);
            piece[3] = (piece[3].0 - 1, piece[3].1);
            if let Some(color) = get_color!(self.piece_active) {
                let color = color.rgba();
                texture_canvas.set_draw_color(Color::RGBA(color.0, color.1, color.2, color.3 / 2));
                for point in piece.iter() {
                    texture_canvas.fill_rect(Rect::new(
                        x_offset + (point.1 as u32 * crate::TILE_SIZE) as i32,
                        y_offset + (point.0 as u32 * crate::TILE_SIZE) as i32,
                        crate::TILE_SIZE,
                        crate::TILE_SIZE,
                    ))?;
                }
            }
        }
        for line in 0..BOARD_HEIGHT {
            for col in 0..BOARD_WIDTH {
                if let Some(color) = get_color!(self.board[self.line_map[line]][col]) {
                    texture_canvas.set_draw_color(color);
                    texture_canvas.fill_rect(Rect::new(
                        x_offset + (col as u32 * crate::TILE_SIZE) as i32,
                        y_offset + (line as u32 * crate::TILE_SIZE) as i32,
                        crate::TILE_SIZE,
                        crate::TILE_SIZE,
                    ))?;
                }
            }
        }
        Ok(())
    }

    fn spawn_next(&mut self, piece: Option<Piece>) {
        // TODO: check for top-out
        let fill = if piece.is_none() {
            if self.queue.len() < 7 {
                self.queue_add_bag();
            }
            self.piece_active = self.queue.pop_front().unwrap_or(Piece::None);
            self.piece_active
        } else {
            piece.unwrap()
        };
        // use fill
        self.col_active = 4;
        self.line_active = 3;
        self.rot_active = Rotation::Spawn;
        self.set_piece_at(
            self.piece_active,
            self.rot_active,
            fill,
            self.line_active,
            self.col_active,
        );
        // println!("spawn piece {:?}", self.piece_active);
    }
}
