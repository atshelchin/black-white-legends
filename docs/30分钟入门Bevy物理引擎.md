# 30分钟入门 Bevy 物理引擎

> 使用 Avian (原 bevy_xpbd) 和 Rapier 构建物理世界

## 前言：Bevy 物理引擎选择

Bevy 本身不包含物理引擎，需要使用第三方插件：

| 物理引擎 | 特点 | 适用场景 |
|---------|------|---------|
| **Avian** (bevy_xpbd) | 新一代，API友好，性能优秀 | 推荐首选，游戏开发 |
| **Rapier** (bevy_rapier) | 成熟稳定，功能完整 | 复杂物理模拟 |
| **bevy_ecs_ldtk** | 专门用于关卡编辑器 | 平台游戏 |

本文档主要介绍 **Avian**，并对比 Rapier 的差异。

## 1. 安装配置

### 添加依赖
```toml
# Cargo.toml
[dependencies]
bevy = "0.15"
avian2d = "0.2"  # 2D 物理
# 或
avian3d = "0.2"  # 3D 物理

# 如果选择 Rapier
# bevy_rapier2d = "0.27"
```

### 初始化插件
```rust
use avian2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())  // 添加物理插件
        .add_systems(Startup, setup)
        .run();
}

// 自定义物理配置
.add_plugins(
    PhysicsPlugins::default()
        .with_length_unit(100.0)  // 100 像素 = 1 米
)
```

## 2. 核心概念

### 物理组件架构
```rust
// 一个物理实体的完整组成
commands.spawn((
    // 渲染组件
    Sprite::from_color(Color::RED, Vec2::new(50.0, 50.0)),
    Transform::from_xyz(0.0, 100.0, 0.0),
    
    // 物理组件
    RigidBody::Dynamic,        // 刚体类型
    Collider::rectangle(50.0, 50.0),  // 碰撞体
    LinearVelocity(Vec2::new(100.0, 0.0)),  // 线速度
    AngularVelocity(1.0),      // 角速度
    Mass(1.0),                 // 质量
    Restitution(0.7),          // 弹性系数
    Friction(0.5),             // 摩擦系数
));
```

## 3. 刚体类型 (RigidBody)

```rust
// 动态刚体 - 受物理影响
RigidBody::Dynamic

// 运动学刚体 - 可控制移动，不受力影响
RigidBody::Kinematic

// 静态刚体 - 不动的物体（地面、墙壁）
RigidBody::Static

// 示例：不同类型的游戏对象
// 玩家
commands.spawn((
    RigidBody::Kinematic,  // 玩家控制，不受重力
    Collider::capsule(20.0, 40.0),
));

// 箱子
commands.spawn((
    RigidBody::Dynamic,    // 可推动，受重力
    Collider::rectangle(30.0, 30.0),
    Mass(5.0),
));

// 地面
commands.spawn((
    RigidBody::Static,     // 固定不动
    Collider::rectangle(500.0, 20.0),
));
```

## 4. 碰撞体 (Collider)

### 基本形状
```rust
// 2D 碰撞体
Collider::circle(radius)                    // 圆形
Collider::rectangle(width, height)          // 矩形
Collider::capsule(radius, length)           // 胶囊体
Collider::triangle(a, b, c)                 // 三角形
Collider::polyline(vec![p1, p2, p3])       // 折线
Collider::convex_hull(points)               // 凸包
Collider::trimesh(vertices, indices)        // 网格

// 围棋棋子示例
fn create_physics_stone(
    commands: &mut Commands,
    x: f32,
    y: f32,
) {
    commands.spawn((
        // 视觉
        Mesh2d(meshes.add(Circle::new(20.0))),
        MeshMaterial2d(materials.add(Color::BLACK)),
        Transform::from_xyz(x, y, 0.0),
        
        // 物理
        RigidBody::Dynamic,
        Collider::circle(20.0),
        Mass(0.5),
        Restitution(0.3),  // 轻微弹性
        Friction(0.7),     // 较高摩擦
    ));
}
```

### 复合碰撞体
```rust
// 创建复杂形状的碰撞体
commands.spawn((
    RigidBody::Dynamic,
    Collider::compound(vec![
        (Vec2::new(0.0, 0.0), 0.0, Collider::circle(20.0)),
        (Vec2::new(0.0, 30.0), 0.0, Collider::rectangle(40.0, 20.0)),
    ]),
));
```

