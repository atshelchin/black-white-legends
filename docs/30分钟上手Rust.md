# 30分钟上手 Rust - 围棋项目必备知识

> 本文档专为具有 JavaScript/TypeScript/PHP 背景的开发者设计，通过类比熟悉的概念，快速掌握理解本项目所需的 Rust 核心知识。

## 1. 项目结构与包管理

### Rust vs JavaScript 对比

| JavaScript/Node.js | Rust | 说明 |
|-------------------|------|------|
| `package.json` | `Cargo.toml` | 项目配置文件 |
| `node_modules/` | `target/` | 依赖和构建产物 |
| `npm/pnpm` | `cargo` | 包管理器 |
| `index.js` | `main.rs` | 入口文件 |
| `require/import` | `use` | 导入模块 |

### Cargo.toml 示例
```toml
[package]
name = "black-white-legends"
version = "0.1.0"
edition = "2021"  # 类似于 TypeScript 的 target

[dependencies]
bevy = "0.16.1"   # 类似于 package.json 的 dependencies
```

## 2. 变量与类型系统

### 变量声明
```rust
// JavaScript
let x = 5;           // 可变
const y = 10;        // 不可变

// Rust
let x = 5;           // 默认不可变（类似 const）
let mut y = 10;      // 可变（需要 mut 关键字）
```

### 类型标注
```rust
// TypeScript
let name: string = "Alice";
let age: number = 30;
let items: number[] = [1, 2, 3];

// Rust
let name: &str = "Alice";        // 字符串切片
let age: i32 = 30;               // 32位整数
let items: Vec<i32> = vec![1, 2, 3];  // 动态数组
```

### 常用类型对照

| JavaScript/TypeScript | Rust | 说明 |
|----------------------|------|------|
| `string` | `String` / `&str` | String 拥有所有权，&str 是借用 |
| `number` | `i32`, `f32`, `usize` 等 | 需要明确数字类型 |
| `boolean` | `bool` | 布尔值 |
| `any[]` | `Vec<T>` | 动态数组 |
| `[number, string]` | `(i32, String)` | 元组 |
| `{ x: number }` | `struct Point { x: i32 }` | 结构体 |
| `null/undefined` | `Option<T>` | 可空类型 |

## 3. 函数定义

### 基本函数
```rust
// JavaScript
function add(a, b) {
    return a + b;
}

// TypeScript
function add(a: number, b: number): number {
    return a + b;
}

// Rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // 注意：没有 return 和分号，表达式即返回值
}
```

### 方法定义
```rust
// TypeScript class
class Point {
    x: number;
    y: number;
    
    distance(): number {
        return Math.sqrt(this.x * this.x + this.y * this.y);
    }
}

// Rust struct + impl
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn distance(&self) -> f32 {  // &self 类似 this
        (self.x * self.x + self.y * self.y).sqrt()
    }
}
```

## 4. 所有权系统（最重要的概念）

### 核心规则
1. 每个值都有一个所有者
2. 同一时间只能有一个所有者
3. 所有者离开作用域时，值被清理

### 借用（References）
```rust
// JavaScript - 引用传递很自然
function process(data) {
    console.log(data.length);
}
let arr = [1, 2, 3];
process(arr);  // arr 仍然可用

// Rust - 需要显式借用
fn process(data: &Vec<i32>) {  // & 表示借用
    println!("{}", data.len());
}
let arr = vec![1, 2, 3];
process(&arr);  // &arr 传递引用
// arr 仍然可用
```

### 可变借用
```rust
// JavaScript
function modify(arr) {
    arr.push(4);
}

// Rust
fn modify(arr: &mut Vec<i32>) {  // &mut 表示可变借用
    arr.push(4);
}
let mut arr = vec![1, 2, 3];
modify(&mut arr);
```

## 5. 结构体与枚举

### 结构体（类似 TypeScript interface）
```rust
// TypeScript
interface User {
    name: string;
    age: number;
}

// Rust
struct User {
    name: String,
    age: i32,
}

// 创建实例
let user = User {
    name: String::from("Alice"),
    age: 30,
};
```

### 枚举（更强大的 union type）
```rust
// TypeScript
type Color = 'Black' | 'White';
type Option<T> = T | null;

// Rust
enum Color {
    Black,
    White,
}

enum Option<T> {
    Some(T),    // 有值
    None,       // 无值
}
```

## 6. 模式匹配（Pattern Matching）

```rust
// JavaScript switch
switch (color) {
    case 'black':
        console.log("Black stone");
        break;
    case 'white':
        console.log("White stone");
        break;
}

// Rust match（更强大）
match color {
    Color::Black => println!("Black stone"),
    Color::White => println!("White stone"),
}

// 处理 Option
match some_value {
    Some(value) => println!("Got: {}", value),
    None => println!("No value"),
}

// if let 简化语法
if let Some(value) = some_value {
    println!("Got: {}", value);
}
```

## 7. 模块系统

### 文件结构
```
src/
├── main.rs                 # 主入口
├── lib.rs                  # 库入口（可选）
└── go_board_component/     # 模块目录
    ├── mod.rs              # 模块定义（类似 index.js）
    ├── plugin.rs           # 子模块
    └── systems.rs          # 子模块
```

