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
    pub lights: Vec<Entity>,
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

        let mut lights = Vec::new();
        
        Game {
            player: player,
            board: board,
            lights: lights,
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
            // Clear screen with black.
            clear([0.0; 4], graphics);

            let player_sprite = [
                (self.player.entity.pos.x * self.board.scale) as f64,
                (self.player.entity.pos.y * self.board.scale) as f64,
                self.board.scale as f64,
                self.board.scale as f64,
            ];

            // Draw white ground on areas visible from torches.
            //TODO: Just calculate positions in torch area instead.
            for x in 0..self.board.size.width {
                for y in 0..self.board.size.height {
                    let pos = utils::Position{x,y};
                    if utils::Position::square_distance(
                        self.player.entity.pos,
                        pos
                    ) < 15 {
                        rectangle(
                            [0.9,0.9,0.9,1.0], // white-ish
                            [(x*self.board.scale) as f64,
                            (y*self.board.scale) as f64,
                            self.board.scale as f64,
                            self.board.scale as f64],
                            context.transform,
                            graphics
                        );
                    }
                }
            }

            rectangle(
                [0.6, 0.0, 0.0, 1.0], // red
                player_sprite,
                context.transform,
                graphics
            );

            for wall in self.board.blocking_map.iter() {
                if utils::Position::square_distance(
                    self.player.entity.pos,
                    wall.pos
                ) < 15 {
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
