use bevy::prelude::*;

pub struct UtilityAiPlugin;

impl Plugin for UtilityAiPlugin {
    fn build(&self, _app: &mut App) {
        // Add utility AI systems here
    }
}

pub trait Scorer: Send + Sync + 'static {
    fn score(&self, entity: Entity, world: &World) -> f32;
}

pub trait Action: Send + Sync + 'static {
    fn execute(&self, entity: Entity, commands: &mut Commands);
}

pub struct Consideration {
    pub scorer: Box<dyn Scorer>,
    pub action: Box<dyn Action>,
}

#[derive(Component)]
pub struct UtilityAi {
    pub considerations: Vec<Consideration>,
}

impl UtilityAi {
    pub fn select_best(&self, entity: Entity, world: &World) -> Option<&Box<dyn Action>> {
        let mut best_score = -1.0;
        let mut best_action = None;

        for consideration in &self.considerations {
            let score = consideration.scorer.score(entity, world);
            if score > best_score {
                best_score = score;
                best_action = Some(&consideration.action);
            }
        }

        best_action
    }
}
