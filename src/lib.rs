extern crate glutin_window;
extern crate graphics;
extern crate piston;
extern crate opengl_graphics;

use piston::input::*;
use piston::window::Window;

pub mod config;
mod geom;
use geom::Direction::*;
mod color;
//Everything in models directory
mod models;
use models::{ GameObject };
use models::player::Player;


pub struct App {
    pub window: config::GraphicsConfig,
    player: Player,
}

impl App {
  pub fn new(window: config::GraphicsConfig) -> App {
    let size = window.settings.size();

    let (x,y) = ((size.width / 2) as f64,
                 (size.height / 2) as f64);

    let player = Player::new(x, y);
    App {window: window, player: player }
  }
  pub fn render(&mut self, args: &RenderArgs) {
    let player = &self.player;

    self.window.gl.draw(args.viewport(), |c, gl| {
      use graphics::*;
      // Clear the screen.
      clear(::color::BLACK, gl);

      player.render(&c, gl);
    });
  }

  pub fn update(&mut self, args: &UpdateArgs) {
    let size = self.window.settings.size();
    self.player.update(args.dt, size);
  }

  pub fn input(&mut self, button: &Button) {
    if let Button::Keyboard(key) = *button {
      //Only need to update direction
      match key {
        Key::Up => self.player.dir[0] = NORTH, 
        Key::Down => self.player.dir[0] = SOUTH,
        Key::Left => self.player.dir[0] = WEST,
        Key::Right => self.player.dir[0] = EAST,
        _ => ()
      }
    }
  }
}