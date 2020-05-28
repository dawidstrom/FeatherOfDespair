mod game;
mod board;
mod entity;
mod utils;

extern crate piston_window;

use piston_window::*;

fn main() {
    // Setup game.
    let mut game = game::Game::new();
    
    // Setup piston.
    let window_size = [
        (game.board.size.width  * game.board.scale) as f64,
        (game.board.size.height * game.board.scale) as f64
    ];

    let mut window: PistonWindow =
        WindowSettings::new(
            "Feather of Despair", window_size
        ).exit_on_esc(true).build().unwrap();

    // Main game loop.
    while let Some(event) = window.next() {
        // Handle input.
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.on_input(key);
        }

        // Update game world.

        // Main draw loop.
        game.on_render(event, &mut window);
    }
}
