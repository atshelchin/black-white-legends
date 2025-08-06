use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// 围棋棋盘配置
/// Go board configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoBoardConfig {
    /// 棋盘大小
    pub board_size: BoardSize,
    /// 是否显示坐标
    pub show_coordinates: bool,
    /// 是否显示手数
    pub show_move_numbers: bool,
    /// 是否使用3D效果棋子
    pub use_3d_stones: bool,
    /// 棋盘背景颜色
    pub board_color: Color,
    /// 棋盘线条颜色
    pub line_color: Color,
    /// 坐标文字颜色
    pub coordinate_color: Color,
    /// 星位点半径比例
    pub star_point_radius_ratio: f32,
    /// 线条宽度比例
    pub line_width_ratio: f32,
    /// 自适应边距
    pub adaptive_padding: bool,
    /// 启用悬停提示
    pub enable_hover_indicator: bool,
    /// 启用落子音效
    pub enable_sound: bool,
    /// 启用捕获规则
    pub enable_captures: bool,
    /// 启用打劫规则
    pub enable_ko_rule: bool,
}

impl Default for GoBoardConfig {
    fn default() -> Self {
        Self {
            board_size: BoardSize::Nineteen,
            show_coordinates: true,
            show_move_numbers: false,
            use_3d_stones: false,
            board_color: Color::srgb(0.82, 0.70, 0.55),
            line_color: Color::srgb(0.35, 0.30, 0.25),
            coordinate_color: Color::srgb(0.40, 0.35, 0.30),
            star_point_radius_ratio: 0.11,
            line_width_ratio: 0.035,
            adaptive_padding: true,
            enable_hover_indicator: true,
            enable_sound: false,
            enable_captures: true,
            enable_ko_rule: true,
        }
    }
}

/// 棋盘大小枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BoardSize {
    Nine = 9,
    Thirteen = 13,
    Nineteen = 19,
}

impl BoardSize {
    pub fn get_value(&self) -> i32 {
        *self as i32
    }
    
    /// 获取星位点坐标
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

/// 配置构建器模式
pub struct GoBoardConfigBuilder {
    config: GoBoardConfig,
}

impl GoBoardConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: GoBoardConfig::default(),
        }
    }
    
    pub fn board_size(mut self, size: BoardSize) -> Self {
        self.config.board_size = size;
        self
    }
    
    pub fn show_coordinates(mut self, show: bool) -> Self {
        self.config.show_coordinates = show;
        self
    }
    
    pub fn show_move_numbers(mut self, show: bool) -> Self {
        self.config.show_move_numbers = show;
        self
    }
    
    pub fn board_color(mut self, color: Color) -> Self {
        self.config.board_color = color;
        self
    }
    
    pub fn line_color(mut self, color: Color) -> Self {
        self.config.line_color = color;
        self
    }
    
    pub fn enable_captures(mut self, enable: bool) -> Self {
        self.config.enable_captures = enable;
        self
    }
    
    pub fn enable_ko_rule(mut self, enable: bool) -> Self {
        self.config.enable_ko_rule = enable;
        self
    }
    
    pub fn build(self) -> GoBoardConfig {
        self.config
    }
}