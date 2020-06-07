extern crate piston_window;

use piston_window::*;

use crate::utils;
use utils::*;

pub struct ControlPanel {
    pub pos: utils::Position,
    pub size: utils::Size,
}

impl ControlPanel {
    pub fn new(pos: utils::Position, size: utils::Size) -> ControlPanel {
        ControlPanel{pos, size}
    }

    pub fn on_render(&mut self,
                     scale: i32,
                     event: &Event,
                     window: &mut PistonWindow) {
        window.draw_2d(event, |context, graphics, _device| {
            // Draw tile selection background.
            let tile_panel = [
                (self.pos.x * scale) as f64,
                (self.pos.y * scale) as f64,
                (self.size.width * scale) as f64,
                (self.size.height * scale) as f64
            ];
            rectangle(
                [0.0, 0.0, 0.0, 1.0], // black
                tile_panel,
                context.transform,
                graphics
            );
        });
    }
}
