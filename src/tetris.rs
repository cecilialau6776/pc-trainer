use array2d::Array2D;
use rand::{prelude::*, seq::SliceRandom};
use std::collections::VecDeque;

pub const SQUARE_SIZE: u32 = 16;
pub const BOARD_HEIGHT: u8 = 24;
pub const BOARD_WIDTH: u8 = 10;
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

#[derive(Clone, PartialEq, Eq, Hash)]
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
    board: Array2D<Piece>,
    queue: VecDeque<Piece>,
    state: State,
    rng: StdRng,
}

impl Tetris {
    pub fn new() -> Tetris {
        let mut board = Array2D::filled_with(Piece::None, BOARD_HEIGHT as usize, BOARD_WIDTH as usize);
        let mut queue: VecDeque<Piece> = VecDeque::new();
        let mut rng: StdRng = SeedableRng::from_entropy();

        let mut pieces_clone = PIECES.clone();
        pieces_clone.shuffle(&mut rng);
        queue.append(&mut VecDeque::from_iter(pieces_clone));
        let mut pieces_clone = PIECES.clone();
        pieces_clone.shuffle(&mut rng);
        queue.append(&mut VecDeque::from_iter(pieces_clone));

        // test code
        board.set(2, 3, Piece::T);
        board.set(3, 2, Piece::T);
        board.set(3, 3, Piece::T);
        board.set(3, 4, Piece::T);

        Tetris {
            board,
            queue,
            state: State::Playing,
            rng,
        }
    }

    pub fn toggle_state(&mut self) {
        match self.state {
            State::Playing => self.state = State::Paused,
            State::Paused => self.state = State::Playing,
        }
    }

    pub fn get(&self, line: u8, col: u8) -> Option<&Piece> {
        self.board.get(line as usize, col as usize)
    }

    fn queue_add_bag(&mut self) {
        let mut pieces_clone = PIECES.clone();
        pieces_clone.shuffle(&mut self.rng);
        self.queue.append(&mut VecDeque::from_iter(pieces_clone));
    }

    pub fn rows_iter(&self) -> impl Iterator<Item = impl Iterator<Item = &Piece>> {
        self.board.rows_iter()
    }
}
