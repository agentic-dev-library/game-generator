# Supplemental Documentation Index

**Last Updated**: 2025-08-02 01:49 CST

## Active Documentation

### Current Architecture
- **`metaprompt-generator.md`** - Revolutionary AI-driven game generation architecture
  - Complete conversation from the design session
  - Five-phase metaprompt chain explanation
  - Integrated studio with SeaORM persistence
  - Style consistency techniques
  - **STATUS: PRIMARY REFERENCE**

- **`integration-guide.md`** - Guide for tying together the two halves of the architecture
  - Wizard → Generator flow implementation
  - Database integration points
  - Event system design
  - Implementation status and next steps
  - **STATUS: ACTIVE IMPLEMENTATION GUIDE**

- **`metaprompt-testing-strategy.md`** - Testing strategy using GUI harness + VCR
  - Leverages existing test harness infrastructure
  - VCR-style API recording for deterministic tests
  - Visual regression testing with screenshots
  - Complete end-to-end flow validation
  - **STATUS: ACTIVE TESTING GUIDE**

### Build System Documentation (2025-08-02)
- **`build-tools-architecture.md`** - Complete architecture for vintage_build_tools crate
  - Separate crate for all build-time logic
  - GiantBomb API integration with proper types
  - Image downloading for game covers
  - Graph pre-computation using vintage_blending_core
  - Template-based Rust code generation
  - Clean separation of build vs runtime concerns
  - **STATUS: ACTIVE BUILD SYSTEM REFERENCE**

### Architecture Evolution (2025-08-01)
- **`metaprompt-architecture.md`** - Revolutionary metaprompt hierarchy design
  - Three-level architecture: Game Config → Metaprompts → Prompts → Game Content
  - Metaprompts analyze game features and generate tailored prompts
  - Database-first philosophy with simple ECS markers
  - Complete architectural shift from hardcoded prompts to dynamic generation
  - **STATUS: PRIMARY ARCHITECTURE REFERENCE**

- **`wizard-to-toml-architecture.md`** - Simplified architecture using wizard + TOML
  - Keep wizard UI for structured data collection
  - Use config-rs to save GameConfiguration to TOML
  - No database needed - just file-based storage
  - Environment variables for API keys
  - Clean integration: Wizard → TOML → Metaprompts → Game
  - **STATUS: CHOSEN ARCHITECTURE**

- **`wizard-ai-hybrid-complete-plan.md`** - Complete plan for wizard + AI conversation hybrid
  - Comprehensive 7-step wizard for structured data collection
  - AI conversation phase that builds on wizard context
  - Detailed implementation architecture and data flow
  - **STATUS: COMPLETE IMPLEMENTATION PLAN**

### Migration & Progress Documents
- **`minijinja-migration-complete.md`** - MinJinja migration completion summary
  - Complete template migration from Tera to MinJinja
  - LocalLLMClient integration with templates
  - Ecosystem libraries added for asset management
  - **STATUS: MIGRATION COMPLETE**

- **`ecosystem-leverage-progress.md`** - Progress on replacing custom code with ecosystem libraries
  - Tracks completed replacements (VCR, caching, templates)
  - Documents valuable custom code to keep (build.rs, pixel art processor)
  - Plans for next ecosystem additions (tilemap, audio, dialog)
  - **STATUS: ACTIVE TRACKING DOCUMENT**

### Technical References
- **`egui-image-loading-approach.md`** - Pure egui image loading alignment with bevy-inspector-egui
  - Moved from Bevy AssetServer to pure egui texture loading
  - Complete image_loader module implementation
  - Overlay system using native egui Areas and layers
  - Aligned dependencies with bevy-inspector-egui
  - **STATUS: ACTIVE UI ARCHITECTURE**

- **`allms-architecture-redesign.md`** - allms + async-openai Architecture Redesign
  - Replace PromptEngine with allms for ALL text generation
  - Use async-openai ONLY for image/audio generation
  - Type-safe responses with `get_answer::<T>()`
  - Multi-provider support with automatic failover
  - **STATUS: ACTIVE ARCHITECTURE IMPROVEMENT**

- **`metaprompt-phase-architecture-complete.md`** - Complete phase-based metaprompt implementation
  - Implemented all 5 phases of game generation with dynamic JSON
  - Created phase-specific templates in metaprompts/phases/
  - Master orchestrator routing system
  - **STATUS: IMPLEMENTATION COMPLETE**

## Implementation Progress

### Completed Components
- **Wizard Navigation System** (2025-07-31)
  - All 8 wizard steps fully wired
  - Validation integrated into navigation
  - Event system triggers on completion
  - Direct state mutation pattern implemented

- **Build Tools System** (2025-08-02)
  - Separate vintage_build_tools crate created
  - GiantBomb API integration complete
  - Image downloading functionality
  - Graph pre-computation with vintage_blending_core
  - Template-based code generation
  - Modular template structure (5 separate files)

### Active Development
- **Integration System**
  - WizardCompleteEvent → Generation pipeline
  - Progress event handling
  - Studio UI updates
  - vintage_games module integration needed

- **VCR Testing**
  - Infrastructure created
  - Recording mode pending
  - Cassette management needed

## Archived Documentation

**Note**: The `memory-bank/supplemental/` directory contains numerous archived documents from the previous template-based architecture. These are retained for historical reference but are **OBSOLETE** and should not be used for current development. The archived documents include old implementation guides, progress reviews, migration documents, and the previous tier system documentation.

## Document Management Policy

### Active Documents
Documents that directly support the current metaprompt-based architecture should be kept in the main supplemental directory and updated as needed.

### Creating New Documents
When creating new supplemental documentation:
1. Use descriptive names with dates
2. Add to this index immediately
3. Focus on metaprompt architecture
4. Cross-reference related documents

## Quick Reference

For current development, focus on:
1. **`metaprompt-generator.md`** - Core architecture and approach
2. **`integration-guide.md`** - Connecting wizard to generator
3. **`metaprompt-testing-strategy.md`** - Testing approach
4. **`build-tools-architecture.md`** - Build system design
5. **Active memory bank files** - Current state and patterns

## Recent Patterns Added to System Patterns

The following patterns were added to systemPatterns.md on 2025-08-02:

1. **Pattern 15: Build Tools Separation** - Build-time logic in dedicated crate
2. **Pattern 16: Compiled Data Over Runtime Files** - Generate Rust code not JSON
3. **Pattern 17: Pre-computed Similarity Graphs** - Build-time graph calculation
4. **Build-Time Optimization** - Move expensive ops to build time
5. **External API Integration** - Typed clients with rate limiting
6. **Anti-pattern: Runtime Data Loading** - Don't load config at runtime
7. **Anti-pattern: Monolithic Build Scripts** - Extract to build tools crate

These patterns emerged from the build tools refactoring and should guide future build system development.
