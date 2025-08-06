# Bevy 连接任天堂手柄完整指南

## 1. 概述

Bevy 通过 `gilrs` (Game Input Library for Rust) 提供了对游戏手柄的支持，包括：
- Nintendo Switch Pro Controller
- Nintendo Switch Joy-Con (单个或组合)
- 其他主流手柄 (Xbox, PlayStation, 8BitDo等)

## 2. 基础设置

### 2.1 添加依赖

```toml
# Cargo.toml
[dependencies]
bevy = { version = "0.15", features = ["default"] }
# Bevy 默认已包含 bevy_gilrs 插件
```

### 2.2 初始化手柄插件

```rust
use bevy::prelude::*;
use bevy::input::gamepad::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // 已包含 GilrsPlugin
        .add_systems(Startup, setup)
        .add_systems(Update, (
            gamepad_connections,
            gamepad_input,
            handle_nintendo_features,
        ))
        .run();
}
```

## 3. 手柄连接管理

### 3.1 检测手柄连接

```rust
/// 监听手柄连接和断开事件
fn gamepad_connections(
    mut connection_events: EventReader<GamepadConnectionEvent>,
    mut gamepads: ResMut<Gamepads>,
) {
    for event in connection_events.read() {
        match &event.connection {
            GamepadConnection::Connected(info) => {
                println!(
                    "手柄 {:?} 已连接: {} ({})",
                    event.gamepad,
                    info.name,
                    identify_controller_type(&info.name)
                );
                
                // 特别处理任天堂手柄
                if is_nintendo_controller(&info.name) {
                    setup_nintendo_controller(event.gamepad, &info.name);
                }
            }
            GamepadConnection::Disconnected => {
                println!("手柄 {:?} 已断开", event.gamepad);
            }
        }
    }
}

/// 识别手柄类型
fn identify_controller_type(name: &str) -> &str {
    if name.contains("Pro Controller") {
        "Switch Pro手柄"
    } else if name.contains("Joy-Con (L)") {
        "左Joy-Con"
    } else if name.contains("Joy-Con (R)") {
        "右Joy-Con"  
    } else if name.contains("Joy-Con") {
        "Joy-Con组合"
    } else if name.contains("Xbox") {
        "Xbox手柄"
    } else if name.contains("DualShock") || name.contains("DualSense") {
        "PlayStation手柄"
    } else {
        "通用手柄"
    }
}

fn is_nintendo_controller(name: &str) -> bool {
    name.contains("Joy-Con") || name.contains("Pro Controller")
}
```

### 3.2 手柄资源管理

```rust
#[derive(Resource)]
pub struct GamepadManager {
    /// 当前活跃的手柄
    pub active_gamepads: Vec<Gamepad>,
    /// 手柄与玩家的映射
    pub player_gamepads: HashMap<usize, Gamepad>,
    /// 手柄配置
    pub configs: HashMap<Gamepad, ControllerConfig>,
}

#[derive(Clone)]
pub struct ControllerConfig {
    pub controller_type: ControllerType,
    pub dead_zone: f32,
    pub vibration_enabled: bool,
    pub button_mapping: ButtonMapping,
}

#[derive(Clone, Debug)]
pub enum ControllerType {
    SwitchPro,
    JoyConLeft,
    JoyConRight,
    JoyConPair,
    Xbox,
    PlayStation,
    Generic,
}
```

## 4. 任天堂手柄按键映射

### 4.1 Switch Pro Controller 布局