### 模块导入导出
```rust
// JavaScript ES6
// export
export function hello() {}
export default class MyClass {}

// import
import { hello } from './module';
import MyClass from './module';

// Rust
// 在 mod.rs 中导出
pub fn hello() {}              // pub 表示公开
pub struct MyStruct {}

// 导入
use crate::go_board_component::hello;  // crate 表示当前包
use super::parent_module;              // super 表示父模块
```

## 8. 错误处理

### Result 类型（类似 Promise）
```rust
// TypeScript async/await
async function readFile(): Promise<string> {
    try {
        return await fs.readFile('file.txt');
    } catch (error) {
        throw error;
    }
}

// Rust Result
fn read_file() -> Result<String, std::io::Error> {
    std::fs::read_to_string("file.txt")  // 返回 Result
}

// 使用 ? 操作符（类似 await）
fn process() -> Result<(), Error> {
    let content = read_file()?;  // ? 自动处理错误
    println!("{}", content);
    Ok(())
}
```

## 9. 迭代器与闭包

### 迭代器链式调用（类似 JavaScript 数组方法）
```rust
// JavaScript
const result = [1, 2, 3]
    .map(x => x * 2)
    .filter(x => x > 2)
    .reduce((sum, x) => sum + x, 0);

// Rust
let result: i32 = vec![1, 2, 3]
    .iter()
    .map(|x| x * 2)
    .filter(|x| *x > 2)
    .sum();
```

### 闭包
```rust
// JavaScript
const add = (x) => (y) => x + y;
const add5 = add(5);

// Rust
let add = |x| move |y| x + y;
let add5 = add(5);
```

## 10. Bevy 特定概念

### ECS 架构
```rust
// Component - 数据
#[derive(Component)]
struct Position { x: f32, y: f32 }

// System - 逻辑（函数）
fn movement_system(mut query: Query<&mut Position>) {
    for mut pos in query.iter_mut() {
        pos.x += 1.0;
    }
}

// Resource - 全局状态
#[derive(Resource)]
struct GameState { score: i32 }

// 注册到 App
app.add_systems(Update, movement_system);
```

### 事件系统
```rust
// 定义事件
#[derive(Event)]
struct ClickEvent { x: f32, y: f32 }

// 发送事件
fn send_events(mut events: EventWriter<ClickEvent>) {
    events.send(ClickEvent { x: 100.0, y: 200.0 });
}

// 接收事件
fn handle_events(mut events: EventReader<ClickEvent>) {
    for event in events.read() {
        println!("Click at {}, {}", event.x, event.y);
    }
}
```

## 11. 项目中的实际应用

### 本项目核心结构
```rust
// 1. 配置结构体
pub struct GoBoardConfig {
    pub board_size: BoardSize,
    pub screen_width: f32,
    pub screen_height: f32,
}

// 2. 插件模式
pub struct GoBoardPlugin {
    initial_config: GoBoardConfig,
}

impl Plugin for GoBoardPlugin {
    fn build(&self, app: &mut App) {
        // 注册资源、事件、系统
    }
}

// 3. 使用插件
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GoBoardPlugin::new(config))
        .run();
}
```

## 12. 常见陷阱与解决

### 字符串处理
```rust
// 错误：不能直接比较 String 和 &str
let s = String::from("hello");
if s == "hello" { }  // 编译错误

// 正确：使用 &s 或 as_str()
if &s == "hello" { }
if s.as_str() == "hello" { }
```

### 循环中的可变引用
```rust
// JavaScript - 没问题
for (let item of items) {
    items.push(newItem);  // 修改数组
}

// Rust - 需要注意借用规则
let mut items = vec![1, 2, 3];
// 错误：不能同时借用和修改
for item in &items {
    items.push(4);  // 编译错误
}

// 正确：使用索引或分离读写
for i in 0..items.len() {
    if items[i] == 2 {
        // 处理逻辑
    }
}
items.push(4);  // 循环后修改
```

## 快速上手步骤

1. **安装 Rust**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **运行项目**
   ```bash
   cargo run
   ```

3. **常用命令**
   ```bash
   cargo build    # 构建
   cargo check    # 快速检查
   cargo fmt      # 格式化
   cargo clippy   # 代码质量检查
   ```

4. **调试技巧**
   - 使用 `println!("{:?}", variable)` 打印调试
   - 使用 `dbg!(expression)` 宏快速调试
   - VSCode + rust-analyzer 提供完整 IDE 体验

## 总结

掌握以上概念后，你应该能够：
1. 理解项目的基本结构和依赖管理
2. 读懂 Rust 的基本语法和类型系统
3. 理解所有权和借用的核心概念
4. 使用 Bevy 的 ECS 架构编写游戏逻辑
5. 处理事件和管理游戏状态

记住：Rust 的编译器是你的朋友，它会在编译时捕获大部分错误，虽然初期可能觉得严格，但这正是 Rust 保证内存安全和并发安全的方式。

**下一步**：打开 `src/main.rs`，对照本文档理解代码结构，然后尝试修改一些参数（如棋盘大小、颜色等）来熟悉项目。