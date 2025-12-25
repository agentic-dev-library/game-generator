//! Freeform Mode - AI-assisted game creation through conversation
//!
//! This mode provides an interactive conversation interface where users can
//! describe their game ideas and the AI helps design and generate the game.

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::wizard::pipeline::GenerationPipeline;
use crate::wizard::state::AppState;

mod types;
mod conversation;
// mod wizard_steps;
// mod ai_interface;

pub use types::*;
pub use conversation::*;
// pub use wizard_steps::*;

/// Main entry point for rendering freeform mode
pub fn render_freeform_mode(
    mut contexts: EguiContexts,
    app_state: ResMut<AppState>,
    mut freeform_state: ResMut<FreeformModeState>,
    commands: Commands,
    pipeline: Res<GenerationPipeline>,
    stream_res: ResMut<ConversationStream>,
) {
    let Ok(ctx) = contexts.ctx_mut() else { return; };

    match &freeform_state.current_step {
        FreeformStep::Introduction => {
            egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
                ui.heading("Welcome to Freeform Mode");
                ui.label("This mode allows you to design your game through conversation with an AI.");
                if ui.button("Start").clicked() {
                    freeform_state.current_step = FreeformStep::Conversation;
                }
            });
        }
        FreeformStep::Conversation => {
            conversation::render_conversation(contexts, app_state, freeform_state, commands, pipeline, stream_res);
        }
        _ => {
            // Default fallback for unimplemented steps
            egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
                ui.heading("Work in Progress");
                ui.label(format!("The {:?} step is not yet implemented.", freeform_state.current_step));
                if ui.button("Go to Conversation").clicked() {
                    freeform_state.current_step = FreeformStep::Conversation;
                }
            });
        }
    }
}

/// Setup resources for freeform mode
pub fn setup_freeform_mode(mut commands: Commands) {
    // Insert the freeform mode state
    commands.insert_resource(FreeformModeState::default());
    commands.insert_resource(ConversationStream::default());

    // Initialize AI client if needed
    if let Ok(_api_key) = std::env::var("OPENAI_API_KEY") {
        // Initialize AI resources
        info!("OpenAI API key found, AI conversation will be available");
    } else {
        warn!("No OpenAI API key found, AI features will be limited");
    }
}

/// Cleanup resources when leaving freeform mode
pub fn cleanup_freeform_mode(mut commands: Commands) {
    commands.remove_resource::<FreeformModeState>();
    commands.remove_resource::<ConversationStream>();
}