```rust
/// Switch Pro 手柄按键映射
pub struct SwitchProMapping;

impl SwitchProMapping {
    /// 获取标准化的按键映射
    pub fn get_button_mapping() -> HashMap<GamepadButtonType, String> {
        let mut map = HashMap::new();
        
        // 面板按键 (ABXY - 注意任天堂的布局与Xbox相反)
        map.insert(GamepadButtonType::South, "B".to_string());  // 下
        map.insert(GamepadButtonType::East, "A".to_string());   // 右
        map.insert(GamepadButtonType::West, "Y".to_string());   // 左
        map.insert(GamepadButtonType::North, "X".to_string());  // 上
        
        // 肩键
        map.insert(GamepadButtonType::LeftTrigger, "L".to_string());
        map.insert(GamepadButtonType::LeftTrigger2, "ZL".to_string());
        map.insert(GamepadButtonType::RightTrigger, "R".to_string());
        map.insert(GamepadButtonType::RightTrigger2, "ZR".to_string());
        
        // 特殊按键
        map.insert(GamepadButtonType::Select, "-".to_string());  // Minus
        map.insert(GamepadButtonType::Start, "+".to_string());   // Plus
        map.insert(GamepadButtonType::Mode, "Home".to_string());
        map.insert(GamepadButtonType::LeftThumb, "L3".to_string());
        map.insert(GamepadButtonType::RightThumb, "R3".to_string());
        
        // 方向键
        map.insert(GamepadButtonType::DPadUp, "↑".to_string());
        map.insert(GamepadButtonType::DPadDown, "↓".to_string());
        map.insert(GamepadButtonType::DPadLeft, "←".to_string());
        map.insert(GamepadButtonType::DPadRight, "→".to_string());
        
        map
    }
}
```

### 4.2 Joy-Con 特殊处理

```rust
/// Joy-Con 配对管理
#[derive(Resource)]
pub struct JoyConPairManager {
    pub left_joycon: Option<Gamepad>,
    pub right_joycon: Option<Gamepad>,
    pub paired: bool,
}

impl JoyConPairManager {
    pub fn try_pair(&mut self, gamepad: Gamepad, is_left: bool) {
        if is_left {
            self.left_joycon = Some(gamepad);
        } else {
            self.right_joycon = Some(gamepad);
        }
        
        self.paired = self.left_joycon.is_some() && self.right_joycon.is_some();
        
        if self.paired {
            println!("Joy-Con 配对成功！");
        }
    }
    
    pub fn get_combined_input(&self, button_inputs: &Res<ButtonInput<GamepadButton>>) -> JoyConInput {
        let mut input = JoyConInput::default();
        
        // 左 Joy-Con
        if let Some(left) = self.left_joycon {
            // 左摇杆
            input.left_stick = self.get_stick_input(left, GamepadAxisType::LeftStickX, GamepadAxisType::LeftStickY);
            
            // 左侧按键
            input.l = button_inputs.pressed(GamepadButton::new(left, GamepadButtonType::LeftTrigger));
            input.zl = button_inputs.pressed(GamepadButton::new(left, GamepadButtonType::LeftTrigger2));
            input.minus = button_inputs.pressed(GamepadButton::new(left, GamepadButtonType::Select));
            
            // 方向键
            input.dpad_up = button_inputs.pressed(GamepadButton::new(left, GamepadButtonType::DPadUp));
            input.dpad_down = button_inputs.pressed(GamepadButton::new(left, GamepadButtonType::DPadDown));
            input.dpad_left = button_inputs.pressed(GamepadButton::new(left, GamepadButtonType::DPadLeft));
            input.dpad_right = button_inputs.pressed(GamepadButton::new(left, GamepadButtonType::DPadRight));
        }
        
        // 右 Joy-Con
        if let Some(right) = self.right_joycon {
            // 右摇杆 (在单独使用时作为主摇杆)
            input.right_stick = self.get_stick_input(right, GamepadAxisType::RightStickX, GamepadAxisType::RightStickY);
            
            // ABXY 按键
            input.a = button_inputs.pressed(GamepadButton::new(right, GamepadButtonType::East));
            input.b = button_inputs.pressed(GamepadButton::new(right, GamepadButtonType::South));
            input.x = button_inputs.pressed(GamepadButton::new(right, GamepadButtonType::North));
            input.y = button_inputs.pressed(GamepadButton::new(right, GamepadButtonType::West));
            
            // 右侧按键
            input.r = button_inputs.pressed(GamepadButton::new(right, GamepadButtonType::RightTrigger));
            input.zr = button_inputs.pressed(GamepadButton::new(right, GamepadButtonType::RightTrigger2));
            input.plus = button_inputs.pressed(GamepadButton::new(right, GamepadButtonType::Start));
        }
        
        input
    }
}

#[derive(Default, Debug)]
pub struct JoyConInput {
    pub left_stick: Vec2,
    pub right_stick: Vec2,
    pub a: bool,
    pub b: bool,
    pub x: bool,
    pub y: bool,
    pub l: bool,
    pub r: bool,
    pub zl: bool,
    pub zr: bool,
    pub plus: bool,
    pub minus: bool,
    pub dpad_up: bool,
    pub dpad_down: bool,
    pub dpad_left: bool,
    pub dpad_right: bool,
}
```

