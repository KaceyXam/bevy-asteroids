use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement);
    }
}

#[derive(Component)]
struct Player {
    speed: f32,
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(crate::create_triangle()).into(),
            transform: Transform::from_xyz(0.0, 0.0, 10.0).with_scale(Vec3::splat(16.)),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()
        })
        .insert(Player { speed: 0.0 });
}

fn player_movement(
    mut player_query: Query<(&mut Transform, &mut Player)>,
    key: Res<Input<KeyCode>>,
    time: Res<Time>,
    window_query: Query<&Window>,
) {
    let window = window_query.get_single().unwrap();
    if let Ok((mut player_transform, mut player)) = player_query.get_single_mut() {
        let new_speed = if key.pressed(KeyCode::Up) {
            player.speed + time.delta_seconds() * 1000f32
        } else {
            player.speed * 0.98
        };
        player.speed = new_speed.clamp(0.0, 250.0);

        if key.pressed(KeyCode::Left) {
            player_transform.rotate_z(5.0 * time.delta_seconds());
        }
        if key.pressed(KeyCode::Right) {
            player_transform.rotate_z(-5.0 * time.delta_seconds());
        }

        let movement = player_transform.rotation * Vec3::Y;
        player_transform.translation += player.speed * time.delta_seconds() * movement;

        if player_transform.translation.x <= -window.width() / 2.0 - 10.0 {
            player_transform.translation.x = window.width() / 2.0;
        }
        if player_transform.translation.x >= window.width() / 2.0 + 10.0 {
            player_transform.translation.x = -window.width() / 2.0;
        }
        if player_transform.translation.y <= -window.height() / 2.0 - 10.0 {
            player_transform.translation.y = window.height() / 2.0;
        }
        if player_transform.translation.y >= window.height() / 2.0 + 10.0 {
            player_transform.translation.y = -window.height() / 2.0;
        }
    }
}
