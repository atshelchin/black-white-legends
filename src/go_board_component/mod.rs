// 围棋棋盘组件 - 高度可复用的独立模块
// Go Board Component - Highly reusable standalone module

pub mod config;
pub mod components;
pub mod events;
pub mod resources;
pub mod systems;
pub mod plugin;
pub mod rules;
pub mod utils;

// Re-export main types for convenience
pub use config::{GoBoardConfig, BoardSize};
pub use components::{GoBoard, Stone, StoneColor};
pub use events::{PlaceStoneEvent, RedrawBoardEvent, UpdateBoardConfigEvent, StoneActionEvent, StoneActionType};
pub use resources::{BoardState, CurrentTurn};
pub use plugin::GoBoardPlugin;
pub use rules::GoBoardRules;

// Component prelude for easy importing
pub mod prelude {
    pub use super::{
        GoBoardPlugin,
        GoBoardConfig,
        BoardSize,
        StoneColor,
        PlaceStoneEvent,
        StoneActionEvent,
        StoneActionType,
        BoardState,
        CurrentTurn,
        GoBoardRules,
    };
}