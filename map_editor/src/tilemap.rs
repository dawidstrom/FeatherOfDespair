extern crate piston_window;

use piston_window::*;

use crate::board;
use crate::entity;
use crate::utils;
use crate::tile;

use board::*;
use entity::*;
use tile::*;

use std::{
    fs::File,
};

pub struct TileMap {
    pub player: Entity,
    pub board: Board,
    pub selected_tile: Tile,
}

impl TileMap {
    pub fn new(size: utils::Rect) -> TileMap {
        let player = Entity{ 
            pos: utils::Position{ 
                x: 5, 
                y: 5,
            },
            blocking: false,
            tile: Tile::Player,
        };

        let board = Board{
            size: size,
            scale: 30,
            blocking_map: Vec::new(),
        };

        TileMap {
            player: player,
            board: board,
            selected_tile: Tile::Wall,
        }
    }

	pub fn on_keyboard_input(&mut self, key: Key) {
		match key {
			Key::P => { self.write_map(String::from("default.map")) },
			Key::L => { self.load_map(String::from("default.map")) },
			_ => {},
		}
	}


    pub fn place_block_if_empty(&mut self, [x,y]: [f64;2]) {
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
                    blocking: true,
                    tile: self.selected_tile,
                },
            );
        }
    }

    pub fn remove_block_if_occupied(&mut self, [x,y]: [f64;2]) {
        let clicked_pos = utils::Position{
            x: x as i32 / self.board.scale,
            y: y as i32 / self.board.scale,
        };

        if let Some(index) = self.board.blocking_map.iter().position(|entity| entity.pos == clicked_pos) {
            self.board.blocking_map.remove(index);
        } else { // Position isn't occupied.
            println!("There is not tile in this position!");
        }
    }

    pub fn on_mouse_input(&mut self, button: MouseButton, [x,y]: [f64;2]) {
        println!("Clicked position {} {}", x, y);
        if x as i32 <= self.board.size.width * self.board.scale {
            match button {
                MouseButton::Left => {
                    self.place_block_if_empty([x,y]);
                },
                MouseButton::Right => {
                    self.remove_block_if_occupied([x,y]);
                },
                _ => {},
            }
        }
    }
    
    pub fn on_render(&mut self,
                     event: &Event,
                     window: &mut PistonWindow) {
        window.draw_2d(event, |context, graphics, _device| {
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

                let color;
                match wall.tile {
                    Tile::Wall => color = [0.4, 0.4, 0.4, 1.0],
                    Tile::Grass => color = [0.0, 1.0, 0.0, 1.0],
                    Tile::Player => color = [0.8, 0.0, 0.0, 1.0],
                }

                rectangle(
                    color,
                    wall_sprite,
                    context.transform,
                    graphics
                );
            }
        });
    }

    pub fn load_map(&mut self, filename: String) {
        println!("Loading tilemap {}...", filename);
        if let Ok(mut file) = File::open(&filename) {
            if let Ok(new_board) = Board::load(&mut file) {
                self.board = new_board;
                println!("Tilemap {} loaded!", filename);
            } else {
                println!("Error parsing tilemap {}!", filename);
            }
        } else {
            println!("Unable to open {}!", filename);
        }
    }

    pub fn write_map(&mut self, mut filename: String) {
        println!("Creating tilemap {}...", filename);
        let mut file = File::create(&mut filename).unwrap();
        self.board.write(&mut file);
        println!("tilemap {} created!", filename);
    }
}
