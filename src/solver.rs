use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
    sync::Arc,
};

use rand::{
    prelude::{SliceRandom, StdRng},
    SeedableRng,
};

use crate::tetris::{self, Piece, BOARD_HEIGHT, BOARD_WIDTH, PIECES};

const 
#[derive(Clone, Eq)]
pub struct SolverState {
    pub board: [[Piece; BOARD_WIDTH]; BOARD_HEIGHT],
    pub queue: VecDeque<Piece>,
    pub piece_active: Piece,
    pub swap_piece: Piece,
    pub rng: StdRng,
}

impl Hash for SolverState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut hash_board = self.board.clone();
        for line in hash_board.iter_mut() {
            for cell in line {
                if cell != &Piece::None {
                    *cell = Piece::T;
                }
            }
        }
        hash_board.hash(state);

        // let mut hash_queue = self.queue.clone();
        // hash_queue.pop_front().hash(state);
        self.queue.hash(state);
        vec![self.piece_active, self.swap_piece].sort().hash(state);
    }
}

impl PartialEq for SolverState {
    fn eq(&self, other: &Self) -> bool {
        // available pieces must match. That is, the set of the swap piece and
        // first element in the queue must be equivalent.
        if self.swap_piece != other.piece_active || self.swap_piece != other.piece_active {
            return false;
        };
        if other.swap_piece != self.piece_active || other.swap_piece != self.piece_active {
            return false;
        };

        // the queue after the first element must match.
        let mut eq_iter_self = self.queue.iter();
        eq_iter_self.next();
        let mut eq_iter_other = self.queue.iter();
        eq_iter_other.next();
        if !(eq_iter_self.eq(eq_iter_other)) {
            return false;
        };

        // the rng must match
        if self.rng != other.rng {
            return false;
        };

        // the same cells on the board must be filled
        let board_vec_self: Vec<Piece> = self
            .board
            .iter()
            .flat_map(|array| array.iter())
            .cloned()
            .collect();
        let board_vec_other: Vec<Piece> = other
            .board
            .iter()
            .flat_map(|array| array.iter())
            .cloned()
            .collect();
        for cell in board_vec_self.into_iter().zip(board_vec_other.into_iter()) {
            if cell.0 == Piece::None && cell.1 != Piece::None
                || cell.1 == Piece::None && cell.0 != Piece::None
            {
                return false;
            }
        }
        false
    }
}

impl SolverState {
    pub fn new() -> SolverState {
        // Fill initial queue
        let mut queue: VecDeque<Piece> = VecDeque::new();
        let mut rng: StdRng = SeedableRng::from_entropy();

        let mut pieces_clone = PIECES.clone();
        pieces_clone.shuffle(&mut rng);
        queue.append(&mut VecDeque::from_iter(pieces_clone));
        let mut pieces_clone = PIECES.clone();
        pieces_clone.shuffle(&mut rng);
        queue.append(&mut VecDeque::from_iter(pieces_clone));
        SolverState {
            board: [[Piece::None; BOARD_WIDTH]; BOARD_HEIGHT],
            queue,
            piece_active: Piece::None,
            swap_piece: Piece::None,
            rng,
        }
    }
}

pub struct Solver {
    dyn_prog: HashMap<SolverState, Arc<Vec<SolverState>>>,
}

impl Solver {
    pub fn new() -> Solver {
        Solver {
            dyn_prog: HashMap::new(),
        }
    }

    pub fn solve(&mut self, state: SolverState) {
        if (self.dyn_prog.contains_key(&state)) {
            return;
        };

        
    }

}
