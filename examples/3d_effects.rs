use bevy::prelude::*;
use rand::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "3D Effects Example".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (
            spawn_particles,
            update_particles,
            animate_lights,
            create_explosions,
            update_trail_effect,
        ))
        .run();
}

#[derive(Component)]
struct ParticleEffect {
    velocity: Vec3,
    lifetime: f32,
    max_lifetime: f32,
    color_start: Color,
    color_end: Color,
    size_start: f32,
    size_end: f32,
}

#[derive(Component)]
struct ExplosionTrigger;

#[derive(Component)]
struct AnimatedLight {
    base_intensity: f32,
    flicker_speed: f32,
    color_cycle: bool,
}

#[derive(Component)]
struct TrailEffect {
    positions: Vec<Vec3>,
    max_positions: usize,
}

#[derive(Component)]
struct FireEffect;

#[derive(Component)]
struct SmokeEffect;

#[derive(Component)]
struct SparkEffect;

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-10.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 100.0,
        affects_lightmapped_meshes: true,
    });

    commands.spawn((
        DirectionalLight {
            illuminance: 5000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -std::f32::consts::PI / 4.0,
            -std::f32::consts::PI / 6.0,
            0.0,
        )),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.2, 0.2),
            perceptual_roughness: 0.9,
            ..default()
        })),
    ));

    spawn_fire_effect(&mut commands, &mut meshes, &mut materials, Vec3::new(-5.0, 0.0, 0.0));
    
    commands.spawn((
        PointLight {
            color: Color::srgb(1.0, 0.5, 0.0),
            intensity: 3000.0,
            radius: 1.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(5.0, 2.0, 0.0),
        AnimatedLight {
            base_intensity: 3000.0,
            flicker_speed: 5.0,
            color_cycle: true,
        },
    ));

    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.2),
            emissive: LinearRgba::rgb(2.0, 0.0, 0.0),
            ..default()
        })),
        Transform::from_xyz(0.0, 3.0, 0.0),
        ExplosionTrigger,
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.2, 0.8),
            metallic: 0.9,
            perceptual_roughness: 0.1,
            ..default()
        })),
        Transform::from_xyz(0.0, 1.0, 5.0),
        TrailEffect {
            positions: Vec::new(),
            max_positions: 20,
        },
    ));
}

fn spawn_fire_effect(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cone {
            radius: 0.5,
            height: 2.0,
        })),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 0.3, 0.0, 0.6),
            emissive: LinearRgba::rgb(5.0, 2.0, 0.0),
            alpha_mode: AlphaMode::Blend,
            ..default()
        })),
        Transform::from_translation(position + Vec3::Y * 1.0)
            .with_scale(Vec3::new(1.0, 2.0, 1.0)),
        FireEffect,
    ));

    commands.spawn((
        PointLight {
            color: Color::srgb(1.0, 0.4, 0.0),
            intensity: 2000.0,
            radius: 0.5,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_translation(position + Vec3::Y * 1.0),
        AnimatedLight {
            base_intensity: 2000.0,
            flicker_speed: 10.0,
            color_cycle: false,
        },
    ));
}

fn spawn_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    fire_effects: Query<&Transform, With<FireEffect>>,
    time: Res<Time>,
) {
    for fire_transform in fire_effects.iter() {
        if rand::random::<f32>() < 0.1 {
            let mut rng = rand::thread_rng();
            let offset = Vec3::new(
                rng.gen_range(-0.3..0.3),
                0.0,
                rng.gen_range(-0.3..0.3),
            );
            
            commands.spawn((
                Mesh3d(meshes.add(Sphere::new(rng.gen_range(0.05..0.15)))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgba(1.0, 0.8, 0.0, 0.8),
                    emissive: LinearRgba::rgb(3.0, 2.0, 0.0),
                    alpha_mode: AlphaMode::Blend,
                    ..default()
                })),
                Transform::from_translation(fire_transform.translation + offset),
                ParticleEffect {
                    velocity: Vec3::new(
                        rng.gen_range(-1.0..1.0),
                        rng.gen_range(2.0..5.0),
                        rng.gen_range(-1.0..1.0),
                    ),
                    lifetime: rng.gen_range(1.0..2.0),
                    max_lifetime: 2.0,
                    color_start: Color::srgb(1.0, 0.8, 0.0),
                    color_end: Color::srgba(0.3, 0.3, 0.3, 0.0),
                    size_start: 0.2,
                    size_end: 0.05,
                },
            ));
        }
    }

    if keyboard.just_pressed(KeyCode::Space) {
        let mut rng = rand::thread_rng();
        for _ in 0..30 {
            commands.spawn((
                Mesh3d(meshes.add(Sphere::new(0.1))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgb(0.0, 0.5, 1.0),
                    emissive: LinearRgba::rgb(0.0, 2.0, 4.0),
                    ..default()
                })),
                Transform::from_xyz(0.0, 5.0, 0.0),
                ParticleEffect {
                    velocity: Vec3::new(
                        rng.gen_range(-5.0..5.0),
                        rng.gen_range(5.0..10.0),
                        rng.gen_range(-5.0..5.0),
                    ),
                    lifetime: 2.0,
                    max_lifetime: 2.0,
                    color_start: Color::srgb(0.0, 0.5, 1.0),
                    color_end: Color::srgba(0.0, 0.0, 0.5, 0.0),
                    size_start: 0.2,
                    size_end: 0.01,
                },
                SparkEffect,
            ));
        }
    }
}

