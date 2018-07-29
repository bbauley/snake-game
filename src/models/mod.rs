use graphics::*;
use opengl_graphics::GlGraphics;

use geom::Position;
use piston::window::Size;

pub mod player;

pub trait GameObject {

  //Returns the position of the GameObject
  fn position(&self) -> &Position;

  //Returns the radius of the GameObject
  fn radius(&self) -> f64;

  //Main draw function for GameObject
  fn render(&self, ctxt: &Context, gl: &mut GlGraphics);

  //Handles updates to movement
  fn update(&mut self, _:f64, _: Size) {}
}