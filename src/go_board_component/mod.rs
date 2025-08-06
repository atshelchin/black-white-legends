// 围棋棋盘组件 - 高度可复用的独立模块
// Go Board Component - Highly reusable standalone module

pub mod components;
pub mod config;
pub mod events;
pub mod plugin;
pub mod resources;
pub mod rules;
pub mod systems;
pub mod utils;

// Re-export main types for convenience
pub use components::{Stone, StoneColor};
pub use config::{BoardSize, GoBoardConfig};
pub use events::{PlaceStoneEvent, RedrawBoardEvent, UpdateBoardConfigEvent};
pub use plugin::GoBoardPlugin;
pub use resources::{BoardState, CurrentTurn};
pub use rules::GoBoardRules;

// Component prelude for easy importing
pub mod prelude {
    pub use super::{
        BoardSize, BoardState, CurrentTurn, GoBoardConfig, GoBoardPlugin, GoBoardRules,
        PlaceStoneEvent, StoneColor,
    };
}
