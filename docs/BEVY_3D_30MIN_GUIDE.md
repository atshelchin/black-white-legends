# 30分钟上手Bevy 3D渲染引擎

## 第1部分：基础3D场景设置（5分钟）

### 1.1 添加Bevy 3D依赖

```toml
# Cargo.toml
[dependencies]
bevy = { version = "0.16.1", features = ["bevy_asset", "bevy_gltf", "bevy_pbr"] }
```

### 1.2 创建基础3D场景

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_3d_scene)
        .run();
}

fn setup_3d_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 添加3D相机
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    
    // 添加光源
    commands.spawn((
        DirectionalLight {
            illuminance: 5000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.8, -0.2, 0.0)),
    ));
    
    // 添加一个简单的立方体
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.7, 0.6),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    
    // 添加地面
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.5, 0.3),
            ..default()
        })),
    ));
}
```

## 第2部分：导入3D模型（10分钟）

### 2.1 准备3D模型文件

支持的格式：
- **GLTF/GLB**（推荐）：开放标准，支持材质和动画
- **OBJ**：简单但功能有限
- **FBX**：需要额外配置

将模型文件放在 `assets/models/` 目录下。

### 2.2 加载GLTF模型

```rust
use bevy::prelude::*;
use bevy::gltf::*;

#[derive(Component)]
struct Character;

fn load_3d_model(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // 加载3D角色模型
    commands.spawn((
        SceneRoot(asset_server.load("models/character.glb#Scene0")),
        Transform::from_xyz(0.0, 0.0, 0.0)
            .with_scale(Vec3::splat(1.0)),
        Character,
    ));
    
    // 加载3D场景
    commands.spawn((
        SceneRoot(asset_server.load("models/environment.glb#Scene0")),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

// 处理加载完成的模型
fn handle_loaded_models(
    mut commands: Commands,
    models: Query<(Entity, &SceneRoot), Added<SceneRoot>>,
    asset_server: Res<AssetServer>,
) {
    for (entity, scene) in models.iter() {
        if asset_server.is_loaded_with_dependencies(&scene.0) {
            println!("模型加载完成!");
            
            // 可以在这里添加额外的组件
            commands.entity(entity).insert(Name::new("LoadedModel"));
        }
    }
}
```

### 2.3 自定义材质

```rust
fn customize_materials(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<&mut MeshMaterial3d<StandardMaterial>>,
) {
    for mut material_handle in query.iter_mut() {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            // 修改材质属性
            material.metallic = 0.8;
            material.perceptual_roughness = 0.3;
            material.base_color = Color::srgb(0.9, 0.6, 0.3);
            
            // 添加纹理
            // material.base_color_texture = Some(asset_server.load("textures/diffuse.png"));
            // material.normal_map_texture = Some(asset_server.load("textures/normal.png"));
        }
    }
}
```

## 第3部分：3D动画系统（10分钟）

### 3.1 加载带动画的模型

```rust
use bevy::animation::*;

#[derive(Component)]
struct AnimationInfo {
    animations: Vec<AnimationNodeIndex>,
    graph: Handle<AnimationGraph>,
}

fn setup_animated_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    // 加载动画
    let mut graph = AnimationGraph::new();
    let animations = vec![
        graph.add_clip(
            asset_server.load("models/character.glb#Animation0"), // 空闲动画
            1.0,
            graph.root,
        ),
        graph.add_clip(
            asset_server.load("models/character.glb#Animation1"), // 行走动画
            1.0,
            graph.root,
        ),
        graph.add_clip(
            asset_server.load("models/character.glb#Animation2"), // 跑步动画
            1.0,
            graph.root,
        ),
    ];
    
    let graph_handle = graphs.add(graph);
    
    // 生成角色
    commands.spawn((
        SceneRoot(asset_server.load("models/character.glb#Scene0")),
        Transform::from_xyz(0.0, 0.0, 0.0),
        AnimationPlayer::default(),
        AnimationInfo {
            animations: animations.clone(),
            graph: graph_handle.clone(),
        },
        Character,
    ));
}
```

### 3.2 控制动画播放

```rust
fn control_animation(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut players: Query<(&mut AnimationPlayer, &AnimationInfo), With<Character>>,
) {
    for (mut player, info) in players.iter_mut() {
        // 根据输入切换动画
        if keyboard.pressed(KeyCode::KeyW) {
            // 播放跑步动画
            player.play(info.animations[2])
                .set_speed(1.5)
                .repeat();
        } else if keyboard.pressed(KeyCode::Space) {
            // 播放跳跃动画（如果有）
            player.play(info.animations[3])
                .set_speed(1.0);
        } else {
            // 播放空闲动画
            player.play(info.animations[0])
                .repeat();
        }
    }
}
```

### 3.3 程序化动画

```rust
fn animate_rotation(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Character>>,
) {
    for mut transform in query.iter_mut() {
        // 简单的旋转动画
        transform.rotate_y(time.delta_secs() * 1.0);
    }
}

fn animate_floating(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<FloatingObject>>,
) {
    for mut transform in query.iter_mut() {
        // 上下浮动
        transform.translation.y = (time.elapsed_secs() * 2.0).sin() * 0.5 + 1.0;
    }
}
```

## 第4部分：3D特效系统（5分钟）

### 4.1 粒子系统

```rust
#[derive(Component)]
struct ParticleEffect {
    lifetime: f32,
    velocity: Vec3,
}

fn spawn_particle_effect(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::KeyE) {
        // 生成粒子爆炸效果
        for i in 0..50 {
            let angle = (i as f32 / 50.0) * std::f32::consts::TAU;
            let velocity = Vec3::new(
                angle.cos() * 5.0,
                rand::random::<f32>() * 10.0,
                angle.sin() * 5.0,
            );
            
            commands.spawn((
                Mesh3d(meshes.add(Sphere::new(0.1))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgb(1.0, 0.5, 0.0),
                    emissive: LinearRgba::rgb(2.0, 1.0, 0.0),
                    ..default()
                })),
                Transform::from_xyz(0.0, 1.0, 0.0),
                ParticleEffect {
                    lifetime: 2.0,
                    velocity,
                },
            ));
        }
    }
}

