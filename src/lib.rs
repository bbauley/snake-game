extern crate glutin_window;
extern crate graphics;
extern crate piston;
extern crate opengl_graphics;
extern crate rand;

use piston::input::*;
use piston::window::Window;
use opengl_graphics::{GlyphCache, TextureSettings};

pub mod config;
mod gfx;
use gfx::{ draw_text, draw_center };
mod geom;
use geom::Direction::*;
mod color;
mod models;
use models::{ GameObject };
use models::player::Player;
use models::fruit::Fruit;

const RESET_VALUE: u64 = 5;
const SCORE_INC: u64 = 10;
const WINNING_SCORE: u64 = 500;

#[derive(PartialEq)]
enum GameStatus {
    // Normal mode
    Normal,
    // Player died
    Died,
    // Player won!
    Win
}

pub struct App<'a> {
    pub window: config::GraphicsConfig,
    player: Player,
    fruit: Fruit,
    glyph_cache: GlyphCache<'a>,
    score: u64,
    status: GameStatus,
    ticker: u64     //This variable acts as a timer on when to call the update function
}

impl<'a> App<'a> {
  pub fn new(window: config::GraphicsConfig) -> App<'a> {
    let size = window.settings.size();
    let (x,y) = ((size.width / 2) as f64,
                 (size.height / 2) as f64);

    // Load font(s) used in the game.
    let glyph_cache = GlyphCache::new("./assets/fonts/PxPlus_IBM_VGA8.ttf", (), TextureSettings::new())
      .expect("Unable to load font");

    assert!(x >= 0.0 && y >= 0.0);
    let player = Player::new(x, y);
    let fruit = Fruit::new(size.width, size.height);

    App {
      window: window, 
      player: player, 
      fruit: fruit, 
      glyph_cache: glyph_cache, 
      score: 0, 
      status: GameStatus::Normal,
      ticker: RESET_VALUE
    }
  }
  pub fn render(&mut self, args: &RenderArgs) {
    let player = &self.player;
    let fruit = &self.fruit;
    let score = self.score;
    let gc = &mut self.glyph_cache;
    let status = &self.status;
    let size = self.window.settings.size();

    self.window.gl.draw(args.viewport(), |c, gl| {
      use graphics::*;
      // Clear the screen.
      clear(::color::BLACK, gl);

      match status {
        GameStatus::Win => {
          draw_center("YOU WIN!", 32, [size.width as f64, size.height as f64], gc, &c, gl);
            return;
        },
        GameStatus::Died => {
          draw_center("YOU DIED!", 32, [size.width as f64, size.height as f64], gc, &c, gl);
            return;
        },
        _ => ()
      }
      // Render the current score
      let score_str = format!("Score: {}", score);
      draw_text(score_str.as_str(), [0.0, 16.0], 16, gc, &c, gl);

      player.render(&c, gl);
      fruit.render(&c, gl);
    });
  }

  pub fn update(&mut self, args: &UpdateArgs) {
    let size = self.window.settings.size();

    self.ticker = self.ticker - 1;
    //Updating player in intervals
    //Issues come up if I don't do this
    if self.ticker <= 0 {
      self.player.update(args.dt, size);
      self.ticker = RESET_VALUE;
    }  
    self.fruit.update(args.dt, size);

    //Check if player collides with a fruit
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
      self.fruit = Fruit::new(size.width - self.fruit.size as u32, 
                              size.height - self.fruit.size as u32);
      self.score = self.score + SCORE_INC;
      if self.score == WINNING_SCORE {
        self.status = GameStatus::Win;
      }
    }
    //Check if player hits a wall
    if self.status != GameStatus::Win && self.player.hits_wall(size.width, size.height) {
      self.status = GameStatus::Died;
    }

    //Check if player collides with body
    if self.status != GameStatus::Win && self.player.hits_body() {
      self.status = GameStatus::Died;
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

