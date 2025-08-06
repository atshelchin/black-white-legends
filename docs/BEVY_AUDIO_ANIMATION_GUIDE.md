# Bevy 音效与动画系统指南

## 一、音效系统 (Audio)

### 基础音频播放

```rust
use bevy::prelude::*;
use bevy::audio::{Volume, PlaybackSettings};

// === 1. 简单播放 ===
fn setup_audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // 播放背景音乐（循环）
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/background_music.ogg"),
        settings: PlaybackSettings::LOOP.with_volume(Volume::new(0.5)),
    });
    
    // 播放一次性音效
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/click.ogg"),
        settings: PlaybackSettings::ONCE,
    });
}
```

### 音频资源管理

```rust
// === 预加载音频资源 ===
#[derive(Resource)]
struct AudioAssets {
    stone_place: Handle<AudioSource>,
    stone_capture: Handle<AudioSource>,
    timer_tick: Handle<AudioSource>,
    victory: Handle<AudioSource>,
}

fn load_audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(AudioAssets {
        stone_place: asset_server.load("sounds/stone_place.ogg"),
        stone_capture: asset_server.load("sounds/capture.ogg"),
        timer_tick: asset_server.load("sounds/tick.ogg"),
        victory: asset_server.load("sounds/victory.ogg"),
    });
}
```

### 动态音效控制

```rust
// === 带控制的音频实体 ===
#[derive(Component)]
struct BackgroundMusic;

#[derive(Component)]
struct SoundEffect;

fn spawn_controlled_audio(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
) {
    // 可控制的背景音乐
    commands.spawn((
        AudioBundle {
            source: audio_assets.background.clone(),
            settings: PlaybackSettings::LOOP.with_volume(Volume::new(0.3)),
        },
        BackgroundMusic,
    ));
}

// 动态调整音量
fn adjust_volume(
    mut music_query: Query<&mut AudioSink, With<BackgroundMusic>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if let Ok(sink) = music_query.get_single_mut() {
        if keyboard.pressed(KeyCode::ArrowUp) {
            sink.set_volume(sink.volume() + 0.1);
        }
        if keyboard.pressed(KeyCode::ArrowDown) {
            sink.set_volume(sink.volume() - 0.1);
        }
        if keyboard.just_pressed(KeyCode::Space) {
            sink.toggle(); // 暂停/继续
        }
    }
}
```

### 空间音频（3D音效）

```rust
fn setup_spatial_audio(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
) {
    // 创建音频监听器（通常附加到摄像机）
    commands.spawn((
        SpatialBundle::default(),
        SpatialListener::new(10.0), // 10单位距离衰减
    ));
    
    // 创建3D音源
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(5.0, 0.0, 0.0),
            ..default()
        },
        AudioBundle {
            source: audio_assets.ambient_sound.clone(),
            settings: PlaybackSettings::LOOP,
        },
    ));
}
```

### 事件驱动音效

```rust
// 围棋游戏音效系统
fn play_stone_sound(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    mut stone_placed_events: EventReader<StonePlacedEvent>,
) {
    for event in stone_placed_events.read() {
        // 播放落子音效
        commands.spawn(AudioBundle {
            source: audio_assets.stone_place.clone(),
            settings: PlaybackSettings::ONCE
                .with_volume(Volume::new(0.7))
                .with_speed(0.9 + rand::random::<f32>() * 0.2), // 随机音调
        });
    }
}
```

## 二、动画系统

### 1. Transform 动画（位置/旋转/缩放）

```rust
// === 简单的Transform动画 ===
#[derive(Component)]
struct AnimatePosition {
    start: Vec3,
    end: Vec3,
    duration: f32,
    elapsed: f32,
}

fn animate_position_system(
    mut query: Query<(&mut Transform, &mut AnimatePosition)>,
    time: Res<Time>,
) {
    for (mut transform, mut anim) in query.iter_mut() {
        anim.elapsed += time.delta_seconds();
        let t = (anim.elapsed / anim.duration).min(1.0);
        
        // 使用缓动函数
        let eased_t = ease_in_out_cubic(t);
        transform.translation = anim.start.lerp(anim.end, eased_t);
        
        if t >= 1.0 {
            // 动画完成，可以移除组件或触发事件
        }
    }
}

fn ease_in_out_cubic(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}
```

### 2. Sprite 序列帧动画

