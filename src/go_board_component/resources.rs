use bevy::prelude::*;
use super::components::StoneColor;
use super::config::{GoBoardConfig, BoardSize};

/// 当前棋盘配置资源
#[derive(Resource)]
pub struct CurrentGoBoardConfig(pub GoBoardConfig);

/// 当前回合
#[derive(Resource)]
pub struct CurrentTurn(pub StoneColor);

/// 棋盘状态
#[derive(Resource)]
pub struct BoardState {
    pub stones: [[Option<StoneColor>; 19]; 19],
    pub move_numbers: [[Option<usize>; 19]; 19],
    pub board_size: BoardSize,
    pub move_count: usize,
    pub captured_black: usize,
    pub captured_white: usize,
    pub ko_position: Option<(i32, i32)>,
    pub last_move: Option<(i32, i32)>,
}

impl Default for BoardState {
    fn default() -> Self {
        Self {
            stones: [[None; 19]; 19],
            move_numbers: [[None; 19]; 19],
            board_size: BoardSize::Nineteen,
            move_count: 0,
            captured_black: 0,
            captured_white: 0,
            ko_position: None,
            last_move: None,
        }
    }
}

impl BoardState {
    /// 创建新的棋盘状态
    pub fn new(board_size: BoardSize) -> Self {
        Self {
            board_size,
            ..Default::default()
        }
    }
    
    /// 获取指定位置的棋子
    pub fn get_stone(&self, x: i32, y: i32) -> Option<StoneColor> {
        if x >= 0 && x < 19 && y >= 0 && y < 19 {
            self.stones[x as usize][y as usize]
        } else {
            None
        }
    }
    
    /// 放置棋子
    pub fn place_stone(&mut self, x: i32, y: i32, color: StoneColor) -> bool {
        if x >= 0 && x < self.board_size.get_value() && y >= 0 && y < self.board_size.get_value() {
            if self.stones[x as usize][y as usize].is_none() {
                self.move_count += 1;
                self.stones[x as usize][y as usize] = Some(color);
                self.move_numbers[x as usize][y as usize] = Some(self.move_count);
                self.last_move = Some((x, y));
                return true;
            }
        }
        false
    }
    
    /// 移除棋子
    pub fn remove_stone(&mut self, x: i32, y: i32) {
        if x >= 0 && x < 19 && y >= 0 && y < 19 {
            self.stones[x as usize][y as usize] = None;
            self.move_numbers[x as usize][y as usize] = None;
        }
    }
    
    /// 清空棋盘
    pub fn clear(&mut self) {
        self.stones = [[None; 19]; 19];
        self.move_numbers = [[None; 19]; 19];
        self.move_count = 0;
        self.captured_black = 0;
        self.captured_white = 0;
        self.ko_position = None;
        self.last_move = None;
    }
    
    /// 获取相邻位置
    pub fn get_neighbors(&self, x: i32, y: i32) -> Vec<(i32, i32)> {
        let mut neighbors = Vec::new();
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        
        for (dx, dy) in directions {
            let nx = x + dx;
            let ny = y + dy;
            if nx >= 0 && nx < self.board_size.get_value() && ny >= 0 && ny < self.board_size.get_value() {
                neighbors.push((nx, ny));
            }
        }
        
        neighbors
    }
}

/// 游戏历史记录
#[derive(Resource, Default)]
pub struct GameHistory {
    pub moves: Vec<Move>,
    pub current_index: usize,
}

/// 单个着法记录
#[derive(Clone)]
pub struct Move {
    pub position: (i32, i32),
    pub color: StoneColor,
    pub captured_stones: Vec<(i32, i32)>,
    pub move_number: usize,
}