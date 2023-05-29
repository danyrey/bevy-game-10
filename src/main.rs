use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, PolygonMode, PrimitiveTopology, RenderPipelineDescriptor, ShaderRef,
            SpecializedMeshPipelineError,
        },
    },
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

// components

#[derive(Component)]
struct Player;

#[derive(Component)]
struct FollowPlayer;

struct PlayerMoved(Transform);

// systems

// TODO: system ordering, make sure inputs run before everything else

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
            let translate = transform.forward() * 0.1;
            transform.translation += translate;
        }
        if keys.pressed(KeyCode::A) {
            debug!("A is being held down");
            let translate = transform.left() * 0.1;
            transform.translation += translate;
        }
        if keys.pressed(KeyCode::S) {
            debug!("S is being held down");
            let translate = transform.back() * 0.1;
            transform.translation += translate;
        }
        if keys.pressed(KeyCode::D) {
            debug!("D is being held down");
            let translate = transform.right() * 0.1;
            transform.translation += translate;
        }
        if keys.pressed(KeyCode::J) {
            debug!("J is being held down");
            transform.rotate_y(0.1);
        }
        if keys.pressed(KeyCode::K) {
            debug!("K is being held down");
            transform.rotate_y(-0.1);
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
            transform.look_at(ev.0.translation, Vec3::Y);
            let distance = ev.0.translation - transform.translation;
            let min_distance = 4.0;
            let delta_factor = distance.length() - min_distance;
            let new_translation = transform.translation + (transform.forward() * delta_factor);

            transform.translation.x = new_translation.x;
            transform.translation.y = ev.0.translation.y + 2.5;
            transform.translation.z = new_translation.z;

            debug!("distance: {:?}", distance.length());
            debug!("forward: {:?}", transform.forward());
        }
    }
}

/// A list of lines with a start and end position
#[derive(Debug, Clone)]
pub struct LineList {
    pub lines: Vec<(Vec3, Vec3)>,
}

impl From<LineList> for Mesh {
    fn from(line: LineList) -> Self {
        // This tells wgpu that the positions are list of lines
        // where every pair is a start and end point
        let mut mesh = Mesh::new(PrimitiveTopology::LineList);

        let vertices: Vec<_> = line.lines.into_iter().flat_map(|(a, b)| [a, b]).collect();
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh
    }
}

#[derive(Default, AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "050ce6ac-080a-4d8c-b6b5-b5bab7560d8f"]
struct LineMaterial {
    #[uniform(0)]
    color: Color,
}

impl Material for LineMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/line_material.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // This is the important part to tell bevy to render this material as a line between vertices
        descriptor.primitive.polygon_mode = PolygonMode::Line;
        Ok(())
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: (ResMut<Assets<StandardMaterial>>,ResMut<Assets<LineMaterial>>),
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        //material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    })
    .insert(Name::new("Plane"));
    // cube
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.0.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(Name::new("Player"))
        .insert(Player {});
    // line
    // Spawn a line strip that goes from point to point
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(LineList {
            lines: vec![(Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0))],
        })),
        //transform: Transform::from_xyz(0.5, 0.0, 0.0),
        material: materials.1.add(LineMaterial { color: Color::BLUE }),
        ..default()
    })
        .insert(Name::new("Y Axis"));
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    })
        .insert(Name::new("Main Point Light"));
    // camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(FollowPlayer {})
        .insert(Name::new("Camera"));
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(MaterialPlugin::<LineMaterial>::default())
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .add_event::<PlayerMoved>()
        .add_system(move_player_system)
        .add_system(follow_player_system)
        .run();
}