fn update_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut particles: Query<(Entity, &mut Transform, &mut ParticleEffect)>,
) {
    for (entity, mut transform, mut particle) in particles.iter_mut() {
        // 更新位置
        transform.translation += particle.velocity * time.delta_secs();
        
        // 应用重力
        particle.velocity.y -= 9.8 * time.delta_secs();
        
        // 更新生命周期
        particle.lifetime -= time.delta_secs();
        
        // 删除过期粒子
        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
```

### 4.2 光效和后处理

```rust
fn create_light_effects(
    mut commands: Commands,
    time: Res<Time>,
    mut lights: Query<&mut PointLight>,
) {
    // 闪烁的光效
    for mut light in lights.iter_mut() {
        light.intensity = 1000.0 + (time.elapsed_secs() * 3.0).sin() * 500.0;
        light.color = Color::srgb(
            1.0,
            0.5 + (time.elapsed_secs() * 2.0).sin() * 0.5,
            0.3,
        );
    }
}

// 添加发光材质
fn create_emissive_material(
    mut materials: ResMut<Assets<StandardMaterial>>,
) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        base_color: Color::srgb(0.1, 0.1, 0.1),
        emissive: LinearRgba::rgb(0.0, 5.0, 0.0), // 绿色发光
        ..default()
    })
}
```

### 4.3 特效组合示例

```rust
// 火焰特效
fn create_fire_effect(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    // 火焰基座
    commands.spawn((
        Mesh3d(meshes.add(Cone {
            radius: 0.5,
            height: 1.5,
        })),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 0.3, 0.0, 0.8),
            emissive: LinearRgba::rgb(3.0, 1.0, 0.0),
            alpha_mode: AlphaMode::Blend,
            ..default()
        })),
        Transform::from_translation(position),
    ));
    
    // 火焰光源
    commands.spawn((
        PointLight {
            color: Color::srgb(1.0, 0.5, 0.0),
            intensity: 2000.0,
            radius: 0.5,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_translation(position + Vec3::Y * 0.5),
    ));
}
```

## 完整示例：整合所有功能

```rust
use bevy::prelude::*;
use bevy::animation::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (
            setup_3d_scene,
            load_character_with_animation,
        ))
        .add_systems(Update, (
            control_animation,
            animate_floating,
            update_particles,
            spawn_particle_on_input,
        ))
        .run();
}

