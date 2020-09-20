extern crate piston_window;

use piston_window::*;

use crate::utils;
use utils::*;

pub struct ControlPanel {
    pub pos: utils::Position,
    pub size: utils::Size,
    pub scale: i32,
}

impl ControlPanel {
    pub fn new(pos: utils::Position, size: utils::Size, scale: i32) -> ControlPanel {
        ControlPanel{pos, size, scale}
    }

    pub fn on_render(&mut self,
                     event: &Event,
                     window: &mut PistonWindow) {
        window.draw_2d(event, |context, graphics, _device| {
            // Draw tile selection background.
            let tile_panel = [
                (self.pos.x * self.scale) as f64,
                (self.pos.y * self.scale) as f64,
                (self.size.width * self.scale) as f64,
                (self.size.height * self.scale) as f64
            ];
            rectangle(
                [0.0, 0.0, 0.0, 1.0], // black
                tile_panel,
                context.transform,
                graphics
            );
        });
    }

    pub fn on_mouse_input(&mut self, button: MouseButton, [x,y]: [f64;2]) {
        if x as i32 >= self.pos.x &&
           x as i32 <= (self.pos.x+self.size.width) * self.scale {
            println!("Clicked control panel position {} {}", x, y)
        }
    }
}
