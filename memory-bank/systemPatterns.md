# System Patterns

**Last Updated**: 2025-08-02 01:45 CST

## Core Architectural Patterns

### Pattern 1: Database-First Architecture
- **Pattern**: Store all game state in SQLite via SeaORM
- **Implementation**: ECS components are simple markers with UUID references to database entities
- **Benefits**: Simplified Rust code, leverages AI's strength in SQL generation
- **Example**: `#[derive(Component)] struct Player { id: Uuid }`

### Pattern 2: Multi-Phase Generation Pipeline
- **Pattern**: Wizard → Metaprompt → Prompt → Content → Integration
- **Implementation**: Each phase builds on the previous, with database persistence between phases
- **Benefits**: Resumable generation, clear separation of concerns
- **Key Classes**: `GenerationPhase`, `GenerationTask`, `PhaseTransition`

### Pattern 3: Event-Driven Communication
- **Pattern**: Components communicate via Bevy events, not direct coupling
- **Implementation**: `EventWriter<T>` and `EventReader<T>` for all inter-system communication
- **Benefits**: Loose coupling, testability, clear data flow
- **Example**: `WizardCompleteEvent` triggers generation pipeline

### Pattern 4: Template-Based Code Generation
- **Pattern**: MinJinja templates generate Rust code from database models
- **Implementation**: Templates in `templates/`, processed by `PromptEngine`
- **Benefits**: Consistent code generation, easy to modify patterns
- **Note**: Moving to metaprompt-driven generation

### Pattern 5: Resource-Based Configuration
- **Pattern**: Configuration stored as Bevy Resources, not globals
- **Implementation**: `#[derive(Resource)]` for all config structs
- **Benefits**: Testable, injectable, follows Bevy patterns
- **Example**: `DatabaseConfig`, `GeneratorConfig`, `StudioConfig`

### Pattern 6: Plugin-Based Architecture
- **Pattern**: Features implemented as Bevy plugins
- **Implementation**: `impl Plugin for XPlugin` with clear setup/teardown
- **Benefits**: Modular, optional features, clear dependencies
- **Example**: `WizardPlugin`, `GeneratorPlugin`, `StudioPlugin`

### Pattern 7: Async Task Management
- **Pattern**: Long-running tasks use `bevy_tokio_tasks`
- **Implementation**: Spawn async tasks that communicate via channels
- **Benefits**: Non-blocking UI, cancellable operations
- **Example**: AI API calls, asset generation, database operations

### Pattern 8: Error Propagation via Events
- **Pattern**: Errors sent as events, not returned
- **Implementation**: `ErrorEvent` types for each subsystem
- **Benefits**: Centralized error handling, UI can react appropriately
- **Example**: `GenerationErrorEvent`, `DatabaseErrorEvent`

### Pattern 9: State Machine Navigation
- **Pattern**: UI flows modeled as state machines
- **Implementation**: Enum states with transition validation
- **Benefits**: Clear flow, impossible states unrepresentable
- **Example**: `WizardStep` enum with `can_transition_to()`

### Pattern 10: Asset Pipeline Integration
- **Pattern**: Generated assets go through Bevy asset pipeline
- **Implementation**: Write to `assets/` directory, use `AssetServer`
- **Benefits**: Hot reloading, consistent asset handling
- **Example**: Generated sprites loaded via `asset_server.load()`

### Pattern 11: Wizard State Management
- **Pattern**: Multi-step configuration with validation at each step
- **Implementation**: Centralized state with step-specific validation
- **Benefits**: Guided user experience, prevents invalid configurations
- **Example**: `WizardState` with `validate_step()` per step

### Pattern 12: Event-Driven Integration
- **Pattern**: Major components integrate via events, not direct calls
- **Implementation**: `WizardCompleteEvent` → `GenerationStartEvent` → etc.
- **Benefits**: Loose coupling between wizard, generator, and studio
- **Example**: Wizard doesn't know about generator implementation

### Pattern 13: Mutable UI State
- **Pattern**: UI components directly mutate state for responsiveness
- **Implementation**: `&mut WizardState` in UI systems
- **Benefits**: Immediate feedback, simpler than message passing
- **Example**: Text inputs directly update state fields

