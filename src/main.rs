extern crate sdl2;

// Import Game struct from game.rs
mod game;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas, Texture};
use std::time::Duration;
// "self" imports the "image" module itself as well as everything else we listed
use sdl2::image::{self, LoadTexture, InitFlag};


fn render(canvas: &mut WindowCanvas, color: Color, texture: &Texture) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    canvas.copy(texture, None, None)?;

    canvas.present();

    Ok(())
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(sdl2::image::InitFlag::PNG)?;

    let window = video_subsystem
        .window("Pong clone", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/bardo.png")?;

    let mut event_pump = sdl_context.event_pump()?;
    
    // Create a new Game instance
    let mut game = game::Game::new();
    // Initialize the game
    game.start();

    'running: loop {
        // Update the game
        game.update();
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // Render the game
        render(&mut canvas, Color::RGB(230, 230, 230), &texture);
        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
