use std::{collections::VecDeque, time::SystemTime};

use sdl2::event::Event;
use sdl2::keyboard::Scancode;

use crate::tetris::{Direction, Rotation, State, Tetris};

pub struct InputManager {
    event_queue: VecDeque<Event>,

    // handling settings
    arr: u32, // auto-repeat rate: time in ms it takes between each movement after das has started
    das: u32, // delayed auto-start: time in ms needed to hold down the left/right keys before piece moves in that direction in its own
    dcd: u32, // das cut delay: any ongoing DAS movement is paused for this long after dropping/rotating a piece
    sdf: u32, // soft drop factor: the factor with which soft drop changes the gravity speed. 100 is instant soft drop
    pahd: u32, // prevent accidental hard drop: time in ms after a piece that locks on its own before hard drop becomes available
    cdwcd: bool, // cancel das when changing directions.

    // keybinds
    left: Vec<Scancode>,
    right: Vec<Scancode>,
    softdrop: Vec<Scancode>,
    harddrop: Vec<Scancode>,
    rot_counterclockwise: Vec<Scancode>,
    rot_clockwise: Vec<Scancode>,
    rot_180: Vec<Scancode>,
    swap: Vec<Scancode>,
}

impl InputManager {
    pub fn new() -> Self {
        InputManager {
            event_queue: VecDeque::new(),
            arr: 0,
            das: 100,
            dcd: 0,
            sdf: 100,
            pahd: 100,
            cdwcd: false,
            left: vec![Scancode::Left],
            right: vec![Scancode::Right],
            softdrop: vec![Scancode::Down],
            harddrop: vec![Scancode::Space],
            rot_counterclockwise: vec![Scancode::Z],
            rot_clockwise: vec![Scancode::X, Scancode::Up],
            rot_180: vec![Scancode::A],
            swap: vec![Scancode::LShift],
        }
    }
    pub fn process_input(&mut self, event: Event, game: &mut Tetris) {
        if let Event::KeyDown {
            repeat: false,
            scancode,
            ..
        } = event
        {
            let sc = scancode.expect("no scancode?");
            if self.left.contains(&sc) {
                if game.state() == State::Playing {
                    game.move_active(Direction::Left);
                    self.event_queue.push_back(event); // for das timings
                }
            } else if self.right.contains(&sc) {
                if game.state() == State::Playing {
                    game.move_active(Direction::Right);
                    self.event_queue.push_back(event); // for das timings
                }
            } else if self.softdrop.contains(&sc) {
                if game.state() == State::Playing {
                    game.move_active(Direction::Down);
                    self.event_queue.push_back(event); // for das timings
                }
            } else if self.rot_counterclockwise.contains(&sc) {
                if game.state() == State::Playing {
                    game.rot_active(Rotation::Left);
                }
            } else if self.rot_clockwise.contains(&sc) {
                if game.state() == State::Playing {
                    game.rot_active(Rotation::Right);
                }
            }
        }
    }

    pub fn update(&self, timestamp: SystemTime) {}
}
