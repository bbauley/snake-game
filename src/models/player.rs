use graphics::{ Context, rectangle, Transformed };
use opengl_graphics::GlGraphics;
use piston::window::Size;

use color;
use geom;
use geom::Direction;
use super::GameObject; 

const PLAYER_SIZE: f64 = 20.0;
const INITIAL_LENGTH: usize = 5;
const UNIT_MOVE: f64 = 20.0;

pub struct Player {
  pub pos: Vec<geom::Position>,
  pub dir: Vec<Direction>,
  pub size: f64,
  pub length: usize
}

impl Player {
  pub fn new(x: f64, y: f64) -> Player {
    let mut v = Vec::new();
    let mut d = Vec::new();
    let mut pos_y = y;

    for _ in 0..INITIAL_LENGTH {
      v.push(geom::Position::new(x, pos_y));
      d.push(Direction::NORTH);
      pos_y -= PLAYER_SIZE;
    }

    return Player {
      pos: v,
      dir: d,
      size: PLAYER_SIZE,
      length: INITIAL_LENGTH
    }
  }
}

impl GameObject for Player {
  fn position(&self) -> &geom::Position { &self.pos[0] }
  fn radius(&self) -> f64 { self.size / 2.0 }

  fn render(&self, ctxt: &Context, gl: &mut GlGraphics) {

    for i in 0..self.length {
      let shape = rectangle::Rectangle::new_round(color::WHITE, self.radius());
      let dir = match self.dir[i] {
        Direction::WEST => 0.0,
        Direction::NORTH => 90.0,
        Direction::EAST => 180.0,
        Direction::SOUTH => 270.0,
      };

      let radius = self.radius();
      let transform = ctxt.transform
        .trans(self.pos[i].x, self.pos[i].y)
        .rot_deg(dir)
        .trans(-self.size - radius, -self.size - radius);
      let points = [self.size; 4];

      shape.draw(
        points,
        &ctxt.draw_state,
        transform,
        gl
      ); 
    }
  }
  fn update(&mut self, _dt: f64, _size: Size) {
    let mut hold_position = self.pos[0];
    let mut hold_direction = self.dir[0];

    //Constant fast movement 
    match self.dir[0] {
      Direction::NORTH => self.pos[0].y -= UNIT_MOVE,
      Direction::WEST => self.pos[0].x -= UNIT_MOVE,
      Direction::SOUTH => self.pos[0].y += UNIT_MOVE,
      Direction::EAST => self.pos[0].x += UNIT_MOVE
    }

    //Update the rest of the snake
    for i in 1..self.length {
      let temp = self.pos[i];
      let temp2 = self.dir[i];
      self.pos[i] = hold_position;
      self.dir[i] = hold_direction;
      hold_position = temp;
      hold_direction = temp2;
    }
  }
}