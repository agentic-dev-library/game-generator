use bevy::prelude::*;

pub struct TargetingPlugin;

impl Plugin for TargetingPlugin {
    fn build(&self, _app: &mut App) {
        // Add targeting systems here
    }
}

#[derive(Component, Default)]
pub struct Target {
    pub entity: Option<Entity>,
}

#[derive(Component)]
pub struct Vision {
    pub range: f32,
    pub field_of_view: f32,
}

pub fn update_targets(
    mut query: Query<(Entity, &GlobalTransform, &Vision, &mut Target)>,
    targets_query: Query<(Entity, &GlobalTransform), With<Targetable>>,
) {
    for (entity, transform, vision, mut target) in query.iter_mut() {
        let mut closest_target = None;
        let mut closest_distance = vision.range;

        for (target_entity, target_transform) in targets_query.iter() {
            if entity == target_entity {
                continue;
            }

            let distance = transform.translation().distance(target_transform.translation());
            if distance < closest_distance {
                closest_distance = distance;
                closest_target = Some(target_entity);
            }
        }

        target.entity = closest_target;
    }
}

#[derive(Component)]
pub struct Targetable;