## 5. 物理属性

### 质量与密度
```rust
// 直接设置质量
Mass(10.0)

// 设置密度（自动计算质量）
ColliderDensity(2.0)

// 质量属性
MassPropertiesBundle {
    mass: Mass(5.0),
    angular_inertia: AngularInertia(100.0),  // 转动惯量
    center_of_mass: CenterOfMass(Vec2::ZERO),
}
```

### 材质属性
```rust
// 弹性系数 (0.0 = 完全非弹性, 1.0 = 完全弹性)
Restitution(0.7)

// 摩擦系数 (0.0 = 无摩擦, 1.0 = 高摩擦)
Friction(0.5)

// 组合示例：不同材质
// 超级弹球
(Restitution(0.95), Friction(0.1))

// 冰面
(Restitution(0.1), Friction(0.01))

// 橡胶
(Restitution(0.8), Friction(0.9))

// 石头
(Restitution(0.2), Friction(0.6))
```

## 6. 力与运动

### 施加力
```rust
// 力的组件
ExternalForce(Vec2::new(100.0, 0.0))        // 持续力
ExternalImpulse(Vec2::new(0.0, 500.0))      // 瞬时冲量
ExternalTorque(10.0)                         // 扭矩
ExternalAngularImpulse(5.0)                  // 角冲量

// 施加跳跃冲量
fn jump_system(
    mut query: Query<&mut ExternalImpulse, With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        for mut impulse in query.iter_mut() {
            impulse.0 = Vec2::new(0.0, 300.0);  // 向上跳
        }
    }
}

// 持续施加推力
fn thrust_system(
    mut query: Query<&mut ExternalForce, With<Rocket>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for mut force in query.iter_mut() {
        if keyboard.pressed(KeyCode::ArrowUp) {
            force.0 = Vec2::new(0.0, 100.0);
        } else {
            force.0 = Vec2::ZERO;
        }
    }
}
```

### 速度控制
```rust
// 线速度
LinearVelocity(Vec2::new(100.0, 0.0))

// 角速度（弧度/秒）
AngularVelocity(2.0)

// 速度阻尼（空气阻力）
LinearDamping(0.5)
AngularDamping(0.5)

// 锁定轴向
LockedAxes::ROTATION_LOCKED           // 锁定旋转
LockedAxes::TRANSLATION_LOCKED_X      // 锁定X轴移动
```

## 7. 重力系统

### 全局重力
```rust
// 设置重力
app.insert_resource(Gravity(Vec2::new(0.0, -981.0)));  // 地球重力

// 零重力（太空）
app.insert_resource(Gravity(Vec2::ZERO));

// 横向重力（特殊效果）
app.insert_resource(Gravity(Vec2::new(100.0, 0.0)));
```

### 个体重力
```rust
// 重力缩放（个体）
GravityScale(0.5)   // 半重力
GravityScale(0.0)   // 无重力（漂浮）
GravityScale(-1.0)  // 反重力（上升）

// 示例：不同重力的物体
// 气球
commands.spawn((
    RigidBody::Dynamic,
    Collider::circle(20.0),
    GravityScale(-0.5),  // 缓慢上升
));

// 羽毛
commands.spawn((
    RigidBody::Dynamic,
    Collider::rectangle(10.0, 5.0),
    GravityScale(0.1),   // 缓慢下落
    LinearDamping(2.0),  // 空气阻力
));
```

## 8. 碰撞检测

### 碰撞事件
```rust
// 碰撞事件类型
#[derive(Event)]
struct CollisionStarted(Entity, Entity);  // 开始碰撞
struct CollisionEnded(Entity, Entity);    // 结束碰撞

// 监听碰撞
fn handle_collisions(
    mut collision_events: EventReader<Collision>,
) {
    for Collision(contacts) in collision_events.read() {
        println!("碰撞发生在: {:?} 和 {:?}", 
                 contacts.entity1, 
                 contacts.entity2);
        
        // 碰撞点信息
        for contact in contacts.manifolds.iter() {
            println!("碰撞点: {:?}", contact.point);
            println!("法线: {:?}", contact.normal);
            println!("穿透深度: {}", contact.penetration);
        }
    }
}
```

