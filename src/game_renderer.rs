use sdl2::{pixels::Color, rect::Rect, render::WindowCanvas, video::Window};

use crate::{
    game_config::DOT_SIZE_IN_PXS,
    game_state::{GameContext, GameState, Point},
};
extern crate sdl2;

pub struct Renderer {
    canvas: WindowCanvas,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().unwrap();
        Ok(Renderer { canvas })
    }

    pub fn draw_dot(&mut self, point: &Point) -> Result<(), String> {
        let Point(x, y) = point;
        self.canvas
            .fill_rect(Rect::new(
                x * DOT_SIZE_IN_PXS,
                y * DOT_SIZE_IN_PXS,
                DOT_SIZE_IN_PXS as u32,
                DOT_SIZE_IN_PXS as u32,
            ))
            .unwrap();
        return Ok(());
    }

    pub fn draw(&mut self, context: &GameContext) -> Result<(), String> {
        self.draw_background(context);
        self.draw_player(context);
        self.draw_food(context);
        self.canvas.present();
        Ok(())
    }

    fn draw_background(&mut self, context: &GameContext) -> Result<(), String> {
        let color = match context.state {
            GameState::Paused => Color::RGB(0, 0, 0),
            GameState::Playing => Color::RGB(30, 30, 30),
        };

        self.canvas.set_draw_color(color);
        self.canvas.clear();
        Ok(())
    }

    fn draw_player(&mut self, context: &GameContext) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(0, 255, 0));
        let player_position = context.player_position.clone();
        for point in player_position {
            self.draw_dot(&point);
        }

        Ok(())
    }

    fn draw_food(&mut self, context: &GameContext) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(255, 0, 0));
        self.draw_dot(&context.food);

        Ok(())
    }
}
