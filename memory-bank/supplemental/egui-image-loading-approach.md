# Pure egui Image Loading Approach

## Date: 2025-01-08

### Overview
We've aligned our image loading approach with bevy-inspector-egui, moving away from Bevy's AssetServer to pure egui image loading for the wizard UI.

### Key Changes

1. **Image Loading Module** (`src/wizard/image_loader.rs`)
   - Uses `egui::Context::load_texture()` directly
   - Caches textures in egui's context
   - Handles multiple asset path locations
   - No dependency on Bevy's AssetServer

2. **Overlay System** (`src/wizard/overlay.rs`)
   - Pure egui rendering using Areas and layers
   - Native egui texture handling
   - Proper layer ordering without Bevy systems
   - Uses egui's built-in interaction system

3. **Dependencies Alignment**
   - `bevy_egui = "0.35"` (same as bevy-inspector-egui)
   - `bevy-inspector-egui = "0.32"`
   - `egui_dock = "0.16"` (same version)
   - `catppuccin-egui = { version = "5.6", features = ["egui31"] }`

### Implementation Pattern

```rust
// Load image using pure egui
let texture = image_loader::load_wizard_asset(
    ui.ctx(),
    "image_name.png",
    "texture_id"
);

// Use texture in UI
if let Some(texture) = texture {
    ui.image(&texture, desired_size);
}
```

### Benefits
1. **Simpler Architecture**: No need for Bevy resources/systems for UI images
2. **Better Integration**: Works seamlessly with egui's immediate mode
3. **Consistency**: Aligns with how bevy-inspector-egui handles UI
4. **Performance**: Images cached efficiently in egui context

### Migration Notes
- Removed all `Handle<Image>` usage in wizard code
- Removed `AssetServer` dependencies from UI components
- Updated all image display code to use egui textures
- Kept RON configuration loading for clickable areas

### Reference Implementation
See bevy-inspector-egui's approach in:
- `/Users/jbogaty/src/bevy-inspector-egui/crates/bevy-inspector-egui/examples/integrations/egui_dock.rs`

This approach ensures our UI is purely egui-based while still leveraging Bevy for the game engine aspects.
