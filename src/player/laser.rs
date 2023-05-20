use bevy::prelude::*;
use bevy::sprite::*;

use super::Player;

const LASER_SPEED: f32 = 1000f32;

#[derive(Component)]
pub struct Laser {
    pub dir: Vec3,
}

pub fn spawn_laser(
    mut commands: Commands,
    key: Res<Input<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player_query: Query<&Transform, With<Player>>,
) {
    if let Ok(player) = player_query.get_single() {
        if key.just_pressed(KeyCode::Space) {
            let dir = player.rotation * Vec3::Y;
            commands
                .spawn(MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(shape::Quad::new(Vec2::new(
                            2.0 / 16.0,
                            20.0 / 16.0,
                        ))))
                        .into(),
                    transform: *player,
                    material: materials.add(ColorMaterial::from(Color::WHITE)),
                    ..default()
                })
                .insert(Laser { dir });
        }
    }
}

pub fn move_laser(
    mut laser_query: Query<&mut Transform, With<Laser>>,
    time: Res<Time>,
    window_query: Query<&Window>,
) {
    let window = window_query.get_single().unwrap();
    for mut laser in laser_query.iter_mut() {
        let movement = laser.rotation * Vec3::Y;
        laser.translation += LASER_SPEED * time.delta_seconds() * movement;

        if laser.translation.x <= -window.width() / 2.0 - 10.0 {
            laser.translation.x = window.width() / 2.0;
        }
        if laser.translation.x >= window.width() / 2.0 + 10.0 {
            laser.translation.x = -window.width() / 2.0;
        }
        if laser.translation.y <= -window.height() / 2.0 - 10.0 {
            laser.translation.y = window.height() / 2.0;
        }
        if laser.translation.y >= window.height() / 2.0 + 10.0 {
            laser.translation.y = -window.height() / 2.0;
        }
    }
}