#[derive(Component)]
struct Character;

#[derive(Component)]
struct FloatingObject;

#[derive(Component)]
struct ParticleEffect {
    lifetime: f32,
    velocity: Vec3,
}

fn setup_3d_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // 相机
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-5.0, 5.0, 10.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));
    
    // 环境光
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 200.0,
    });
    
    // 主光源
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
    
    // 地面
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.5, 0.3),
            ..default()
        })),
    ));
    
    // 浮动的立方体
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.3, 0.3),
            emissive: LinearRgba::rgb(0.5, 0.0, 0.0),
            ..default()
        })),
        Transform::from_xyz(3.0, 1.0, 0.0),
        FloatingObject,
    ));
}

fn load_character_with_animation(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // 加载3D角色模型
    commands.spawn((
        SceneRoot(asset_server.load("models/character.glb#Scene0")),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Character,
        AnimationPlayer::default(),
    ));
}

fn control_animation(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut characters: Query<&mut Transform, With<Character>>,
) {
    for mut transform in characters.iter_mut() {
        // 简单的键盘控制
        if keyboard.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= 0.1;
        }
        if keyboard.pressed(KeyCode::ArrowRight) {
            transform.translation.x += 0.1;
        }
        if keyboard.pressed(KeyCode::ArrowUp) {
            transform.translation.z -= 0.1;
        }
        if keyboard.pressed(KeyCode::ArrowDown) {
            transform.translation.z += 0.1;
        }
    }
}

fn animate_floating(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<FloatingObject>>,
) {
    for mut transform in query.iter_mut() {
        transform.translation.y = (time.elapsed_secs() * 2.0).sin() * 0.5 + 2.0;
        transform.rotate_y(time.delta_secs() * 0.5);
    }
}

fn spawn_particle_on_input(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        // 生成粒子效果
        for _ in 0..20 {
            let velocity = Vec3::new(
                rand::random::<f32>() * 4.0 - 2.0,
                rand::random::<f32>() * 8.0,
                rand::random::<f32>() * 4.0 - 2.0,
            );
            
            commands.spawn((
                Mesh3d(meshes.add(Sphere::new(0.1))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgb(1.0, 0.8, 0.0),
                    emissive: LinearRgba::rgb(2.0, 1.6, 0.0),
                    ..default()
                })),
                Transform::from_xyz(0.0, 0.5, 0.0),
                ParticleEffect {
                    lifetime: 1.5,
                    velocity,
                },
            ));
        }
    }
}

fn update_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut particles: Query<(Entity, &mut Transform, &mut ParticleEffect)>,
) {
    for (entity, mut transform, mut particle) in particles.iter_mut() {
        transform.translation += particle.velocity * time.delta_secs();
        particle.velocity.y -= 9.8 * time.delta_secs();
        particle.lifetime -= time.delta_secs();
        
        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
```

## 实践建议

### 性能优化
1. **LOD（细节层次）**：远处使用低模，近处使用高模
2. **视锥剔除**：Bevy自动处理，确保相机设置正确
3. **批处理**：使用相同材质的网格会自动批处理

### 资源管理
1. **预加载**：在加载界面预加载所有资源
2. **资源热重载**：开发时Bevy支持资源热重载
3. **纹理压缩**：使用压缩的纹理格式（DDS、KTX2）

### 调试技巧
1. 使用 `bevy_inspector_egui` 实时调整参数
2. 启用 `bevy::diagnostic::FrameTimeDiagnosticsPlugin` 监控性能
3. 使用 `bevy_mod_debugdump` 可视化ECS结构

## 下一步学习

1. **高级渲染**：自定义着色器、后处理效果
2. **物理系统**：集成 `bevy_rapier3d` 或 `bevy_xpbd`
3. **UI系统**：3D UI、HUD显示
4. **网络同步**：多人游戏支持

## 常见问题

**Q: 模型加载失败？**
A: 检查文件路径和格式，确保在 `assets/` 目录下

**Q: 动画不播放？**
A: 确保模型包含动画数据，使用正确的动画索引

**Q: 性能问题？**
A: 减少粒子数量，使用LOD，优化着色器

通过这30分钟的学习，你已经掌握了Bevy 3D渲染的核心概念！