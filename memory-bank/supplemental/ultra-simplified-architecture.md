# Ultra-Simplified Architecture - Letting Metaprompts Drive Everything

**Created**: 2025-01-08
**Status**: Proposed Architecture
**Key Insight**: The AI is the engine - the code should just be the chassis

## Core Philosophy

Instead of complex abstractions and enterprise patterns, we embrace:
1. **Vertical Integration** - Everything for a feature in one file
2. **Metaprompt-Driven Flow** - Let the AI handle complexity
3. **Minimal State Management** - Just Wizard → Generation → Complete
4. **Direct Communication** - No complex pipelines or abstractions

## Architecture Overview

```rust
// Just 3 states, that's it
enum AppState {
    Wizard,      // Collect config
    Generation,  // Run 6-phase metaprompts
    Complete,    // Show results
}
```

## Key Simplifications

### 1. Single File Integration
- `main.rs` - Complete app with UI
- `metaprompts.rs` - All 6 phases in one file
- `config.rs` - Simple TOML handling
- That's basically it!

### 2. Direct Metaprompt Execution
```rust
// No complex abstractions, just:
let style_guide = generator.generate_style_guide(&toml_path).await?;
let architecture = generator.generate_architecture(&toml_path, &style_guide).await?;
// etc...
```

### 3. UI Simplicity
- Use egui directly, no complex dock systems
- Show progress inline
- Simple buttons and text

### 4. Generation Cache
- Simple file-based cache
- Resume from any phase
- No complex database

## The 6-Phase System

### Phase 1: Style Guide
- Input: TOML config
- Output: Comprehensive constraints document
- This becomes the "constitution" for all other phases

### Phase 2: Architecture
- Input: TOML + Style Guide
- Output: main.rs, lib.rs, Cargo.toml
- Sets up the skeleton

### Phase 3: Visual Assets
- Input: TOML + Style Guide
- Output: Detailed prompts for DALL-E/Midjourney
- Enforces pixel-perfect constraints

### Phase 4: Core Systems
- Input: TOML + Style Guide
- Output: Movement, camera, tilemap systems
- Basic game loop

### Phase 5: Features
- Input: TOML + Style Guide + Enabled features
- Output: Feature-specific systems
- Only generates what's enabled

### Phase 6: Integration
- Input: Everything so far
- Output: Final wiring, README, missing pieces
- Makes it all work together

## Benefits of This Approach

1. **Faster Development** - Less code to write/maintain
2. **Better AI Integration** - AI excels with clear, focused prompts
3. **Easier Debugging** - Everything is in one place
4. **Simpler Testing** - Just test the output, not the machinery
5. **More Flexible** - Easy to add new phases or modify prompts

## Implementation Strategy

1. **Strip Down Current Code**
   - Remove all the complex abstractions
   - Delete unused modules
   - Simplify the wizard to just save TOML

2. **Create Ultra-Simple main.rs**
   - Basic Bevy app
   - Simple egui UI
   - Direct metaprompt calls

3. **Implement metaprompts.rs**
   - 6 simple async functions
   - Direct OpenAI API calls
   - Simple response parsing

4. **Add Minimal Features**
   - File-based caching
   - Progress updates
   - Error recovery

## What We're NOT Doing

- ❌ Complex dependency injection
- ❌ Abstract trait systems
- ❌ Multiple layers of indirection
- ❌ Enterprise patterns
- ❌ Over-engineered pipelines
- ❌ Complex state machines
- ❌ Database persistence (just files)

## Example Code Structure

```
src/
├── main.rs          # ~500 lines - complete app
├── metaprompts.rs   # ~600 lines - all 6 phases
├── config.rs        # ~100 lines - TOML handling
└── export.rs        # ~200 lines - Tauri bundling

metaprompts/
├── phase1_style_guide.txt
├── phase2_architecture.txt
├── phase3_assets.txt
├── phase4_core.txt
├── phase5_features.txt
└── phase6_integration.txt
```

## The Key Insight

We've been thinking like traditional software engineers when we should be thinking like AI orchestrators. The metaprompts ARE the application logic - the Rust code is just there to:

1. Show a UI
2. Call the AI
3. Save the results
4. Show progress

That's it. Everything else is overengineering.

## Next Steps

1. Create a proof-of-concept with just main.rs
2. Test the 6-phase generation
3. If it works, migrate everything to this model
4. Delete 90% of the current code

## Validation Approach

Instead of complex validation systems, we use:
1. **Metaprompt Self-Validation** - Each prompt includes validation rules
2. **Compilation Check** - Does the generated game compile?
3. **Visual Check** - Do the assets look right?
4. **Play Test** - Does it run?

If it passes these 4 checks, it's good enough.

## Conclusion

This approach aligns perfectly with the project's goal: let AI generate games. We've been building a complex machine when all we needed was a simple orchestrator. The metaprompts do the real work - our job is just to call them in the right order and save the results.
