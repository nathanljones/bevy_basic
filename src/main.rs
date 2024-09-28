use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Ball;

#[derive(Bundle)]
struct BallBundle {
    ball: Ball,
    position: Position,
}

impl BallBundle {
    fn new(x: f32, y: f32) -> Self {
        Self {
            ball: Ball,
            position: Position(Vec2::new(x, y)),
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_ball)
        .add_systems(Update, project_positions)
        .run();
}
fn spawn_camera(mut commands: Commands) {
    // commands.spawn_empty().insert(Camera2dBundle::default());

    commands.spawn((Camera2dBundle { ..default() }));
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Mesh::from(Circle::new(10.0));
    let color = ColorMaterial::from(Color::srgb(1., 0., 0.));

    // `Assets::add` will load these into memory and return a
    // `Handle` (an ID) to these assets. When all references
    // to this `Handle` are cleaned up the asset is cleaned up.
    let mesh_handle = meshes.add(shape);
    let material_handle = materials.add(color);

    commands.spawn((
        BallBundle::new(200., 200.),
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
            ..default()
        },
    ));
}

fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
    // Here we are iterating over the query to get the
    // components from our game world
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.)
    }
}
