# 30分钟入门 Bevy 2D 渲染

> 通过围棋项目实例，快速掌握 Bevy 的 2D 渲染核心概念

## 1. Bevy 坐标系统

### 坐标系基础
```rust
// Bevy 使用右手坐标系
// X轴：向右为正
// Y轴：向上为正（注意：不是向下）
// Z轴：向屏幕外为正（用于层级控制）

Transform::from_xyz(
    0.0,   // X: 屏幕中心
    0.0,   // Y: 屏幕中心  
    0.0    // Z: 基准层
)
```

### 屏幕坐标 vs 世界坐标
```rust
// 屏幕坐标：像素单位，(0,0) 在左上角
// 世界坐标：Bevy单位，(0,0) 在屏幕中心

// 本项目中的坐标转换
fn screen_to_world(x: f32, y: f32, screen_width: f32, screen_height: f32) -> (f32, f32) {
    (
        x - screen_width / 2.0,   // 屏幕X转世界X
        screen_height / 2.0 - y,   // 屏幕Y转世界Y（Y轴翻转）
    )
}
```

## 2. 基本图形绘制

### 创建 2D 相机
```rust
// 每个 2D 场景都需要相机
commands.spawn(Camera2d::default());
```

### 绘制基本形状
```rust
// 圆形（围棋棋子）
commands.spawn((
    Mesh2d(meshes.add(Circle::new(radius))),
    MeshMaterial2d(materials.add(Color::BLACK)),
    Transform::from_xyz(x, y, z),
));

// 矩形（棋盘背景）
commands.spawn((
    Mesh2d(meshes.add(Rectangle::new(width, height))),
    MeshMaterial2d(materials.add(Color::srgb(0.9, 0.7, 0.4))),
    Transform::from_xyz(0.0, 0.0, 0.0),
));

// 线条（棋盘网格）- 使用细长矩形模拟
commands.spawn((
    Mesh2d(meshes.add(Rectangle::new(length, thickness))),
    MeshMaterial2d(materials.add(Color::BLACK)),
    Transform::from_xyz(x, y, 0.1),  // Z=0.1 确保在背景之上
));
```

## 3. 材质与颜色

### 颜色系统
```rust
// Bevy 0.15+ 使用 sRGB 色彩空间
Color::WHITE                          // 预定义颜色
Color::srgb(1.0, 0.0, 0.0)           // RGB值（0.0-1.0）
Color::srgb_u8(255, 128, 0)          // RGB值（0-255）
Color::hsla(0.5, 1.0, 0.5, 1.0)      // HSL颜色
Color::srgba(0.0, 0.0, 0.0, 0.5)     // 带透明度

// 本项目中的棋子颜色
let black_color = Color::srgb(0.1, 0.1, 0.1);    // 深黑
let white_color = Color::srgb(0.95, 0.95, 0.95); // 亮白
```

### 材质创建
```rust
// 创建材质需要 ResMut<Assets<ColorMaterial>>
fn create_stone(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let material = materials.add(Color::BLACK);
    let mesh = meshes.add(Circle::new(20.0));
    
    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::default(),
    ));
}
```

## 4. Transform 组件详解

### Transform 的三要素
```rust
Transform {
    translation: Vec3::new(x, y, z),  // 位置
    rotation: Quat::IDENTITY,         // 旋转（2D通常不用）
    scale: Vec3::ONE,                 // 缩放
}

// 常用构造方法
Transform::from_xyz(100.0, 50.0, 1.0)
Transform::from_translation(Vec3::new(x, y, z))
Transform::from_scale(Vec3::splat(2.0))  // 统一缩放2倍

// 链式调用
Transform::from_xyz(0.0, 0.0, 0.0)
    .with_scale(Vec3::splat(0.8))
    .with_rotation(Quat::from_rotation_z(0.5))
```

### Z轴层级管理
```rust
// Z值越大，越靠近屏幕（越在上层）
const Z_BACKGROUND: f32 = 0.0;   // 背景层
const Z_GRID: f32 = 0.1;         // 网格层
const Z_STONE: f32 = 0.2;        // 棋子层
const Z_SHADOW: f32 = 0.15;      // 阴影层（棋子下方）
const Z_TEXT: f32 = 0.3;         // 文字层（最上方）
```

## 5. 实体组件系统（ECS）

### 创建实体
```rust
// 实体 = 组件的集合
let entity = commands.spawn((
    // 渲染组件
    Mesh2d(mesh),
    MeshMaterial2d(material),
    Transform::from_xyz(x, y, z),
    
    // 自定义组件
    Stone {
        color: StoneColor::Black,
        grid_pos: (3, 3),
    },
    BoardPosition { x: 3, y: 3 },
))
.id();  // 获取实体ID
```