## 5. 输入处理

### 5.1 基础输入读取

```rust
fn gamepad_input(
    gamepads: Res<Gamepads>,
    button_inputs: Res<ButtonInput<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
) {
    for gamepad in gamepads.iter() {
        // 读取摇杆
        let left_stick_x = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
            .unwrap_or(0.0);
        let left_stick_y = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
            .unwrap_or(0.0);
            
        // 应用死区
        let left_stick = apply_deadzone(Vec2::new(left_stick_x, left_stick_y), 0.15);
        
        if left_stick.length() > 0.0 {
            println!("手柄 {:?} 左摇杆: {:?}", gamepad, left_stick);
        }
        
        // 检查按键
        if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::South)) {
            println!("按下 B/○ 键");
        }
        
        if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::East)) {
            println!("按下 A/× 键");
        }
        
        // 扳机键（模拟值）
        let left_trigger = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftZ))
            .unwrap_or(0.0);
            
        if left_trigger > 0.1 {
            println!("左扳机: {:.2}", left_trigger);
        }
    }
}

/// 应用死区
fn apply_deadzone(input: Vec2, deadzone: f32) -> Vec2 {
    let length = input.length();
    if length < deadzone {
        Vec2::ZERO
    } else {
        // 重新映射到 0-1 范围
        let normalized = (length - deadzone) / (1.0 - deadzone);
        input.normalize() * normalized
    }
}
```

### 5.2 高级输入处理

```rust
/// 统一的输入系统，支持多种手柄
#[derive(Component)]
pub struct PlayerController {
    pub gamepad: Option<Gamepad>,
    pub keyboard_controls: KeyboardControls,
    pub input_buffer: InputBuffer,
}

#[derive(Default)]
pub struct InputBuffer {
    pub movement: Vec2,
    pub aim: Vec2,
    pub action_a: bool,
    pub action_b: bool,
    pub action_x: bool,
    pub action_y: bool,
    pub trigger_left: f32,
    pub trigger_right: f32,
}

fn unified_input_system(
    mut players: Query<&mut PlayerController>,
    gamepads: Res<Gamepads>,
    button_inputs: Res<ButtonInput<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for mut controller in players.iter_mut() {
        controller.input_buffer = InputBuffer::default();
        
        // 优先使用手柄输入
        if let Some(gamepad) = controller.gamepad {
            if gamepads.contains(gamepad) {
                // 移动
                let lx = axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX)).unwrap_or(0.0);
                let ly = axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY)).unwrap_or(0.0);
                controller.input_buffer.movement = apply_deadzone(Vec2::new(lx, ly), 0.15);
                
                // 瞄准
                let rx = axes.get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickX)).unwrap_or(0.0);
                let ry = axes.get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickY)).unwrap_or(0.0);
                controller.input_buffer.aim = apply_deadzone(Vec2::new(rx, ry), 0.15);
                
                // 按键
                controller.input_buffer.action_a = button_inputs.pressed(
                    GamepadButton::new(gamepad, GamepadButtonType::East)
                );
                controller.input_buffer.action_b = button_inputs.pressed(
                    GamepadButton::new(gamepad, GamepadButtonType::South)
                );
                
                // 扳机
                controller.input_buffer.trigger_left = axes
                    .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftZ))
                    .unwrap_or(0.0)
                    .max(0.0);
                controller.input_buffer.trigger_right = axes
                    .get(GamepadAxis::new(gamepad, GamepadAxisType::RightZ))
                    .unwrap_or(0.0)
                    .max(0.0);
                
                continue;
            }
        }
        
        // 回退到键盘输入
        let mut movement = Vec2::ZERO;
        if keyboard.pressed(controller.keyboard_controls.up) { movement.y += 1.0; }
        if keyboard.pressed(controller.keyboard_controls.down) { movement.y -= 1.0; }
        if keyboard.pressed(controller.keyboard_controls.left) { movement.x -= 1.0; }
        if keyboard.pressed(controller.keyboard_controls.right) { movement.x += 1.0; }
        
        if movement.length() > 0.0 {
            controller.input_buffer.movement = movement.normalize();
        }
        
        controller.input_buffer.action_a = keyboard.pressed(controller.keyboard_controls.action_a);
        controller.input_buffer.action_b = keyboard.pressed(controller.keyboard_controls.action_b);
    }
}
```

