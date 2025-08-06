# Bevy UI系统快速指南

## Bevy内置UI组件

Bevy提供了完整的UI系统，基于Flexbox布局，你**不需要从零开始绘制**！

### 核心UI组件

#### 1. Node (基础容器)
```rust
commands.spawn(Node {
    width: Val::Px(200.0),
    height: Val::Px(100.0),
    ..default()
});
```

#### 2. Button (按钮)
```rust
commands.spawn((
    Button,
    Node {
        width: Val::Px(150.0),
        height: Val::Px(65.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    },
    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
));
```

#### 3. Text (文本)
```rust
commands.spawn((
    Text::new("Hello Bevy!"),
    TextFont {
        font_size: 30.0,
        ..default()
    },
    TextColor(Color::WHITE),
));
```

#### 4. ImageNode (图片)
```rust
commands.spawn((
    ImageNode::new(asset_server.load("icon.png")),
    Node {
        width: Val::Px(64.0),
        height: Val::Px(64.0),
        ..default()
    },
));
```

## 完整UI示例

### 创建一个游戏菜单
```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_menu)
        .add_systems(Update, button_system)
        .run();
}

fn setup_menu(mut commands: Commands) {
    // 相机
    commands.spawn(Camera2d);
    
    // 根容器 - 全屏
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
    ))
    .with_children(|parent| {
        // 菜单面板
        parent.spawn((
            Node {
                width: Val::Px(400.0),
                height: Val::Px(500.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            BorderColor(Color::WHITE),
            BorderRadius::all(Val::Px(10.0)),
        ))
        .with_children(|panel| {
            // 标题
            panel.spawn((
                Text::new("游戏菜单"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            
            // 开始按钮
            spawn_button(panel, "开始游戏", ButtonAction::Start);
            
            // 设置按钮
            spawn_button(panel, "设置", ButtonAction::Settings);
            
            // 退出按钮
            spawn_button(panel, "退出", ButtonAction::Quit);
        });
    });
}

#[derive(Component)]
enum ButtonAction {
    Start,
    Settings,
    Quit,
}

fn spawn_button(parent: &mut ChildBuilder, text: &str, action: ButtonAction) {
    parent.spawn((
        Button,
        Node {
            width: Val::Px(200.0),
            height: Val::Px(65.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
        BorderColor(Color::WHITE),
        action,
    ))
    .with_children(|button| {
        button.spawn((
            Text::new(text),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
    });
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgb(0.5, 0.5, 0.5));
                match action {
                    ButtonAction::Start => println!("开始游戏！"),
                    ButtonAction::Settings => println!("打开设置"),
                    ButtonAction::Quit => println!("退出游戏"),
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.4, 0.4, 0.4));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.3, 0.3, 0.3));
            }
        }
    }
}
```

## 布局系统 (Flexbox)

Bevy使用CSS Flexbox布局模型：

### 主要属性
```rust
Node {
    // 布局方向
    flex_direction: FlexDirection::Row,  // Row | Column
    
    // 主轴对齐
    justify_content: JustifyContent::Center,  // Start | End | Center | SpaceBetween | SpaceAround
    
    // 交叉轴对齐
    align_items: AlignItems::Center,  // Start | End | Center | Stretch
    
    // 尺寸
    width: Val::Px(100.0),   // Px | Percent | Auto
    height: Val::Percent(50.0),
    
    // 内边距
    padding: UiRect::all(Val::Px(10.0)),
    
    // 外边距
    margin: UiRect::all(Val::Px(5.0)),
    
    // 边框
    border: UiRect::all(Val::Px(2.0)),
    
    // 定位
    position_type: PositionType::Absolute,  // Relative | Absolute
    left: Val::Px(10.0),
    top: Val::Px(10.0),
    
    ..default()
}
```

## 常用UI模式

### 1. HUD (游戏信息显示)
```rust
fn setup_hud(mut commands: Commands) {
    // 顶部信息栏
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            right: Val::Px(10.0),
            height: Val::Px(50.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
    ))
    .with_children(|parent| {
        // 分数
        parent.spawn((
            Text::new("Score: 0"),
            TextFont { font_size: 24.0, ..default() },
            TextColor(Color::WHITE),
        ));
        
        // 生命值
        parent.spawn((
            Text::new("Lives: 3"),
            TextFont { font_size: 24.0, ..default() },
            TextColor(Color::WHITE),
        ));
    });
}
```

### 2. 对话框
```rust
fn spawn_dialog(mut commands: Commands) {
    // 半透明背景
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
    ))
    .with_children(|parent| {
        // 对话框
        parent.spawn((
            Node {
                width: Val::Px(400.0),
                height: Val::Auto,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                row_gap: Val::Px(10.0),
                ..default()
            },
            BackgroundColor(Color::WHITE),
            BorderRadius::all(Val::Px(10.0)),
        ))
        .with_children(|dialog| {
            dialog.spawn((
                Text::new("确定要退出吗？"),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::BLACK),
            ));
            
            // 按钮容器
            dialog.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(10.0),
                    ..default()
                },
            ))
            .with_children(|buttons| {
                // 确定/取消按钮
                spawn_button(buttons, "确定", ButtonAction::Confirm);
                spawn_button(buttons, "取消", ButtonAction::Cancel);
            });
        });
    });
}
```

### 3. 进度条
```rust
fn create_progress_bar(parent: &mut ChildBuilder, progress: f32) {
    // 进度条背景
    parent.spawn((
        Node {
            width: Val::Px(200.0),
            height: Val::Px(20.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
        BorderColor(Color::WHITE),
        BorderRadius::all(Val::Px(10.0)),
    ))
    .with_children(|bar| {
        // 进度条填充
        bar.spawn((
            Node {
                width: Val::Percent(progress * 100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.0, 0.8, 0.0)),
            BorderRadius::all(Val::Px(10.0)),
        ));
    });
}
```

## 第三方UI库

如果需要更高级的UI组件，可以使用：

### 1. bevy_egui
即时模式GUI，适合工具和调试界面：
```toml
[dependencies]
bevy_egui = "0.31"
```

```rust
use bevy_egui::{egui, EguiContexts, EguiPlugin};

fn ui_system(mut contexts: EguiContexts) {
    egui::Window::new("设置").show(contexts.ctx_mut(), |ui| {
        ui.heading("游戏设置");
        ui.slider(&mut volume, 0.0..=1.0, "音量");
        if ui.button("应用").clicked() {
            // 应用设置
        }
    });
}
```

### 2. bevy_ui_navigation
提供键盘/手柄导航支持：
```toml
[dependencies]
bevy_ui_navigation = "0.38"
```

## 总结

Bevy提供了：
1. **完整的UI组件**：Node、Button、Text、Image等
2. **Flexbox布局**：灵活的响应式布局
3. **样式系统**：背景色、边框、圆角等
4. **交互系统**：鼠标悬停、点击检测
5. **可扩展性**：可集成egui等第三方库

你不需要从零开始绘制UI，Bevy的UI系统已经相当完善！