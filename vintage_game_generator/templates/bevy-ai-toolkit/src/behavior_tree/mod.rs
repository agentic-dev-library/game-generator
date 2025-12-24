use bevy::prelude::*;

pub struct BehaviorTreePlugin;

impl Plugin for BehaviorTreePlugin {
    fn build(&self, _app: &mut App) {
        // Add behavior tree systems here
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeStatus {
    Success,
    Failure,
    Running,
}

pub trait BehaviorNode: Send + Sync + 'static {
    fn tick(&mut self, entity: Entity, world: &mut World) -> NodeStatus;
}

#[derive(Component)]
pub struct BehaviorTree {
    pub root: Box<dyn BehaviorNode>,
}

pub struct Selector {
    pub children: Vec<Box<dyn BehaviorNode>>,
}

impl BehaviorNode for Selector {
    fn tick(&mut self, entity: Entity, world: &mut World) -> NodeStatus {
        for child in &mut self.children {
            match child.tick(entity, world) {
                NodeStatus::Success => return NodeStatus::Success,
                NodeStatus::Running => return NodeStatus::Running,
                NodeStatus::Failure => continue,
            }
        }
        NodeStatus::Failure
    }
}

pub struct Sequence {
    pub children: Vec<Box<dyn BehaviorNode>>,
}

impl BehaviorNode for Sequence {
    fn tick(&mut self, entity: Entity, world: &mut World) -> NodeStatus {
        for child in &mut self.children {
            match child.tick(entity, world) {
                NodeStatus::Success => continue,
                NodeStatus::Running => return NodeStatus::Running,
                NodeStatus::Failure => return NodeStatus::Failure,
            }
        }
        NodeStatus::Success
    }
}
