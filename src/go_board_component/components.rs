use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// 围棋棋盘根实体标记
#[derive(Component)]
pub struct GoBoard;

/// 棋盘根节点
#[derive(Component)]
pub struct GoBoardRoot;

/// 棋盘线条
#[derive(Component)]
pub struct BoardLine;

/// 星位点
#[derive(Component)]
pub struct StarPoint;

/// 坐标标签
#[derive(Component)]
pub struct CoordinateLabel;

/// 棋子颜色
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StoneColor {
    Black,
    White,
}

impl StoneColor {
    pub fn opposite(&self) -> Self {
        match self {
            StoneColor::Black => StoneColor::White,
            StoneColor::White => StoneColor::Black,
        }
    }
}

/// 棋子组件
#[derive(Component)]
pub struct Stone {
    pub color: StoneColor,
    pub position: (i32, i32),
    pub move_number: usize,
}

/// 棋子阴影
#[derive(Component)]
pub struct StoneShadow;

/// 棋子高光
#[derive(Component)]
pub struct StoneHighlight;

/// 手数标签
#[derive(Component)]
pub struct MoveNumberLabel;

/// 悬停指示器
#[derive(Component)]
pub struct HoverIndicator;

/// 最后一手标记
#[derive(Component)]
pub struct LastMoveMarker;

/// 死子标记
#[derive(Component)]
pub struct DeadStoneMarker;

/// 领地标记
#[derive(Component)]
pub struct TerritoryMarker {
    pub owner: StoneColor,
}