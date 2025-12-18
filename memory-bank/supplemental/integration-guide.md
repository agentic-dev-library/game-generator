# Integration Guide: Tying Together the Two Halves

## Overview

We have successfully split the architecture into two main components:
1. **Database-Driven egui Wizard Half** - A studio interface for project configuration
2. **Optimized Style Guide Driven Metaprompt Half** - AI-powered game generation

## Current Architecture

### Module Structure
```
src/
├── database/          # SeaORM entities and migrations
├── services/          # Service layer for database operations
├── wizard/            # Project configuration wizard
├── generator/         # AI metaprompt generation engine
├── studio/            # egui studio interface (now modularized!)
│   ├── dock.rs        # Dock tab definitions
│   ├── menu.rs        # Menu bar and status bar
│   ├── notifications.rs # Notification system
│   ├── state.rs       # Studio state management
│   ├── theme.rs       # Theme configuration
│   └── tabs/          # Individual tab implementations
└── integration/       # Glue code between wizard and generator
```

## Key Integration Points

### 1. Wizard → Generator Flow
The `integration/mod.rs` module handles:
- Converting `GameConfiguration` from wizard to `GameConcept` for generator
- Managing background generation tasks with cancellation support
- Sending progress events back to the UI
- Storing generated assets in the database

### 2. Database Integration
All components share the same database:
- **Projects** - Created by wizard, referenced by generator
- **Assets** - Created by generator, displayed in studio
- **Generation Tasks** - Track progress and history
- **Wizard State** - Persist configuration between sessions

### 3. Event System
Bevy events connect the async world with the ECS:
- `GenerationProgressEvent` - Updates UI during generation
- `GenerationCompleteEvent` - Triggers phase transitions
- `AssetGeneratedEvent` - Notifies gallery of new assets
- `NotificationEvent` - User feedback

## Implementation Status

### ✅ Completed
- Database schema and entities
- Service layer for all entities
- Wizard UI with comprehensive configuration
- Generator with 5-phase metaprompt chain
- Studio shell with modular tab system
- Basic integration module

### 🚧 Needs Implementation

#### 1. Wire Up Generation Start
In `wizard/mod.rs`, when user completes wizard:
```rust
// Add to wizard completion handler
if ui.button("Generate Game").clicked() {
    // Save project to database
    let project_id = project_service.create_project(config).await?;
    
    // Start generation
    integration::start_generation_from_wizard(
        world,
        project_id,
        config,
        api_key,
    )?;
    
    // Transition to generation phase
    next_phase.set(StudioPhase::Generation);
}
```

#### 2. Connect Progress Events
The integration module sends events, but studio needs to display them:
```rust
// Add system to studio plugin
fn handle_generation_progress(
    mut events: EventReader<GenerationProgressEvent>,
    mut gen_state: ResMut<GenerationState>,
) {
    for event in events.read() {
        // Update generation state
        gen_state.active_phase = Some(event.phase);
        gen_state.total_progress = event.progress;
        
        // Update specific task
        if let Some(task) = gen_state.current_tasks.iter_mut()
            .find(|t| t.phase == event.phase) {
            task.progress = event.progress;
            task.status = TaskStatus::InProgress;
        }
    }
}
```

#### 3. Asset Loading from Database
When generation completes, load assets:
```rust
// In asset gallery tab
async fn load_project_assets(
    project_id: i32,
    asset_service: &AssetService,
) -> Vec<AssetMetadata> {
    let assets = asset_service.get_project_assets(project_id).await?;
    
    // Convert database assets to UI metadata
    assets.into_iter().map(|asset| AssetMetadata {
        id: asset.id,
        name: asset.name,
        asset_type: asset.asset_type,
        preview: generate_preview(&asset.data),
    }).collect()
}
```

#### 4. Live Preview Integration
Generated code needs to compile and run:
```rust
// In live preview tab
fn compile_and_run_game(
    code_files: HashMap<String, String>,
) -> Result<BevySubApp> {
    // Create temporary Cargo project
    let temp_dir = create_temp_project()?;
    
    // Write generated code files
    for (path, content) in code_files {
        write_file(temp_dir.join(path), content)?;
    }
    
    // Compile with cargo
    let output = Command::new("cargo")
        .args(&["build", "--release"])
        .current_dir(&temp_dir)
        .output()?;
    
    // Load as Bevy sub-app
    // ...
}
```