## 6. 任天堂特有功能

### 6.1 HD振动（Rumble）

```rust
use bevy::input::gamepad::{GamepadRumbleIntensity, GamepadRumbleRequest};

/// HD振动管理器
#[derive(Resource)]
pub struct RumbleManager {
    pub patterns: HashMap<String, RumblePattern>,
}

#[derive(Clone)]
pub struct RumblePattern {
    pub duration: f32,
    pub low_frequency: f32,   // 低频马达 (0.0-1.0)
    pub high_frequency: f32,  // 高频马达 (0.0-1.0)
    pub pattern_type: RumbleType,
}

#[derive(Clone)]
pub enum RumbleType {
    Constant,
    Pulse { interval: f32 },
    Wave { frequency: f32 },
    Custom(Vec<(f32, f32, f32)>), // (时间, 低频, 高频)
}

fn setup_rumble_patterns(mut rumble_manager: ResMut<RumbleManager>) {
    // 击中反馈
    rumble_manager.patterns.insert(
        "hit".to_string(),
        RumblePattern {
            duration: 0.1,
            low_frequency: 0.8,
            high_frequency: 0.3,
            pattern_type: RumbleType::Constant,
        },
    );
    
    // 充能
    rumble_manager.patterns.insert(
        "charge".to_string(),
        RumblePattern {
            duration: 1.0,
            low_frequency: 0.2,
            high_frequency: 0.1,
            pattern_type: RumbleType::Wave { frequency: 5.0 },
        },
    );
    
    // 爆炸
    rumble_manager.patterns.insert(
        "explosion".to_string(),
        RumblePattern {
            duration: 0.5,
            low_frequency: 1.0,
            high_frequency: 0.5,
            pattern_type: RumbleType::Pulse { interval: 0.05 },
        },
    );
}

fn play_rumble(
    mut rumble_events: EventWriter<GamepadRumbleRequest>,
    rumble_manager: Res<RumbleManager>,
    trigger_events: EventReader<RumbleTriggerEvent>,
) {
    for event in trigger_events.read() {
        if let Some(pattern) = rumble_manager.patterns.get(&event.pattern_name) {
            rumble_events.send(GamepadRumbleRequest::Add {
                gamepad: event.gamepad,
                duration: Duration::from_secs_f32(pattern.duration),
                intensity: GamepadRumbleIntensity {
                    strong_motor: pattern.low_frequency,
                    weak_motor: pattern.high_frequency,
                },
            });
        }
    }
}

#[derive(Event)]
struct RumbleTriggerEvent {
    gamepad: Gamepad,
    pattern_name: String,
}
```

### 6.2 陀螺仪和加速度计（Motion Controls）