### 查询系统
```rust
// 查询所有带 Stone 组件的实体
fn update_stones(
    query: Query<(&Stone, &Transform)>,
) {
    for (stone, transform) in query.iter() {
        println!("Stone at {:?}", transform.translation);
    }
}

// 带过滤器的查询
fn update_black_stones(
    query: Query<&Transform, With<Stone>>,
    black_stones: Query<Entity, (With<Stone>, With<BlackStone>)>,
) {
    // 只处理黑棋
}
```

### 修改组件
```rust
fn move_stones(
    mut query: Query<&mut Transform, With<Stone>>,
) {
    for mut transform in query.iter_mut() {
        transform.translation.x += 1.0;
    }
}
```

## 6. 资源管理

### 全局资源
```rust
// 定义资源
#[derive(Resource)]
struct BoardConfig {
    size: usize,
    cell_size: f32,
}

// 插入资源
app.insert_resource(BoardConfig {
    size: 19,
    cell_size: 30.0,
});

// 使用资源
fn use_config(config: Res<BoardConfig>) {
    println!("Board size: {}", config.size);
}

// 修改资源
fn update_config(mut config: ResMut<BoardConfig>) {
    config.size = 13;
}
```

### Assets 资源
```rust
// Mesh 和 Material 是特殊的资源
fn setup(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 创建并存储资源，返回 Handle
    let mesh_handle = meshes.add(Circle::new(10.0));
    let material_handle = materials.add(Color::RED);
}
```

## 7. 事件系统

### 定义事件
```rust
#[derive(Event)]
struct PlaceStoneEvent {
    x: i32,
    y: i32,
    color: StoneColor,
}

// 注册事件
app.add_event::<PlaceStoneEvent>();
```

### 发送事件
```rust
fn send_events(
    mut events: EventWriter<PlaceStoneEvent>,
) {
    events.send(PlaceStoneEvent {
        x: 3,
        y: 3,
        color: StoneColor::Black,
    });
}
```

### 接收事件
```rust
fn handle_events(
    mut events: EventReader<PlaceStoneEvent>,
) {
    for event in events.read() {
        println!("Placing stone at {}, {}", event.x, event.y);
    }
}
```

## 8. 文字渲染

### 创建文字
```rust
// 需要 TextBundle
commands.spawn((
    Text::new("Hello Bevy"),
    TextFont {
        font_size: 30.0,
        ..default()
    },
    TextColor(Color::WHITE),
    Transform::from_xyz(0.0, 0.0, 1.0),
));

// 本项目中的手数显示
commands.spawn((
    Text::new(move_number.to_string()),
    TextFont {
        font_size: stone_radius * 0.8,
        ..default()
    },
    TextColor(if stone.color == StoneColor::Black {
        Color::WHITE
    } else {
        Color::BLACK
    }),
    Transform::from_xyz(x, y, Z_TEXT),
));
```

## 9. 响应式设计

### 窗口大小变化
```rust
// 监听窗口变化事件
fn handle_resize(
    mut resize_events: EventReader<WindowResized>,
    mut redraw_events: EventWriter<RedrawBoardEvent>,
) {
    for event in resize_events.read() {
        println!("Window resized to {}x{}", event.width, event.height);
        redraw_events.send(RedrawBoardEvent);
    }
}

// 获取窗口尺寸
fn get_window_size(windows: Query<&Window>) -> (f32, f32) {
    let window = windows.single();
    (window.width(), window.height())
}
```

### 自适应缩放
```rust
// 根据窗口大小计算棋盘大小
let board_size = window_width.min(window_height) * 0.9;
let cell_size = board_size / (grid_count as f32);
let stone_radius = cell_size * 0.4;
```

## 10. 输入处理

### 鼠标输入
```rust
fn handle_mouse(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let window = windows.single();
        if let Some(cursor_pos) = window.cursor_position() {
            // 转换为世界坐标
            let (camera, transform) = camera.single();
            if let Ok(world_pos) = camera.viewport_to_world_2d(transform, cursor_pos) {
                // 处理点击
            }
        }
    }
}
```

### 键盘输入
```rust
fn handle_keyboard(
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        // 空格键按下
    }
    if keyboard.pressed(KeyCode::ShiftLeft) {
        // Shift 持续按住
    }
}
```

## 11. 动画基础

### 简单移动动画
```rust
fn animate_stones(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Stone>>,
) {
    for mut transform in query.iter_mut() {
        // 使用 sin 函数创建上下浮动
        transform.translation.y += (time.elapsed_secs() * 2.0).sin() * 0.5;
    }
}
```

### 渐变效果
```rust
fn fade_in(
    time: Res<Time>,
    mut query: Query<&mut MeshMaterial2d<ColorMaterial>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for mat_handle in query.iter_mut() {
        if let Some(material) = materials.get_mut(&mat_handle.0) {
            // 渐变透明度
            let alpha = (time.elapsed_secs() * 0.5).min(1.0);
            material.color.set_alpha(alpha);
        }
    }
}
```

## 12. 性能优化技巧

