mod tilemap;
mod board;
mod entity;
mod utils;

extern crate piston_window;

use piston_window::*;

fn main() {
    // Setup game.
    let mut tile_map = tilemap::TileMap::new();
    
    // Setup piston.
    let window_size = [
        (tile_map.board.size.width  * tile_map.board.scale) as f64,
        (tile_map.board.size.height * tile_map.board.scale) as f64
    ];

    let mut mouse_position: [f64;2] = [0.0, 0.0];

    let mut window: PistonWindow =
        WindowSettings::new(
            "Map-Editor of Despair", window_size
        ).exit_on_esc(true).build().unwrap();

    // Main game loop.
    while let Some(event) = window.next() {
        // Handle input.
        if let Some(Button::Keyboard(key)) = event.press_args() {
            tile_map.on_keyboard_input(key);
        }
        if let Some(pos) = event.mouse_cursor_args() {
            mouse_position = pos;
        }
        if let Some(Button::Mouse(button)) = event.press_args() {
            tile_map.on_mouse_input(button, mouse_position);
        }

        // Update game world.

        // Main draw loop.
        tile_map.on_render(event, &mut window);
    }
}