```rust
/// 运动控制数据
#[derive(Component, Default)]
pub struct MotionControls {
    pub gyroscope: Vec3,      // 角速度
    pub accelerometer: Vec3,   // 加速度
    pub orientation: Quat,     // 当前朝向
    pub calibrated: bool,
}

/// 注意：Bevy 原生不支持 Switch 的运动控制
/// 需要使用额外的库如 hidapi 直接访问
fn handle_motion_controls(
    mut motion_controllers: Query<(&PlayerController, &mut MotionControls)>,
) {
    // 这里需要集成 hidapi 或其他低级库来读取 Switch 手柄的运动数据
    // 示例代码：
    
    for (controller, mut motion) in motion_controllers.iter_mut() {
        if let Some(gamepad) = controller.gamepad {
            // 模拟运动控制（实际需要从硬件读取）
            // let raw_gyro = read_gyroscope_from_switch(gamepad);
            // let raw_accel = read_accelerometer_from_switch(gamepad);
            
            // 应用校准
            if !motion.calibrated {
                calibrate_motion_controls(&mut motion);
            }
            
            // 更新朝向
            // motion.orientation = calculate_orientation(motion.gyroscope, motion.accelerometer);
        }
    }
}

fn calibrate_motion_controls(motion: &mut MotionControls) {
    // 校准逻辑
    motion.calibrated = true;
    println!("运动控制已校准");
}
```

### 6.3 Amiibo/NFC 支持

```rust
/// Amiibo 功能（需要额外的 NFC 库支持）
#[derive(Resource)]
pub struct AmiiboManager {
    pub nfc_enabled: bool,
    pub last_scanned: Option<AmiiboData>,
}

#[derive(Clone, Debug)]
pub struct AmiiboData {
    pub character: String,
    pub series: String,
    pub data: Vec<u8>,
}

// 注意：实际的 Amiibo 支持需要特殊的硬件访问权限
// 在 PC 上通常无法直接使用
```

## 7. 实战示例：围棋游戏的手柄支持

```rust
use bevy::prelude::*;
use bevy::input::gamepad::*;

/// 为围棋游戏添加手柄支持
fn setup_go_game_controls(mut commands: Commands) {
    // 创建手柄控制组件
    commands.spawn((
        GoGameController::default(),
        CursorPosition { x: 9, y: 9 },
    ));
}

#[derive(Component, Default)]
pub struct GoGameController {
    pub gamepad: Option<Gamepad>,
    pub cursor_speed: f32,
    pub place_stone_button: GamepadButtonType,
    pub cancel_button: GamepadButtonType,
    pub rapid_move_enabled: bool,
}

#[derive(Component)]
pub struct CursorPosition {
    pub x: i32,
    pub y: i32,
}

impl Default for GoGameController {
    fn default() -> Self {
        Self {
            gamepad: None,
            cursor_speed: 5.0,
            place_stone_button: GamepadButtonType::East,  // A键
            cancel_button: GamepadButtonType::South,      // B键
            rapid_move_enabled: false,
        }
    }
}

fn go_game_gamepad_input(
    mut controllers: Query<(&mut GoGameController, &mut CursorPosition)>,
    gamepads: Res<Gamepads>,
    button_inputs: Res<ButtonInput<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    time: Res<Time>,
    mut place_stone_events: EventWriter<PlaceStoneEvent>,
) {
    for (mut controller, mut cursor) in controllers.iter_mut() {
        // 自动分配第一个连接的手柄
        if controller.gamepad.is_none() {
            if let Some(gamepad) = gamepads.iter().next() {
                controller.gamepad = Some(gamepad);
                println!("手柄已连接到围棋游戏");
            } else {
                continue;
            }
        }
        
        let gamepad = controller.gamepad.unwrap();
        
        // 左摇杆控制光标
        let lx = axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX)).unwrap_or(0.0);
        let ly = axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY)).unwrap_or(0.0);
        let movement = apply_deadzone(Vec2::new(lx, ly), 0.2);
        
        // 方向键精确移动
        if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadUp)) {
            cursor.y = (cursor.y - 1).max(0);
        }
        if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadDown)) {
            cursor.y = (cursor.y + 1).min(18);
        }
        if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadLeft)) {
            cursor.x = (cursor.x - 1).max(0);
        }
        if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadRight)) {
            cursor.x = (cursor.x + 1).min(18);
        }
        
        // 摇杆平滑移动
        if movement.length() > 0.0 {
            // 实现平滑移动逻辑
            let move_speed = if controller.rapid_move_enabled { 10.0 } else { 5.0 };
            // ... 移动逻辑
        }
        
        // A键落子
        if button_inputs.just_pressed(GamepadButton::new(gamepad, controller.place_stone_button)) {
            place_stone_events.send(PlaceStoneEvent {
                x: cursor.x,
                y: cursor.y,
                color: StoneColor::Black, // 根据当前回合决定
            });
            
            // 振动反馈
            // play_rumble("place_stone", gamepad);
        }
        
        // B键撤销
        if button_inputs.just_pressed(GamepadButton::new(gamepad, controller.cancel_button)) {
            // 撤销逻辑
            println!("撤销上一步");
        }
        
        // 肩键快速移动
        controller.rapid_move_enabled = button_inputs.pressed(
            GamepadButton::new(gamepad, GamepadButtonType::LeftTrigger)
        );
        
        // ZR键显示菜单
        if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::RightTrigger2)) {
            // 显示游戏菜单
        }
    }
}
```

