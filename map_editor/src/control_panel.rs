extern crate piston_window;

use piston_window::*;

use crate::utils;
use crate::button::{Button};
use crate::graphics::{Drawable};
use crate::interaction::{Clickable};
use crate::tilemap::{TileMap};

pub struct ControlPanel {
    pub pos: utils::Position,
    pub size: utils::Rect,
    pub scale: i32,
    pub buttons: Vec<Button>,
    pub tilemap: TileMap,
}

impl Clickable for ControlPanel {
    fn is_clicked(&self, position_clicked: &utils::Position) -> bool {
        position_clicked.x as i32 >= self.pos.x &&
            position_clicked.x as i32 <= self.pos.x + (self.size.width * self.scale)
    }
}

impl ControlPanel {
    pub fn new(tilemap_size: utils::Rect,
               size: utils::Rect,
               scale: i32) -> ControlPanel {
        let offset = utils::Position{
            x: tilemap_size.width * scale,
            y: 0,
        };

        let tile_swap_button = Button{
            pos: utils::Position{ x: offset.x+10, y: offset.y+10 },
            size: utils::Rect{ width: 30, height: 30 },
            color: [1.0, 1.0, 0.0, 1.0],
        };

        ControlPanel{
            pos: offset,
            size,
            scale,
            buttons: vec![tile_swap_button],
            tilemap: TileMap::new( tilemap_size ),
        }
    }

    pub fn on_render(&mut self,
                     event: &Event,
                     window: &mut PistonWindow) {
		self.tilemap.on_render(event, window);
        window.draw_2d(event, |context, graphics, _device| {
            // Draw tile selection background.
            let control_panel = [
                self.pos.x as f64,
                self.pos.y as f64,
                (self.size.width * self.scale) as f64,
                (self.size.height * self.scale) as f64
            ];
            rectangle(
                [0.0, 0.0, 0.0, 1.0], // black
                control_panel,
                context.transform,
                graphics
            );

            for button in self.buttons.iter() {
                Drawable::draw(button, &context, graphics);
            }
        });
    }

    pub fn on_mouse_input(&mut self, button: MouseButton, [x,y]: [f64;2]) {
        self.tilemap.on_mouse_input(button, [x,y]);

        let pos = utils::Position{x: x as i32, y: y as i32};
        if Clickable::is_clicked(self, &pos) {
            println!("Clicked control panel position {} {}", x, y);

            for button in self.buttons.iter() {
                if Clickable::is_clicked(button, &pos) {
                    self.tilemap.selected_tile = self.tilemap.selected_tile.next();
                    println!("Clicked button, {:?}", self.tilemap.selected_tile);
                }
            }
        }
    }

    pub fn on_keyboard_input(&mut self, key: Key) {
        self.tilemap.on_keyboard_input(key);
    }
}