### Pattern 14: Metaprompt Architecture ⭐ NEW
- **Pattern**: Three-level hierarchy for dynamic prompt generation
- **Implementation**: Game Config → Metaprompts → Prompts → Game Content
- **Benefits**: Infinite flexibility, feature-driven generation, maintainable
- **Example**: `metaprompts/database/schema_generator.jinja` analyzes features
- **Key Insight**: Metaprompts generate prompts based on game configuration

### Pattern 15: Build Tools Separation ⭐ NEW
- **Pattern**: Build-time logic separated into dedicated crate
- **Implementation**: `vintage_build_tools` crate handles all build operations
- **Benefits**: Clean separation of concerns, reusable, testable
- **Example**: `VintageBuildTools::new(api_key, start, end).build()`
- **Key Components**: API client, image downloader, graph builder, template processor

### Pattern 16: Compiled Data Over Runtime Files ⭐ NEW
- **Pattern**: Generate Rust code instead of loading JSON at runtime
- **Implementation**: Templates generate `.rs` files with static data
- **Benefits**: No runtime parsing, type safety, better performance
- **Example**: `vintage_games::TIMELINE_GAMES` as static array
- **Note**: All game data compiled into binary

### Pattern 17: Pre-computed Similarity Graphs ⭐ NEW
- **Pattern**: Calculate game similarities at build time
- **Implementation**: `GraphBuilder` uses `vintage_blending_core` during build
- **Benefits**: No runtime computation, instant blending operations
- **Example**: Graph data embedded in `graph.rs` module

### Pattern 18: Unified AI Configuration ⭐ NEW
- **Pattern**: Single source of truth for AI configuration across crates
- **Implementation**: `AiConfig` struct in `vintage_ai_client` with Bevy Resource support
- **Benefits**: Consistent configuration, CLI integration, type safety
- **Example**: `--text-model gpt-4 --temperature 1.2 --image-quality hd`
- **Key Features**: Automatic validation, builder pattern, feature-gated Bevy integration

## Testing Patterns

### Testing Pattern 1: GUI Test Harness
- **Pattern**: Headless Bevy app for testing UI
- **Implementation**: `TestHarness` with `MinimalPlugins`
- **Benefits**: Fast tests, no window required
- **Example**: `tests/gui/test_harness.rs`

### Testing Pattern 2: Database Test Fixtures
- **Pattern**: In-memory SQLite for tests
- **Implementation**: `create_test_database()` helper
- **Benefits**: Fast, isolated, no cleanup needed
- **Example**: `tests/gui/test_database.rs`

### Testing Pattern 3: Async Test Utilities
- **Pattern**: `tokio::test` for async test functions
- **Implementation**: Test utilities return futures
- **Benefits**: Test async code naturally
- **Example**: `test_generation_pipeline()`

### Testing Pattern 4: VCR Testing
- **Pattern**: Record and replay API interactions
- **Implementation**: `reqwest-vcr` for deterministic tests
- **Benefits**: Test without API keys, consistent results
- **Example**: `tests/metaprompt/vcr_tests.rs`

## Code Organization Patterns

### Module Organization
```
src/
├── wizard/       # User configuration UI
├── generator/    # Metaprompt and generation logic
├── database/     # SeaORM entities and migrations
├── services/     # Business logic layer
├── studio/       # Development environment UI
└── integration/  # Cross-module integration

crates/
├── vintage_blending_core/  # Game blending algorithms
├── vintage_build_tools/    # Build-time data generation
└── vintage_game_generator/ # Main application
```

### Build Tools Organization ⭐ NEW
```
vintage_build_tools/
├── lib.rs        # Public API
├── types.rs      # API data structures
├── api.rs        # External API clients
├── images.rs     # Asset downloading
├── graph.rs      # Pre-computation logic
├── templates.rs  # Code generation
└── generator.rs  # Orchestration
```

### Naming Conventions
- **Events**: `{Action}{Target}Event` (e.g., `GenerateAssetEvent`)
- **Resources**: `{Domain}Config` or `{Domain}State`
- **Components**: Simple nouns (e.g., `Player`, `Enemy`)
- **Systems**: `{action}_{target}_system` (e.g., `update_wizard_system`)