## 8. 手柄配置UI

```rust
use bevy_egui::{egui, EguiContexts};

fn gamepad_settings_ui(
    mut contexts: EguiContexts,
    mut gamepad_configs: ResMut<HashMap<Gamepad, ControllerConfig>>,
    gamepads: Res<Gamepads>,
) {
    egui::Window::new("手柄设置").show(contexts.ctx_mut(), |ui| {
        for gamepad in gamepads.iter() {
            ui.collapsing(format!("手柄 {}", gamepad.id), |ui| {
                if let Some(config) = gamepad_configs.get_mut(&gamepad) {
                    // 死区设置
                    ui.horizontal(|ui| {
                        ui.label("摇杆死区:");
                        ui.add(egui::Slider::new(&mut config.dead_zone, 0.0..=0.5));
                    });
                    
                    // 振动开关
                    ui.checkbox(&mut config.vibration_enabled, "启用振动");
                    
                    // 按键映射
                    ui.separator();
                    ui.label("按键映射:");
                    
                    ui.horizontal(|ui| {
                        ui.label("跳跃:");
                        if ui.button(format!("{:?}", config.button_mapping.jump)).clicked() {
                            // 开始按键映射监听
                        }
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("攻击:");
                        if ui.button(format!("{:?}", config.button_mapping.attack)).clicked() {
                            // 开始按键映射监听
                        }
                    });
                    
                    // 测试振动
                    if ui.button("测试振动").clicked() {
                        // 触发测试振动
                    }
                }
            });
        }
        
        ui.separator();
        
        if ui.button("恢复默认设置").clicked() {
            for config in gamepad_configs.values_mut() {
                *config = ControllerConfig::default();
            }
        }
    });
}
```

## 9. 常见问题与解决方案

### 9.1 连接问题

```rust
/// 手柄连接故障排除
fn troubleshoot_connection() {
    // Windows
    #[cfg(target_os = "windows")]
    {
        println!("Windows 手柄连接提示:");
        println!("1. 确保蓝牙已开启");
        println!("2. 在设置中添加蓝牙设备");
        println!("3. 长按手柄的配对按钮");
        println!("4. 可能需要安装 ViGEmBus 驱动");
    }
    
    // macOS
    #[cfg(target_os = "macos")]
    {
        println!("macOS 手柄连接提示:");
        println!("1. 打开系统偏好设置 > 蓝牙");
        println!("2. 长按手柄的配对按钮");
        println!("3. 点击连接");
        println!("4. 可能需要安装 360Controller 驱动");
    }
    
    // Linux
    #[cfg(target_os = "linux")]
    {
        println!("Linux 手柄连接提示:");
        println!("1. 安装 joycond 服务");
        println!("2. 运行: sudo systemctl start joycond");
        println!("3. 使用 bluetoothctl 配对");
        println!("4. 可能需要添加 udev 规则");
    }
}
```

### 9.2 按键映射差异

