use std::time::SystemTime;

use sdl2::event::Event;
use sdl2::keyboard::Scancode;

use crate::tetris::{Direction, Rotation, State, Tetris};

pub struct InputManager {
    events: Vec<(Input, SystemTime)>,

    // handling settings
    arr: u32, // auto-repeat rate: time in ms it takes between each movement after das has started
    das: u32, // delayed auto-start: time in ms needed to hold down the left/right keys before piece moves in that direction in its own
    dcd: u32, // das cut delay: any ongoing DAS movement is paused for this long after dropping/rotating a piece
    sdf: u32, // soft drop factor: the factor with which soft drop changes the gravity speed. 100 is instant soft drop
    pahd: u32, // prevent accidental hard drop: time in ms after a piece that locks on its own before hard drop becomes available
    cdwcd: u32, // cancel das when changing directions: time in ms to wait until existing das kicks in again

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

#[derive(PartialEq, Eq)]
enum Input {
    Left,
    Right,
}

impl InputManager {
    pub fn new() -> Self {
        InputManager {
            events: Vec::new(),
            arr: 0,
            das: 100,
            dcd: 0,
            sdf: 100,
            pahd: 100,
            cdwcd: 100,
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
    pub fn process_input(&mut self, event: Event, game: &mut Tetris, timestamp: SystemTime) {
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
                    self.events.push((Input::Left, timestamp)); // for das timings
                    if self.cdwcd > 0 {
                        for ev in self.events.iter_mut() {
                            if ev.0 == Input::Right {
                                *ev = (Input::Right, timestamp);
                            }
                        }
                    }
                }
            } else if self.right.contains(&sc) {
                if game.state() == State::Playing {
                    game.move_active(Direction::Right);
                    self.events.push((Input::Right, timestamp)); // for das timings
                    if self.cdwcd > 0 {
                        for ev in self.events.iter_mut() {
                            if ev.0 == Input::Left {
                                *ev = (Input::Left, timestamp);
                            }
                        }
                    }
                }
            } else if self.softdrop.contains(&sc) {
                if game.state() == State::Playing {
                    match self.sdf {
                        100 => game.softdrop_instant(timestamp),
                        _ => game.softdrop_start(self.sdf),
                    }
                }
            } else if self.rot_counterclockwise.contains(&sc) {
                if game.state() == State::Playing {
                    game.rot_active(Rotation::Left);
                }
            } else if self.rot_clockwise.contains(&sc) {
                if game.state() == State::Playing {
                    game.rot_active(Rotation::Right);
                }
            } else if self.harddrop.contains(&sc) {
                if game.state() == State::Playing {
                    game.harddrop();
                }
            }
        } else if let Event::KeyUp {
            repeat: false,
            scancode,
            ..
        } = event
        {
            let sc = scancode.expect("no scancode?");
            if self.left.contains(&sc) {
                self.events.retain(|e| e.0 != Input::Left)
            } else if self.right.contains(&sc) {
                self.events.retain(|e| e.0 != Input::Right)
            }
        }
    }

    pub fn update(&self, timestamp: SystemTime, game: &mut Tetris) {
        for event in self.events.iter() {
            if let Ok(dur) = timestamp.duration_since(event.1) {
                if dur.as_millis() as u32 > self.das {
                    if self.arr > 0 {
                        let moves = (dur.as_millis() as u32 - self.das) / self.arr;
                    } else {
                        match event.0 {
                            Input::Left => while game.move_active(Direction::Left) {},
                            Input::Right => while game.move_active(Direction::Right) {},
                        }
                    }
                }
            }
        }
    }
}
