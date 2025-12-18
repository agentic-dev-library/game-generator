# Minijinja + Database-Driven Architecture Migration

## Date: 2025-01-08

### Key Insight
Using `bevy_sqlx` for runtime game state dramatically simplifies AI generation requirements.

## What We've Implemented

### 1. Migrated from Tera to Minijinja
- Replaced `.tera` files with `.jinja` files for better tooling support
- Created cleaner prompt structure in `crates/ai_rpg_generator/prompts/`
- Added `prompt_engine.rs` using minijinja with template auto-discovery

### 2. Database-First Approach
Created new prompt template: `phases/database_schema.jinja`
- AI generates simple SQL migrations instead of complex Rust structs
- Database tables for:
  - `entities` - Core game objects with position/type
  - `sprites` - Visual representation  
  - Feature-specific tables (combat_stats, inventory, dialogue_state, etc.)
  - `world_tiles` - Persistent world changes
  - `game_state` - Global state storage

### 3. Simplified ECS Mapping
Created `code/ecs_mapper.jinja` template:
```rust
// Simple marker components - no data storage!
#[derive(Component)]
struct Player;

// Database reference
#[derive(Component)] 
struct DbEntity { id: Uuid }

// Sync from DB to ECS
fn sync_entities_from_db() {
    // Simple SQL query to load entities
    // Spawn Bevy entities with DbEntity component
}
```

## Benefits of This Approach

### For AI Generation:
1. **SQL is simpler** - AI excels at generating database schemas
2. **Less Rust complexity** - No need for complex trait implementations
3. **Standard patterns** - CRUD operations are well-understood
4. **Clearer structure** - Tables map directly to game concepts

### For Development:
1. **Save games for free** - Database persistence built-in
2. **Easy debugging** - Can inspect game state with SQL
3. **Simpler testing** - Database fixtures instead of mock components
4. **Hot reload potential** - Change DB values while game runs

### For Performance:
1. **Selective loading** - Query only needed entities
2. **Batch updates** - SQL transactions for consistency
3. **Built-in indexing** - Database optimization tools
4. **Memory efficiency** - ECS only holds active entities

## Migration Status

### Completed:
- ✅ Created minijinja templates for all phases
- ✅ Added database schema generation phase
- ✅ Created simplified ECS mapper template
- ✅ Updated prompt_engine.rs to use minijinja
- ✅ Added walkdir dependency for template loading

### Next Steps:
1. Remove old Tera-based metaprompt.rs code
2. Update GameGenerator to use new PromptEngine
3. Add bevy_sqlx or similar to generated games
4. Create standard DB ↔ ECS sync systems
5. Update tests to use new approach

## Example Generated Output

Instead of complex Rust:
```rust
#[derive(Component, Serialize, Deserialize)]
pub struct CombatStats {
    pub health: i32,
    pub max_health: i32,
    pub attack: i32,
    // ... many fields
}
```

AI generates simple SQL:
```sql
CREATE TABLE combat_stats (
    entity_id UUID PRIMARY KEY,
    health INT DEFAULT 100,
    max_health INT DEFAULT 100,
    attack INT DEFAULT 10
);
```

## Ecosystem Libraries to Add

For generated games:
- `bevy_sqlx` or custom integration
- `sqlx` with SQLite for single-player games
- `uuid` for entity IDs
- Standard Bevy components remain in ECS

## Architecture Pattern

```
User Input → Wizard → Game Concept
                ↓
         PromptEngine (minijinja)
                ↓
    Phase 1: Style Guide (visual rules)
                ↓
    Phase 2a: Database Schema (SQL)
    Phase 2b: Technical Architecture
                ↓
    Phase 3: Asset Generation (sprites, UI)
                ↓
    Phase 4: ECS Mapper (simple boilerplate)
                ↓
    Phase 5: Main Game Loop (standard structure)
```

## Key Principle
The database holds the "truth" of game state. ECS is just a performant view layer that syncs with the database. This aligns perfectly with AI capabilities - generating data schemas is much easier than generating complex runtime behavior.