fn update_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut particles: Query<(
        Entity,
        &mut Transform,
        &mut ParticleEffect,
        &MeshMaterial3d<StandardMaterial>,
    )>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, mut transform, mut particle, mesh_material) in particles.iter_mut() {
        particle.lifetime -= time.delta_secs();
        
        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }
        
        transform.translation += particle.velocity * time.delta_secs();
        particle.velocity.y -= 9.8 * time.delta_secs() * 0.5;
        
        let t = 1.0 - (particle.lifetime / particle.max_lifetime);
        
        let current_size = particle.size_start + (particle.size_end - particle.size_start) * t;
        transform.scale = Vec3::splat(current_size);
        
        if let Some(material) = materials.get_mut(&mesh_material.0) {
            let color = particle.color_start.mix(&particle.color_end, t);
            material.base_color = color;
            
            let alpha = 1.0 - t;
            material.base_color.set_alpha(alpha);
        }
    }
}

fn animate_lights(
    time: Res<Time>,
    mut lights: Query<(&mut PointLight, &AnimatedLight)>,
) {
    for (mut light, animated) in lights.iter_mut() {
        let flicker = (time.elapsed_secs() * animated.flicker_speed).sin() * 0.3 + 0.7;
        light.intensity = animated.base_intensity * flicker;
        
        if animated.color_cycle {
            let hue = (time.elapsed_secs() * 0.5) % 1.0;
            light.color = Color::hsla(hue * 360.0, 1.0, 0.5, 1.0);
        }
    }
}

fn create_explosions(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    explosion_triggers: Query<&Transform, With<ExplosionTrigger>>,
) {
    if keyboard.just_pressed(KeyCode::KeyE) {
        for trigger_transform in explosion_triggers.iter() {
            let mut rng = rand::thread_rng();
            
            for _ in 0..50 {
                let direction = Vec3::new(
                    rng.gen_range(-1.0..1.0),
                    rng.gen_range(-1.0..1.0),
                    rng.gen_range(-1.0..1.0),
                ).normalize();
                
                let speed = rng.gen_range(5.0..15.0);
                let size = rng.gen_range(0.05..0.2);
                
                commands.spawn((
                    Mesh3d(meshes.add(Sphere::new(size))),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: Color::srgb(1.0, rng.gen_range(0.3..0.7), 0.0),
                        emissive: LinearRgba::rgb(
                            5.0,
                            rng.gen_range(1.0..3.0),
                            0.0
                        ),
                        ..default()
                    })),
                    Transform::from_translation(trigger_transform.translation),
                    ParticleEffect {
                        velocity: direction * speed,
                        lifetime: rng.gen_range(0.5..1.5),
                        max_lifetime: 1.5,
                        color_start: Color::srgb(1.0, 0.5, 0.0),
                        color_end: Color::srgba(0.5, 0.0, 0.0, 0.0),
                        size_start: size,
                        size_end: size * 0.1,
                    },
                ));
            }
            
            commands.spawn((
                PointLight {
                    color: Color::srgb(1.0, 0.5, 0.0),
                    intensity: 10000.0,
                    radius: 2.0,
                    shadows_enabled: false,
                    ..default()
                },
                Transform::from_translation(trigger_transform.translation),
                ParticleEffect {
                    velocity: Vec3::ZERO,
                    lifetime: 0.2,
                    max_lifetime: 0.2,
                    color_start: Color::WHITE,
                    color_end: Color::BLACK,
                    size_start: 1.0,
                    size_end: 0.0,
                },
            ));
        }
    }
}

fn update_trail_effect(
    time: Res<Time>,
    mut trail_objects: Query<(&mut Transform, &mut TrailEffect)>,
    mut gizmos: Gizmos,
) {
    for (mut transform, mut trail) in trail_objects.iter_mut() {
        let movement = Vec3::new(
            (time.elapsed_secs() * 2.0).cos() * 5.0,
            (time.elapsed_secs() * 3.0).sin() * 2.0 + 3.0,
            (time.elapsed_secs() * 1.5).sin() * 5.0,
        );
        transform.translation = movement;
        
        trail.positions.push(transform.translation);
        if trail.positions.len() > trail.max_positions {
            trail.positions.remove(0);
        }
        
        for i in 1..trail.positions.len() {
            let alpha = i as f32 / trail.positions.len() as f32;
            let color = Color::srgba(0.0, 0.5, 1.0, alpha);
            
            gizmos.line(
                trail.positions[i - 1],
                trail.positions[i],
                color,
            );
        }
        
        transform.rotate_local_x(time.delta_secs() * 2.0);
        transform.rotate_local_y(time.delta_secs() * 1.5);
    }
}