use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_asteroid)
            .add_system(move_asteroids);
    }
}

const ASTEROID_SPEED: f32 = 50f32;

#[derive(Component)]
struct Asteroid {
    size: f32,
    vel: Vec2,
}

fn spawn_asteroid(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(create_asteroid(meshes, materials, 128.));
}

fn create_asteroid(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    size: f32,
) -> (MaterialMesh2dBundle<ColorMaterial>, Asteroid) {
    (
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
            transform: Transform::default().with_scale(Vec3::splat(size)),
            material: materials.add(ColorMaterial::from(Color::BLACK)),
            ..default()
        },
        Asteroid {
            size,
            vel: Vec2::ONE,
        },
    )
}

fn move_asteroids(
    mut asteroid_query: Query<(&mut Transform, &Asteroid)>,
    window_query: Query<&Window>,
    time: Res<Time>,
) {
    let window = window_query.get_single().unwrap();
    for (mut transform, asteroid) in asteroid_query.iter_mut() {
        let delta = time.delta_seconds();
        transform.translation.x += asteroid.vel.x * delta * ASTEROID_SPEED;
        transform.translation.y += asteroid.vel.y * delta * ASTEROID_SPEED;

        if transform.translation.x <= -window.width() / 2.0 - asteroid.size / 2.0 {
            transform.translation.x = window.width() / 2.0;
        }
        if transform.translation.x >= window.width() / 2.0 + asteroid.size / 2.0 {
            transform.translation.x = -window.width() / 2.0;
        }
        if transform.translation.y <= -window.height() / 2.0 - asteroid.size / 2.0 {
            transform.translation.y = window.height() / 2.0;
        }
        if transform.translation.y >= window.height() / 2.0 + asteroid.size / 2.0 {
            transform.translation.y = -window.height() / 2.0;
        }
    }
}
