# 围棋棋盘组件

一个高度可复用的围棋棋盘 Bevy 组件。

## 功能特性

- **多种棋盘规格**：支持 9路、13路、19路棋盘
- **可配置外观**：自定义颜色、线宽、星位点
- **坐标系统**：可选的坐标标注（横向A-T，纵向1-19）
- **响应式设计**：自动适应窗口大小变化
- **演示模式**：针对不同屏幕尺寸的自适应边距
- **事件驱动**：通过事件系统轻松集成游戏逻辑

## 使用方法

### 基础设置

```rust
use bevy::prelude::*;
use go_board::{GoBoardPlugin, GoBoardConfig, BoardSize};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GoBoardPlugin::default())
        .run();
}
```

### 自定义配置

```rust
use go_board::{GoBoardPlugin, GoBoardConfig, BoardSize};

App::new()
    .add_plugins(GoBoardPlugin {
        initial_config: GoBoardConfig {
            board_size: BoardSize::Nineteen,
            show_coordinates: true,
            board_color: Color::srgb(0.82, 0.70, 0.55),
            line_color: Color::BLACK,
            coordinate_color: Color::srgb(0.2, 0.2, 0.2),
            star_point_radius_ratio: 0.125,
            line_width_ratio: 0.06,
            adaptive_padding: true,
        }
    })
```

### 运行时更新配置

```rust
use go_board::{UpdateBoardConfigEvent, GoBoardConfig, BoardSize};

fn change_board_size(
    mut config_events: EventWriter<UpdateBoardConfigEvent>,
    current_config: Res<go_board::CurrentGoBoardConfig>,
) {
    // 切换到9路棋盘
    config_events.send(UpdateBoardConfigEvent {
        config: GoBoardConfig {
            board_size: BoardSize::Nine,
            ..current_config.0.clone()
        }
    });
}
```

## 配置选项

### GoBoardConfig

| 字段 | 类型 | 描述 | 默认值 |
|------|------|------|--------|
| `board_size` | `BoardSize` | 棋盘尺寸（9x9, 13x13, 19x19） | `BoardSize::Nineteen` |
| `show_coordinates` | `bool` | 显示坐标标注 | `true` |
| `board_color` | `Color` | 棋盘背景色 | 木纹色 |
| `line_color` | `Color` | 棋盘线和星位颜色 | 黑色 |
| `coordinate_color` | `Color` | 坐标标注颜色 | 深灰色 |
| `star_point_radius_ratio` | `f32` | 星位大小相对格子的比例 | `0.125` |
| `line_width_ratio` | `f32` | 线宽相对格子的比例 | `0.06` |
| `adaptive_padding` | `bool` | 大屏幕时减少边距 | `true` |

## 组件

插件创建的可查询组件：

- `GoBoardRoot`: 主棋盘实体
- `GoBoard`: 棋盘背景
- `BoardLine`: 网格线
- `StarPoint`: 星位点
- `CoordinateLabel`: 坐标标注

## 事件

- `UpdateBoardConfigEvent`: 更新棋盘配置
- `RedrawBoardEvent`: 强制重绘棋盘

## 架构设计原则

- **高内聚**：所有棋盘相关逻辑封装在模块内
- **低耦合**：通过事件和资源进行通信
- **可配置性**：丰富的配置选项
- **响应式**：事件驱动的更新机制
- **高性能**：仅在需要时重绘