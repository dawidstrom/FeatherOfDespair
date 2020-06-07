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
        println!("Clicked position {} {}", x, y);
        match button {
            MouseButton::Left => {
                let clicked_pos = utils::Position{
                    x: x as i32 / self.board.scale,
                    y: y as i32 / self.board.scale,
                };

                if self.board.blocking_map.iter().any(|entity| entity.pos == clicked_pos) {
                    println!("Position already occupied!");
                } else { // Position isn't occupied.
                    println!("Spawn new entity on board position {} {}", clicked_pos.x, clicked_pos.y);

                    self.board.blocking_map.push(
                        Entity{
                            pos: utils::Position{
                                x: clicked_pos.x,
                                y: clicked_pos.y,
                            }, 
                            blocking: true
                        },
                    );
                }
            },
            MouseButton::Right => {
                let clicked_pos = utils::Position{
                    x: x as i32 / self.board.scale,
                    y: y as i32 / self.board.scale,
                };

                if let Some(index) = self.board.blocking_map.iter().position(|entity| entity.pos == clicked_pos) {
                    self.board.blocking_map.remove(index);
                } else { // Position isn't occupied.
                    println!("There is not tile in this position!");
                }
            },
            _ => {},
        }
    }

    pub fn on_keyboard_input(&mut self, key: Key) {
        match key {
            Key::P => { self.write_map(String::from("default.map")) },
            Key::L => { self.load_map(String::from("default.map")) },
            _ => {},
        }
    }
    
    pub fn on_render(&mut self, event: Event, window: &mut PistonWindow) {
        window.draw_2d(&event, |context, graphics, _device| {
            // Clear screen with white-ish color.
            clear([0.9; 4], graphics);

            //TODO: create method to draw sprite for entities.
            let player_sprite = [
                (self.player.pos.x * self.board.scale) as f64,
                (self.player.pos.y * self.board.scale) as f64,
                self.board.scale as f64,
                self.board.scale as f64,
            ];

            rectangle(
                [0.6, 0.0, 0.0, 1.0], // red
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
                    [0.4, 0.4, 0.4, 1.0], // dark-grey
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
        println!("Loading tilemap {}...", filename);
        if let Ok(mut file) = File::open(&filename) {
            if let Ok(mut new_board) = Board::load(&mut file) {
                self.board = new_board;
                println!("Tilemap {} loaded!", filename);
            }
            println!("Error parsing tilemap {}!", filename);
        }
        println!("Unable to open {}!", filename);
    }

    pub fn write_map(&mut self, mut filename: String) {
        println!("Creating tilemap {}...", filename);
        let mut file = File::create(&mut filename).unwrap();
        self.board.write(&mut file);
        println!("tilemap {} created!", filename);
    }
}
