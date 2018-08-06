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
  pub dir: Direction,
  pub size: f64,
  pub length: usize
}

impl Player {
  pub fn new(x: f64, y: f64) -> Player {
    let mut v = Vec::new();
    let mut pos_y = y;

    for _ in 0..INITIAL_LENGTH {
      v.push(geom::Position::new(x, pos_y));
      pos_y += PLAYER_SIZE;
    }

    Player {
      pos: v,
      dir: Direction::NORTH,
      size: PLAYER_SIZE,
      length: INITIAL_LENGTH
    }
  }

  //Checks if the player hits a wall 
  //given the width and height of the window size
  pub fn hits_wall(&self, width: u32, height: u32) -> bool {
    let position = self.position();
    if (position.x < 0.0 || position.x > width.into()) ||
       (position.y < 0.0 || position.y > height.into()) {
      return true;
    }
    else {
      return false;
    }
  }

  //Checks if the player hits his own snake body
  pub fn hits_body(&self) -> bool {
    let position = *self.position();
    for i in 1..self.length {
      if position == self.pos[i] {
        return true;
      }
    }
    return false;
  }
}

impl GameObject for Player {
  fn position(&self) -> &geom::Position { &self.pos[0] }
  fn radius(&self) -> f64 { self.size / 2.0 }

  fn render(&self, ctxt: &Context, gl: &mut GlGraphics) {

    for i in 0..self.length {
      let shape = rectangle::Rectangle::new_round(color::WHITE, self.radius());
      let dir = match self.dir {
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

    //Constant fast movement 
    match self.dir {
      Direction::NORTH => self.pos[0].y -= UNIT_MOVE,
      Direction::WEST => self.pos[0].x -= UNIT_MOVE,
      Direction::SOUTH => self.pos[0].y += UNIT_MOVE,
      Direction::EAST => self.pos[0].x += UNIT_MOVE
    }

    //Update the rest of the snake
    for i in 1..self.length {
      let temp = self.pos[i];
      self.pos[i] = hold_position;
      hold_position = temp;
    }
  }
}

#[test]
fn test_player_new() {
  let x_value = -100.0;
  let y_value = -100.0;
  let player = Player::new(x_value, y_value);
  assert!(player.length == 5);
  assert!(player.pos[0].x == x_value &&
          player.pos[0].y == y_value);
  assert!(player.pos.len() == 5);
}

#[test]
fn test_hits_wall() {
  let var = 200.0;
  let player = Player::new(var + 1.0, var + 1.0);
  let wall = (var, var);
  assert!(player.hits_wall(wall.0 as u32, wall.1 as u32) == true);
}

#[test]
fn test_hits_body() {
  let var = 100.0;
  let mut player = Player::new(var, var);
  player.pos[0].y = player.pos[0].y + PLAYER_SIZE;
  assert!(player.hits_body() == true);
}

#[test]
fn test_pos_and_rad() {
  let player = Player::new(100.0, 100.0);
  assert!(*player.position() == geom::Position::new(player.pos[0].x, player.pos[0].y));
  assert!(player.radius() == PLAYER_SIZE / 2.0);
}