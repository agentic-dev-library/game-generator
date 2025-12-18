# Radical Simplification: No Wizard, No Database, Pure Metaprompts

## Date: 2025-01-08

### The Realization

With the complete metaprompt architecture in place, we don't need:
- Complex wizard UI with state tracking
- Development database for storing configurations
- Service layers for preferences and project management
- Complex state persistence between steps

### New Ultra-Simple Architecture

```
1. Simple CLI/Minimal UI
   └── Ask macro/micro questions
       └── Feed to discovery metaprompt
           └── Generate complete game
               └── bevy_inspector_egui for live editing
```

### Why This Works

1. **Discovery Phase Does Everything**
   - The discovery metaprompt can ask ALL necessary questions
   - It can handle both macro (genre, theme) and micro (specific mechanics) details
   - Dynamic JSON means it adapts to what's needed

2. **No State Needed**
   - Everything happens in one conversation
   - No need to persist wizard states
   - No complex navigation or validation

3. **Live Editing is Better**
   - Generate the game, then tweak it live
   - bevy_inspector_egui provides runtime editing
   - egui_dock gives professional editor UI
   - Changes can be saved back to files

### Implementation Approach

#### Phase 1: Ultra-Simple CLI
```rust
#[derive(Parser)]
struct Args {
    /// Interactive mode (ask questions)
    #[arg(short, long)]
    interactive: bool,
    
    /// Quick start with genre
    #[arg(short, long)]
    genre: Option<String>,
    
    /// Output directory
    #[arg(short, long, default_value = "./generated_game")]
    output: PathBuf,
}

async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Simple Q&A or use provided genre
    let concept = if args.interactive {
        interactive_questionnaire().await?
    } else {
        quick_concept_from_genre(args.genre)?
    };
    
    // Generate entire game
    let orchestrator = MetapromptOrchestrator::new(...)?;
    let game = orchestrator.generate_game(&concept, |phase, progress| {
        println!("{:?}: {:.0}%", phase, progress * 100.0);
    }).await?;
    
    // Save to disk
    save_generated_game(&game, &args.output).await?;
    
    println!("Game generated! Run with: cd {} && cargo run", args.output);
}
```

#### Phase 2: Enhanced Discovery Metaprompt
Update `phases/discovery.jinja` to handle interactive Q&A:
```jinja
{# Enhanced Discovery Phase with Q&A #}
You are having a conversation with a game developer to understand their vision.

## Initial Input
{% if concept %}
{{ concept | tojson(indent=2) }}
{% else %}
No initial concept provided - ask questions to discover what they want.
{% endif %}

## Your Task
1. If details are missing, generate questions to ask:
   - Genre preferences (if not specified)
   - Art style vision
   - Core mechanics they're excited about
   - Target audience
   - Scope (jam game vs full game)
   - Unique features they want

2. Once you have enough information, generate the complete game design.

Generate JSON with either:
- `questions`: Array of questions to ask the developer
- `game_design`: Complete game design once ready
```

#### Phase 3: Game Inspector Plugin
The generated game includes bevy_inspector_egui by default:
```rust
// In generated main.rs
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_egui::EguiPlugin;
use egui_dock::{DockArea, Style};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(GameEditorPlugin) // Custom dock-based editor
        .add_plugins(GeneratedGamePlugin)
        .run();
}
```

### Benefits of This Approach

1. **Drastically Reduced Complexity**
   - No wizard UI code to maintain
   - No database schema or migrations
   - No service layers or state management
   - No complex validation logic

2. **Better Developer Experience**
   - Quick generation from CLI
   - Live editing in the actual game
   - See changes immediately
   - Export modifications back to code

3. **More Flexible**
   - Easy to add new question types
   - Metaprompts adapt to any game type
   - No UI constraints on creativity

4. **Faster Development**
   - Remove thousands of lines of code
   - Focus on the AI generation quality
   - Iterate on metaprompts, not UI

### What We Can Delete

1. **Entire Wizard Module**
   - `src/wizard/` - All of it
   - Wizard assets and step PNGs
   - Complex navigation logic

2. **Database Layer**
   - `src/database/` - Entire module
   - SeaORM entities and migrations
   - All persistence code

3. **Service Layers**
   - `wizard_service.rs`
   - `preference_service.rs` 
   - `project_service.rs`
   - Most of `asset_service.rs`

4. **Studio Complexity**
   - Most of `src/studio/`
   - Keep only what's needed for game preview

### New Minimal Structure

```
ai_rpg_generator/
├── src/
│   ├── main.rs              # Simple CLI
│   ├── lib.rs               # Exports
│   ├── questionnaire.rs     # Interactive Q&A
│   └── generator/           # Core generation
│       ├── mod.rs
│       ├── metaprompt_orchestrator.rs
│       ├── client/          # AI clients
│       └── prompt_engine.rs
├── metaprompts/             # All the magic
└── Cargo.toml              # Minimal deps
```

### Migration Path

1. **Keep metaprompt work** - It's the core value
2. **Create simple CLI** - Can coexist with wizard initially
3. **Test end-to-end** - Ensure quality matches
4. **Delete wizard/database** - Once CLI proves sufficient
5. **Add game editor plugin** - For generated games

### Conclusion

The metaprompt architecture is so powerful that it eliminates the need for complex UI and state management. A simple questionnaire feeding into the discovery phase, followed by in-game editing with bevy_inspector_egui, provides a better developer experience with 90% less code.

This is the true power of AI-driven development - not building complex systems to manage AI, but letting AI eliminate the need for complex systems.
