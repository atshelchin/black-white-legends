# 围棋游戏开发文档

使用 Rust 和 Bevy 引擎开发的模块化围棋游戏组件。

## 📚 文档列表

| 编号 | 文档 | 内容 | 适合读者 |
|------|------|------|----------|
| 01 | [快速开始](01_快速开始.md) | 环境配置、运行说明、快速集成 | 所有人 |
| 02 | [技术指南](02_技术指南.md) | Rust/Bevy核心概念、ECS架构 | 开发者 |
| 03 | [围棋规则](03_围棋规则.md) | 围棋基础、程序实现 | 围棋学习者 |
| 04 | [架构设计](04_架构设计.md) | 模块设计、设计模式、扩展点 | 架构师 |
| 05 | [问题与解决](05_问题与解决.md) | 常见问题、调试技巧 | 维护者 |

## 🎯 核心特性

- ✅ 标准围棋规则
- ✅ 响应式设计
- ✅ 模块化架构
- ✅ 事件驱动
- ✅ 易于集成

## 🚀 30秒上手

```bash
cargo run
```

- 点击落子
- R键重置
- 1/2/3切换棋盘大小

## 💡 技术亮点

- **ECS架构**：高性能游戏开发
- **事件系统**：解耦的组件通信
- **插件模式**：即插即用
- **响应式**：自适应屏幕

## 📦 作为库使用

```toml
[dependencies]
go-board-component = { path = "path/to/component" }
```

```rust
use go_board_component::prelude::*;

App::new()
    .add_plugins(GoBoardPlugin::default())
    .run();
```

## 🔧 开发

```bash
# 检查代码
cargo clippy

# 运行测试
cargo test

# 构建发布版
cargo build --release
```

## 📄 License

MIT