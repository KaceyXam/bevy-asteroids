use bevy::prelude::*;
use bevy::render::{mesh::Indices, render_resource::PrimitiveTopology};
use bevy::sprite::MaterialMesh2dBundle;

#[derive(Component)]
struct Player {
    speed: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Asteroids"),
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_triangle)
        .add_system(player_movement)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_triangle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(create_triangle()).into(),
            transform: Transform::default().with_scale(Vec3::splat(16.)),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()
        })
        .insert(Player { speed: 0.0 });
}

fn create_triangle() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[-1.0, -1.0, 0.0], [1.0, -1.0, 0.0], [0.0, 1.0, 0.0]],
    );
    mesh.set_indices(Some(Indices::U32(vec![0, 1, 2])));
    mesh
}

fn player_movement(
    mut player_query: Query<(&mut Transform, &mut Player)>,
    key: Res<Input<KeyCode>>,
    time: Res<Time>,
    window_query: Query<&Window>,
) {
    let window = window_query.get_single().unwrap();
    if let Ok((mut player_transform, mut player)) = player_query.get_single_mut() {
        let mut new_speed = player.speed;
        new_speed += if key.pressed(KeyCode::Up) {
            1.0f32
        } else {
            -2.5f32
        };
        player.speed = new_speed.clamp(0.0, 750.0);

        if key.pressed(KeyCode::Left) {
            player_transform.rotate_z(5.0 * time.delta_seconds());
        }
        if key.pressed(KeyCode::Right) {
            player_transform.rotate_z(-5.0 * time.delta_seconds());
        }

        let movement = player_transform.rotation * Vec3::Y;
        player_transform.translation +=
            player.speed.clamp(0.0, 50.0) * time.delta_seconds() * movement;

        if player_transform.translation.x <= -window.width() / 2.0 {
            player_transform.translation.x = window.width() / 2.0;
        }
    }
}
