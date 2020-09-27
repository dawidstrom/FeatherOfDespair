use crate::graphics::{Drawable};
use crate::interaction::{Clickable};
use piston_window::{Context, G2d, rectangle};
use crate::utils::*;

pub struct Button {
    pub pos: Position,
    pub size: Rect,
    pub color: [f32; 4],
}

impl Drawable for Button {
    fn draw(&self, context: &Context, graphics: &mut G2d) {
        rectangle(
            self.color,
            [
                self.pos.x as f64,
                self.pos.y as f64,
                self.size.width as f64,
                self.size.height as f64,
            ],
            context.transform,
            graphics
        );
    }
}

impl Clickable for Button {
    fn is_clicked(&self, position_clicked: &Position) -> bool {
        position_clicked.x >= self.pos.x && 
        position_clicked.y >= self.pos.y && 
        position_clicked.x <= self.pos.x + self.size.width && 
        position_clicked.y <= self.pos.y + self.size.height
    }
}
