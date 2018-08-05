extern crate glutin_window;
extern crate graphics;
extern crate piston;
extern crate opengl_graphics;
extern crate rand;

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
use models::fruit::Fruit;

pub struct App {
    pub window: config::GraphicsConfig,
    player: Player,
    fruit: Fruit,
    ticker: u64     //This variable acts as a timer on when to call the update function
}

impl App {
  pub fn new(window: config::GraphicsConfig) -> App {
    let size = window.settings.size();
    let (x,y) = ((size.width / 2) as f64,
                 (size.height / 2) as f64);

    let player = Player::new(x, y);
    let fruit = Fruit::new(size.width, size.height);
    App {window: window, player: player, fruit: fruit, ticker: 5}
  }
  pub fn render(&mut self, args: &RenderArgs) {
    let player = &self.player;
    let fruit = &self.fruit;
    self.window.gl.draw(args.viewport(), |c, gl| {
      use graphics::*;
      // Clear the screen.
      clear(::color::BLACK, gl);

      player.render(&c, gl);
      fruit.render(&c, gl);
    });
  }

  pub fn update(&mut self, args: &UpdateArgs) {
    let size = self.window.settings.size();
    self.ticker = self.ticker - 1;
    if self.ticker <= 0 {
      self.player.update(args.dt, size);
      self.ticker = 5;
    }  
    self.fruit.update(args.dt, size);
    if self.player.collides(&self.fruit) {
      let pos_x = self.player.pos[self.player.length - 1].x;
      let pos_y = self.player.pos[self.player.length - 1].y;
      match self.player.dir {
        WEST => self.player.pos.push(geom::Position::new(pos_x - self.player.size, pos_y)),
        NORTH => self.player.pos.push(geom::Position::new(pos_x, pos_y - self.player.size)),
        EAST => self.player.pos.push(geom::Position::new(pos_x + self.player.size, pos_y)),
        SOUTH => self.player.pos.push(geom::Position::new(pos_x, pos_y + self.player.size)),
      };
      self.player.length = self.player.length + 1;
      self.fruit = Fruit::new(size.width, size.height);
      //Need to update score
    }
  }

  pub fn input(&mut self, button: &Button) {
    if let Button::Keyboard(key) = *button {
      //Only need to update direction for head of snake
      match key {
        Key::Up => self.player.dir = NORTH, 
        Key::Down => self.player.dir = SOUTH,
        Key::Left => self.player.dir = WEST,
        Key::Right => self.player.dir = EAST,
        _ => ()
      }
    }
  }
}