# Wizard to TOML Architecture - The Perfect Balance

## Date: 2025-01-08

### The Realization

We already have a comprehensive `GameConfiguration` struct that captures everything the wizard collects. Instead of a complex database, we can simply:

1. **Keep the wizard UI** - It provides a nice structured way to collect game requirements
2. **Use config-rs** to serialize GameConfiguration to TOML
3. **No database needed** - Just save to `dev.toml` 
4. **Environment variables for API keys** - No need to store/encrypt in database
5. **Feed TOML to metaprompts** - The structured data guides AI generation

### Architecture Overview

```
Wizard UI (Bevy + egui)
    ↓
GameConfiguration struct
    ↓
config-rs → dev.toml
    ↓
MetapromptOrchestrator reads TOML
    ↓
Generate complete game
    ↓
bevy_inspector_egui for live editing
```

### Implementation

#### 1. Add config-rs to Save Configuration
```rust
// In wizard completion handler
use config::{Config, File, FileFormat};

pub fn save_game_configuration(config: &GameConfiguration) -> Result<()> {
    // Serialize to TOML
    let toml_string = toml::to_string_pretty(config)?;
    
    // Save to project config directory
    let config_dir = dirs::config_dir()
        .ok_or_else(|| anyhow!("Could not find config directory"))?
        .join("ai-rpg-generator");
    
    fs::create_dir_all(&config_dir)?;
    
    let config_path = config_dir.join("dev.toml");
    fs::write(&config_path, toml_string)?;
    
    println!("Configuration saved to: {}", config_path.display());
    Ok(())
}
```

#### 2. Environment Variables for API Keys
```rust
// No database storage needed!
pub struct ApiConfig {
    pub openai_key: String,
    pub anthropic_key: Option<String>,
    pub google_key: Option<String>,
}

impl ApiConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            openai_key: env::var("OPENAI_API_KEY")
                .context("OPENAI_API_KEY environment variable required")?,
            anthropic_key: env::var("ANTHROPIC_API_KEY").ok(),
            google_key: env::var("GOOGLE_AI_KEY").ok(),
        })
    }
}
```

#### 3. Load Configuration for Generation
```rust
pub async fn generate_from_config() -> Result<()> {
    // Load the saved configuration
    let config = Config::builder()
        .add_source(File::with_name("dev.toml"))
        .build()?;
    
    let game_config: GameConfiguration = config.try_deserialize()?;
    
    // Convert to format for metaprompts
    let concept = GameConcept::from(game_config);
    
    // Generate the game
    let orchestrator = MetapromptOrchestrator::new(...)?;
    let game = orchestrator.generate_game(&concept, progress_callback).await?;
    
    // Save generated game
    save_generated_game(&game, "./generated_game").await?;
}
```

#### 4. Example dev.toml Output
```toml
name = "Crystal Quest"
genre = "RPG"
tagline = "A retro RPG adventure in a crystalline world"
target_audience = "Core"
description = "Explore a world made of living crystals..."

[sprite_style]
detail_level = "Moderate"
use_outline = true
outline_color = "Black"
shading_style = "Simple"
pixel_size = 16

[[core_mechanics]]
value = "Combat"

[[core_mechanics]]
value = "Exploration"

[features.CombatSystem]
enabled = true
priority = "High"
combat_type = "RealTime"

[features.Inventory]
enabled = true
priority = "Medium"
inventory_slots = 24

# ... rest of configuration
```

### Benefits

1. **Best of Both Worlds**
   - Nice wizard UI for structured data collection
   - No database complexity
   - Simple file-based storage

2. **Developer Friendly**
   - Configuration is human-readable TOML
   - Can be edited manually if needed
   - Version controlled with project

3. **Clean Integration**
   - GameConfiguration → TOML → Metaprompts
   - No impedance mismatch
   - Type-safe all the way through

4. **Minimal Dependencies**
   - Just config-rs and toml crates
   - No SeaORM, no migrations
   - No encryption/decryption complexity

### What We Can Delete

1. **Entire Database Module**
   - `src/database/` - All of it
   - No entities, no migrations
   - No database connections

2. **Most Services**
   - `preference_service.rs` - Not needed
   - `project_service.rs` - Simplified to just config
   - Keep minimal `generation_service.rs`

3. **Complex State Management**
   - No need to persist wizard state
   - No session management
   - Just in-memory state during wizard

### New Simplified Flow

1. **Launch Wizard**
   ```bash
   cargo run --bin wizard
   ```

2. **Fill Out Configuration**
   - Use the existing wizard steps
   - All data goes into GameConfiguration
   - Nice UI with validation

3. **Save to TOML**
   - On completion, save to `~/.config/ai-rpg-generator/dev.toml`
   - User can review/edit if desired

4. **Generate Game**
   ```bash
   cargo run --bin generate
   ```
   - Reads dev.toml
   - Feeds to metaprompts
   - Generates complete game

5. **Run Generated Game**
   ```bash
   cd generated_game && cargo run
   ```
   - Includes bevy_inspector_egui
   - Live editing capabilities

### Migration Path

1. **Add config-rs to Cargo.toml**
2. **Add save_game_configuration() to wizard completion**
3. **Update main.rs to have wizard and generate modes**
4. **Delete database module and services**
5. **Update MetapromptOrchestrator to accept GameConfiguration**

### Conclusion

This approach keeps the valuable parts of the wizard (structured data collection with a nice UI) while eliminating all the complexity of databases and state management. The TOML file serves as both:
- A simple persistence mechanism
- A human-readable configuration format
- The input to the metaprompt system

It's clean, simple, and exactly what the project needs.
