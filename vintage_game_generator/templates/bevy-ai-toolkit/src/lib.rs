pub mod state_machine;
pub mod behavior_tree;
pub mod utility_ai;
pub mod targeting;

pub mod prelude {
    pub use crate::state_machine::*;
    pub use crate::behavior_tree::*;
    pub use crate::utility_ai::*;
    pub use crate::targeting::*;
}

use bevy::prelude::*;

pub struct AiToolkitPlugin;

impl Plugin for AiToolkitPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            state_machine::StateMachinePlugin,
            behavior_tree::BehaviorTreePlugin,
            utility_ai::UtilityAiPlugin,
            targeting::TargetingPlugin,
        ));
    }
}
