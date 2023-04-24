use bevy::prelude::*;

// components

// right handed, y axis == up
#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

struct Player;

// systems

fn keyboard_input(keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        // Space was pressed
        info!("Space was pressed");
    }
    if keys.just_released(KeyCode::LControl) {
        // Left Ctrl was released
        info!("Left Ctrl was released");
    }
    if keys.pressed(KeyCode::W) {
        // W is being held down
        info!("W is being held down");
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.9, 0.3, 0.6))) // not working yet
        .add_plugins(DefaultPlugins)
        .add_system(bevy::window::close_on_esc)
        .add_system(keyboard_input)
        .run();
}
