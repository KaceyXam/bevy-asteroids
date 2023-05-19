use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_asteroid)
            .add_system(move_asteroids);
    }
}

const ASTEROID_SPEED: f32 = 50f32;
const NO_SPAWN_RADIUS: f32 = 150f32;

#[derive(Component)]
pub struct Asteroid {
    pub size: f32,
    vel: Vec2,
}

fn spawn_asteroid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window>,
) {
    let window = window_query.get_single().unwrap();
    for _ in 0..=10 {
        let size = rand::thread_rng().gen::<f32>() * 200f32;
        let vel = Vec2::new(
            rand::thread_rng().gen_range(-1.0..1.0),
            rand::thread_rng().gen_range(-1.0..1.0),
        );

        let mut pos = Vec2::new(
            rand::thread_rng().gen_range(-window.width() / 2.0..window.width() / 2.0),
            rand::thread_rng().gen_range(-window.height() / 2.0..window.height() / 2.0),
        );

        while pos.distance(Vec2::ZERO) < NO_SPAWN_RADIUS {
            pos = Vec2::new(
                rand::thread_rng().gen_range(-window.width() / 2.0..window.width() / 2.0),
                rand::thread_rng().gen_range(-window.height() / 2.0..window.height() / 2.0),
            );
        }

        commands.spawn(create_asteroid(&mut meshes, &mut materials, size, vel, pos));
    }
}

fn create_asteroid(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    size: f32,
    vel: Vec2,
    pos: Vec2,
) -> (MaterialMesh2dBundle<ColorMaterial>, Asteroid) {
    (
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
            transform: Transform::from_xyz(pos.x, pos.y, 0.0).with_scale(Vec3::splat(size)),
            material: materials.add(ColorMaterial::from(Color::BLACK)),
            ..default()
        },
        Asteroid { size, vel },
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
            transform.translation.x = window.width() / 2.0 + asteroid.size / 4.0;
        }
        if transform.translation.x >= window.width() / 2.0 + asteroid.size / 2.0 {
            transform.translation.x = -window.width() / 2.0 - asteroid.size / 4.0;
        }
        if transform.translation.y <= -window.height() / 2.0 - asteroid.size / 2.0 {
            transform.translation.y = window.height() / 2.0 + asteroid.size / 4.0;
        }
        if transform.translation.y >= window.height() / 2.0 + asteroid.size / 2.0 {
            transform.translation.y = -window.height() / 2.0 - asteroid.size / 4.0;
        }
    }
}
