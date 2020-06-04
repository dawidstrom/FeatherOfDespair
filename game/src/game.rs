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

pub struct Game {
    pub player: Player,
    pub board: Board,
}

impl Game {
    pub fn new() -> Game {
        let player = Player {
            entity: Entity { 
                pos: utils::Position{ 
                    x: 5, 
                    y: 5,
                },
                blocking: false,
            },
            max_hp:     100,
            current_hp: 100,
        };

        let mut board = Board{
            size: utils::Size{ width: 20, height: 15 },
            scale: 30,
            blocking_map: Vec::new(),
        };

        // Add one wall to board.
        board.blocking_map.push(
            Entity{ pos: utils::Position{ x:1, y:1 }, blocking: true }
        );
        
        Game {
            player: player,
            board: board,
        }
    }

    fn on_update(&mut self) {
    }
    
    pub fn on_input(&mut self, key: Key) {
        match key {
            Key::W => self.player.entity.move_dir(1, Direction::Up,    &mut self.board),
            Key::D => self.player.entity.move_dir(1, Direction::Right, &mut self.board),
            Key::S => self.player.entity.move_dir(1, Direction::Down,  &mut self.board),
            Key::A => self.player.entity.move_dir(1, Direction::Left,  &mut self.board),
            _ => {},
        }
    }

    pub fn on_render(&mut self, event: Event, window: &mut PistonWindow) {
        window.draw_2d(&event, |context, graphics, _device| {
            // Clear screen.
            clear([1.0; 4], graphics);

            //TODO: create method to draw sprite for entities.
            let player_sprite = [
                (self.player.entity.pos.x * self.board.scale) as f64,
                (self.player.entity.pos.y * self.board.scale) as f64,
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
}
