use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "3D Model Loader Example".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_scene, load_models))
        .add_systems(Update, (rotate_models, handle_input))
        .run();
}

#[derive(Component)]
struct LoadedModel;

#[derive(Component)]
struct RotatingModel {
    speed: f32,
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-5.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
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
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.5, 0.3),
            perceptual_roughness: 0.8,
            ..default()
        })),
        Transform::from_xyz(0.0, -0.5, 0.0),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Torus::new(1.0, 0.3))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.3, 0.3),
            metallic: 0.8,
            perceptual_roughness: 0.2,
            ..default()
        })),
        Transform::from_xyz(0.0, 1.0, 0.0),
        RotatingModel { speed: 1.0 },
    ));
}

fn load_models(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneRoot(asset_server.load("models/character.glb#Scene0")),
        Transform::from_xyz(-3.0, 0.0, 0.0).with_scale(Vec3::splat(1.0)),
        LoadedModel,
        RotatingModel { speed: 0.5 },
    ));

    commands.spawn((
        SceneRoot(asset_server.load("models/environment.glb#Scene0")),
        Transform::from_xyz(3.0, 0.0, 0.0).with_scale(Vec3::splat(0.5)),
        LoadedModel,
    ));
}

fn rotate_models(time: Res<Time>, mut query: Query<(&mut Transform, &RotatingModel)>) {
    for (mut transform, rotating) in query.iter_mut() {
        transform.rotate_y(time.delta_secs() * rotating.speed);
    }
}

fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = camera.single_mut() {
        let speed = 5.0 * time.delta_secs();
        
        if keyboard.pressed(KeyCode::KeyW) {
            let forward = transform.forward();
            transform.translation += forward * speed;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            let forward = transform.forward();
            transform.translation -= forward * speed;
        }
        if keyboard.pressed(KeyCode::KeyA) {
            let left = transform.left();
            transform.translation += left * speed;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            let right = transform.right();
            transform.translation += right * speed;
        }
        if keyboard.pressed(KeyCode::KeyQ) {
            transform.translation.y -= speed;
        }
        if keyboard.pressed(KeyCode::KeyE) {
            transform.translation.y += speed;
        }
    }
}