use bevy::prelude::*;

/// States for the combat system
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Reflect)]
pub enum CombatState {
    #[default]
    None,
    Starting,
    PlayerTurn,
    EnemyTurn,
    Processing,
    Victory,
    Defeat,
}

/// Resource tracking the current round and turn
#[derive(Resource, Debug, Clone, Default, Reflect)]
#[reflect(Resource)]
pub struct CombatManager {
    pub round: u32,
    pub current_turn_entity: Option<Entity>,
}

/// System for transitioning between combat states
pub fn manage_combat_state(
    state: Res<State<CombatState>>,
    mut next_state: ResMut<NextState<CombatState>>,
) {
    match state.get() {
        CombatState::Starting => {
            // Setup battle, then go to player turn
            next_state.set(CombatState::PlayerTurn);
        }
        _ => {}
    }
}
