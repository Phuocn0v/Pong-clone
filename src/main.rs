extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

mod game_state;
use game_state::{GameContext, GameState, PlayerDirection, Point};
mod game_renderer;
use game_renderer::Renderer;
mod game_config;
use game_config::{DOT_SIZE_IN_PXS, GRID_X_SIZE, GRID_Y_SIZE};

pub fn update(renderer: &mut Renderer, context: &mut GameContext) {
    // Update game logic in here.
    if (context.state == GameState::Playing) {
        context.moving();
    }
    if (context.player_position[0].0 == context.food.0
        && context.player_position[0].1 == context.food.1)
    {
        context.eat();
    }
    renderer.draw(&context);
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(
            "game window",
            GRID_X_SIZE * DOT_SIZE_IN_PXS as u32,
            GRID_Y_SIZE * DOT_SIZE_IN_PXS as u32,
        )
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    // Game state and renderer
    let mut game_context = GameContext::new();
    let mut renderer = Renderer::new(window).unwrap();
    let mut frame_counter = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    game_context.player_direction = PlayerDirection::Up;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    game_context.player_direction = PlayerDirection::Down;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    game_context.player_direction = PlayerDirection::Left;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    game_context.player_direction = PlayerDirection::Right;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    game_context.state = match game_context.state {
                        GameState::Paused => GameState::Playing,
                        GameState::Playing => GameState::Paused,
                    }
                }

                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
        frame_counter += 1;
        if (frame_counter == 10) {
            update(&mut renderer, &mut game_context);
            frame_counter = 0;
        }
    }
    return Ok(());
}