```rust
#[derive(Component)]
struct SpriteAnimation {
    frames: Vec<Handle<Image>>,
    current_frame: usize,
    frame_time: f32,
    elapsed: f32,
}

fn animate_sprites(
    mut query: Query<(&mut Handle<Image>, &mut SpriteAnimation)>,
    time: Res<Time>,
) {
    for (mut texture, mut animation) in query.iter_mut() {
        animation.elapsed += time.delta_seconds();
        
        if animation.elapsed >= animation.frame_time {
            animation.elapsed = 0.0;
            animation.current_frame = (animation.current_frame + 1) % animation.frames.len();
            *texture = animation.frames[animation.current_frame].clone();
        }
    }
}
```

### 3. 材质动画

```rust
#[derive(Component)]
struct PulseMaterial {
    base_color: Color,
    pulse_speed: f32,
}

fn pulse_material_system(
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(&Handle<StandardMaterial>, &PulseMaterial)>,
    time: Res<Time>,
) {
    for (material_handle, pulse) in query.iter() {
        if let Some(material) = materials.get_mut(material_handle) {
            let intensity = (time.elapsed_seconds() * pulse.pulse_speed).sin() * 0.5 + 0.5;
            material.base_color = pulse.base_color.with_alpha(intensity);
            material.emissive = pulse.base_color * intensity;
        }
    }
}
```

### 4. 使用 bevy_tweening 插件（推荐）

```toml
# Cargo.toml
[dependencies]
bevy_tweening = "0.11"
```

```rust
use bevy_tweening::{*, lens::*};

fn spawn_with_tween(mut commands: Commands) {
    let tween = Tween::new(
        EaseFunction::QuadraticInOut,
        Duration::from_secs(2),
        TransformPositionLens {
            start: Vec3::ZERO,
            end: Vec3::new(10.0, 0.0, 0.0),
        },
    )
    .with_repeat_count(RepeatCount::Infinite)
    .with_repeat_strategy(RepeatStrategy::MirroredRepeat);
    
    commands.spawn((
        SpriteBundle::default(),
        Animator::new(tween),
    ));
}
```

## 三、围棋游戏中的应用示例

### 落子动画与音效

```rust
use bevy::prelude::*;

// === 组件定义 ===
#[derive(Component)]
struct Stone {
    color: StoneColor,
    grid_pos: IVec2,
}

#[derive(Component)]
struct FallingStone {
    target_y: f32,
    speed: f32,
}

#[derive(Component)]
struct PlacementEffect;

// === 落子系统 ===
fn place_stone_with_effects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    audio_assets: Res<AudioAssets>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    game_state: Res<GameState>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        let grid_pos = get_grid_position(); // 获取点击位置
        
        // 创建棋子实体，从高处落下
        let stone_entity = commands.spawn((
            PbrBundle {
                mesh: meshes.add(Sphere::new(0.4)),
                material: materials.add(match game_state.current_player {
                    StoneColor::Black => Color::BLACK,
                    StoneColor::White => Color::WHITE,
                }),
                transform: Transform::from_xyz(
                    grid_pos.x as f32,
                    10.0, // 从高处开始
                    grid_pos.y as f32,
                ),
                ..default()
            },
            Stone {
                color: game_state.current_player,
                grid_pos,
            },
            FallingStone {
                target_y: 0.2,
                speed: 15.0,
            },
        )).id();
        
        // 播放音效
        commands.spawn(AudioBundle {
            source: audio_assets.stone_place.clone(),
            settings: PlaybackSettings::ONCE,
        });
        
        // 生成放置特效
        spawn_placement_effect(&mut commands, grid_pos);
    }
}

// === 下落动画 ===
fn animate_falling_stones(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &FallingStone)>,
    time: Res<Time>,
    audio_assets: Res<AudioAssets>,
) {
    for (entity, mut transform, falling) in query.iter_mut() {
        let delta = falling.speed * time.delta_seconds();
        transform.translation.y -= delta;
        
        if transform.translation.y <= falling.target_y {
            transform.translation.y = falling.target_y;
            
            // 移除下落组件
            commands.entity(entity).remove::<FallingStone>();
            
            // 播放碰撞音效
            commands.spawn(AudioBundle {
                source: audio_assets.stone_impact.clone(),
                settings: PlaybackSettings::ONCE.with_volume(Volume::new(0.3)),
            });
            
            // 添加弹跳效果
            commands.entity(entity).insert(BounceAnimation {
                amplitude: 0.1,
                frequency: 3.0,
                duration: 0.5,
                elapsed: 0.0,
            });
        }
    }
}

// === 弹跳动画 ===
#[derive(Component)]
struct BounceAnimation {
    amplitude: f32,
    frequency: f32,
    duration: f32,
    elapsed: f32,
}

fn bounce_animation_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut BounceAnimation)>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut bounce) in query.iter_mut() {
        bounce.elapsed += time.delta_seconds();
        
        if bounce.elapsed < bounce.duration {
            let damping = 1.0 - (bounce.elapsed / bounce.duration);
            let offset = bounce.amplitude * damping * 
                        (bounce.elapsed * bounce.frequency * 2.0 * PI).sin();
            transform.scale = Vec3::splat(1.0 + offset);
        } else {
            transform.scale = Vec3::ONE;
            commands.entity(entity).remove::<BounceAnimation>();
        }
    }
}
```

