mod go_board;

use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResizeConstraints, MonitorSelection};
use go_board::{GoBoardPlugin, GoBoardConfig, BoardSize, UpdateBoardConfigEvent, RedrawBoardEvent};

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
        // Add the Go Board plugin with custom configuration
        .add_plugins(GoBoardPlugin {
            initial_config: GoBoardConfig {
                board_size: BoardSize::Nineteen,
                show_coordinates: true,
                ..default()
            }
        })
        .add_systems(Startup, (setup_camera, setup_ui, initial_board_draw))
        .add_systems(Update, (handle_keyboard_input, handle_window_resize))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn setup_ui(mut commands: Commands) {
    // UI Camera
    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            ..default()
        },
        IsDefaultUiCamera,
    ));
    
    // Help text
    commands.spawn((
        Text::new("Go Board Component Demo\n1-3: Board sizes | C: Toggle coords | F: Fullscreen | ESC: Exit fullscreen"),
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
    ));
}

fn initial_board_draw(
    mut redraw_events: EventWriter<RedrawBoardEvent>,
) {
    redraw_events.send(RedrawBoardEvent);
}

fn handle_keyboard_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut config_events: EventWriter<UpdateBoardConfigEvent>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    current_config: Res<go_board::CurrentGoBoardConfig>,
) {
    let mut new_config = None;
    
    if keyboard.just_pressed(KeyCode::Digit1) {
        new_config = Some(GoBoardConfig {
            board_size: BoardSize::Nine,
            ..current_config.0.clone()
        });
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        new_config = Some(GoBoardConfig {
            board_size: BoardSize::Thirteen,
            ..current_config.0.clone()
        });
    } else if keyboard.just_pressed(KeyCode::Digit3) {
        new_config = Some(GoBoardConfig {
            board_size: BoardSize::Nineteen,
            ..current_config.0.clone()
        });
    } else if keyboard.just_pressed(KeyCode::KeyC) {
        new_config = Some(GoBoardConfig {
            show_coordinates: !current_config.0.show_coordinates,
            ..current_config.0.clone()
        });
    } else if keyboard.just_pressed(KeyCode::KeyF) {
        if let Ok(mut window) = windows.get_single_mut() {
            window.mode = bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Current);
        }
    } else if keyboard.just_pressed(KeyCode::Escape) {
        if let Ok(mut window) = windows.get_single_mut() {
            window.mode = bevy::window::WindowMode::Windowed;
        }
    }
    
    if let Some(config) = new_config {
        config_events.send(UpdateBoardConfigEvent { config });
    }
}

fn handle_window_resize(
    mut resize_events: EventReader<bevy::window::WindowResized>,
    mut redraw_events: EventWriter<RedrawBoardEvent>,
) {
    if !resize_events.is_empty() {
        resize_events.clear();
        redraw_events.send(RedrawBoardEvent);
    }
}