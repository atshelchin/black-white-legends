# 30分钟上手 Bevy Engine - 从前端框架视角理解游戏引擎

## 核心概念对比

### 1. ECS vs 组件化前端框架

**React/Svelte 模式：**
```javascript
// React: 组件 = 数据 + 渲染逻辑
function Player({ health, position }) {
  return <div>Health: {health}</div>
}
```

**Bevy ECS 模式：**
```rust
// Entity = 实体ID
// Component = 纯数据
#[derive(Component)]
struct Health(i32);

#[derive(Component)]
struct Position { x: f32, y: f32 }

// System = 纯逻辑（类似 React hooks/Svelte reactive statements）
fn damage_system(mut query: Query<&mut Health>) {
    for mut health in query.iter_mut() {
        health.0 -= 1;
    }
}
```

**关键区别：**
- React/Svelte: 组件包含数据+逻辑
- Bevy: Entity只是ID，Component是纯数据，System是纯逻辑
- 类比: Entity像DOM节点ID，Component像props/state，System像useEffect

### 2. App 生命周期

**React/Svelte：**
```javascript
// React
useEffect(() => { /* 组件挂载 */ }, [])
useEffect(() => { /* 每次渲染 */ })

// Svelte
onMount(() => { /* 初始化 */ })
$: { /* 响应式更新 */ }
```

**Bevy：**
```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)  // 类似 React.StrictMode 或 SvelteKit 配置
        .add_systems(Startup, setup)  // 类似 onMount/useEffect(,[])
        .add_systems(Update, (     // 类似 requestAnimationFrame 循环
            movement_system,
            collision_system,
        ))
        .run();
}
```

### 3. 状态管理

**React/Svelte Context vs Bevy Resources**

```javascript
// React Context
const GameContext = createContext();
<GameContext.Provider value={{ score: 0 }}>
```

```rust
// Bevy Resource - 全局单例状态
#[derive(Resource)]
struct GameScore(u32);

// 在系统中访问
fn score_system(score: Res<GameScore>) {
    println!("Score: {}", score.0);
}

// 修改资源
fn update_score(mut score: ResMut<GameScore>) {
    score.0 += 10;
}
```

### 4. 查询系统 - 类似 React Query Selectors

**Bevy Query 类似于前端的选择器：**

```rust
// 查询所有带 Health 和 Position 的实体
fn health_display_system(
    query: Query<(&Health, &Position)>  // 类似 querySelectorAll('.health.position')
) {
    for (health, pos) in query.iter() {
        // 处理每个实体
    }
}

// 带过滤器的查询
fn player_system(
    query: Query<&Health, With<Player>>  // 类似 querySelector('.health.player')
) { }

// 排除某些组件
fn enemy_ai(
    query: Query<&Transform, (With<Enemy>, Without<Player>)>
) { }
```

### 5. 事件系统 - 类似 DOM Events

**React/Svelte：**
```javascript
// React
onClick={(e) => handleClick(e)}

// Svelte
on:click={handleClick}
```

**Bevy：**
```rust
// 定义事件
#[derive(Event)]
struct CollisionEvent {
    entity_a: Entity,
    entity_b: Entity,
}

// 发送事件
fn detect_collisions(mut events: EventWriter<CollisionEvent>) {
    events.send(CollisionEvent { 
        entity_a: entity1, 
        entity_b: entity2 
    });
}

// 接收事件
fn handle_collisions(mut events: EventReader<CollisionEvent>) {
    for event in events.read() {
        // 处理碰撞
    }
}
```

## 实战例子：创建一个简单的围棋棋盘

```rust
use bevy::prelude::*;

// === Components (类似 Props/State) ===
#[derive(Component)]
struct GoBoard;

#[derive(Component)]
struct Stone {
    color: StoneColor,
    grid_pos: IVec2,
}

#[derive(Component, Clone, Copy)]
enum StoneColor {
    Black,
    White,
}

// === Resources (类似 Context/Store) ===
#[derive(Resource)]
struct GameState {
    current_player: StoneColor,
    board_size: u8,  // 19x19
}

// === Plugin (类似 React Component/Svelte Module) ===
pub struct GoBoardPlugin;

impl Plugin for GoBoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GameState {
                current_player: StoneColor::Black,
                board_size: 19,
            })
            .add_systems(Startup, setup_board)
            .add_systems(Update, (
                handle_mouse_input,
                render_stones,
            ));
    }
}

// === Systems (类似 Hooks/Reactive Statements) ===

// Startup System - 类似 useEffect(() => {}, [])
fn setup_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 创建摄像机 - 类似设置 viewport
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 20.0, 0.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // 创建棋盘实体
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10.0))),
            material: materials.add(Color::srgb(0.8, 0.7, 0.5)),
            ..default()
        },
        GoBoard,
    ));
}

// Update System - 类似 React 的重渲染循环
fn handle_mouse_input(
    buttons: Res<ButtonInput<MouseButton>>,
    mut game_state: ResMut<GameState>,
    mut commands: Commands,
) {
    if buttons.just_pressed(MouseButton::Left) {
        // 放置棋子逻辑
        // 切换玩家
        game_state.current_player = match game_state.current_player {
            StoneColor::Black => StoneColor::White,
            StoneColor::White => StoneColor::Black,
        };
    }
}

fn render_stones(
    query: Query<(&Stone, &Transform)>,
) {
    // 渲染逻辑 - Bevy 自动处理，类似 React 的虚拟 DOM
}
```

## 关键概念速查

| 前端概念 | Bevy 对应 | 说明 |
|---------|----------|------|
| Component | Component | 纯数据结构 |
| Props | Component 字段 | 实体的属性 |
| State | Resource | 全局状态 |
| Context | Resource | 共享数据 |
| useEffect | System | 逻辑处理 |
| Event Handler | EventReader/Writer | 事件处理 |
| DOM Node | Entity | 游戏对象 |
| render() | 自动渲染 | Bevy 自动处理渲染 |
| Component Tree | Entity Hierarchy | 父子关系 |

## 快速开始步骤

1. **创建 Component** - 定义数据结构
2. **创建 System** - 编写游戏逻辑
3. **注册到 App** - 连接所有部分
4. **使用 Query** - 访问和修改数据

## 常用模式

### 1. 初始化模式
```rust
fn setup(mut commands: Commands) {
    commands.spawn(/* 组件元组 */);
}
```

### 2. 更新模式
```rust
fn update(mut query: Query<&mut Transform, With<Player>>) {
    for mut transform in query.iter_mut() {
        // 更新逻辑
    }
}
```

### 3. 响应输入
```rust
fn input(keys: Res<ButtonInput<KeyCode>>) {
    if keys.pressed(KeyCode::Space) {
        // 处理输入
    }
}
```

### 4. 状态机模式
```rust
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    Playing,
    GameOver,
}
```

## 调试技巧

1. **使用 info!() 宏** - 类似 console.log
2. **Bevy Inspector** - 类似 React DevTools
3. **查询调试** - `.single()` 确保只有一个实体

## 性能优化

- **Change Detection**: `Changed<T>` - 类似 React.memo
- **Run Conditions**: 条件执行系统 - 类似 useMemo
- **Parallel Systems**: 自动并行 - 比前端框架更高效

这就是 Bevy 的核心概念。记住：
- **Entity** = ID
- **Component** = 数据
- **System** = 逻辑
- **Resource** = 全局状态

与 React/Svelte 最大的不同是：Bevy 完全分离了数据和逻辑，这使得游戏逻辑更容易测试和复用。