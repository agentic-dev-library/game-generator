# Active Context - Vintage Game Generator

## Current Work: AI Client Extraction and Standalone Publication

### What Was Completed
1. **Extracted AI Client to standalone `ai-client-rs` crate**:
   - Multi-provider support (OpenAI + Anthropic)
   - Intelligent caching (Memory + Disk with Zstd compression)
   - Accurate token counting and cost estimation
   - Support for Text, Image, Audio (TTS), and Embeddings
   - Clean, generic API separated from game-specific logic
   - Comprehensive documentation (README) and examples
   - Standard GitHub Actions CI/CD workflow
   - Verified build and example compilation

2. **Implemented Anthropic Support**:
   - Basic implementation using `reqwest` for Claude 3 models
   - Pricing and token estimation for Claude 3 Opus, Sonnet, and Haiku
   - Integrated with the unified `TextGenerator` interface

3. **Created `bevy-combat` crate as a reusable combat system template**:
   - Extracted from `cosmic-cults` (simulated) as a generic framework
   - Damage calculation supporting multiple types (Eldritch, Corrupted, etc.)
   - Effect stacking system with thematic flavors (Madness, Void Corruption)
   - Progression system for XP and leveling
   - Combat state machine using Bevy States

4. **Implemented MinJinja template system for all AI prompts**:
   - System prompts (game designer, dialogue writer)
   - Text generation prompts (descriptions, marketing, code)
   - Image generation prompts (sprites, tilesets, style guides)
   - Audio generation prompts (music, sound effects)
   - Style consistency prompts

5. **Fixed Build Environment**:
   - Installed Rust and cargo (v1.92.0)
   - Verified crate compilation and resolved `async-openai` 0.32 API changes

### Current State
The project now has a dedicated, production-ready AI client library at `/workspace/ai-client-rs/`:
- **Independent Workspace**: Can be moved out of the main repository easily
- **Clean API**: No dependencies on `vintage_*` crates
- **Multi-Provider**: Unified interface for OpenAI and Anthropic
- **Robust Caching**: Saves costs and improves performance
- **Ready for crates.io**: Includes necessary metadata and documentation

### Key Technical Details

#### Multi-Provider Abstraction
- `AiService` coordinates providers and shared resources (cache, token counter)
- `TextGenerator` handles provider routing based on model name or explicit config
- `AiGenerator` trait provides common interface for all generation services

#### Caching System
- Two-tier caching: Fast in-memory (RwLock<HashMap>) + Persistent disk (Bincode + Zstd)
- Cache keys generated from hash of prompt and parameters
- Automatic TTL and expiration management
- Stats tracking for hits, misses, and cost savings

#### Token Management
- Uses `tiktoken-rs` for OpenAI models
- Approximates Anthropic token usage
- Built-in pricing for accurate cost tracking across different models and providers

### Next Steps
1. **Integrate `ai-client-rs` back into the main project** (optional):
   - Update `vintage_ai_client` to depend on `ai-client-rs`
   - Refactor `vintage_ai_client` to be a game-specific wrapper around `ai-client-rs`

2. **Publishing**:
   - Upload to crates.io (requires API token)
   - Create jbcom/rust-ai-client repository on GitHub

3. **Future Enhancements**:
   - Add more providers (Google Gemini, Mistral, etc.)
   - Implement more advanced conversation management in a generic way
   - Add support for vision models in `ImageGenerator`

### Important Notes
- The new `ai-client-rs` crate is independent and uses recent Rust features (2024 edition)
- API keys are expected from environment variables (`OPENAI_API_KEY`, `ANTHROPIC_API_KEY`)
- The crate is designed to be lightweight and extensible

## Session: 2025-12-31 07:40
### Completed
- [x] Fixed CI in `game-generator` PR #51 (Bevy 0.17, bincode 2.x migration)
- [x] Merged PR #51 into `game-generator` main branch
- [x] Fixed `control-center` Rust CI template with missing dependencies (PR #8)
- [x] Overhauled `.gitignore` in `control-center` to prevent tracking of `target/` and other build artifacts
- [x] Cleaned up accidentally tracked `target/` directory in `game-generator`
- [x] Verified organization-wide sync is now propagating correct configurations

### For Next Agent
- [ ] **Game Generator Split**: Proceed with Issue #21 (Epic: Split repository). Now that CI is stable, the repo can be split into `rust-agentic-game-development` (core lib) and `rust-agentic-game-generator` (app).
- [ ] **Triage PR #94**: Investigate CodeQL and SonarCloud failures in the massive "marketplace-ready bots" PR.
- [ ] **Control Center PR #7**: Check if the sync mechanism overhaul is ready for merge.