### 吃子动画

```rust
#[derive(Component)]
struct CaptureAnimation {
    duration: f32,
    elapsed: f32,
}

fn animate_capture(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut CaptureAnimation)>,
    time: Res<Time>,
    audio_assets: Res<AudioAssets>,
) {
    for (entity, mut transform, mut capture) in query.iter_mut() {
        capture.elapsed += time.delta_seconds();
        let t = capture.elapsed / capture.duration;
        
        if t < 1.0 {
            // 缩小并上升
            transform.scale = Vec3::splat(1.0 - t);
            transform.translation.y += time.delta_seconds() * 5.0;
            transform.rotate_y(time.delta_seconds() * 10.0);
            
            // 透明度渐变（需要透明材质）
        } else {
            // 动画结束，移除实体
            commands.entity(entity).despawn();
            
            // 播放吃子音效
            commands.spawn(AudioBundle {
                source: audio_assets.stone_capture.clone(),
                settings: PlaybackSettings::ONCE,
            });
        }
    }
}
```

## 四、性能优化建议

### 音频优化
1. **预加载常用音效** - 避免运行时加载延迟
2. **音效池** - 重用AudioBundle实体
3. **限制同时播放数量** - 防止音频过载

### 动画优化
1. **使用GPU动画** - Shader动画比CPU动画高效
2. **批处理相似动画** - 减少系统调用
3. **LOD系统** - 远处物体使用简单动画
4. **动画剔除** - 不在视野内的动画暂停

## 五、常用缓动函数

```rust
// 线性
fn linear(t: f32) -> f32 { t }

// 二次
fn ease_in_quad(t: f32) -> f32 { t * t }
fn ease_out_quad(t: f32) -> f32 { t * (2.0 - t) }
fn ease_in_out_quad(t: f32) -> f32 {
    if t < 0.5 { 2.0 * t * t } 
    else { -1.0 + (4.0 - 2.0 * t) * t }
}

// 三次
fn ease_in_cubic(t: f32) -> f32 { t * t * t }
fn ease_out_cubic(t: f32) -> f32 { (t - 1.0).powi(3) + 1.0 }

// 弹性
fn ease_elastic(t: f32) -> f32 {
    (t * t * t * t * t).sin() * (1.0 - t).exp()
}

// 回弹
fn ease_bounce(t: f32) -> f32 {
    if t < 0.363636 {
        7.5625 * t * t
    } else if t < 0.727272 {
        let t = t - 0.545454;
        7.5625 * t * t + 0.75
    } else if t < 0.909090 {
        let t = t - 0.818181;
        7.5625 * t * t + 0.9375
    } else {
        let t = t - 0.954545;
        7.5625 * t * t + 0.984375
    }
}
```

## 六、推荐插件

```toml
[dependencies]
# 动画
bevy_tweening = "0.11"      # Tween动画
bevy_easings = "0.14"       # 缓动函数库
bevy_sprite_anim = "0.6"    # Sprite动画

# 音频
bevy_kira_audio = "0.19"    # 高级音频功能
bevy_oddio = "0.7"          # 空间音频

# 特效
bevy_hanabi = "0.12"        # 粒子系统
bevy_prototype_lyon = "0.11" # 2D矢量图形
```

这些系统配合使用可以创建丰富的游戏体验！