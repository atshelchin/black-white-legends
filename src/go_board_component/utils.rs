use super::config::BoardSize;
use bevy::prelude::*;

/// 坐标转换工具
pub struct CoordinateUtils;

impl CoordinateUtils {
    /// 将世界坐标转换为棋盘坐标
    pub fn world_to_board(
        world_pos: Vec2,
        board_size: BoardSize,
        window_size: f32,
        adaptive_padding: bool,
    ) -> Option<(i32, i32)> {
        let board_size_value = board_size.get_value();

        let padding = if adaptive_padding && window_size > 1400.0 {
            50.0
        } else {
            100.0
        };

        let board_background_size = window_size - padding;
        let cell_size = board_background_size / (board_size_value as f32 + 1.0);
        let board_size_pixels = (board_size_value - 1) as f32 * cell_size;
        let half_board = board_size_pixels / 2.0;

        let board_x = ((world_pos.x + half_board) / cell_size).round() as i32;
        let board_y = ((half_board - world_pos.y) / cell_size).round() as i32;

        if board_x >= 0 && board_x < board_size_value && board_y >= 0 && board_y < board_size_value
        {
            Some((board_x, board_y))
        } else {
            None
        }
    }

    /// 将棋盘坐标转换为世界坐标
    pub fn board_to_world(
        board_pos: (i32, i32),
        board_size: BoardSize,
        window_size: f32,
        adaptive_padding: bool,
    ) -> Vec3 {
        let board_size_value = board_size.get_value();

        let padding = if adaptive_padding && window_size > 1400.0 {
            50.0
        } else {
            100.0
        };

        let board_background_size = window_size - padding;
        let cell_size = board_background_size / (board_size_value as f32 + 1.0);
        let board_size_pixels = (board_size_value - 1) as f32 * cell_size;
        let half_board = board_size_pixels / 2.0;

        let x = board_pos.0 as f32 * cell_size - half_board;
        let y = half_board - board_pos.1 as f32 * cell_size;

        Vec3::new(x, y, 0.0)
    }

    /// 将棋盘坐标转换为SGF格式坐标
    pub fn board_to_sgf(x: i32, y: i32) -> String {
        let col = (b'a' + x as u8) as char;
        let row = (b'a' + y as u8) as char;
        format!("{}{}", col, row)
    }

    /// 将SGF格式坐标转换为棋盘坐标
    pub fn sgf_to_board(sgf: &str) -> Option<(i32, i32)> {
        if sgf.len() != 2 {
            return None;
        }

        let chars: Vec<char> = sgf.chars().collect();
        let x = (chars[0] as u8 - b'a') as i32;
        let y = (chars[1] as u8 - b'a') as i32;

        if x >= 0 && x < 19 && y >= 0 && y < 19 {
            Some((x, y))
        } else {
            None
        }
    }

    /// 将棋盘坐标转换为人类可读格式（如 "A1", "K10"）
    pub fn board_to_human(x: i32, y: i32, board_size: BoardSize) -> String {
        let letters = [
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
            'S', 'T',
        ];
        if x < letters.len() as i32 {
            let col = letters[x as usize];
            let row = board_size.get_value() - y;
            format!("{}{}", col, row)
        } else {
            format!("({},{})", x, y)
        }
    }
}

/// 棋盘渲染计算工具
pub struct RenderUtils;

impl RenderUtils {
    /// 计算棋盘尺寸参数
    pub fn calculate_board_metrics(
        window_size: f32,
        board_size: BoardSize,
        adaptive_padding: bool,
    ) -> BoardMetrics {
        let board_size_value = board_size.get_value();

        let padding = if adaptive_padding && window_size > 1400.0 {
            50.0
        } else {
            100.0
        };

        let board_background_size = window_size - padding;
        let cell_size = board_background_size / (board_size_value as f32 + 1.0);
        let board_size_pixels = (board_size_value - 1) as f32 * cell_size;
        let half_board = board_size_pixels / 2.0;

        BoardMetrics {
            padding,
            board_background_size,
            cell_size,
            board_size_pixels,
            half_board,
        }
    }
}

/// 棋盘度量参数
pub struct BoardMetrics {
    pub padding: f32,
    pub board_background_size: f32,
    pub cell_size: f32,
    pub board_size_pixels: f32,
    pub half_board: f32,
}
