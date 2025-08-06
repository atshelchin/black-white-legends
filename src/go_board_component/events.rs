use super::components::StoneColor;
use super::config::GoBoardConfig;
use bevy::prelude::*;

/// 重绘棋盘事件
#[derive(Event)]
pub struct RedrawBoardEvent;

/// 更新棋盘配置事件
#[derive(Event)]
pub struct UpdateBoardConfigEvent {
    pub config: GoBoardConfig,
}

/// 落子事件
#[derive(Event)]
pub struct PlaceStoneEvent {
    pub position: (i32, i32),
    pub color: StoneColor,
}

/// 棋子动作类型
#[derive(Debug, Clone, Copy)]
pub enum StoneActionType {
    Place,   // 落子
    Capture, // 提子
    Pass,    // 虚手
    Resign,  // 认输
}

/// 棋子动作事件 - 更高层级的事件，可以触发游戏逻辑
#[derive(Event)]
pub struct StoneActionEvent {
    pub action_type: StoneActionType,
    pub position: Option<(i32, i32)>,
    pub color: StoneColor,
}

/// 游戏结束事件
#[derive(Event)]
pub struct GameEndEvent {
    pub winner: Option<StoneColor>,
    pub black_score: f32,
    pub white_score: f32,
}

/// 撤销事件
#[derive(Event)]
pub struct UndoMoveEvent;

/// 重做事件
#[derive(Event)]
pub struct RedoMoveEvent;

/// 清空棋盘事件
#[derive(Event)]
pub struct ClearBoardEvent;

/// 加载棋谱事件
#[derive(Event)]
pub struct LoadGameEvent {
    pub sgf_content: String,
}

/// 保存棋谱事件
#[derive(Event)]
pub struct SaveGameEvent {
    pub file_path: String,
}
