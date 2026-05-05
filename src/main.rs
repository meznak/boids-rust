use bevy::{
    prelude::*,
    window::close_when_requested
};
use rand::prelude::*;

const TIME_STEP: f32 = 1.0 / 60.0;
const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);
const BOID_COUNT: usize = 400;
const BORDER_MARGIN: f32 = 50.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, close_when_requested)
        .add_systems(Startup, setup)
        .add_systems(Update, (text_update_system, input_system))
        .add_systems(FixedUpdate, boid_movement_system)
        .insert_resource(SimParams {
            perception_radius: 75.0,
            separation_radius: 25.0,
            movement_speed: 50.0,
            rotation_speed: f32::to_radians(135.0),
            align_weight: 1.0,
            cohere_weight: 1.0,
            avoid_weight: 1.0,
            border_weight: 3.0,
        })
        .run();
}

#[derive(Resource)]
struct SimParams {
    perception_radius: f32,
    separation_radius: f32,
    movement_speed: f32,
    rotation_speed: f32,
    align_weight: f32,
    cohere_weight: f32,
    avoid_weight: f32,
    border_weight: f32,
}

#[derive(Component)]
struct Boid {
    // linear speed in meters per second
    movement_speed: f32,
    // rotation speed in radians per second
    rotation_speed: f32,
}

// UI elements
#[derive(Component)]
struct ParamDisplay;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, params: Res<SimParams>) {
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
                .with_scale(Vec3::splat(0.2))
                .with_rotation(Quat::from_rotation_z(rng.random_range(0.0..std::f32::consts::TAU))),

            Boid {
                movement_speed: params.movement_speed * rng.random_range(0.9..=1.1),
                rotation_speed: params.rotation_speed * rng.random_range(0.9..=1.1),
            }
        ));
    }

    commands.spawn((
        Text::new(""),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        TextFont {
            font_size: 10.0,
            ..default()
        },
        Visibility::Visible,
        ParamDisplay,
    ));
}

