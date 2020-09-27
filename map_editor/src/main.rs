mod tilemap;
mod board;
mod entity;
mod utils;
mod control_panel;
mod tile;
mod graphics;
mod button;
mod interaction;

extern crate piston_window;
#[macro_use] extern crate enum_primitive;

use piston_window::*;

fn main() {
    // Setup game.
    let scale = 30;
    let tilemap_size = utils::Rect{width:20,height:15};
    let mut control_panel = control_panel::ControlPanel::new(
        tilemap_size,
        utils::Rect{ width: 6, height: tilemap_size.height },
        scale,
    );

    // Setup piston.
    let window_size = [
        ((tilemap_size.width+control_panel.size.width) * control_panel.scale) as f64,
        (tilemap_size.height * scale) as f64
    ];

    let mut mouse_position: [f64;2] = [0.0, 0.0];

    let mut window: PistonWindow =
        WindowSettings::new(
            "Map-Editor of Despair", window_size
        ).exit_on_esc(true).build().unwrap();

    // Main game loop.
    while let Some(event) = window.next() {
        // Handle input.
        if let Some(piston_window::Button::Keyboard(key)) = event.press_args() {
            control_panel.on_keyboard_input(key);
        }
        if let Some(pos) = event.mouse_cursor_args() {
            mouse_position = pos;
        }
        if let Some(piston_window::Button::Mouse(button)) = event.press_args() {
            // Check if tilemap or control panel pressed.
            control_panel.on_mouse_input(button, mouse_position);
        }

        // Update game world.

        // Main draw loop.
        // Clear screen with white-ish color.
        window.draw_2d(&event, |_context, graphics, _device| {
            clear([0.9; 4], graphics);
        });
        control_panel.on_render(&event, &mut window);
    }
}
