use super::{components::StoneColor, config::GoBoardConfig, events::*, resources::*, systems::*};
use bevy::prelude::*;

/// 围棋棋盘插件
///
/// # 使用示例
/// ```rust
/// use bevy::prelude::*;
/// use go_board_component::prelude::*;
///
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .add_plugins(GoBoardPlugin::default())
///         .run();
/// }
/// ```
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
        // 添加资源
        app.insert_resource(CurrentGoBoardConfig(self.initial_config.clone()))
            .insert_resource(CurrentTurn(StoneColor::Black))
            .insert_resource(BoardState::new(self.initial_config.board_size))
            .insert_resource(GameHistory::default());

        // 添加事件
        app.add_event::<RedrawBoardEvent>()
            .add_event::<UpdateBoardConfigEvent>()
            .add_event::<PlaceStoneEvent>()
            .add_event::<StoneActionEvent>()
            .add_event::<ClearBoardEvent>()
            .add_event::<UndoMoveEvent>()
            .add_event::<RedoMoveEvent>()
            .add_event::<GameEndEvent>()
            .add_event::<LoadGameEvent>()
            .add_event::<SaveGameEvent>();

        // 添加系统
        app.add_systems(
            Update,
            (
                handle_config_update,
                handle_place_stone,
                handle_board_redraw,
                handle_clear_board,
            )
                .chain(),
        );
    }
}

/// 插件扩展构建器
pub struct GoBoardPluginBuilder {
    config: GoBoardConfig,
}

impl GoBoardPluginBuilder {
    pub fn new() -> Self {
        Self {
            config: GoBoardConfig::default(),
        }
    }

    pub fn with_board_size(mut self, size: super::config::BoardSize) -> Self {
        self.config.board_size = size;
        self
    }

    pub fn with_coordinates(mut self, show: bool) -> Self {
        self.config.show_coordinates = show;
        self
    }

    pub fn with_move_numbers(mut self, show: bool) -> Self {
        self.config.show_move_numbers = show;
        self
    }

    pub fn with_captures(mut self, enable: bool) -> Self {
        self.config.enable_captures = enable;
        self
    }

    pub fn with_ko_rule(mut self, enable: bool) -> Self {
        self.config.enable_ko_rule = enable;
        self
    }

    pub fn build(self) -> GoBoardPlugin {
        GoBoardPlugin {
            initial_config: self.config,
        }
    }
}
