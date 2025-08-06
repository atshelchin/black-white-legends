# 围棋棋盘组件 / Go Board Component

一个基于 Bevy 游戏引擎的高度可复用围棋棋盘组件，支持标准围棋规则和丰富的自定义选项。

## 特性

- ✅ **多种棋盘尺寸**: 支持 9x9、13x13、19x19 棋盘
- ✅ **响应式设计**: 自动适应不同屏幕尺寸
- ✅ **完整的围棋规则**: 包括提子、打劫等规则
- ✅ **可视化选项**: 坐标显示、手数显示、悬停提示
- ✅ **高度可定制**: 颜色、样式、规则都可配置
- ✅ **模块化设计**: 易于集成到任何 Bevy 项目

## 快速开始

### 1. 添加依赖

```toml
[dependencies]
bevy = "0.16"
serde = { version = "1.0", features = ["derive"] }
```

### 2. 基础使用

```rust
use bevy::prelude::*;
use go_board_component::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GoBoardPlugin::default())  // 使用默认配置
        .run();
}
```

### 3. 自定义配置

```rust
use bevy::prelude::*;
use go_board_component::prelude::*;
use go_board_component::plugin::GoBoardPluginBuilder;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            GoBoardPluginBuilder::new()
                .with_board_size(BoardSize::Nineteen)
                .with_coordinates(true)
                .with_move_numbers(false)
                .with_captures(true)
                .with_ko_rule(true)
                .build()
        )
        .run();
}
```

## 组件架构

### 模块结构

```
go_board_component/
├── mod.rs           # 模块入口和公共接口
├── config.rs        # 配置结构和构建器
├── components.rs    # ECS 组件定义
├── events.rs        # 事件定义
├── resources.rs     # 资源定义
├── systems.rs       # 系统实现
├── plugin.rs        # Bevy 插件
├── rules.rs         # 围棋规则引擎
└── utils.rs         # 工具函数
```

### 核心组件

#### 配置 (GoBoardConfig)
```rust
pub struct GoBoardConfig {
    pub board_size: BoardSize,              // 棋盘大小
    pub show_coordinates: bool,             // 显示坐标
    pub show_move_numbers: bool,            // 显示手数
    pub use_3d_stones: bool,                // 3D 效果
    pub board_color: Color,                 // 棋盘颜色
    pub line_color: Color,                  // 线条颜色
    pub coordinate_color: Color,            // 坐标颜色
    pub star_point_radius_ratio: f32,       // 星位大小
    pub line_width_ratio: f32,              // 线条宽度
    pub adaptive_padding: bool,             // 自适应边距
    pub enable_hover_indicator: bool,       // 悬停提示
    pub enable_sound: bool,                 // 音效
    pub enable_captures: bool,              // 提子规则
    pub enable_ko_rule: bool,               // 打劫规则
}
```

#### 事件系统

- `PlaceStoneEvent`: 落子事件
- `ClearBoardEvent`: 清空棋盘
- `UpdateBoardConfigEvent`: 更新配置
- `RedrawBoardEvent`: 重绘棋盘
- `UndoMoveEvent`: 撤销
- `RedoMoveEvent`: 重做

### 使用示例

#### 处理落子
```rust
fn handle_mouse_click(
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut stone_events: EventWriter<PlaceStoneEvent>,
    current_turn: Res<CurrentTurn>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        // 计算棋盘坐标...
        stone_events.write(PlaceStoneEvent {
            position: (board_x, board_y),
            color: current_turn.0,
        });
    }
}
```

#### 监听游戏状态
```rust
fn check_game_end(
    mut game_end_events: EventReader<GameEndEvent>,
) {
    for event in game_end_events.read() {
        if let Some(winner) = event.winner {
            println!("游戏结束！获胜方: {:?}", winner);
            println!("黑方得分: {}", event.black_score);
            println!("白方得分: {}", event.white_score);
        }
    }
}
```

#### 自定义渲染
```rust
fn custom_stone_render(
    stones: Query<(&Stone, &Transform)>,
) {
    for (stone, transform) in stones.iter() {
        // 自定义渲染逻辑...
    }
}
```

## 键盘快捷键（示例应用）

- `1/2/3`: 切换棋盘大小 (9x9/13x13/19x19)
- `C`: 显示/隐藏坐标
- `M`: 显示/隐藏手数
- `R`: 重置棋盘
- `F`: 全屏
- `ESC`: 退出全屏
- `鼠标点击`: 落子

## API 参考

### 资源 (Resources)

- `CurrentGoBoardConfig`: 当前棋盘配置
- `CurrentTurn`: 当前回合（黑/白）
- `BoardState`: 棋盘状态，包含所有棋子位置
- `GameHistory`: 游戏历史记录

### 组件 (Components)

- `GoBoard`: 棋盘根实体
- `Stone`: 棋子
- `BoardLine`: 棋盘线条
- `StarPoint`: 星位点
- `CoordinateLabel`: 坐标标签
- `MoveNumberLabel`: 手数标签

### 工具函数

```rust
// 坐标转换
CoordinateUtils::world_to_board(world_pos, board_size, window_size, adaptive_padding)
CoordinateUtils::board_to_world(board_pos, board_size, window_size, adaptive_padding)

// 规则检查
GoBoardRules::is_valid_move(board_state, x, y, color)
GoBoardRules::capture_stones(board_state, x, y, color)
GoBoardRules::calculate_score(board_state)
```

## 扩展性

组件设计为高度模块化，易于扩展：

1. **自定义规则**: 继承或修改 `GoBoardRules`
2. **自定义渲染**: 替换 `systems.rs` 中的渲染函数
3. **添加功能**: 通过事件系统添加新功能
4. **集成AI**: 监听 `PlaceStoneEvent` 实现 AI 对战

## 性能优化

- 使用 Bevy ECS 架构，高效的组件查询
- 事件驱动，避免不必要的重绘
- 智能的实体管理，避免内存泄漏

## 许可证

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！