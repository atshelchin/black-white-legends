use super::{
    components::*,
    config::*,
    events::*,
    resources::*,
    rules::GoBoardRules,
    utils::{CoordinateUtils, RenderUtils},
};
use bevy::prelude::*;

/// 处理配置更新
pub fn handle_config_update(
    mut config_events: EventReader<UpdateBoardConfigEvent>,
    mut current_config: ResMut<CurrentGoBoardConfig>,
    mut redraw_events: EventWriter<RedrawBoardEvent>,
) {
    for event in config_events.read() {
        current_config.0 = event.config.clone();
        redraw_events.write(RedrawBoardEvent);
    }
}

/// 处理棋盘重绘
pub fn handle_board_redraw(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut redraw_events: EventReader<RedrawBoardEvent>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    board_entities: Query<
        Entity,
        Or<(
            With<GoBoard>,
            With<BoardLine>,
            With<StarPoint>,
            With<CoordinateLabel>,
        )>,
    >,
    stone_entities: Query<Entity, With<Stone>>,
    shadow_entities: Query<Entity, With<StoneShadow>>,
    highlight_entities: Query<Entity, With<StoneHighlight>>,
    move_label_entities: Query<Entity, With<MoveNumberLabel>>,
    config: Res<CurrentGoBoardConfig>,
    board_state: Res<BoardState>,
) {
    if redraw_events.is_empty() {
        return;
    }

    redraw_events.clear();

    // 清除现有棋盘
    for entity in board_entities.iter() {
        commands.entity(entity).despawn();
    }

    // 清除所有棋子相关实体（每次重绘都清除，确保位置正确）
    for entity in stone_entities.iter() {
        commands.entity(entity).despawn();
    }
    for entity in shadow_entities.iter() {
        commands.entity(entity).despawn();
    }
    for entity in highlight_entities.iter() {
        commands.entity(entity).despawn();
    }
    for entity in move_label_entities.iter() {
        commands.entity(entity).despawn();
    }

    // 重绘棋盘和棋子
    if let Ok(window) = windows.single() {
        draw_board(
            &mut commands,
            &mut meshes,
            &mut materials,
            window,
            &config.0,
        );
        // 总是重绘棋子以确保位置正确
        draw_stones(
            &mut commands,
            &mut meshes,
            &mut materials,
            window,
            &config.0,
            &board_state,
        );
    }
}

/// 处理落子事件
pub fn handle_place_stone(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut stone_events: EventReader<PlaceStoneEvent>,
    mut board_state: ResMut<BoardState>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    config: Res<CurrentGoBoardConfig>,
) {
    for event in stone_events.read() {
        let (x, y) = event.position;

        // 使用规则引擎检查是否合法
        if config.0.enable_captures || config.0.enable_ko_rule {
            if !GoBoardRules::is_valid_move(&board_state, x, y, event.color) {
                continue;
            }
        } else {
            // 简单检查
            if x < 0
                || x >= config.0.board_size.get_value()
                || y < 0
                || y >= config.0.board_size.get_value()
            {
                continue;
            }
            if board_state.stones[x as usize][y as usize].is_some() {
                continue;
            }
        }

        // 放置棋子
        if board_state.place_stone(x, y, event.color) {
            // 处理提子
            if config.0.enable_captures {
                let _captured = GoBoardRules::capture_stones(&mut board_state, x, y, event.color);
                // TODO: 移除被提棋子的实体
            }

            // 绘制棋子
            if let Ok(window) = windows.single() {
                draw_single_stone(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    window,
                    &config.0,
                    event.position,
                    event.color,
                    board_state.move_count,
                );
            }
        }
    }
}

/// 处理清空棋盘事件
pub fn handle_clear_board(
    mut commands: Commands,
    mut clear_events: EventReader<ClearBoardEvent>,
    mut board_state: ResMut<BoardState>,
    mut current_turn: ResMut<CurrentTurn>,
    stone_entities: Query<Entity, With<Stone>>,
    shadow_entities: Query<Entity, With<StoneShadow>>,
    highlight_entities: Query<Entity, With<StoneHighlight>>,
    move_label_entities: Query<Entity, With<MoveNumberLabel>>,
) {
    for _ in clear_events.read() {
        // 清除所有棋子实体
        for entity in stone_entities.iter() {
            commands.entity(entity).despawn();
        }
        for entity in shadow_entities.iter() {
            commands.entity(entity).despawn();
        }
        for entity in highlight_entities.iter() {
            commands.entity(entity).despawn();
        }
        for entity in move_label_entities.iter() {
            commands.entity(entity).despawn();
        }

        // 重置棋盘状态
        board_state.clear();
        current_turn.0 = StoneColor::Black;
    }
}

/// 绘制棋盘
fn draw_board(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    window: &Window,
    config: &GoBoardConfig,
) {
    let window_size = window.resolution.width().min(window.resolution.height());
    let metrics = RenderUtils::calculate_board_metrics(
        window_size,
        config.board_size,
        config.adaptive_padding,
    );

    // 绘制棋盘背景
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(
            metrics.board_background_size,
            metrics.board_background_size,
        ))),
        MeshMaterial2d(materials.add(config.board_color)),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        GoBoard,
        GoBoardRoot,
    ));

    let board_size_value = config.board_size.get_value();
    let line_width = (metrics.cell_size * config.line_width_ratio).max(1.5);
    let star_point_radius = metrics.cell_size * config.star_point_radius_ratio;

    // 绘制网格线
    for i in 0..board_size_value {
        let offset = i as f32 * metrics.cell_size - metrics.half_board;

        // 垂直线
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(line_width, metrics.board_size_pixels))),
            MeshMaterial2d(materials.add(config.line_color)),
            Transform::from_translation(Vec3::new(offset, 0.0, 1.0)),
            BoardLine,
        ));

        // 水平线
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(metrics.board_size_pixels, line_width))),
            MeshMaterial2d(materials.add(config.line_color)),
            Transform::from_translation(Vec3::new(0.0, offset, 1.0)),
            BoardLine,
        ));
    }

    // 绘制星位点
    for (row, col) in config.board_size.get_star_points() {
        let x = col as f32 * metrics.cell_size - metrics.half_board;
        let y = row as f32 * metrics.cell_size - metrics.half_board;
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(star_point_radius))),
            MeshMaterial2d(materials.add(config.line_color)),
            Transform::from_translation(Vec3::new(x, y, 2.0)),
            StarPoint,
        ));
    }

    // 绘制坐标（如果启用）
    if config.show_coordinates {
        draw_coordinates(
            commands,
            board_size_value,
            metrics.cell_size,
            metrics.half_board,
            config.coordinate_color,
        );
    }
}

