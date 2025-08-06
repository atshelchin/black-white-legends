// 示例应用 - 展示如何使用围棋棋盘组件
// Example app - demonstrating how to use the Go Board component

mod go_board_component;

use bevy::prelude::*;
use bevy::window::{MonitorSelection, PrimaryWindow, WindowResizeConstraints};
use go_board_component::plugin::GoBoardPluginBuilder;
use go_board_component::prelude::*;
use go_board_component::utils::CoordinateUtils;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Go Board Component Demo".to_string(),
                resolution: (1200.0, 1200.0).into(),
                resize_constraints: WindowResizeConstraints {
                    min_width: 600.0,
                    min_height: 600.0,
                    ..default()
                },
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.4, 0.4, 0.4)))
        // 使用围棋棋盘插件
        .add_plugins(
            GoBoardPluginBuilder::new()
                .with_board_size(BoardSize::Nineteen)
                .with_coordinates(true)
                .with_move_numbers(false)
                .with_captures(true)
                .with_ko_rule(true)
                .build(),
        )
        // 添加示例应用的系统
        .add_systems(Startup, (setup_camera, setup_ui, initial_board_draw))
        .add_systems(
            Update,
            (
                handle_keyboard_input,
                handle_window_resize,
                handle_mouse_hover,
                handle_mouse_click,
                update_turn_display,
            ),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d::default(), Name::new("Main Camera")));
}

fn setup_ui(mut commands: Commands) {
    // 帮助文字 - 使用英文避免字体问题
    commands.spawn((
        Text::new("Go Game - Black's Turn\\n1-3: Board sizes | C: Toggle coords | M: Toggle move numbers\\nF: Fullscreen | ESC: Exit | Click to place stones | R: Reset board"),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        HelpText,
    ));
}

#[derive(Component)]
struct HelpText;

#[derive(Component)]
struct HoverIndicator;

fn initial_board_draw(
    mut redraw_events: EventWriter<go_board_component::events::RedrawBoardEvent>,
) {
    redraw_events.write(go_board_component::events::RedrawBoardEvent);
}

fn handle_keyboard_input(
    _commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut config_events: EventWriter<go_board_component::events::UpdateBoardConfigEvent>,
    mut clear_events: EventWriter<go_board_component::events::ClearBoardEvent>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    current_config: Res<go_board_component::resources::CurrentGoBoardConfig>,
    mut board_state: ResMut<go_board_component::resources::BoardState>,
) {
    let mut new_config = None;
    let mut should_clear_board = false;

    if keyboard.just_pressed(KeyCode::Digit1) {
        new_config = Some(GoBoardConfig {
            board_size: BoardSize::Nine,
            ..current_config.0.clone()
        });
        board_state.board_size = BoardSize::Nine;
        should_clear_board = true;
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        new_config = Some(GoBoardConfig {
            board_size: BoardSize::Thirteen,
            ..current_config.0.clone()
        });
        board_state.board_size = BoardSize::Thirteen;
        should_clear_board = true;
    } else if keyboard.just_pressed(KeyCode::Digit3) {
        new_config = Some(GoBoardConfig {
            board_size: BoardSize::Nineteen,
            ..current_config.0.clone()
        });
        board_state.board_size = BoardSize::Nineteen;
        should_clear_board = true;
    } else if keyboard.just_pressed(KeyCode::KeyC) {
        new_config = Some(GoBoardConfig {
            show_coordinates: !current_config.0.show_coordinates,
            ..current_config.0.clone()
        });
    } else if keyboard.just_pressed(KeyCode::KeyM) {
        new_config = Some(GoBoardConfig {
            show_move_numbers: !current_config.0.show_move_numbers,
            ..current_config.0.clone()
        });
    } else if keyboard.just_pressed(KeyCode::KeyV) {
        new_config = Some(GoBoardConfig {
            use_3d_stones: !current_config.0.use_3d_stones,
            ..current_config.0.clone()
        });
    } else if keyboard.just_pressed(KeyCode::KeyR) {
        // 重置棋盘
        clear_events.write(go_board_component::events::ClearBoardEvent);
    } else if keyboard.just_pressed(KeyCode::KeyF) {
        if let Ok(mut window) = windows.single_mut() {
            window.mode = bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Current);
        }
    } else if keyboard.just_pressed(KeyCode::Escape) {
        if let Ok(mut window) = windows.single_mut() {
            window.mode = bevy::window::WindowMode::Windowed;
        }
    }

    if let Some(config) = new_config {
        if should_clear_board {
            clear_events.write(go_board_component::events::ClearBoardEvent);
        }
        config_events.write(go_board_component::events::UpdateBoardConfigEvent { config });
    }
}

