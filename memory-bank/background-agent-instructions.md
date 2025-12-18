# Background Agent Instructions - AI RPG Generator

## 🎯 Primary Objective

Continue the database integration and generator wiring for the AI RPG Generator GUI. The UI framework is complete - now connect it to the actual generation systems.

## 📋 Task List (Priority Order)

### 1. Complete Database Integration ⚡
- [ ] Implement `load_ecs_from_database()` in `game_config.rs`
- [ ] Use the passed `database_manager` Arc to query actual ECS data
- [ ] Remove the mock data fallback completely
- [ ] Test with actual database queries

### 2. Bootstrap Orchestrator Integration (Phase 1) 🚀
- [ ] In `generation.rs`, connect the BootstrapOrchestrator
- [ ] Pass the database_manager Arc to orchestrator
- [ ] Implement GenerationReporter for real-time progress
- [ ] Generate outputs to `<build-dir>/<game-name>/prompts/`, `/templates/`, `/runtime/`

### 3. Asset Generator Migration (Phase 2) 🎨
- [ ] Create `crates/ai_rpg_generator/src/app/game/assets/` directory structure
- [ ] Migrate generators from `crates/ai_rpg_game_build_tools/src/assets/`
- [ ] Skip the `code/` subdirectory (that's Phase 3)
- [ ] Connect to database for asset queries
- [ ] Consume prompts/templates from Phase 1

### 4. Code Generator Migration (Phase 3) 💻
- [ ] Migrate code generation from `build_tools/src/assets/code/`
- [ ] Create final game structure in `<build-dir>/<game-name>/game/`
- [ ] Use templates and assets from previous phases

## 🔧 Technical Guidelines

### Arc Reference Pattern
```rust
// NEVER create new instances - always use shared Arcs
something.with_database(database_manager.clone())
         .with_defaults(defaults_loader.clone())
```

### No Mock Data Policy
- NO hardcoded lists
- NO mock fallbacks  
- NO placeholder data
- Everything from database or defaults loader

### Async/Sync Bridge
```rust
// GUI is sync, database is async
let rt = tokio::runtime::Runtime::new().unwrap();
rt.block_on(async {
    // database operations here
})
```

### Generation Reporter
```rust
// Already defined in generation.rs
// Implement for each generator module
impl GenerationReporter for YourReporter {
    fn report_progress(&self, task: &str, progress: f32) {
        // Update UI
    }
}
```

## 📁 Key Files Reference

- `crates/ai_rpg_generator/src/app/steps/generation.rs` - Generation step UI
- `crates/ai_rpg_generator/src/app/bootstrap/` - Orchestrator to integrate
- `crates/ai_rpg_game_build_tools/src/database.rs` - Database schema
- `crates/ai_rpg_game_build_tools/src/assets/` - Source for migrations

## 🧪 Testing Commands

```bash
# Test with custom paths
cargo run --bin ai_rpg_generator -- --build-dir /tmp/rpg-test --game-db test_game

# Check preset loading
# Should see "✅ Loaded X presets from defaults" in console

# Verify generation outputs
ls -la /tmp/rpg-test/[game-name]/
```

## ⚠️ Critical Reminders

1. **Jsonpath Usage**: Use `defaults_loader.query("path.to.data")` for all config queries
2. **Error Handling**: Propagate errors, don't panic or use unwrap in production code
3. **Progress Reporting**: Keep UI responsive with frequent progress updates
4. **Directory Creation**: Ensure all output directories are created before writing

## 🎮 Success Criteria

- [ ] Presets load dynamically from filesystem
- [ ] ECS world shows actual database content
- [ ] Generation creates real output files
- [ ] Progress bar updates during generation
- [ ] No hardcoded data anywhere

## 📝 Notes from Tonight

- Fixed Arc sharing between wizard and steps
- Removed ALL hardcoded preset lists
- Implemented preset to ECS configuration converter
- User emphasized: NO mock data, NO fallbacks

---

Good luck! The foundation is solid - just need to connect the pipes. 🚀

Last Updated: 2025-07-31 01:25:00 CST
