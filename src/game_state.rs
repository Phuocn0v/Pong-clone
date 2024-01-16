use std::fmt;

use crate::game_config::{GRID_X_SIZE, GRID_Y_SIZE};
use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameState {
    Playing,
    Paused,
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GameState::Playing => write!(f, "Playing"),
            GameState::Paused => write!(f, "Paused"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PlayerDirection {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for PlayerDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PlayerDirection::Up => write!(f, "Up"),
            PlayerDirection::Down => write!(f, "Down"),
            PlayerDirection::Left => write!(f, "Left"),
            PlayerDirection::Right => write!(f, "Right"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Point(pub i32, pub i32);

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Clone, Debug)]
pub struct GameContext {
    pub player_direction: PlayerDirection,
    pub player_position: Vec<Point>,
    pub food: Point,
    pub state: GameState,
}

impl GameContext {
    pub fn new() -> GameContext {
        GameContext {
            player_position: vec![Point(3, 1), Point(2, 1), Point(1, 1)],
            player_direction: PlayerDirection::Right,
            state: GameState::Paused,
            food: Point(3, 3),
        }
    }

    pub fn moving(&mut self) {
        let head = self.player_position[0];
        let new_head = match self.player_direction {
            PlayerDirection::Up => Point(head.0, head.1 - 1),
            PlayerDirection::Down => Point(head.0, head.1 + 1),
            PlayerDirection::Left => Point(head.0 - 1, head.1),
            PlayerDirection::Right => Point(head.0 + 1, head.1),
        };

        self.player_position.pop();
        self.player_position.reserve(1);
        self.player_position.insert(0, new_head);
        self.player_position.reserve(1);
    }

    pub fn change_direction(&mut self, direction: PlayerDirection) {
        self.player_direction = direction;
    }

    pub fn eat(&mut self) {
        let head = self.player_position[0];
        let new_head = match self.player_direction {
            PlayerDirection::Up => Point(head.0, head.1 - 1),
            PlayerDirection::Down => Point(head.0, head.1 + 1),
            PlayerDirection::Left => Point(head.0 - 1, head.1),
            PlayerDirection::Right => Point(head.0 + 1, head.1),
        };

        self.player_position.reserve(1);
        self.player_position.insert(0, new_head);
        self.player_position.reserve(1);

        // TODO: Generate new food
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..GRID_X_SIZE);
        let y = rng.gen_range(0..GRID_Y_SIZE);
        self.food = Point(x as i32, y as i32);
    }
}

impl fmt::Display for GameContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GameContext {{\n\tstate: {},\n\tplayer_direction: {},\n\tplayer_position: {:?},\n\tfood: {}\n}}",
               self.state, self.player_direction, self.player_position, self.food)
    }
}