### 传感器（Sensor）
```rust
// 传感器只检测碰撞，不产生物理反应
commands.spawn((
    Collider::circle(50.0),
    Sensor,  // 标记为传感器
    Transform::from_xyz(0.0, 0.0, 0.0),
));

// 检测进入传感器区域
fn detect_sensor_entry(
    mut events: EventReader<CollisionStarted>,
    sensors: Query<Entity, With<Sensor>>,
) {
    for CollisionStarted(e1, e2) in events.read() {
        if sensors.contains(*e1) || sensors.contains(*e2) {
            println!("有物体进入传感器区域！");
        }
    }
}
```

## 9. 射线检测 (Raycasting)

```rust
use avian2d::prelude::*;

// 射线检测系统
fn raycast_system(
    spatial_query: SpatialQuery,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let window = windows.single();
    if let Some(cursor_pos) = window.cursor_position() {
        let (camera, transform) = camera.single();
        
        // 屏幕坐标转世界坐标
        if let Ok(ray_origin) = camera.viewport_to_world_2d(transform, cursor_pos) {
            let ray_direction = Vec2::new(1.0, 0.0);  // 向右发射
            
            // 执行射线检测
            if let Some(hit) = spatial_query.cast_ray(
                ray_origin,
                ray_direction,
                f32::MAX,
                true,  // 是否检测传感器
                SpatialQueryFilter::default(),
            ) {
                println!("射线击中: {:?} 在距离 {}", hit.entity, hit.distance);
            }
        }
    }
}

// 形状投射（Shape casting）
fn shape_cast_system(spatial_query: SpatialQuery) {
    let shape = Collider::circle(10.0);
    let origin = Vec2::ZERO;
    let direction = Vec2::new(1.0, 0.0);
    
    if let Some(hit) = spatial_query.cast_shape(
        &shape,
        origin,
        0.0,  // 旋转角度
        direction,
        f32::MAX,
        true,
        SpatialQueryFilter::default(),
    ) {
        println!("形状投射击中: {:?}", hit.entity);
    }
}
```

## 10. 关节与约束

### 关节类型
```rust
// 固定关节 - 两个物体相对位置固定
FixedJoint::new(entity1, entity2)
    .with_local_anchor_1(Vec2::new(10.0, 0.0))
    .with_local_anchor_2(Vec2::new(-10.0, 0.0))

// 旋转关节 - 允许旋转
RevoluteJoint::new(entity1, entity2)
    .with_local_anchor_1(Vec2::ZERO)
    .with_local_anchor_2(Vec2::ZERO)
    .with_angle_limits(-PI / 4.0, PI / 4.0)  // 限制角度

// 距离关节 - 保持固定距离
DistanceJoint::new(entity1, entity2)
    .with_rest_length(100.0)
    .with_stiffness(1000.0)
    .with_damping(10.0)

// 创建链条
fn create_chain(commands: &mut Commands) {
    let mut previous = None;
    
    for i in 0..10 {
        let current = commands.spawn((
            RigidBody::Dynamic,
            Collider::rectangle(10.0, 20.0),
            Transform::from_xyz(i as f32 * 15.0, 0.0, 0.0),
        )).id();
        
        if let Some(prev) = previous {
            commands.spawn(
                RevoluteJoint::new(prev, current)
                    .with_local_anchor_1(Vec2::new(5.0, 0.0))
                    .with_local_anchor_2(Vec2::new(-5.0, 0.0))
            );
        }
        
        previous = Some(current);
    }
}
```

## 11. 碰撞层与过滤

```rust
// 定义碰撞层
#[derive(PhysicsLayer)]
enum GameLayer {
    Player,     // 0b00001
    Enemy,      // 0b00010
    Ground,     // 0b00100
    Bullet,     // 0b01000
    Sensor,     // 0b10000
}

// 设置碰撞层
commands.spawn((
    Collider::circle(10.0),
    CollisionLayers::new(
        GameLayer::Player,  // 自己的层
        [GameLayer::Enemy, GameLayer::Ground],  // 可以碰撞的层
    ),
));

// 玩家子弹不会击中玩家
commands.spawn((
    Collider::circle(5.0),
    CollisionLayers::new(
        GameLayer::Bullet,
        [GameLayer::Enemy, GameLayer::Ground],  // 不包含 Player
    ),
));
```

