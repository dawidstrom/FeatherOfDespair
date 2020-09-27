use piston_window::{Context, G2d};

pub trait Drawable {
    fn draw(&self, context: &Context, graphics: &mut G2d);
}
