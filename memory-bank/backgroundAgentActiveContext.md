# Background Agent Active Context - AI RPG Generator

## Current Task: Database Integration and Generator Wiring

### 🎯 Tonight's Progress (2025-07-31)

Successfully completed database integration for the game configuration step:
- Removed ALL hardcoded preset lists
- Implemented dynamic preset discovery using jsonpath
- Fixed Arc reference sharing between wizard and steps
- Created preset to ECS configuration converter

### 🚨 Critical Context for Next Session

1. **Database Manager**: The `game_config_step` now properly receives the shared database_manager Arc from the wizard. This needs to be used for actual ECS world queries.

2. **Defaults Loader**: Presets are now loaded dynamically using `defaults_loader.query("presets")`. Each preset directory (e.g., "action-rpg") contains a `default.toml` with metadata and system configurations.

3. **No More Mocks**: The user was very clear - NO mock data, NO fallbacks, NO hardcoding. Everything should come from the database or defaults loader.

### 🔥 Immediate Next Steps

1. **Complete ECS Database Loading**
   ```rust
   // In game_config.rs - load_ecs_from_database()
   // This function exists but needs proper implementation
   // Should query actual components, systems, resources from database
   ```

2. **Wire Bootstrap Orchestrator**
   ```rust
   // In generation.rs - Phase 1
   // Connect the BootstrapOrchestrator from app/bootstrap/
   // Pass database_manager Arc
   // Implement progress reporting
   ```

3. **Generation Output Structure**
   ```
   <build-dir>/<game-name>/
   ├── prompts/      # Phase 1 output
   ├── templates/    # Phase 1 output  
   ├── runtime/      # Phase 1 output
   ├── assets/       # Phase 2 output
   └── game/         # Phase 3 output
   ```

### ⚠️ Important Patterns to Follow

1. **Arc Sharing Pattern**:
   ```rust
   // Always pass existing Arcs, never create new instances
   .with_database(database_manager.clone())
   .with_defaults(defaults_loader.clone())
   ```

2. **Jsonpath Queries**:
   ```rust
   // Use jsonpath for dynamic discovery
   defaults_loader.query("presets")  // Gets all presets
   defaults_loader.query("presets.action-rpg.default")  // Gets specific preset
   ```

3. **GenerationReporter Trait**:
   ```rust
   pub trait GenerationReporter: Send + Sync {
       fn report_progress(&self, task: &str, progress: f32);
       fn report_overall_progress(&self, progress: f32);
       fn report_log(&self, message: &str);
       fn report_error(&self, error: &str);
   }
   ```

### 📁 Key Files to Work With

1. `crates/ai_rpg_generator/src/app/steps/generation.rs` - Needs bootstrap orchestrator integration
2. `crates/ai_rpg_generator/src/app/bootstrap/` - Contains the orchestrator to integrate
3. `crates/ai_rpg_game_build_tools/src/assets/` - Source for Phase 2 migration
4. `crates/ai_rpg_game_build_tools/src/database.rs` - Database schema reference

### 🐛 Watch Out For

1. **Async/Sync Bridge**: The GUI is sync but database operations are async. Use `tokio::runtime::Runtime::new()` pattern.
2. **Error Handling**: Don't panic on database errors, propagate them properly.
3. **Progress Updates**: The GenerationReporter needs to update the UI in real-time.

### 💡 Architecture Notes

- The wizard owns the database and defaults loader as Arc<T>
- Steps receive these via builder pattern methods
- Generation has 3 phases that build on each other
- Each phase outputs to specific directories under build-dir/game-name/

### 🎮 Testing the Changes

```bash
# Run with custom build directory and database
cargo run --bin ai_rpg_generator -- --build-dir /tmp/test-build --game-db test_game

# The GUI should now:
# 1. Show dynamically loaded presets in dropdown
# 2. Apply preset configurations to ECS tree
# 3. Pass through to generation step properly
```

---

**Priority**: Get the BootstrapOrchestrator connected and generating Phase 1 outputs (prompts, templates, runtime).

Last Updated: 2025-07-31 01:20:00 CST
