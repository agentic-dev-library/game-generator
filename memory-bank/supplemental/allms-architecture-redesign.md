# allms + async-openai Architecture Redesign

## Overview

Replace the complex PromptEngine with a cleaner architecture using:
- **allms**: ALL text generation and file analysis (multi-provider)
- **async-openai**: ONLY image (DALL-E) and audio (TTS/Whisper) generation

## Key Benefits

1. **Simplified Architecture**
   - Remove PromptEngine entirely
   - Direct metaprompt → allms → typed response
   - No manual prompt assembly/parsing

2. **Multi-Provider Support**
   - allms handles OpenAI, Anthropic, Google, AWS Bedrock, etc.
   - Automatic failover between providers
   - Consistent API across all providers

3. **Type-Safe Responses**
   - `get_answer::<T>()` automatically deserializes to Rust types
   - No manual JSON parsing
   - Compile-time type safety

## Implementation Pattern

```rust
use allms::{Completions, llm::{OpenAIModels, AnthropicModels, GoogleModels}};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

// Define response types for each phase
#[derive(Deserialize, Serialize, JsonSchema)]
struct StyleGuideResponse {
    palette: ColorPalette,
    shading_technique: String,
    outline_style: String,
    // ... other fields
}

#[derive(Deserialize, Serialize, JsonSchema)]
struct DatabaseSchemaResponse {
    tables: Vec<TableDefinition>,
    relationships: Vec<Relationship>,
    // ... other fields
}

// Metaprompt executor
struct MetapromptExecutor {
    env: minijinja::Environment,
    providers: Vec<Box<dyn LLMProvider>>,
}

impl MetapromptExecutor {
    async fn execute_metaprompt<T: JsonSchema + DeserializeOwned>(
        &self,
        metaprompt_name: &str,
        context: &GameConcept,
    ) -> Result<T> {
        // 1. Render metaprompt with MinJinja
        let template = self.env.get_template(metaprompt_name)?;
        let prompt = template.render(context)?;
        
        // 2. Execute with allms (automatic provider selection)
        let completions = Completions::new(
            self.get_best_provider(),
            &self.api_key,
            None,
            None
        );
        
        // 3. Get typed response
        completions.get_answer::<T>(&prompt).await
    }
}
```

## Configuration Format

```toml
# metaprompt_config.toml
[phases.style_guide]
metaprompt = "metaprompts/style_guide_generator.jinja"
response_type = "StyleGuideResponse"
providers = ["anthropic", "openai", "google"]  # Priority order
temperature = 0.7

[phases.database_schema]
metaprompt = "metaprompts/database/schema_generator.jinja"
response_type = "DatabaseSchemaResponse"
providers = ["openai", "anthropic"]  # Different provider preferences
temperature = 0.3

[phases.sprite_generation]
# This phase would use async-openai for DALL-E
type = "image"
prompt_template = "metaprompts/sprites/sprite_generator.jinja"
model = "dall-e-3"
size = "1024x1024"
quality = "hd"
```

## Migration Steps

1. **Add allms to Cargo.toml**
   ```toml
   [dependencies]
   allms = "0.5"
   async-openai = "0.29"  # Keep for images/audio only
   ```

2. **Remove PromptEngine**
   - Delete `src/generator/prompt_engine.rs`
   - Remove complex prompt assembly logic

3. **Create Response Types**
   - Define JsonSchema types for each phase
   - Mirror the expected AI output structure

4. **Implement MetapromptExecutor**
   - Simple wrapper around allms
   - Handles provider selection/failover
   - Returns typed responses

5. **Update Client Modules**
   - `async_openai.rs` → Only for images/audio
   - Create `allms_client.rs` → All text generation

## Example Usage

```rust
// Phase 1: Generate style guide
let style_guide: StyleGuideResponse = executor
    .execute_metaprompt("style_guide_generator.jinja", &game_concept)
    .await?;

// Phase 2: Generate database schema
let schema: DatabaseSchemaResponse = executor
    .execute_metaprompt("database/schema_generator.jinja", &game_concept)
    .await?;

// Phase 3: Generate sprite (using async-openai)
let sprite_prompt = executor.render_template("sprites/sprite_generator.jinja", &context)?;
let image = openai_client.create_image(&sprite_prompt).await?;
```

## Provider-Specific Features

allms supports advanced features per provider:
- **OpenAI**: Assistants API, Vector Stores, File uploads
- **Anthropic**: Computer use, artifacts
- **Google**: Grounding, code execution
- **AWS Bedrock**: Converse API

We can leverage these for specific tasks:
```rust
// Use OpenAI Assistant for complex multi-file analysis
let assistant = OpenAIAssistant::new(OpenAIModels::Gpt4o, &api_key)
    .vector_store(vector_store)
    .await?
    .get_answer::<ComplexAnalysis>("Analyze these game files", &[])
    .await?;
```

## Benefits Summary

1. **Cleaner Code**: No manual prompt construction
2. **Type Safety**: Compile-time guarantees on AI responses
3. **Multi-Provider**: Automatic failover and cost optimization
4. **Extensible**: Easy to add new providers/phases
5. **Testable**: Can mock allms responses easily
