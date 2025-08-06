use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "3D Animation Example".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_scene, setup_animated_character))
        .add_systems(Update, (
            control_animation,
            procedural_animation,
            blend_animations,
        ))
        .run();
}

#[derive(Component)]
struct AnimatedCharacter {
    idle_animation: AnimationNodeIndex,
    walk_animation: AnimationNodeIndex,
    run_animation: AnimationNodeIndex,
    jump_animation: AnimationNodeIndex,
    graph: Handle<AnimationGraph>,
}

#[derive(Component)]
struct ProcedurallyAnimated {
    frequency: f32,
    amplitude: f32,
}

#[derive(Component, Default)]
struct CharacterState {
    is_moving: bool,
    is_running: bool,
    is_jumping: bool,
    blend_weight: f32,
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-10.0, 8.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 200.0,
        affects_lightmapped_meshes: true,
    });

    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
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
        Mesh3d(meshes.add(Plane3d::default().mesh().size(30.0, 30.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.5, 0.3),
            ..default()
        })),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.3, 0.3),
            metallic: 0.8,
            ..default()
        })),
        Transform::from_xyz(3.0, 1.0, 0.0),
        ProcedurallyAnimated {
            frequency: 2.0,
            amplitude: 2.0,
        },
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 2.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.3, 0.8),
            ..default()
        })),
        Transform::from_xyz(-3.0, 1.0, 0.0),
        ProcedurallyAnimated {
            frequency: 1.0,
            amplitude: 0.5,
        },
    ));
}

fn setup_animated_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let mut graph = AnimationGraph::new();
    
    let idle_animation = graph.add_clip(
        asset_server.load("models/character.glb#Animation0"),
        1.0,
        graph.root,
    );
    
    let walk_animation = graph.add_clip(
        asset_server.load("models/character.glb#Animation1"),
        1.0,
        graph.root,
    );
    
    let run_animation = graph.add_clip(
        asset_server.load("models/character.glb#Animation2"),
        1.0,
        graph.root,
    );
    
    let jump_animation = graph.add_clip(
        asset_server.load("models/character.glb#Animation3"),
        1.0,
        graph.root,
    );
    
    let graph_handle = graphs.add(graph);
    
    commands.spawn((
        SceneRoot(asset_server.load("models/character.glb#Scene0")),
        Transform::from_xyz(0.0, 0.0, 0.0),
        AnimationPlayer::default(),
        AnimatedCharacter {
            idle_animation,
            walk_animation,
            run_animation,
            jump_animation,
            graph: graph_handle,
        },
        CharacterState::default(),
    ));
}

fn control_animation(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut characters: Query<(
        &mut AnimationPlayer,
        &AnimatedCharacter,
        &mut CharacterState,
        &mut Transform,
    )>,
    time: Res<Time>,
) {
    for (mut player, character, mut state, mut transform) in characters.iter_mut() {
        state.is_moving = false;
        state.is_running = false;
        state.is_jumping = false;
        
        let mut movement = Vec3::ZERO;
        
        if keyboard.pressed(KeyCode::KeyW) {
            movement.z -= 1.0;
            state.is_moving = true;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            movement.z += 1.0;
            state.is_moving = true;
        }
        if keyboard.pressed(KeyCode::KeyA) {
            movement.x -= 1.0;
            state.is_moving = true;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            movement.x += 1.0;
            state.is_moving = true;
        }
        
        if keyboard.pressed(KeyCode::ShiftLeft) && state.is_moving {
            state.is_running = true;
        }
        
        if keyboard.just_pressed(KeyCode::Space) {
            state.is_jumping = true;
        }
        
        if movement.length() > 0.0 {
            movement = movement.normalize();
            let speed = if state.is_running { 8.0 } else { 4.0 };
            transform.translation += movement * speed * time.delta_secs();
            
            let target_rotation = Quat::from_rotation_y(
                (-movement.z).atan2(-movement.x) + std::f32::consts::PI / 2.0
            );
            transform.rotation = transform.rotation.slerp(target_rotation, 10.0 * time.delta_secs());
        }
        
        if state.is_jumping {
            player.play(character.jump_animation)
                .set_speed(1.5);
        } else if state.is_running {
            player.play(character.run_animation)
                .set_speed(1.2)
                .repeat();
        } else if state.is_moving {
            player.play(character.walk_animation)
                .set_speed(1.0)
                .repeat();
        } else {
            player.play(character.idle_animation)
                .set_speed(1.0)
                .repeat();
        }
    }
}

fn procedural_animation(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &ProcedurallyAnimated)>,
) {
    for (mut transform, animated) in query.iter_mut() {
        let elapsed = time.elapsed_secs();
        
        transform.translation.y = 
            1.0 + (elapsed * animated.frequency).sin() * animated.amplitude;
        
        transform.rotation = Quat::from_euler(
            EulerRot::XYZ,
            (elapsed * 0.5).sin() * 0.2,
            elapsed,
            (elapsed * 0.7).cos() * 0.2,
        );
        
        let scale_factor = 1.0 + (elapsed * 3.0).sin() * 0.1;
        transform.scale = Vec3::splat(scale_factor);
    }
}

fn blend_animations(
    mut characters: Query<&mut CharacterState>,
    time: Res<Time>,
) {
    for mut state in characters.iter_mut() {
        let target_weight = if state.is_moving { 1.0 } else { 0.0 };
        state.blend_weight = state.blend_weight.lerp(target_weight, 5.0 * time.delta_secs());
        
        // Note: In Bevy 0.16, animation blending API may differ
        // This is a placeholder for proper animation blending
    }
}

trait Lerp {
    fn lerp(&self, target: &Self, t: f32) -> Self;
}

impl Lerp for f32 {
    fn lerp(&self, target: &f32, t: f32) -> f32 {
        self + (target - self) * t.clamp(0.0, 1.0)
    }
}