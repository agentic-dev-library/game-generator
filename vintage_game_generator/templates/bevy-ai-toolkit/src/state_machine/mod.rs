use bevy::prelude::*;
use std::marker::PhantomData;

pub struct StateMachinePlugin;

impl Plugin for StateMachinePlugin {
    fn build(&self, _app: &mut App) {
        // Add state machine systems here
    }
}

#[derive(Component)]
pub struct StateMachine<S: Component + Clone> {
    pub current_state: S,
    _phantom: PhantomData<S>,
}

impl<S: Component + Clone> StateMachine<S> {
    pub fn new(initial_state: S) -> Self {
        Self {
            current_state: initial_state,
            _phantom: PhantomData,
        }
    }

    pub fn transition_to(&mut self, next_state: S) {
        self.current_state = next_state;
    }
}

pub trait StateAction<S>: Send + Sync + 'static {
    fn execute(&self, entity: Entity, commands: &mut Commands, state: &S);
}
