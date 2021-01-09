extern crate piston_window;

use piston_window::*;

use crate::board;
use crate::entity;
use crate::utils;
use crate::camera;
use crate::tile::Tile;

use board::*;
use entity::*;
use camera::*;

use std::fs::File;

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
                tile_type: Tile::Player,
                is_movement_blocking: false,
                is_vision_blocking: false,
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
            entities: Vec::new(),
        };

        let debug = Debug{
            pos: utils::Position{ x: 7, y: 5 },
            is_active: false,
            path: Vec::new(),
            sample_points: Vec::new()
        };
        
        Game {
            player,
            board,
            lights: Vec::new(),
            debug,
            camera: Camera{
                pos: utils::Position{ x: 0, y: 0 },
                size: utils::Rect{ width: 15, height: 13 },
            },
        }
    }

    pub fn update_key(&mut self, key: &Key, is_pressed: bool) {
        match key {
            Key::W => self.player.entity.moving.up = is_pressed,
            Key::D => self.player.entity.moving.right = is_pressed,
            Key::S => self.player.entity.moving.down = is_pressed,
            Key::A => self.player.entity.moving.left = is_pressed,
            _ => {},
        }
    }

    pub fn on_input(&mut self, button_args: ButtonArgs) {
        if let Button::Keyboard(key) = button_args.button {
            if button_args.state == ButtonState::Press {
                self.update_key(&key, true);
            }
            if button_args.state == ButtonState::Release {
                self.update_key(&key, false);
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

    /// Returns whether start_pos is in line of sight of end_pos.
    fn is_in_line_of_sight(&self,
                           number_of_samples: usize,
                           x_step: f64,
                           y_step: f64,
                           start_pos: utils::Position) -> bool {
        // Start position.
        let mut current_pos = self.player.entity.pos;
        let mut last_pos = self.player.entity.pos;
        let mut is_visible = true;

        // Sample for vision blocking tiles along line of sight between start
        // and end position.
        for i in 0..number_of_samples as i32 {

            // Used to check if difference between sample points are diagonal.
            // let dx = start_pos.x-last_pos.x;
            // let dy = start_pos.y-last_pos.y;

            // let mut entity_x = false;
            // let mut entity_y = false;

            // Check that point is not occupied by vision blocking tile.
            for entity in self.board.entities.iter() {

                // Not a vision blocking tile, skip check.
                if !entity.is_vision_blocking {
                    continue;
                }

                // A tile cannot block itself.
                if entity.pos == start_pos {
                    continue;
                }

                if entity.pos == current_pos {
                    is_visible = false;
                    break;
                }
                
                // // Check if movement is horizontal.
                // // Note: assumes only 1 tile movement.
                // if start_pos != last_pos && dx.abs() + dy.abs() == 2 {
                //     // Check if neighbouring tiles are walls.
                //     if entity.pos.x == last_pos.x+dx && entity.pos.y == last_pos.y {
                //         entity_x = true;
                //     }
                //     if entity.pos.y == last_pos.y+dy && entity.pos.x == last_pos.x {
                //         entity_y = true;
                //     }

                //     if entity_x && entity_y {
                //         is_visible = false;
                //         break;
                //     }
                // }
            }

            // Update last visited position.
            if current_pos != last_pos {
                last_pos = current_pos;
            }

            // // For drawing the path checked between player and debug position.
            // if self.debug.is_active && start_pos == self.debug.pos {
            //     self.debug.path.push(start_pos);

            //     // Save the actual sample point (no rounding).
            //     self.debug.sample_points.push([
            //         current_pos.x as f64 + (i as f64 * x_step),
            //         current_pos.y as f64 + (i as f64 * y_step)
            //     ]);
            // }

            // Move one step in direction towards end_pos.
            // round() => get point closest to sample.
            current_pos = utils::Position{
                x: start_pos.x + (i as f64 * x_step).round() as i32,
                y: start_pos.y + (i as f64 * y_step).round() as i32
            };
        }

        is_visible
    }

    pub fn is_visible_by_player(&self, pos: utils::Position) -> bool {

        // Don't bother checking  player position, will be filled by player.
        if pos == self.player.entity.pos { return true; }

        let dist = utils::Position::distance(self.player.entity.pos, pos);
        let x_diff = self.player.entity.pos.x - pos.x;
        let y_diff = self.player.entity.pos.y - pos.y;

        // Really shitty method to check line-of-sight.
        //
        // Check if the closest point to the line formed between pos and player 
        // is a wall when sampled 2*dist times along the line.
        let number_of_samples = dist * 2.0;
        // How much to step in x/y direction for each sample.
        let x_step = x_diff as f64 / number_of_samples;
        let y_step = y_diff as f64 / number_of_samples;
        
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

        self.is_in_line_of_sight(number_of_samples as usize,
                                 x_step,
                                 y_step,
                                 pos)
    }

    pub fn draw_ground(&mut self,
                       ground_pos: utils::Position,
                       ctx: &Context,
                       graphics: &mut G2d) {

        let player_pos = self.player.entity.pos;
        // Reduce light intensity based on distance from player.
        let distance = utils::Position::distance(player_pos, ground_pos) as f32;
        let intensity = 1.0-distance/RENDER_DISTANCE;

        // Workaround since cmp::max doesn't work for f32...
        let mut color = 0.0;
        if intensity > 0.0 { color = intensity; }

        Game::draw_block(self, &ground_pos, [color, color, color, 1.0], ctx, graphics);
    }

    pub fn draw_light(&mut self,
                      ctx: &Context,
                      graphics: &mut G2d) {

        self.debug.clear();

        // For each point on board...
        let camera_edge_x = self.camera.pos.x+self.camera.size.width;
        let camera_edge_y = self.camera.pos.y+self.camera.size.height;
        for x in self.camera.pos.x..camera_edge_x {
            for y in self.camera.pos.y..camera_edge_y {
                let pos = utils::Position{x,y};

                // If visible, draw ground.
                if Game::is_visible_by_player(self, pos) {
                    Game::draw_ground(self, pos, ctx, graphics);
                }
            }
        }
    }

    pub fn draw_wall(&self,
                     light_intensity: f32,
                     wall: &Entity,
                     context: &Context,
                     graphics: &mut G2d) {
        let mut color;
        match wall.tile_type {
            Tile::Player => color = [1.0, 0.0, 0.0, 1.0],
            Tile::Grass => color = [0.0, 1.0, 0.0, 1.0],
            Tile::Wall => color = [0.4, 0.4, 0.4, 1.0],
            Tile::Monster => color = [0.8, 0.0, 0.8, 1.0],
        };
        // Workaround since cmp::max doesn't work for f32...
        if light_intensity > 0.0 {
            color[0] *= light_intensity;
            color[1] *= light_intensity;
            color[2] *= light_intensity;
        }

        Game::draw_block(
            self,
            &wall.pos,
            color,
            &context,
            graphics,
        );
    }

    pub fn draw_wall_if_visible(&mut self,
                             wall_index: usize,
                             context: &Context,
                             graphics: &mut G2d) {
        let pos = self.player.entity.pos;

        let wall = &self.board.entities[wall_index];

        let distance = utils::Position::distance(pos, wall.pos) as f32;
        let light_intensity = 1.0-distance/RENDER_DISTANCE;

        let half_camera_width = (self.camera.size.width/2) as i32;
        let half_camera_height = (self.camera.size.height/2) as i32;

        if distance < RENDER_DISTANCE {
            let mut draw_pos = Position{
                x: std::cmp::max(pos.x-half_camera_width, 0),
                y: std::cmp::max(pos.y-half_camera_height, 0),
            };
            let dx = self.board.size.width - self.camera.size.width;
            let dy = self.board.size.height - self.camera.size.height;
            draw_pos.x = std::cmp::min(draw_pos.x, dx);
            draw_pos.y = std::cmp::min(draw_pos.y, dy);

            // TODO: Check if wall is in camera range.
            // If visible, draw tile.
            if self.is_visible_by_player(wall.pos) {
                Game::draw_wall(self, light_intensity, wall, context, graphics);
            }
        }
    }

    pub fn calculate_camera_offset(&mut self) {
        let pos = self.player.entity.pos;

        let mut camera_offset = utils::Position{
            x: std::cmp::max(pos.x-(self.camera.size.width/2) as i32, 0),
            y: std::cmp::max(pos.y-(self.camera.size.height/2) as i32, 0),
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
            for i in 0..self.board.entities.len() {
                self.draw_wall_if_visible(i, &context, graphics);
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