fn text_update_system(
    mut query: Query<&mut Text, With<ParamDisplay>>,
    params: Res<SimParams>,
) {
    let perception_radius = params.perception_radius;
    let separation_radius = params.separation_radius;
    let align_weight = params.align_weight;
    let cohere_weight = params.cohere_weight;
    let avoid_weight = params.avoid_weight;
    let border_weight = params.border_weight;

    if let Ok(mut text) = query.single_mut() {
        **text = format!("
        Perception: {perception_radius:.2}
        Separation: {separation_radius:.2}
        Alignment:  {align_weight:.2}
        Cohesion:   {cohere_weight:.2}
        Avoidance:  {avoid_weight:.2}
        Border:     {border_weight:.2}
        ");
    }
}

fn input_system(
    input: Res<ButtonInput<KeyCode>>,
    mut params: ResMut<SimParams>,
) {
    let shift = input.pressed(KeyCode::ShiftLeft) || input.pressed(KeyCode::ShiftRight);

    if input.just_pressed(KeyCode::KeyQ) {
        if shift {
            params.perception_radius += 50.0;
        } else {
            params.perception_radius += 10.0;
        }
    }

    if input.just_pressed(KeyCode::KeyA) {
        if shift {
            params.perception_radius -= 50.0;
        } else {
            params.perception_radius -= 10.0;
        }

        params.perception_radius = params.perception_radius.max(params.separation_radius);
    }

    if input.just_pressed(KeyCode::KeyW) {
        if shift {
            params.separation_radius += 50.0;
        } else {
            params.separation_radius += 10.0;
        }

        params.separation_radius = params.separation_radius.min(params.perception_radius);
    }

    if input.just_pressed(KeyCode::KeyS) {
        if shift {
            params.separation_radius -= 50.0;
        } else {
            params.separation_radius -= 10.0;
        }

        params.separation_radius = params.separation_radius.max(0.0);
    }

    if input.just_pressed(KeyCode::KeyE) {
        if shift {
            params.align_weight += 1.0;
        } else {
            params.align_weight += 0.1;
        }
    }

    if input.just_pressed(KeyCode::KeyD) {
        if shift {
            params.align_weight -= 1.0;
        } else {
            params.align_weight -= 0.1;
        }

        params.align_weight = params.align_weight.max(0.0);
    }

    if input.just_pressed(KeyCode::KeyR) {
        if shift {
            params.cohere_weight += 1.0;
        } else {
            params.cohere_weight += 0.1;
        }
    }

    if input.just_pressed(KeyCode::KeyF) {
        if shift {
            params.cohere_weight -= 1.0;
        } else {
            params.cohere_weight -= 0.1;
        }

        params.cohere_weight = params.cohere_weight.max(0.0);
    }

    if input.just_pressed(KeyCode::KeyT) {
        if shift {
            params.avoid_weight += 1.0;
        } else {
            params.avoid_weight += 0.1;
        }
    }

    if input.just_pressed(KeyCode::KeyG) {
        if shift {
            params.avoid_weight -= 1.0;
        } else {
            params.avoid_weight -= 0.1;
        }

        params.avoid_weight = params.avoid_weight.max(0.0);
    }

    if input.just_pressed(KeyCode::KeyY) {
        if shift {
            params.border_weight += 1.0;
        } else {
            params.border_weight += 0.1;
        }
    }

    if input.just_pressed(KeyCode::KeyH) {
        if shift {
            params.border_weight -= 1.0;
        } else {
            params.border_weight -= 0.1;
        }

        params.border_weight = params.border_weight.max(0.0);
    }
}

fn collect_snapshot(query: &Query<(Entity, &Boid, &mut Transform)>) -> Vec<(Entity, Vec3, Quat)> {

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
    perception_radius: f32
 ) -> Vec<(Entity, Vec3, Quat)> {
    let mut neighbors = Vec::new();

    for (other_entity, other_position, other_rotation) in snapshot {
        if other_entity == &entity {
            continue;
        }

        let distance = other_position.distance(position);

        if distance < perception_radius {
            neighbors.push((*other_entity, *other_position, *other_rotation));
        }
    }

    neighbors
}

fn steer_alignment(current_rotation: Quat, neighbors: &[(Entity, Vec3, Quat)]) -> f32 {
    let mut avg = Vec3::ZERO;

    for (_, _, other_rotation) in neighbors.iter() {
        avg += other_rotation * Vec3::Y;
    }

    let current = (current_rotation * Vec3::Y).truncate();
    let desired = avg.truncate();

    current.perp_dot(desired).atan2(current.dot(desired))
}

fn steer_cohesion(current_rotation: Quat, current_position: Vec3, neighbors: &[(Entity, Vec3, Quat)]) -> f32 {
    let mut avg: Vec3 = Vec3::ZERO;

    for (_, other_position, _) in neighbors.iter() {
        avg += other_position;
    }

    avg /= neighbors.len() as f32;

    let toward_centroid = avg - current_position;
    let current = (current_rotation * Vec3::Y).truncate();
    let desired = toward_centroid.truncate();

    current.perp_dot(desired).atan2(current.dot(desired))
}
fn steer_avoidance(current_rotation: Quat, current_position: Vec3, neighbors: &[(Entity, Vec3, Quat)], separation_radius: f32) -> f32 {
    let mut force: Vec3 = Vec3::ZERO;

    for (_, other_position, _) in neighbors.iter() {
        let distance: f32 = other_position.distance(current_position);

        if distance < separation_radius {
            let away = current_position - other_position;
            force += away / distance.powf(2.);
        }
    }

    let current = (current_rotation * Vec3::Y).truncate();
    let desired = force.truncate();

    current.perp_dot(desired).atan2(current.dot(desired))
}

fn steer_border(current_position: Vec3, current_rotation: Quat) -> f32 {
    let mut push_x: f32 = 0.;
    let mut push_y: f32 = 0.;
    let border_x = BOUNDS.x/2. - BORDER_MARGIN;
    let border_y = BOUNDS.y/2. - BORDER_MARGIN;

    if current_position.x > border_x {
        push_x = -(current_position.x - border_x)
    }
    else if current_position.x < -border_x {
        push_x = -(current_position.x + border_x);
    }

    if current_position.y > border_y {
        push_y = -(current_position.y - border_y)
    }
    else if current_position.y < -border_y {
        push_y = -(current_position.y + border_y);
    }

    let current = (current_rotation * Vec3::Y).truncate();
    let desired: Vec2 = [push_x, push_y].into();
    let desired_norm = desired.normalize_or_zero();

    if desired_norm == Vec2::ZERO { return 0.0; }

    current.perp_dot(desired_norm).atan2(current.dot(desired_norm))

}

fn boid_movement_system(mut query: Query<(Entity, &Boid, &mut Transform)>, params: Res<SimParams>) {
    let snapshot = collect_snapshot(&query);

    for (entity, boid, mut transform) in query.iter_mut() {
        let neighbors = find_neighbors(entity, transform.translation, &snapshot, params.perception_radius);
        let mut align_force: f32 = 0.;
        let mut cohere_force: f32 = 0.;
        let mut avoid_force: f32 = 0.;

        // Only process neighbors if they exist
        if !neighbors.is_empty() {
            // Calculate forces
            align_force = steer_alignment(transform.rotation, &neighbors);
            cohere_force = steer_cohesion(transform.rotation, transform.translation, &neighbors);
            avoid_force = steer_avoidance(transform.rotation, transform.translation, &neighbors, params.separation_radius);
        }

        // Always avoid borders
        let border_force = steer_border(transform.translation, transform.rotation);

        // Apply forces
        let forces = align_force * params.align_weight
                        + cohere_force * params.cohere_weight
                        + avoid_force * params.avoid_weight
                        + border_force * params.border_weight;
        let clamped = forces.clamp(
            -boid.rotation_speed * TIME_STEP,
            boid.rotation_speed * TIME_STEP,
        );
        transform.rotate_z(clamped);


        // Move boid
        let movement_direction = transform.rotation * Vec3::Y;
        let movement_distance = boid.movement_speed * TIME_STEP;
        let translation_delta = movement_direction * movement_distance;

        transform.translation += translation_delta;

    }
}
