use bevy::prelude::*;

// components

// right handed, y axis == up
#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Component)]
struct Player;

// systems
fn move_player_system(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, Option<&Player>)>,
) {
    for (mut transform, player) in query.iter_mut() {
        if let Some(player) = player {
            if keys.pressed(KeyCode::W) {
                debug!("W is being held down");
                transform.translation.z += 0.1;
            }
            if keys.pressed(KeyCode::A) {
                debug!("A is being held down");
                transform.translation.x -= 0.1;
            }
            if keys.pressed(KeyCode::S) {
                debug!("S is being held down");
                transform.translation.z -= 0.1;
            }
            if keys.pressed(KeyCode::D) {
                debug!("D is being held down");
                transform.translation.x += 0.1;
            }
        }
    }
}

fn keyboard_input(keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::W) {
        info!("Up(W) was pressed");
    }
    if keys.just_pressed(KeyCode::A) {
        info!("Left(A) was pressed");
    }
    if keys.just_pressed(KeyCode::S) {
        info!("Down(S) was pressed");
    }
    if keys.just_pressed(KeyCode::D) {
        info!("Right(D) was pressed");
    }
    if keys.just_released(KeyCode::W) {
        // W was released
        info!("W was released");
    }
    if keys.pressed(KeyCode::W) {
        // W is being held down
        info!("W is being held down");
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(Player {});
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.9, 0.3, 0.6))) // not working yet
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .add_system(keyboard_input)
        .add_system(move_player_system)
        .run();
}