#### 5. Async Runtime Bridge
Handle tokio tasks in Bevy:
```rust
// In main.rs
fn setup_async_runtime(mut commands: Commands) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let handle = runtime.handle().clone();
    
    // Store runtime handle as resource
    commands.insert_resource(AsyncRuntime(handle));
    
    // Spawn runtime thread
    std::thread::spawn(move || {
        runtime.block_on(async {
            loop {
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });
    });
}
```

## Configuration Flow

1. **User Opens Studio** → Shows wizard in Setup phase
2. **User Configures Game** → Wizard collects `GameConfiguration`
3. **User Clicks Generate** → 
   - Save project to database
   - Convert config to concept
   - Start async generation
   - Switch to Generation phase
4. **Generation Runs** →
   - Progress events update UI
   - Assets saved to database
   - Style guide established first
5. **Generation Completes** →
   - Load assets from database
   - Enable live preview
   - Switch to LiveEdit phase

## Error Handling

### Generation Failures
- Capture errors in `GenerationOrchestrator`
- Send `GenerationCompleteEvent` with error
- Display in notifications
- Allow retry with modified settings

### Database Errors
- All service methods return `Result<T>`
- UI shows user-friendly messages
- Log detailed errors for debugging

### API Errors
- Handle rate limits gracefully
- Retry with exponential backoff
- Show estimated time remaining

## Testing Strategy

### Unit Tests
- Test `configuration_to_concept` conversion
- Test event handling
- Test database operations

### Integration Tests
```rust
#[tokio::test]
async fn test_full_generation_flow() {
    // Setup test database
    let db = setup_test_db().await;
    
    // Create test configuration
    let config = create_test_config();
    
    // Run generation
    let orchestrator = GenerationOrchestrator::new(db, "test_key")?;
    let result = orchestrator.start_generation(1, config).await;
    
    // Verify assets created
    assert!(result.is_ok());
    let assets = asset_service.get_project_assets(1).await?;
    assert!(!assets.is_empty());
}
```

## Performance Considerations

### Memory Management
- Stream large assets instead of loading fully
- Use weak references for preview images
- Clean up completed tasks periodically

### Database Optimization
- Index foreign keys
- Batch insert assets
- Use connection pooling

### UI Responsiveness
- Run generation in background
- Update progress at reasonable intervals
- Don't block on database queries

## Next Steps

1. **Implement Event Handlers** - Connect progress events to UI
2. **Test Generation Flow** - Ensure wizard → generator works
3. **Add Asset Display** - Show generated sprites/code
4. **Enable Live Preview** - Compile and run generated games
5. **Polish UI** - Add loading states, error handling

## Code Snippets

### Starting Generation from UI
```rust
// In wizard completion
pub fn complete_wizard(
    config: GameConfiguration,
    world: &mut World,
) -> Result<()> {
    // Get required resources
    let api_key = env::var("OPENAI_API_KEY")?;
    
    // Create project
    let project_id = {
        let db = world.resource::<Arc<DatabaseConnection>>();
        let project_service = ProjectService::new(db.clone());
        
        let project = project_service.create_project(
            config.name.clone(),
            serde_json::to_value(&config)?,
        ).await?
        
        project.id
    };
    
    // Start generation
    integration::start_generation_from_wizard(
        world,
        project_id,
        config,
        api_key,
    )?;
    
    Ok(())
}
```

### Displaying Generation Progress
```rust
// In generation_queue tab
pub fn render(ui: &mut egui::Ui, gen_state: &GenerationState) {
    // Show overall progress
    let progress_text = match &gen_state.active_phase {
        Some(phase) => format!("{:?}: {:.0}%", phase, gen_state.total_progress * 100.0),
        None => "Waiting...".to_string(),
    };
    
    ui.add(egui::ProgressBar::new(gen_state.total_progress)
        .text(progress_text));
    
    // Show individual tasks
    for task in &gen_state.current_tasks {
        render_task(ui, task);
    }
}
```

## Conclusion

The architecture is well-structured with clear separation of concerns. The key to tying it together is:
1. Proper event flow from async tasks to Bevy systems
2. Consistent database usage across all components
3. Clear phase transitions in the studio
4. Robust error handling throughout

With these integration points implemented, the system will provide a seamless experience from configuration to playing the generated game.