```rust
/// 处理不同平台的按键映射差异
fn normalize_button_mapping(
    controller_type: ControllerType,
    button: GamepadButtonType,
) -> GamepadButtonType {
    match controller_type {
        ControllerType::SwitchPro => {
            // Switch 的 A/B 和 Xbox 相反
            match button {
                GamepadButtonType::South => GamepadButtonType::East,
                GamepadButtonType::East => GamepadButtonType::South,
                _ => button,
            }
        }
        _ => button,
    }
}
```

## 10. 完整示例：手柄测试工具

```rust
use bevy::prelude::*;
use bevy::input::gamepad::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GamepadTestState::default())
        .add_systems(Startup, setup_test_ui)
        .add_systems(Update, (
            monitor_gamepad_events,
            display_gamepad_state,
            test_rumble_patterns,
        ))
        .run();
}

#[derive(Resource, Default)]
struct GamepadTestState {
    pub active_gamepad: Option<Gamepad>,
    pub button_states: HashMap<GamepadButtonType, bool>,
    pub axis_values: HashMap<GamepadAxisType, f32>,
    pub last_event: String,
}

fn monitor_gamepad_events(
    mut state: ResMut<GamepadTestState>,
    mut connection_events: EventReader<GamepadConnectionEvent>,
    mut button_events: EventReader<GamepadButtonChangedEvent>,
    mut axis_events: EventReader<GamepadAxisChangedEvent>,
) {
    // 监控连接事件
    for event in connection_events.read() {
        match &event.connection {
            GamepadConnection::Connected(info) => {
                state.active_gamepad = Some(event.gamepad);
                state.last_event = format!("已连接: {}", info.name);
            }
            GamepadConnection::Disconnected => {
                if state.active_gamepad == Some(event.gamepad) {
                    state.active_gamepad = None;
                }
                state.last_event = "手柄已断开".to_string();
            }
        }
    }
    
    // 监控按键事件
    for event in button_events.read() {
        if Some(event.gamepad) == state.active_gamepad {
            state.button_states.insert(event.button_type, event.value > 0.5);
            state.last_event = format!("按键 {:?}: {}", event.button_type, event.value);
        }
    }
    
    // 监控摇杆事件
    for event in axis_events.read() {
        if Some(event.gamepad) == state.active_gamepad {
            state.axis_values.insert(event.axis_type, event.value);
            state.last_event = format!("轴 {:?}: {:.2}", event.axis_type, event.value);
        }
    }
}

fn display_gamepad_state(
    state: Res<GamepadTestState>,
    mut query: Query<&mut Text, With<GamepadDisplay>>,
) {
    for mut text in query.iter_mut() {
        let mut output = String::new();
        
        output.push_str(&format!("活动手柄: {:?}\n", state.active_gamepad));
        output.push_str(&format!("最后事件: {}\n\n", state.last_event));
        
        output.push_str("按键状态:\n");
        for (button, pressed) in &state.button_states {
            if *pressed {
                output.push_str(&format!("  {:?}: 按下\n", button));
            }
        }
        
        output.push_str("\n摇杆/扳机:\n");
        for (axis, value) in &state.axis_values {
            if value.abs() > 0.01 {
                output.push_str(&format!("  {:?}: {:.2}\n", axis, value));
            }
        }
        
        text.0 = output;
    }
}

#[derive(Component)]
struct GamepadDisplay;

fn setup_test_ui(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    
    commands.spawn((
        Text::new("手柄测试工具\n等待连接..."),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        GamepadDisplay,
    ));
}
```

## 总结

Bevy 对任天堂手柄的支持已经相当完善，通过 `bevy_gilrs` 可以：
- ✅ 自动检测和连接 Switch Pro 手柄
- ✅ 支持 Joy-Con（单个或配对）
- ✅ 读取所有按键和摇杆输入
- ✅ 支持振动反馈
- ⚠️ 运动控制需要额外的库支持
- ⚠️ Amiibo/NFC 功能在 PC 上受限

关键是要处理好按键映射的差异，并为玩家提供自定义配置的选项。