### 批量操作
```rust
// 好：批量生成实体
commands.spawn_batch((0..100).map(|i| {
    (
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
        Transform::from_xyz(i as f32 * 10.0, 0.0, 0.0),
    )
}));

// 避免：循环中单独生成
for i in 0..100 {
    commands.spawn(...);
}
```

### 资源复用
```rust
// 好：复用材质和网格
let mesh = meshes.add(Circle::new(10.0));
let material = materials.add(Color::BLACK);

for pos in positions {
    commands.spawn((
        Mesh2d(mesh.clone()),        // 复用 Handle
        MeshMaterial2d(material.clone()),
        Transform::from_translation(pos),
    ));
}
```

### 选择性更新
```rust
// 使用 Changed 过滤器只处理变化的组件
fn update_changed(
    query: Query<&Transform, Changed<Transform>>,
) {
    for transform in query.iter() {
        // 只处理本帧改变的 Transform
    }
}
```

## 13. 调试技巧

### 可视化调试
```rust
// 添加调试用的可视化标记
#[cfg(debug_assertions)]
commands.spawn((
    Mesh2d(meshes.add(Circle::new(5.0))),
    MeshMaterial2d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
    Transform::from_xyz(click_x, click_y, 10.0),  // 高Z值确保可见
));
```

### 日志输出
```rust
// 使用 bevy 的日志系统
use bevy::log::{info, warn, error};

info!("Stone placed at ({}, {})", x, y);
warn!("Invalid position");
error!("Failed to create mesh");

// 启动时设置日志级别
app.insert_resource(bevy::log::LogSettings {
    level: bevy::log::Level::DEBUG,
    ..default()
});
```

## 14. 围棋项目实战示例

### 绘制棋盘网格
```rust
fn draw_board_grid(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    size: usize,
    cell_size: f32,
) {
    let line_material = materials.add(Color::BLACK);
    let board_size = cell_size * (size - 1) as f32;
    
    // 绘制横线
    for i in 0..size {
        let y = i as f32 * cell_size - board_size / 2.0;
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(board_size, 1.0))),
            MeshMaterial2d(line_material.clone()),
            Transform::from_xyz(0.0, y, 0.1),
        ));
    }
    
    // 绘制竖线
    for i in 0..size {
        let x = i as f32 * cell_size - board_size / 2.0;
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(1.0, board_size))),
            MeshMaterial2d(line_material.clone()),
            Transform::from_xyz(x, 0.0, 0.1),
        ));
    }
}
```

### 放置棋子
```rust
fn place_stone_at_position(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    grid_x: i32,
    grid_y: i32,
    color: StoneColor,
    cell_size: f32,
) {
    let world_x = (grid_x as f32 - 9.0) * cell_size;
    let world_y = (9.0 - grid_y as f32) * cell_size;
    
    let stone_color = match color {
        StoneColor::Black => Color::srgb(0.1, 0.1, 0.1),
        StoneColor::White => Color::srgb(0.95, 0.95, 0.95),
    };
    
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(cell_size * 0.4))),
        MeshMaterial2d(materials.add(stone_color)),
        Transform::from_xyz(world_x, world_y, 0.2),
        Stone { color, grid_pos: (grid_x, grid_y) },
    ));
}
```

## 快速参考

### 必备 imports
```rust
use bevy::prelude::*;
use bevy::sprite::{Mesh2d, MeshMaterial2d};
use bevy::window::WindowResized;
use bevy::input::ButtonInput;
```

### 常用组件组合
```rust
// 2D 精灵
(Sprite, Transform)

// 2D 网格图形
(Mesh2d, MeshMaterial2d, Transform)

// 文字
(Text, TextFont, TextColor, Transform)

// 相机
Camera2d
```

### 系统执行顺序
```rust
app.add_systems(Startup, setup)           // 启动时执行一次
   .add_systems(Update, (                 // 每帧执行
       input_system,
       update_system,
       render_system,
   ).chain())                              // chain() 保证顺序
   .add_systems(FixedUpdate, physics);    // 固定时间间隔执行
```

## 总结

掌握以上概念后，你可以：
1. ✅ 理解 Bevy 的坐标系统和 Transform
2. ✅ 绘制基本 2D 图形（圆、矩形、线条）
3. ✅ 管理材质和颜色
4. ✅ 使用 ECS 架构组织代码
5. ✅ 处理鼠标和键盘输入
6. ✅ 实现响应式布局
7. ✅ 使用事件系统解耦代码
8. ✅ 优化渲染性能

**实践建议**：
1. 先运行 `cargo run` 看效果
2. 修改颜色、大小等参数观察变化
3. 尝试添加新的图形元素
4. 实现简单的交互功能

记住：Bevy 的 2D 渲染基于 ECS 架构，一切都是组件的组合。理解了 Transform + Mesh2d + Material 的组合，就掌握了 2D 渲染的核心。