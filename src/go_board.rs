use bevy::prelude::*;

// ===== Configuration =====

#[derive(Debug, Clone)]
pub struct GoBoardConfig {
    pub board_size: BoardSize,
    pub show_coordinates: bool,
    pub show_move_numbers: bool,
    pub use_3d_stones: bool,
    pub board_color: Color,
    pub line_color: Color,
    pub coordinate_color: Color,
    pub star_point_radius_ratio: f32,
    pub line_width_ratio: f32,
    pub adaptive_padding: bool,
}

impl Default for GoBoardConfig {
    fn default() -> Self {
        Self {
            board_size: BoardSize::Nineteen,
            show_coordinates: true,
            show_move_numbers: false,
            use_3d_stones: false,  // Default to pure color for cleaner look
            board_color: Color::srgb(0.82, 0.70, 0.55),
            line_color: Color::srgb(0.35, 0.30, 0.25),  // Even softer brown color for lines
            coordinate_color: Color::srgb(0.40, 0.35, 0.30),  // Matching softer coordinate color
            star_point_radius_ratio: 0.11,  // Slightly smaller star points
            line_width_ratio: 0.035,  // Even thinner lines
            adaptive_padding: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoardSize {
    Nine = 9,
    Thirteen = 13,
    Nineteen = 19,
}

impl BoardSize {
    pub fn get_value(&self) -> i32 {
        *self as i32
    }
    
    pub fn get_star_points(&self) -> Vec<(i32, i32)> {
        match self {
            BoardSize::Nine => vec![
                (2, 2), (2, 6),
                (4, 4),
                (6, 2), (6, 6),
            ],
            BoardSize::Thirteen => vec![
                (3, 3), (3, 9),
                (6, 6),
                (9, 3), (9, 9),
            ],
            BoardSize::Nineteen => vec![
                (3, 3), (3, 9), (3, 15),
                (9, 3), (9, 9), (9, 15),
                (15, 3), (15, 9), (15, 15),
            ],
        }
    }
}

// ===== Components =====

#[derive(Component)]
pub struct GoBoard;

#[derive(Component)]
pub struct BoardLine;

#[derive(Component)]
pub struct StarPoint;

#[derive(Component)]
pub struct CoordinateLabel;

#[derive(Component)]
pub struct GoBoardRoot;

#[derive(Component)]
pub struct Stone {
    pub color: StoneColor,
    pub position: (i32, i32),
    pub move_number: usize,
}

#[derive(Component)]
pub struct StoneShadow;

#[derive(Component)]
pub struct StoneHighlight;

#[derive(Component)]
pub struct MoveNumberLabel;

#[derive(Component)]
pub struct HoverIndicator;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoneColor {
    Black,
    White,
}

// ===== Resources =====

#[derive(Resource)]
pub struct CurrentGoBoardConfig(pub GoBoardConfig);

#[derive(Resource)]
pub struct CurrentTurn(pub StoneColor);

#[derive(Resource)]
pub struct BoardState {
    pub stones: [[Option<StoneColor>; 19]; 19],
    pub move_numbers: [[Option<usize>; 19]; 19],
    pub board_size: BoardSize,
    pub move_count: usize,
}

impl Default for BoardState {
    fn default() -> Self {
        Self {
            stones: [[None; 19]; 19],
            move_numbers: [[None; 19]; 19],
            board_size: BoardSize::Nineteen,
            move_count: 0,
        }
    }
}

// ===== Events =====

#[derive(Event)]
pub struct RedrawBoardEvent;

#[derive(Event)]
pub struct UpdateBoardConfigEvent {
    pub config: GoBoardConfig,
}

#[derive(Event)]
pub struct PlaceStoneEvent {
    pub position: (i32, i32),
    pub color: StoneColor,
}

// ===== Systems =====

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

pub fn handle_board_redraw(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut redraw_events: EventReader<RedrawBoardEvent>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    board_entities: Query<Entity, Or<(With<GoBoard>, With<BoardLine>, With<StarPoint>, With<CoordinateLabel>)>>,
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
    
    // Clear existing board
    for entity in board_entities.iter() {
        commands.entity(entity).despawn();
    }
    
    // Handle different redraw scenarios
    if config.is_changed() {
        if board_state.board_size != config.0.board_size {
            // Board size changed - clear all stone-related entities
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
        } else {
            // Other config changed (coordinates or move numbers) - clear labels if needed
            for entity in move_label_entities.iter() {
                commands.entity(entity).despawn();
            }
        }
    }
    
    // Redraw board
    if let Ok(window) = windows.single() {
        draw_board(&mut commands, &mut meshes, &mut materials, window, &config.0);
        // Only redraw stones if we didn't clear them
        if board_state.board_size == config.0.board_size {
            draw_stones(&mut commands, &mut meshes, &mut materials, window, &config.0, &board_state);
        }
    }
}

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
        
        // Check if position is valid
        let board_size = config.0.board_size.get_value();
        if x < 0 || x >= board_size || y < 0 || y >= board_size {
            continue;
        }
        
        // Check if position is empty
        if board_state.stones[x as usize][y as usize].is_some() {
            continue;
        }
        
        // Increment move count and place stone
        board_state.move_count += 1;
        let move_number = board_state.move_count;
        board_state.stones[x as usize][y as usize] = Some(event.color);
        board_state.move_numbers[x as usize][y as usize] = Some(move_number);
        
        // Draw the stone
        if let Ok(window) = windows.single() {
            draw_single_stone(
                &mut commands,
                &mut meshes,
                &mut materials,
                window,
                &config.0,
                event.position,
                event.color,
                move_number,
            );
        }
    }
}

pub fn draw_board(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    window: &Window,
    config: &GoBoardConfig,
) {
    let window_size = window.resolution.width().min(window.resolution.height());
    let board_size_value = config.board_size.get_value();
    
    // Calculate padding
    let padding = if config.adaptive_padding && window_size > 1400.0 { 
        50.0
    } else {
        100.0
    };
    
    let board_background_size = window_size - padding;
    let cell_size = board_background_size / (board_size_value as f32 + 1.0);
    let line_width = (cell_size * config.line_width_ratio).max(1.5);
    let star_point_radius = cell_size * config.star_point_radius_ratio;
    let board_size_pixels = (board_size_value - 1) as f32 * cell_size;
    
    // Draw board background
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(
            board_background_size,
            board_background_size,
        ))),
        MeshMaterial2d(materials.add(config.board_color)),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        GoBoard,
        GoBoardRoot,
    ));
    
    let half_board = board_size_pixels / 2.0;
    
    // Draw grid lines
    for i in 0..board_size_value {
        let offset = i as f32 * cell_size - half_board;
        
        // Vertical line
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(line_width, board_size_pixels))),
            MeshMaterial2d(materials.add(config.line_color)),
            Transform::from_translation(Vec3::new(offset, 0.0, 1.0)),
            BoardLine,
        ));
        
        // Horizontal line
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(board_size_pixels, line_width))),
            MeshMaterial2d(materials.add(config.line_color)),
            Transform::from_translation(Vec3::new(0.0, offset, 1.0)),
            BoardLine,
        ));
    }
    
    // Draw star points
    for (row, col) in config.board_size.get_star_points() {
        let x = col as f32 * cell_size - half_board;
        let y = row as f32 * cell_size - half_board;
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(star_point_radius))),
            MeshMaterial2d(materials.add(config.line_color)),
            Transform::from_translation(Vec3::new(x, y, 2.0)),
            StarPoint,
        ));
    }
    
    // Draw coordinates if enabled
    if config.show_coordinates {
        draw_coordinates(commands, board_size_value, cell_size, half_board, config.coordinate_color);
    }
}

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
                draw_single_stone(commands, meshes, materials, window, config, (x, y), color, move_number);
            }
        }
    }
}


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
    let board_size_value = config.board_size.get_value();
    
    let padding = if config.adaptive_padding && window_size > 1400.0 { 
        50.0
    } else {
        100.0
    };
    
    let board_background_size = window_size - padding;
    let cell_size = board_background_size / (board_size_value as f32 + 1.0);
    let board_size_pixels = (board_size_value - 1) as f32 * cell_size;
    let half_board = board_size_pixels / 2.0;
    
    // Calculate stone position
    let x = position.0 as f32 * cell_size - half_board;
    let y = half_board - position.1 as f32 * cell_size;
    
    // Stone size
    let stone_radius = cell_size * 0.47;
    
    // Simple pure color stones
    
    // Very subtle shadow under the stone
    let shadow_offset = cell_size * 0.03;
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(stone_radius * 1.04))),
        MeshMaterial2d(materials.add(Color::srgba(0.0, 0.0, 0.0, 0.05))),
        Transform::from_translation(Vec3::new(x + shadow_offset, y - shadow_offset, 3.8)),
        StoneShadow,
    ));
    
    // Main stone body
    let stone_color = match color {
        StoneColor::Black => Color::srgb(0.05, 0.05, 0.05),  // Pure black
        StoneColor::White => Color::srgb(0.95, 0.95, 0.94),  // Pure white (slightly off-white)
    };
    
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(stone_radius))),
        MeshMaterial2d(materials.add(stone_color)),
        Transform::from_translation(Vec3::new(x, y, 3.9)),
        Stone { color, position, move_number },
    ));
    
    // Single small highlight for minimal 3D effect
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
                x - highlight_offset,
                y + highlight_offset,
                4.0
            )),
            StoneHighlight,
        ));
    }
    
    // Add move number if enabled
    if config.show_move_numbers {
        let text_color = match color {
            StoneColor::Black => Color::srgb(0.95, 0.95, 0.95),
            StoneColor::White => Color::srgb(0.05, 0.05, 0.05),
        };
        
        let font_size = (cell_size * 0.32).max(10.0).min(30.0);
        
        commands.spawn((
            Text2d::new(move_number.to_string()),
            TextFont {
                font_size,
                ..default()
            },
            TextColor(text_color),
            Transform::from_translation(Vec3::new(x, y, 4.7)),
            MoveNumberLabel,
        ));
    }
}

fn draw_coordinates(
    commands: &mut Commands,
    board_size: i32,
    cell_size: f32,
    half_board: f32,
    color: Color,
) {
    let label_offset = cell_size * 0.7;
    let font_size = (cell_size * 0.35).max(14.0).min(40.0);
    
    // Horizontal coordinates (A-T, skipping I)
    let letters = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T'];
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
    
    // Vertical coordinates (1-19)
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

// ===== Plugin =====

pub struct GoBoardPlugin {
    pub initial_config: GoBoardConfig,
}

impl Default for GoBoardPlugin {
    fn default() -> Self {
        Self {
            initial_config: GoBoardConfig::default(),
        }
    }
}

impl Plugin for GoBoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentGoBoardConfig(self.initial_config.clone()))
            .insert_resource(CurrentTurn(StoneColor::Black))
            .insert_resource(BoardState::default())
            .add_event::<RedrawBoardEvent>()
            .add_event::<UpdateBoardConfigEvent>()
            .add_event::<PlaceStoneEvent>()
            .add_systems(Update, (
                handle_config_update,
                handle_place_stone,
                handle_board_redraw,
            ).chain());
    }
}