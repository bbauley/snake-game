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

const UNIT_MOVE: f64 = 1.0;

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
    //Constant fast movement 
    for _ in 0..2 {
      match self.player.dir {
        NORTH => self.player.pos.y -= UNIT_MOVE,
        WEST => self.player.pos.x -= UNIT_MOVE,
        SOUTH => self.player.pos.y += UNIT_MOVE,
        EAST => self.player.pos.x += UNIT_MOVE
      }
    }
    //TODO: Figure out what to do here 
    let size = self.window.settings.size();
    self.player.update(args.dt, size);
  }

  pub fn input(&mut self, button: &Button) {
    if let Button::Keyboard(key) = *button {
      match key {
        Key::Up => { 
          self.player.pos.y -= UNIT_MOVE;
          self.player.dir = NORTH;
        },
        Key::Down => {
          self.player.pos.y += UNIT_MOVE;
          self.player.dir = SOUTH;
        },
        Key::Left => {
          self.player.pos.x -= UNIT_MOVE;
          self.player.dir = WEST;
        },
        Key::Right => {
          self.player.pos.x += UNIT_MOVE;
          self.player.dir = EAST;
        },
        _ => ()
      }
    }
  }
}