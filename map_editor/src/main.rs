mod tilemap;
mod board;
mod entity;
mod utils;
mod control_panel;

extern crate piston_window;

use piston_window::*;

fn main() {
    // Setup game.
    let scale = 30;
    let mut tilemap = tilemap::TileMap::new(
        utils::Size{width:20,height:15}
    );
    let mut control_panel = control_panel::ControlPanel::new(
        utils::Position{ x: tilemap.board.size.width, y: 0 },
        utils::Size{ width: 6, height: tilemap.board.size.height },
    );
    
    // Setup piston.
    let window_size = [
        ((tilemap.board.size.width+control_panel.size.width) * scale) as f64,
        (tilemap.board.size.height * scale) as f64
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
            tilemap.on_keyboard_input(key);
        }
        if let Some(pos) = event.mouse_cursor_args() {
            mouse_position = pos;
        }
        if let Some(Button::Mouse(button)) = event.press_args() {
            tilemap.on_mouse_input(button, mouse_position);
        }

        // Update game world.

        // Main draw loop.
        tilemap.on_render(scale, &event, &mut window);
        control_panel.on_render(scale, &event, &mut window);
    }
}
