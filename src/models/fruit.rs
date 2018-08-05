use graphics::{Context, rectangle, Transformed};
use opengl_graphics::GlGraphics;
use rand;
use rand::Rng;

use color;
use geom;
use piston::window::Size;
use super::GameObject;

const FRUIT_SIZE: f64 = 15.0;
pub struct Fruit {
    pub pos: geom::Position,
    pub size: f64,
    pub rotation: f64
}

impl Fruit {
    pub fn new(max_x: u32, max_y: u32) -> Fruit {
      let mut rng = rand::thread_rng();
      let randx = rng.gen_range(0.0, max_x as f64);
      let randy = rng.gen_range(0.0, max_y as f64);
      Fruit {pos: geom::Position {x: randx,y: randy}, size: FRUIT_SIZE, rotation: 0.0}
    }
}

impl GameObject for Fruit {
  fn position(&self) -> &geom::Position { &self.pos }
  fn radius(&self) -> f64 { self.size / 2.0 }

  fn render(&self, ctxt: &Context, gl: &mut GlGraphics) {
      // Render the player as a little square
      let square = rectangle::square(0.0, 0.0, self.size);
      let transform = ctxt.transform.trans(self.pos.x, self.pos.y)
        // Handle any rotation
        .rot_rad(self.rotation)
        // Center object on coordinate.
        .trans(-self.radius(), -self.radius());
      // Draw a box rotating around the middle of the screen.
      rectangle(color::RED, square, transform, gl);
  }

  fn update(&mut self, dt: f64, _size: Size) {
      self.rotation += 2.0 * dt;
  }

}
 