### Error Handling
- **Pattern**: Use `thiserror` for error types
- **Implementation**: Domain-specific error enums
- **Benefits**: Rich error information, good error messages
- **Example**: `GeneratorError`, `DatabaseError`

## Performance Patterns

### Asset Loading
- **Pattern**: Use `bevy_asset_loader` for declarative loading
- **Implementation**: Loading states with asset collections
- **Benefits**: Automatic loading, progress tracking
- **Example**: `#[derive(AssetCollection)]` for game assets

### Batch Processing
- **Pattern**: Process multiple items in single database transaction
- **Implementation**: Collect operations, execute in batch
- **Benefits**: Reduced database overhead
- **Example**: Batch insert generated entities

### Caching Strategy
- **Pattern**: Use `cached` crate for expensive computations
- **Implementation**: Function-level caching with TTL
- **Benefits**: Avoid regenerating identical content
- **Example**: Cache metaprompt outputs

### Build-Time Optimization ⭐ NEW
- **Pattern**: Move expensive operations to build time
- **Implementation**: Pre-compute data during build, embed as static
- **Benefits**: Zero runtime cost, instant access
- **Example**: Game similarity calculations done in build.rs

## Integration Patterns

### Database Integration
- **Pattern**: Services layer mediates between UI and database
- **Implementation**: Service functions return domain types, not entities
- **Benefits**: UI doesn't know about database schema
- **Example**: `ProjectService::create_project()`

### AI Provider Integration
- **Pattern**: Provider-agnostic interface with multiple implementations
- **Implementation**: Trait for AI operations, provider-specific impls
- **Benefits**: Easy to add new providers, switchable at runtime
- **Example**: `AiClient` trait with OpenAI, Anthropic impls

### Asset Pipeline Integration
- **Pattern**: Generated assets follow standard Bevy asset flow
- **Implementation**: Write to filesystem, load via AssetServer
- **Benefits**: Leverages existing Bevy infrastructure
- **Example**: Generated sprites hot-reload automatically

### External API Integration ⭐ NEW
- **Pattern**: Dedicated client modules for external APIs
- **Implementation**: Typed clients with error handling and rate limiting
- **Benefits**: Type safety, consistent error handling, respectful of APIs
- **Example**: `GiantBombClient` with automatic rate limiting

## Future Patterns (Planned)

### Multi-Provider Fallback
- **Pattern**: Automatic fallback between AI providers
- **Implementation**: Try primary, fall back on rate limit/error
- **Benefits**: Resilient generation, cost optimization

### Incremental Generation
- **Pattern**: Generate game in resumable chunks
- **Implementation**: Track progress in database, resume from last point
- **Benefits**: Handle long generation times, API failures

### Style Consistency
- **Pattern**: Ensure visual consistency across generated assets
- **Implementation**: Style embedding passed to all generation calls
- **Benefits**: Cohesive art style throughout game

## Anti-Patterns to Avoid

### ❌ Complex ECS Components
- **Don't**: Store game logic in components
- **Do**: Use components as markers, logic in systems
- **Why**: AI generates data better than behavior

### ❌ Hardcoded Prompts
- **Don't**: Write specific prompts for every scenario
- **Do**: Use metaprompts to generate prompts dynamically
- **Why**: Inflexible, unmaintainable with many combinations

### ❌ Direct Module Coupling
- **Don't**: Import types from other modules directly
- **Do**: Use events and services for communication
- **Why**: Makes testing and refactoring difficult

### ❌ Blocking Operations in Systems
- **Don't**: Perform I/O in Bevy systems
- **Do**: Use async tasks with channel communication
- **Why**: Blocks entire frame, causes stuttering

### ❌ Global State
- **Don't**: Use static variables or singletons
- **Do**: Use Bevy Resources and States
- **Why**: Untestable, causes race conditions

### ❌ Runtime Data Loading ⭐ NEW
- **Don't**: Load configuration files at runtime
- **Do**: Generate Rust code with embedded data
- **Why**: Runtime parsing overhead, potential failure points

### ❌ Monolithic Build Scripts ⭐ NEW
- **Don't**: Put all build logic in build.rs
- **Do**: Extract to dedicated build tools crate
- **Why**: Unmaintainable, not reusable, hard to test