/// 绘制所有棋子
fn draw_stones(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    window: &Window,
    config: &GoBoardConfig,
    board_state: &BoardState,
) {
    let board_size_value = config.board_size.get_value();

    for x in 0..board_size_value {
        for y in 0..board_size_value {
            if let Some(color) = board_state.stones[x as usize][y as usize] {
                let move_number = board_state.move_numbers[x as usize][y as usize].unwrap_or(0);
                draw_single_stone(
                    commands,
                    meshes,
                    materials,
                    window,
                    config,
                    (x, y),
                    color,
                    move_number,
                );
            }
        }
    }
}

/// 绘制单个棋子
fn draw_single_stone(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    window: &Window,
    config: &GoBoardConfig,
    position: (i32, i32),
    color: StoneColor,
    move_number: usize,
) {
    let window_size = window.resolution.width().min(window.resolution.height());
    let metrics = RenderUtils::calculate_board_metrics(
        window_size,
        config.board_size,
        config.adaptive_padding,
    );

    // 计算棋子位置
    let world_pos = CoordinateUtils::board_to_world(
        position,
        config.board_size,
        window_size,
        config.adaptive_padding,
    );

    // 棋子大小
    let stone_radius = metrics.cell_size * 0.47;

    // 简单的纯色棋子

    // 非常微妙的阴影
    let shadow_offset = metrics.cell_size * 0.03;
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(stone_radius * 1.04))),
        MeshMaterial2d(materials.add(Color::srgba(0.0, 0.0, 0.0, 0.05))),
        Transform::from_translation(Vec3::new(
            world_pos.x + shadow_offset,
            world_pos.y - shadow_offset,
            3.8,
        )),
        StoneShadow,
    ));

    // 主棋子体
    let stone_color = match color {
        StoneColor::Black => Color::srgb(0.05, 0.05, 0.05), // 纯黑色
        StoneColor::White => Color::srgb(0.95, 0.95, 0.94), // 纯白色（略微偏灰）
    };

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(stone_radius))),
        MeshMaterial2d(materials.add(stone_color)),
        Transform::from_translation(Vec3::new(world_pos.x, world_pos.y, 3.9)),
        Stone {
            color,
            position,
            move_number,
        },
    ));

    // 单个小高光以获得最小的3D效果
    if config.use_3d_stones {
        let highlight_radius = stone_radius * 0.2;
        let highlight_offset = stone_radius * 0.25;
        let highlight_color = match color {
            StoneColor::Black => Color::srgba(0.25, 0.25, 0.27, 0.2),
            StoneColor::White => Color::srgba(1.0, 1.0, 1.0, 0.25),
        };

        commands.spawn((
            Mesh2d(meshes.add(Circle::new(highlight_radius))),
            MeshMaterial2d(materials.add(highlight_color)),
            Transform::from_translation(Vec3::new(
                world_pos.x - highlight_offset,
                world_pos.y + highlight_offset,
                4.0,
            )),
            StoneHighlight,
        ));
    }

    // 添加手数（如果启用）
    if config.show_move_numbers {
        let text_color = match color {
            StoneColor::Black => Color::srgb(0.95, 0.95, 0.95),
            StoneColor::White => Color::srgb(0.05, 0.05, 0.05),
        };

        let font_size = (metrics.cell_size * 0.32).max(10.0).min(30.0);

        commands.spawn((
            Text2d::new(move_number.to_string()),
            TextFont {
                font_size,
                ..default()
            },
            TextColor(text_color),
            Transform::from_translation(Vec3::new(world_pos.x, world_pos.y, 4.7)),
            MoveNumberLabel,
        ));
    }
}

/// 绘制坐标
fn draw_coordinates(
    commands: &mut Commands,
    board_size: i32,
    cell_size: f32,
    half_board: f32,
    color: Color,
) {
    let label_offset = cell_size * 0.7;
    let font_size = (cell_size * 0.35).max(14.0).min(40.0);

    // 水平坐标（A-T，跳过I）
    let letters = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
        'T',
    ];
    for i in 0..board_size {
        if i < letters.len() as i32 {
            let x = i as f32 * cell_size - half_board;
            commands.spawn((
                Text2d::new(letters[i as usize].to_string()),
                TextFont {
                    font_size,
                    ..default()
                },
                TextColor(color),
                Transform::from_translation(Vec3::new(x, half_board + label_offset, 3.0)),
                CoordinateLabel,
            ));
        }
    }

    // 垂直坐标（1-19）
    for i in 0..board_size {
        let y = half_board - i as f32 * cell_size;
        let number = (i + 1).to_string();
        commands.spawn((
            Text2d::new(number),
            TextFont {
                font_size,
                ..default()
            },
            TextColor(color),
            Transform::from_translation(Vec3::new(-half_board - label_offset, y, 3.0)),
            CoordinateLabel,
        ));
    }
}
