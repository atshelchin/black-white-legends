use super::components::StoneColor;
use super::resources::BoardState;
use std::collections::HashSet;

/// 围棋规则引擎
pub struct GoBoardRules;

impl GoBoardRules {
    /// 检查落子是否合法
    pub fn is_valid_move(board_state: &BoardState, x: i32, y: i32, _color: StoneColor) -> bool {
        // 检查位置是否在棋盘内
        if x < 0 || x >= board_state.board_size.get_value() || y < 0 || y >= board_state.board_size.get_value() {
            return false;
        }
        
        // 检查位置是否已有棋子
        if board_state.stones[x as usize][y as usize].is_some() {
            return false;
        }
        
        // 检查是否违反打劫规则
        if let Some(ko_pos) = board_state.ko_position {
            if ko_pos == (x, y) {
                return false;
            }
        }
        
        // TODO: 检查自杀规则
        
        true
    }
    
    /// 获取一个棋串（相连的同色棋子）
    pub fn get_group(board_state: &BoardState, x: i32, y: i32) -> HashSet<(i32, i32)> {
        let mut group = HashSet::new();
        
        if let Some(color) = board_state.get_stone(x, y) {
            let mut stack = vec![(x, y)];
            group.insert((x, y));
            
            while let Some((cx, cy)) = stack.pop() {
                for (nx, ny) in board_state.get_neighbors(cx, cy) {
                    if !group.contains(&(nx, ny)) {
                        if let Some(neighbor_color) = board_state.get_stone(nx, ny) {
                            if neighbor_color == color {
                                group.insert((nx, ny));
                                stack.push((nx, ny));
                            }
                        }
                    }
                }
            }
        }
        
        group
    }
    
    /// 计算一个棋串的气数
    pub fn count_liberties(board_state: &BoardState, group: &HashSet<(i32, i32)>) -> usize {
        let mut liberties = HashSet::new();
        
        for &(x, y) in group {
            for (nx, ny) in board_state.get_neighbors(x, y) {
                if board_state.get_stone(nx, ny).is_none() {
                    liberties.insert((nx, ny));
                }
            }
        }
        
        liberties.len()
    }
    
    /// 检查并移除被提的棋子
    pub fn capture_stones(board_state: &mut BoardState, x: i32, y: i32, color: StoneColor) -> Vec<(i32, i32)> {
        let mut captured = Vec::new();
        let opponent = color.opposite();
        
        // 检查相邻的对手棋子
        for (nx, ny) in board_state.get_neighbors(x, y) {
            if board_state.get_stone(nx, ny) == Some(opponent) {
                let group = Self::get_group(board_state, nx, ny);
                if Self::count_liberties(board_state, &group) == 0 {
                    // 提子
                    for &(gx, gy) in &group {
                        board_state.remove_stone(gx, gy);
                        captured.push((gx, gy));
                    }
                }
            }
        }
        
        // 更新提子计数
        match color {
            StoneColor::Black => board_state.captured_white += captured.len(),
            StoneColor::White => board_state.captured_black += captured.len(),
        }
        
        captured
    }
    
    /// 检查自杀规则
    pub fn is_suicide(board_state: &BoardState, x: i32, y: i32, color: StoneColor) -> bool {
        // 创建临时棋盘状态
        let mut temp_board = board_state.clone();
        temp_board.stones[x as usize][y as usize] = Some(color);
        
        // 首先检查是否能提对手的子
        for (nx, ny) in temp_board.get_neighbors(x, y) {
            if temp_board.get_stone(nx, ny) == Some(color.opposite()) {
                let group = Self::get_group(&temp_board, nx, ny);
                if Self::count_liberties(&temp_board, &group) == 0 {
                    return false; // 能提对手的子，不是自杀
                }
            }
        }
        
        // 检查自己这一串是否有气
        let own_group = Self::get_group(&temp_board, x, y);
        Self::count_liberties(&temp_board, &own_group) == 0
    }
    
    /// 计算终局分数（中国规则）
    pub fn calculate_score(board_state: &BoardState) -> (f32, f32) {
        let mut black_score = 0.0;
        let mut white_score = 7.5; // 贴目
        
        // 计算棋子数和领地
        for x in 0..board_state.board_size.get_value() {
            for y in 0..board_state.board_size.get_value() {
                match board_state.get_stone(x, y) {
                    Some(StoneColor::Black) => black_score += 1.0,
                    Some(StoneColor::White) => white_score += 1.0,
                    None => {
                        // TODO: 计算领地归属
                    }
                }
            }
        }
        
        (black_score, white_score)
    }
}

// 为BoardState实现Clone trait以支持规则检查
impl Clone for BoardState {
    fn clone(&self) -> Self {
        Self {
            stones: self.stones,
            move_numbers: self.move_numbers,
            board_size: self.board_size,
            move_count: self.move_count,
            captured_black: self.captured_black,
            captured_white: self.captured_white,
            ko_position: self.ko_position,
            last_move: self.last_move,
        }
    }
}