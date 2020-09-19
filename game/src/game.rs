extern crate piston_window;

use piston_window::*;

use crate::board;
use crate::entity;
use crate::utils;
use crate::camera;

use board::*;
use entity::*;
use camera::*;

use std::{
    fs::File,
};

static RENDER_DISTANCE: f32 = 10.0;

pub struct Game {
    pub player:         Player,
    pub board:          Board,
    pub lights:         Vec<Entity>,
    pub debug:          Debug,
    pub camera:         Camera,
}

pub struct Debug {
    pub pos:            utils::Position,
    pub is_active:      bool,
    pub path:           Vec<utils::Position>,
    pub sample_points:  Vec<[f64;2]>
}

impl Debug {
    pub fn clear(&mut self) {
        self.path.clear();
        self.sample_points.clear();
    }
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
                moving: Direction::default(),
                move_timer: Some(Timer {
                    remaining: 0,
                    duration: 100,
                    looping: true
                }),
            },
            max_hp:     100,
            current_hp: 100,
        };

        let board = Board{
            size: utils::Rect{ width: 20, height: 15 },
            scale: 30,
            blocking_map: Vec::new(),
        };

        let debug = Debug{
            pos: utils::Position{ x: 11, y: 1 },
            is_active: false,
            path: Vec::new(),
            sample_points: Vec::new()
        };
        
        Game {
            player: player,
            board: board,
            lights: Vec::new(),
            debug: debug,
            camera: Camera{
                pos: utils::Position{ x: 0, y: 0 },
                size: utils::Rect{ width: 15, height: 13 },
            },
        }
    }

    pub fn on_input(&mut self, button_args: ButtonArgs) {
        if let Button::Keyboard(key) = button_args.button {
            if button_args.state == ButtonState::Press {
                match key {
                    Key::W => self.player.entity.moving.up = true,
                    Key::D => self.player.entity.moving.right = true,
                    Key::S => self.player.entity.moving.down = true,
                    Key::A => self.player.entity.moving.left = true,
                    _ => {},
                }
            }

            if button_args.state == ButtonState::Release {
                match key {
                    Key::W => self.player.entity.moving.up = false,
                    Key::D => self.player.entity.moving.right = false,
                    Key::S => self.player.entity.moving.down = false,
                    Key::A => self.player.entity.moving.left = false,
                    _ => {},
                }
            }
        }
    }

    pub fn on_update(&mut self, elapsed: i64) {
        self.player.update(&mut self.board, elapsed);
    }
                

    pub fn draw_debug(&self, ctx: &Context, graphics: &mut G2d) {

        // Draw debug path.
        for pos in self.debug.path.iter() {
            Game::draw_block(
                self,
                pos,
                [1.0, 1.0, 0.0, 1.0], // yellow
                &ctx,
                graphics,
            );
        }

        // Draw debug position.
        Game::draw_block(
            self,
            &self.debug.pos,
            [0.0, 1.0, 0.0, 1.0], // dark-grey
            &ctx,
            graphics,
        );
        
        // Draw debug sample points.
        for [x,y] in self.debug.sample_points.iter() {
            // Offset sample point to match logical sample point.
            // Take note that the sample points arent rounded!
            let sprite = [
                12.5 + x * self.board.scale as f64,
                12.5 + y * self.board.scale as f64,
                5.0,
                5.0
            ];
            rectangle(
                [0.0, 1.0, 1.0, 1.0],
                sprite,
                ctx.transform,
                graphics
            );
        }
    }
    
    pub fn draw_block(&self,
                      pos: &utils::Position,
                      color: [f32; 4],
                      ctx: &Context,
                      graphics: &mut G2d) {
        let x = pos.x - self.camera.pos.x;
        let y = pos.y - self.camera.pos.y;

        let sprite = [
            (x * self.board.scale) as f64,
            (y * self.board.scale) as f64,
            self.board.scale as f64,
            self.board.scale as f64,
        ];
        rectangle(
            color,
            sprite,
            ctx.transform,
            graphics
        );
    }

    pub fn draw_player(&self, ctx: &Context, graphics: &mut G2d) {
        Game::draw_block(
            self,
            &self.player.entity.pos,
            [1.0, 0.0, 0.0, 1.0],
            ctx,
            graphics
        );
    }

    pub fn is_visible(&mut self, pos: utils::Position) -> bool {

        // Don't bother checking  player position, will be filled by player.
        if pos == self.player.entity.pos { return true; }

        // Distance between pos and player.
        let dist = utils::Position::distance(self.player.entity.pos, pos);

        // Delta between pos and player.
        let x_diff = self.player.entity.pos.x - pos.x;
        let y_diff = self.player.entity.pos.y - pos.y;

        // Really shitty method to check line-of-sight.
        //
        // Check if the closest point to the line formed between pos and player 
        // is a wall when sampled 2*dist times along the line.
        let steps = dist * 2.0;
        // How much to step in x/y direction for each sample.
        let x_step = x_diff as f64 / steps;
        let y_step = y_diff as f64 / steps;
        
        if self.debug.is_active && pos == self.debug.pos {
            println!(
                "player x,y: ({},{}), green x,y: ({},{}), \
                 x/y-diff: ({},{}), dist: {}, x,y-step: \
                 ({},{})", 
                 self.player.entity.pos.x,
                 self.player.entity.pos.y,
                 pos.x,
                 pos.y,
                 x_diff,
                 y_diff,
                 dist,
                 x_step,
                 y_step
            );
        }
        
        // Start at pos.
        let mut start_pos = pos;
        let mut last_pos = pos;
        let mut is_visible = true;

        // Sample "steps" times along the line formed between player and pos.
        for i in 0..steps as i32 {
            if self.debug.is_active && pos == self.debug.pos {
                println!(
                    "index: {}, pos: ({},{})",
                     i,
                     start_pos.x, start_pos.y
                );
            }
            
            let dx = start_pos.x-last_pos.x;
            let dy = start_pos.y-last_pos.y;

            let mut wall_x = false;
            let mut wall_y = false;

            // Check that point is not occupied by wall.
            for wall in self.board.blocking_map.iter() {
                if wall.pos == start_pos {
                    is_visible = false;
                    break;
                }
                
                // Check if movement is horizontal.
                // Note: assumes only 1 tile movement.
                if start_pos != last_pos && dx.abs() + dy.abs() == 2 {
                    // Check if neighbouring tiles are walls.
                    if wall.pos.x == last_pos.x+dx && wall.pos.y == last_pos.y {
                        wall_x = true;
                    }
                    if wall.pos.y == last_pos.y+dy && wall.pos.x == last_pos.x {
                        wall_y = true;
                    }

                    if wall_x && wall_y {
                        is_visible = false;
                        break;
                    }
                }
            }

            // Update last visited position.
            if start_pos != last_pos {
                last_pos = start_pos;
            }

            // For drawing the path checked between player and debug position.
            if self.debug.is_active && pos == self.debug.pos {
                self.debug.path.push(start_pos);

                // Save the actual sample point (no rounding).
                self.debug.sample_points.push([
                    pos.x as f64 + (i as f64 * x_step),
                    pos.y as f64 + (i as f64 * y_step)
                ]);
            }

            // Move one step in direction towards player.
            // round() => get point closest to sample.
            start_pos = utils::Position{
                x: pos.x + (i as f64 * x_step).round() as i32,
                y: pos.y + (i as f64 * y_step).round() as i32
            };
        }

        is_visible
    }

    pub fn draw_light(&mut self,
                      ctx: &Context,
                      graphics: &mut G2d) {

        self.debug.clear();

        // For each point on board...
        for x in self.camera.pos.x..(self.camera.pos.x+self.camera.size.width) {
            for y in self.camera.pos.y..(self.camera.pos.y+self.camera.size.height) {
                let pos = utils::Position{x,y};

                // If visible, draw ground.
                if Game::is_visible(self, pos) {
                    // Reduce light intensity based on distance from player.
                    let distance = utils::Position::distance(self.player.entity.pos, pos) as f32;
                    let intensity = 1.0-distance/RENDER_DISTANCE;

                    // Workaround since cmp::max doesn't work for f32...
                    let mut color = 0.0;
                    if intensity > 0.0 { color = intensity; }

                    Game::draw_block(self, &pos, [color, color, color, 1.0], ctx, graphics);
                }
            }
        }
    }

    pub fn draw_wall(&self,
                     light_intensity: f32,
                     wall: &Entity,
                     context: &Context,
                     graphics: &mut G2d) {
        let mut color = 0.4;
        // Workaround since cmp::max doesn't work for f32...
        if light_intensity > 0.0 { color *= light_intensity; }

        Game::draw_block(
            self,
            &wall.pos,
            [color, color, color, 1.0], // dark-grey
            &context,
            graphics,
        );
    }

    pub fn draw_wall_if_visible(&self,
                             wall: &Entity,
                             context: &Context,
                             graphics: &mut G2d) {
        let distance = utils::Position::distance(self.player.entity.pos, wall.pos) as f32;
        let light_intensity = 1.0-distance/RENDER_DISTANCE;

        if distance < RENDER_DISTANCE {
            let mut draw_pos = Position{
                x: std::cmp::max(self.player.entity.pos.x-(self.camera.size.width/2) as i32, 0),
                y: std::cmp::max(self.player.entity.pos.y-(self.camera.size.height/2) as i32, 0),
            };
            let dx = self.board.size.width - self.camera.size.width;
            let dy = self.board.size.height - self.camera.size.height;
            draw_pos.x = std::cmp::min(draw_pos.x, dx);
            draw_pos.y = std::cmp::min(draw_pos.y, dy);

            // TODO: Check if wall is in camera range.
            self.draw_wall(light_intensity, wall, context, graphics);
        }
    }

    pub fn calculate_camera_offset(&mut self) {
        let mut camera_offset = utils::Position{
            x: std::cmp::max(self.player.entity.pos.x-(self.camera.size.width/2) as i32, 0),
            y: std::cmp::max(self.player.entity.pos.y-(self.camera.size.height/2) as i32, 0),
        };
        let dx = self.board.size.width - self.camera.size.width;
        let dy = self.board.size.height - self.camera.size.height;
        camera_offset.x = std::cmp::min(camera_offset.x, dx);
        camera_offset.y = std::cmp::min(camera_offset.y, dy);

        self.camera.pos = camera_offset;
    }

    pub fn on_render(&mut self, event: Event, window: &mut PistonWindow) {
        window.draw_2d(&event, |context, graphics, _device| {
            // Clear screen with black.
            clear([0.0; 4], graphics);

            // Calculate camera offset.
            self.calculate_camera_offset();

            // Draw white ground on areas visible from torches.
            self.draw_light(&context, graphics);

            // Draw walls if visible from player.
            for wall in self.board.blocking_map.iter() {
                self.draw_wall_if_visible(wall, &context, graphics);
            }
            
            // Draw player.
            self.draw_player(&context, graphics);

            // Draw debug.
            if self.debug.is_active {
                self.draw_debug(
                    &context,
                    graphics,
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
}