## 12. 性能优化

### 空间分区
```rust
// 配置空间哈希
app.insert_resource(SpatialQuerySettings {
    max_entities: 10000,
    cell_size: 100.0,
});
```

### 休眠系统
```rust
// 自动休眠不活动的物体
Sleeping::default()

// 手动控制休眠
fn sleep_distant_objects(
    mut query: Query<(&Transform, &mut Sleeping)>,
    player: Query<&Transform, With<Player>>,
) {
    let player_pos = player.single().translation.truncate();
    
    for (transform, mut sleeping) in query.iter_mut() {
        let distance = transform.translation.truncate().distance(player_pos);
        
        if distance > 1000.0 {
            sleeping.sleeping = true;  // 休眠远处物体
        } else {
            sleeping.sleeping = false;  // 唤醒近处物体
        }
    }
}
```

### 连续碰撞检测 (CCD)
```rust
// 高速物体使用 CCD 防止穿透
SweptCcd::default()

// 子弹示例
commands.spawn((
    RigidBody::Dynamic,
    Collider::circle(2.0),
    LinearVelocity(Vec2::new(1000.0, 0.0)),  // 高速
    SweptCcd::default(),  // 启用 CCD
));
```

## 13. 实战示例：弹珠台围棋

```rust
// 创建具有物理效果的围棋游戏
fn setup_physics_go_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 相机
    commands.spawn(Camera2d::default());
    
    // 设置重力（轻微倾斜效果）
    commands.insert_resource(Gravity(Vec2::new(0.0, -200.0)));
    
    // 棋盘边框（物理墙壁）
    let board_size = 400.0;
    let wall_thickness = 10.0;
    
    // 上边框
    commands.spawn((
        Sprite::from_color(Color::srgb(0.3, 0.2, 0.1), 
                          Vec2::new(board_size, wall_thickness)),
        Transform::from_xyz(0.0, board_size / 2.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(board_size, wall_thickness),
    ));
    
    // 下边框
    commands.spawn((
        Sprite::from_color(Color::srgb(0.3, 0.2, 0.1), 
                          Vec2::new(board_size, wall_thickness)),
        Transform::from_xyz(0.0, -board_size / 2.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(board_size, wall_thickness),
    ));
    
    // 左边框
    commands.spawn((
        Sprite::from_color(Color::srgb(0.3, 0.2, 0.1), 
                          Vec2::new(wall_thickness, board_size)),
        Transform::from_xyz(-board_size / 2.0, 0.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(wall_thickness, board_size),
    ));
    
    // 右边框
    commands.spawn((
        Sprite::from_color(Color::srgb(0.3, 0.2, 0.1), 
                          Vec2::new(wall_thickness, board_size)),
        Transform::from_xyz(board_size / 2.0, 0.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(wall_thickness, board_size),
    ));
    
    // 创建交叉点的小凹槽（吸引棋子）
    for i in 0..9 {
        for j in 0..9 {
            let x = (i as f32 - 4.0) * 40.0;
            let y = (j as f32 - 4.0) * 40.0;
            
            // 凹槽传感器
            commands.spawn((
                Transform::from_xyz(x, y, 0.0),
                Collider::circle(15.0),
                Sensor,
                GridPosition { x: i, y: j },
            ));
        }
    }
}

// 投掷棋子
fn throw_stone(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let window = windows.single();
        if let Some(pos) = window.cursor_position() {
            // 创建物理棋子
            commands.spawn((
                Mesh2d(meshes.add(Circle::new(15.0))),
                MeshMaterial2d(materials.add(Color::BLACK)),
                Transform::from_xyz(pos.x - 400.0, 300.0 - pos.y, 1.0),
                
                // 物理组件
                RigidBody::Dynamic,
                Collider::circle(15.0),
                Mass(1.0),
                Restitution(0.4),        // 有弹性
                Friction(0.3),           // 适中摩擦
                LinearDamping(0.5),      // 空气阻力
                AngularDamping(0.8),     // 旋转阻尼
                
                // 初始速度（向下投掷）
                LinearVelocity(Vec2::new(0.0, -300.0)),
                AngularVelocity(5.0),    // 旋转
            ));
        }
    }
}

// 磁性吸附到交叉点
fn magnetic_snap(
    mut stones: Query<(&Transform, &mut ExternalForce), With<Stone>>,
    grid_points: Query<(&Transform, &GridPosition), With<Sensor>>,
) {
    for (stone_transform, mut force) in stones.iter_mut() {
        let stone_pos = stone_transform.translation.truncate();
        
        // 找最近的交叉点
        let mut min_distance = f32::MAX;
        let mut target_pos = Vec2::ZERO;
        
        for (grid_transform, _) in grid_points.iter() {
            let grid_pos = grid_transform.translation.truncate();
            let distance = stone_pos.distance(grid_pos);
            
            if distance < min_distance && distance < 30.0 {
                min_distance = distance;
                target_pos = grid_pos;
            }
        }
        
        // 施加吸引力
        if min_distance < 30.0 {
            let direction = (target_pos - stone_pos).normalize();
            let strength = 100.0 * (1.0 - min_distance / 30.0);
            force.0 = direction * strength;
        } else {
            force.0 = Vec2::ZERO;
        }
    }
}
```

