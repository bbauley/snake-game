extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate snake;

use piston::event_loop::*;
use piston::input::*;
use snake::App;
use snake::config::GraphicsConfig;

fn main() {

    let mut app = App::new(GraphicsConfig::new("Snake", 960,768));

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut app.window.settings) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
        if let Some(u) = e.update_args() {
            app.update(&u);
        }
        if let Some(i) = e.press_args() {
            app.input(&i);
        }

    }    
}
