use graphics::{ Context, polygon, Transformed };
use opengl_graphics::GlGraphics;
use piston::window::Size;

use color;
use geom;
use geom::Direction;
use super::GameObject; 

const PLAYER_SIZE: f64 = 20.0;

pub struct Player {
  pub pos: geom::Position,
  pub dir: Direction,
  pub size: f64
}

impl Player {
  pub fn new(x: f64, y: f64) -> Player {
    return Player {
      pos: geom::Position::new(x, y),
      dir: Direction::NORTH,
      size: PLAYER_SIZE
    }
  }
}

impl GameObject for Player {
  fn position(&self) -> &geom::Position { &self.pos }
  fn radius(&self) -> f64 { self.size / 2.0 }

  fn render(&self, ctxt: &Context, gl: &mut GlGraphics) {
    //TODO: Change this to be a ellipse
    let shape = polygon::Polygon::new(color::WHITE);
    let dir = match self.dir {
      Direction::WEST => 0.0,
      Direction::NORTH => 90.0,
      Direction::EAST => 180.0,
      Direction::SOUTH => 270.0,
    };

    let radius = self.radius();
    let transform = ctxt.transform
      .trans(self.pos.x, self.pos.y)
      .rot_deg(dir)
      .trans(-radius, -radius);

    let points = [
      [0.0, radius],
      [self.size, self.size],
      [self.size, 0.0]
    ];

    shape.draw(
      &points,
      &ctxt.draw_state,
      transform,
      gl
    );  
  }
  fn update(&mut self, _dt: f64, _size: Size) {
    //TODO: Figure out what to do here
    //Maybe change design?
  }

}