## 14. 调试工具

### 启用调试渲染
```rust
// 添加调试插件
use avian2d::prelude::*;

app.add_plugins(PhysicsDebugPlugin::default());

// 配置调试显示
app.insert_resource(PhysicsDebugConfig {
    aabb_color: Some(Color::srgb(0.0, 1.0, 0.0)),
    shape_color: Some(Color::srgb(1.0, 0.0, 0.0)),
    joint_color: Some(Color::srgb(0.0, 0.0, 1.0)),
    ..default()
});
```

### 性能监控
```rust
// 显示物理统计
fn show_physics_stats(
    diagnostics: Res<DiagnosticsStore>,
) {
    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            println!("FPS: {:.2}", value);
        }
    }
}
```

## 15. Avian vs Rapier 对比

| 特性 | Avian | Rapier |
|-----|-------|--------|
| API 风格 | ECS原生，组件化 | 传统物理引擎风格 |
| 性能 | 优秀 | 优秀 |
| 文档 | 简洁清晰 | 详细完整 |
| 功能完整度 | 核心功能完备 | 功能最全面 |
| 学习曲线 | 平缓 | 稍陡峭 |
| 社区 | 快速增长 | 成熟稳定 |

### Rapier 特有功能
```rust
// Rapier 的一些高级功能
use bevy_rapier2d::prelude::*;

// 更多关节类型
SpringJoint
PrismaticJoint
BallJoint

// 高级碰撞检测
ConvexDecomposition
TriMesh
HeightField

// 流体模拟
Fluid
```

## 快速参考卡

### 必备 imports
```rust
use avian2d::prelude::*;
// 或
use bevy_rapier2d::prelude::*;
```

### 创建物理对象模板
```rust
// 动态物体
(
    Sprite,
    Transform,
    RigidBody::Dynamic,
    Collider::circle(10.0),
    Mass(1.0),
)

// 静态地面
(
    Sprite,
    Transform,
    RigidBody::Static,
    Collider::rectangle(100.0, 10.0),
)

// 可控角色
(
    Sprite,
    Transform,
    RigidBody::Kinematic,
    Collider::capsule(10.0, 20.0),
    LinearVelocity::default(),
)
```

### 常用物理参数
```rust
// 材质预设
// 弹球
(Restitution(0.9), Friction(0.1))

// 木头
(Restitution(0.2), Friction(0.5))

// 金属
(Restitution(0.1), Friction(0.3))

// 橡胶
(Restitution(0.7), Friction(0.9))
```

## 总结

掌握物理引擎后，你可以：
1. ✅ 创建具有真实物理效果的游戏对象
2. ✅ 实现重力、碰撞、弹跳等效果
3. ✅ 处理复杂的物理交互
4. ✅ 优化物理性能
5. ✅ 调试物理问题

**下一步建议**：
1. 为围棋项目添加棋子掉落动画
2. 实现棋子之间的碰撞效果
3. 创建物理驱动的特效
4. 尝试制作物理解谜玩法

记住：物理引擎是游戏感觉（Game Feel）的关键。合理的物理参数能让游戏更有趣味性和真实感。