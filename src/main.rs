use bevy::{
    prelude::*,
    window::close_when_requested
};
use rand::prelude::*;

const TIME_STEP: f32 = 1.0 / 60.0;
const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);
const BOID_COUNT: usize = 50;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, close_when_requested)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, boid_movement_system)
        .run();
}

#[derive(Component)]
struct Boid {
    // linear speed in meters per second
    movement_speed: f32,
    // rotation speed in radians per second
    rotation_speed: f32,
}

impl Boid {
    const MOVEMENT_SPEED: f32 = 30.0;
    const ROTATION_SPEED: f32 = f32::to_radians(90.0);
    const PERCEPTION_RADIUS: f32 = 75.0;
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let boid_handle = asset_server.load("boid.png");
    let mut rng = rand::rng();

    let horizontal_margin = BOUNDS.x / 4.0;
    let vertical_margin = BOUNDS.y / 4.0;

    commands.spawn(Camera2d);

    for _ in 0..BOID_COUNT {
        commands.spawn((
            Sprite {
                image: boid_handle.clone(),
                ..default()
            },

            Transform::from_xyz(
                    rng.random_range(-BOUNDS.x/2. + horizontal_margin..BOUNDS.x/2. - horizontal_margin),
                    rng.random_range(-BOUNDS.y/2. + vertical_margin..BOUNDS.y/2. - vertical_margin),
                    0.)
                .with_scale(Vec3::splat(0.5))
                .with_rotation(Quat::from_rotation_z(rng.random_range(0.0..std::f32::consts::TAU))),

            Boid {
                movement_speed: Boid::MOVEMENT_SPEED * rng.random_range(0.9..=1.1),
                rotation_speed: Boid::ROTATION_SPEED * rng.random_range(0.9..=1.1),
            }
        ));
    }
}

fn collect_snapshot(query: &Query<(Entity, &Boid, &mut Transform)>,) -> Vec<(Entity, Vec3, Quat)> {

    // Entity ID, location, heading
    let mut boids: Vec<(Entity, Vec3, Quat)> = Vec::with_capacity(BOID_COUNT);

    for (entity, _boid, transform) in query.iter() {
        boids.push((entity, transform.translation, transform.rotation))
    }

    boids
}

fn find_neighbors(
    entity: Entity,
    position: Vec3,
    snapshot: &[(Entity, Vec3, Quat)],
 ) -> Vec<(Entity, Vec3, Quat)> {
    let mut neighbors = Vec::new();

    for (other_entity, other_position, other_rotation) in snapshot {
        if other_entity == &entity {
            continue;
        }

        let distance = other_position.distance(position);

        if distance < Boid::PERCEPTION_RADIUS {
            neighbors.push((*other_entity, *other_position, *other_rotation));
        }
    }

    neighbors
}

fn steer_alignment(current_rotation: Quat, neighbors: &[(Entity, Vec3,  Quat)]) -> f32 {
    let mut avg = Vec3::ZERO;

    for (_, _, other_rotation) in neighbors.iter() {
        avg += other_rotation * Vec3::Y;
    }

    let current = (current_rotation * Vec3::Y).truncate();
    let desired = avg.truncate();

    current.perp_dot(desired).atan2(current.dot(desired))
}

fn boid_movement_system(mut query: Query<(Entity, &Boid, &mut Transform)>) {
    let snapshot = collect_snapshot(&query);

    for (entity, boid, mut transform) in query.iter_mut() {
        let neighbors = find_neighbors(entity, transform.translation, &snapshot);

        // Only process neighbors if they exist
        if !neighbors.is_empty() {
            // Calculate forces
            let align_force = steer_alignment(transform.rotation, &neighbors);

            // Apply forces
            let forces = align_force; // + cohere_force; + avoid_force;
            let clamped = forces.clamp(
                -boid.rotation_speed * TIME_STEP,
                boid.rotation_speed * TIME_STEP,
            );
            transform.rotate_z(clamped)
        }

        // Move boid
        let movement_direction = transform.rotation * Vec3::Y;
        let movement_distance = boid.movement_speed * TIME_STEP;
        let translation_delta = movement_direction * movement_distance;

        transform.translation += translation_delta;

//        let extents = Vec3::from((BOUNDS / 2.0, 0.0));
//        transform.translation = transform.translation.min(extents).max(-extents);
    }
}
