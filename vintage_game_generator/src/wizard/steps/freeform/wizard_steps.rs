use bevy::prelude::*;
use bevy_egui::EguiContexts;
use crate::wizard::state::AppState;
use super::{FreeformModeState, FreeformStep};

pub fn render_introduction(_contexts: EguiContexts, _app_state: ResMut<AppState>, mut freeform_state: ResMut<FreeformModeState>) {
    freeform_state.current_step = FreeformStep::Conversation; // Skip to conversation for now
}

pub fn render_basic_info(_contexts: EguiContexts, _app_state: ResMut<AppState>, _freeform_state: ResMut<FreeformModeState>) {}
pub fn render_gameplay_design(_contexts: EguiContexts, _app_state: ResMut<AppState>, _freeform_state: ResMut<FreeformModeState>) {}
pub fn render_visual_style(_contexts: EguiContexts, _app_state: ResMut<AppState>, _freeform_state: ResMut<FreeformModeState>) {}
pub fn render_features(_contexts: EguiContexts, _app_state: ResMut<AppState>, _freeform_state: ResMut<FreeformModeState>) {}
pub fn render_technical_settings(_contexts: EguiContexts, _app_state: ResMut<AppState>, _freeform_state: ResMut<FreeformModeState>) {}
pub fn render_review(_contexts: EguiContexts, _app_state: ResMut<AppState>, _freeform_state: ResMut<FreeformModeState>) {}
