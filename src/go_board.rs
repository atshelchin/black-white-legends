use bevy::prelude::*;

// ===== Configuration =====

#[derive(Debug, Clone)]
pub struct GoBoardConfig {
    pub board_size: BoardSize,
    pub show_coordinates: bool,
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
            board_color: Color::srgb(0.82, 0.70, 0.55),
            line_color: Color::srgb(0.1, 0.1, 0.1),
            coordinate_color: Color::srgb(0.2, 0.2, 0.2),
            star_point_radius_ratio: 0.125,
            line_width_ratio: 0.06,
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

// ===== Resources =====

#[derive(Resource)]
pub struct CurrentGoBoardConfig(pub GoBoardConfig);

// ===== Events =====

#[derive(Event)]
pub struct RedrawBoardEvent;

#[derive(Event)]
pub struct UpdateBoardConfigEvent {
    pub config: GoBoardConfig,
}

// ===== Systems =====

pub fn handle_config_update(
    mut commands: Commands,
    mut config_events: EventReader<UpdateBoardConfigEvent>,
    mut current_config: ResMut<CurrentGoBoardConfig>,
    mut redraw_events: EventWriter<RedrawBoardEvent>,
) {
    for event in config_events.read() {
        current_config.0 = event.config.clone();
        redraw_events.send(RedrawBoardEvent);
    }
}

pub fn handle_board_redraw(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut redraw_events: EventReader<RedrawBoardEvent>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    board_entities: Query<Entity, Or<(With<GoBoard>, With<BoardLine>, With<StarPoint>, With<CoordinateLabel>)>>,
    config: Res<CurrentGoBoardConfig>,
) {
    if redraw_events.is_empty() {
        return;
    }
    
    redraw_events.clear();
    
    // Clear existing board
    for entity in board_entities.iter() {
        commands.entity(entity).despawn();
    }
    
    // Redraw board
    if let Ok(window) = windows.get_single() {
        draw_board(&mut commands, &mut meshes, &mut materials, window, &config.0);
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
            .add_event::<RedrawBoardEvent>()
            .add_event::<UpdateBoardConfigEvent>()
            .add_systems(Update, (
                handle_config_update,
                handle_board_redraw,
            ).chain());
    }
}