use graphics::*;
use opengl_graphics::GlGraphics;

use geom::Position;
use piston::window::Size;

pub mod player;
pub mod fruit;

pub trait GameObject {

  //Returns the position of the GameObject
  fn position(&self) -> &Position;

  //Returns the radius of the GameObject
  fn radius(&self) -> f64;

  //Main draw function for GameObject
  fn render(&self, ctxt: &Context, gl: &mut GlGraphics);

  //Handles updates to movement
  fn update(&mut self, _:f64, _: Size) {}

   fn collides(&self, other: &GameObject) -> bool {
      // Two circles intersect if the distance between their centers is
      // between the sum and the difference of their radii.
      let x2 = self.position().x - other.position().x;
      let y2 = self.position().y - other.position().y;
      let sum = x2.powf(2.0) + y2.powf(2.0);

      let r_start: f64 = 0.0;
      let r_end = self.radius() + other.radius();

      return r_start.powf(2.0) <= sum && sum <= r_end.powf(2.0);
    }
}