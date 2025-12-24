use bevy::prelude::*;
use bevy_ai_toolkit::prelude::*;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
enum EnemyState {
    Idle,
    Chasing,
    Attacking,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AiToolkitPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_enemy_state, update_enemy_behavior))
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn player
    commands.spawn((
        SpatialBundle::default(),
        Targetable,
        Name::new("Player"),
    ));

    // Spawn enemy with AI
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(5.0, 0.0, 5.0),
            ..default()
        },
        StateMachine::new(EnemyState::Idle),
        Vision {
            range: 10.0,
            field_of_view: 360.0,
        },
        Target::default(),
        Name::new("Enemy"),
    ));
}

fn update_enemy_state(
    mut query: Query<(&mut StateMachine<EnemyState>, &Target, &GlobalTransform)>,
    player_query: Query<&GlobalTransform, With<Targetable>>,
) {
    for (mut state_machine, target, transform) in query.iter_mut() {
        if let Some(target_entity) = target.entity {
            if let Ok(target_transform) = player_query.get(target_entity) {
                let distance = transform.translation().distance(target_transform.translation());
                
                if distance < 2.0 {
                    state_machine.transition_to(EnemyState::Attacking);
                } else {
                    state_machine.transition_to(EnemyState::Chasing);
                }
            }
        } else {
            state_machine.transition_to(EnemyState::Idle);
        }
    }
}

fn update_enemy_behavior(query: Query<(&StateMachine<EnemyState>, &Name)>) {
    for (state_machine, name) in query.iter() {
        println!("{} is currently {:?}", name, state_machine.current_state);
    }
}
