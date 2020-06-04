extern crate piston_window;

use piston_window::*;

use crate::board;
use crate::entity;
use crate::utils;

use board::*;
use entity::*;
use utils::*;

use std::{
    fs::File,
    io::{self, Read, Write},
};

pub struct TileMap {
    pub player: Entity,
    pub board: Board,
}

impl TileMap {
    pub fn new() -> TileMap {
        let player = Entity{ 
            pos: utils::Position{ 
                x: 5, 
                y: 5,
            },
            blocking: false,
        };

        let mut board = Board{
            size: utils::Size{ width: 26, height: 15 },
            scale: 30,
            blocking_map: Vec::new(),
        };

        // Add one wall to board.
        board.blocking_map.push(
            Entity{ pos: utils::Position{ x:1, y:1 }, blocking: true },
        );

        TileMap {
            player: player,
            board: board,
        }
    }

    pub fn on_mouse_input(&mut self, button: MouseButton, [x,y]: [f64;2]) {
        println!("clicked position {} {}", x, y);
        match button {
            MouseButton::Left => {
                let new_x = x as i32 / self.board.scale;
                let new_y = y as i32 / self.board.scale;
                println!("spawn new entity on board position {} {}", new_x, new_y);

                self.board.blocking_map.push(
                    Entity{
                        pos: utils::Position{
                            x: new_x,
                            y: new_y,
                        }, 
                        blocking: true
                    },
                )},
            _ => {},
        }
    }

    pub fn on_keyboard_input(&mut self, key: Key) {
        match key {
            Key::P => { self.write_map(String::from("foo.txt")) },
            Key::L => { self.load_map(String::from("foo.txt")) },
            _ => {},
        }
    }
    
    pub fn on_render(&mut self, event: Event, window: &mut PistonWindow) {
        window.draw_2d(&event, |context, graphics, _device| {
            // Clear screen.
            clear([1.0; 4], graphics);

            //TODO: create method to draw sprite for entities.
            let player_sprite = [
                (self.player.pos.x * self.board.scale) as f64,
                (self.player.pos.y * self.board.scale) as f64,
                self.board.scale as f64,
                self.board.scale as f64,
            ];

            rectangle(
                [1.0, 0.0, 0.0, 1.0], // red
                player_sprite,
                context.transform,
                graphics
            );

            for wall in self.board.blocking_map.iter() {
                let wall_sprite = [
                    (wall.pos.x * self.board.scale) as f64,
                    (wall.pos.y * self.board.scale) as f64,
                    self.board.scale as f64,
                    self.board.scale as f64,
                ];

                rectangle(
                    [1.0, 1.0, 0.0, 1.0], // red
                    wall_sprite,
                    context.transform,
                    graphics
                );
            }

            // Draw tile selection background.
            let tile_panel = [
                20.0 * self.board.scale as f64,
                0.0,
                6.0 * self.board.scale as f64,
                (self.board.scale * self.board.size.height) as f64
            ];
            rectangle(
                [0.0, 0.0, 0.0, 1.0], // black
                tile_panel,
                context.transform,
                graphics
            );
        });
    }

    pub fn load_map(&mut self, filename: String) {
        let mut file = File::open(filename).unwrap();
        self.board = Board::load(&mut file).unwrap();
    }

    pub fn write_map(&mut self, filename: String) {
        let mut file = File::create(filename).unwrap();
        self.board.write(&mut file);
    }
}
