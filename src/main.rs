use bevy::prelude::*;

// components

#[derive(Component)]
struct Player;

#[derive(Component)]
struct FollowPlayer;

struct PlayerMoved(Transform);

// systems

// TODO: Player and FollowPlayer does not make sure there is only one Player at a time

/// move player with wasd
fn move_player_system(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, With<Player>)>,
    mut ev_player_moved: EventWriter<PlayerMoved>,
) {
    for (mut transform, _) in query.iter_mut() {
        if keys.pressed(KeyCode::W) {
            debug!("W is being held down");
            transform.translation.z -= 0.1;
        }
        if keys.pressed(KeyCode::A) {
            debug!("A is being held down");
            transform.translation.x -= 0.1;
        }
        if keys.pressed(KeyCode::S) {
            debug!("S is being held down");
            transform.translation.z += 0.1;
        }
        if keys.pressed(KeyCode::D) {
            debug!("D is being held down");
            transform.translation.x += 0.1;
        }
        ev_player_moved.send(PlayerMoved(transform.clone()))
    }
}

/// follow a player: follow position and look at
fn follow_player_system(
    mut follow_player_query: Query<(&mut Transform, With<FollowPlayer>)>,
    mut ev_player_moved: EventReader<PlayerMoved>,
) {
    for ev in ev_player_moved.iter() {
        for (mut transform, _) in follow_player_query.iter_mut() {
            // TODO: adjust translation to follow at a distance. current one is hacky constant
            transform.look_at(ev.0.translation, Vec3::Y);
            transform.translation.x = ev.0.translation.x - 2.0;
            transform.translation.y = ev.0.translation.y + 2.5;
            transform.translation.z = ev.0.translation.z + 5.0;
        }
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
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(FollowPlayer {});
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .add_event::<PlayerMoved>()
        .add_system(move_player_system)
        .add_system(follow_player_system)
        .run();
}