fn handle_window_resize(
    mut resize_events: EventReader<bevy::window::WindowResized>,
    mut redraw_events: EventWriter<go_board_component::events::RedrawBoardEvent>,
) {
    if !resize_events.is_empty() {
        resize_events.clear();
        redraw_events.write(go_board_component::events::RedrawBoardEvent);
    }
}

fn handle_mouse_hover(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    hover_query: Query<Entity, With<HoverIndicator>>,
    current_config: Res<go_board_component::resources::CurrentGoBoardConfig>,
    board_state: Res<go_board_component::resources::BoardState>,
    current_turn: Res<go_board_component::resources::CurrentTurn>,
) {
    // 移除现有悬停指示器
    for entity in hover_query.iter() {
        commands.entity(entity).despawn();
    }

    if !current_config.0.enable_hover_indicator {
        return;
    }

    let Ok(window) = windows.single() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };

    let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    let window_size = window.resolution.width().min(window.resolution.height());

    if let Some((board_x, board_y)) = CoordinateUtils::world_to_board(
        world_position,
        current_config.0.board_size,
        window_size,
        current_config.0.adaptive_padding,
    ) {
        // 检查位置是否为空
        if board_state.get_stone(board_x, board_y).is_some() {
            return;
        }

        // 绘制悬停指示器
        let world_pos = CoordinateUtils::board_to_world(
            (board_x, board_y),
            current_config.0.board_size,
            window_size,
            current_config.0.adaptive_padding,
        );

        let padding = if current_config.0.adaptive_padding && window_size > 1400.0 {
            50.0
        } else {
            100.0
        };
        let board_background_size = window_size - padding;
        let cell_size =
            board_background_size / (current_config.0.board_size.get_value() as f32 + 1.0);
        let stone_radius = cell_size * 0.45;

        let hover_color = match current_turn.0 {
            StoneColor::Black => Color::srgba(0.1, 0.1, 0.1, 0.5),
            StoneColor::White => Color::srgba(0.95, 0.95, 0.95, 0.5),
        };

        commands.spawn((
            Mesh2d(meshes.add(Circle::new(stone_radius))),
            MeshMaterial2d(materials.add(hover_color)),
            Transform::from_translation(Vec3::new(world_pos.x, world_pos.y, 4.5)),
            HoverIndicator,
        ));
    }
}

fn handle_mouse_click(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut stone_events: EventWriter<go_board_component::events::PlaceStoneEvent>,
    mut current_turn: ResMut<go_board_component::resources::CurrentTurn>,
    current_config: Res<go_board_component::resources::CurrentGoBoardConfig>,
    board_state: Res<go_board_component::resources::BoardState>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = windows.single() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };

    let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    let window_size = window.resolution.width().min(window.resolution.height());

    if let Some((board_x, board_y)) = CoordinateUtils::world_to_board(
        world_position,
        current_config.0.board_size,
        window_size,
        current_config.0.adaptive_padding,
    ) {
        // 检查位置是否为空
        if board_state.get_stone(board_x, board_y).is_some() {
            return;
        }

        // 发送落子事件
        stone_events.write(go_board_component::events::PlaceStoneEvent {
            position: (board_x, board_y),
            color: current_turn.0,
        });

        // 切换回合
        current_turn.0 = current_turn.0.opposite();
    }
}

fn update_turn_display(
    current_turn: Res<go_board_component::resources::CurrentTurn>,
    mut query: Query<&mut Text, With<HelpText>>,
) {
    if current_turn.is_changed() {
        for mut text in query.iter_mut() {
            let turn_text = match current_turn.0 {
                StoneColor::Black => "Black's Turn",
                StoneColor::White => "White's Turn",
            };
            text.0 = format!(
                "Go Game - {}\\n1-3: Board sizes | C: Toggle coords | M: Toggle move numbers\\nF: Fullscreen | ESC: Exit | Click to place stones | R: Reset board",
                turn_text
            );
        }
    }
}
