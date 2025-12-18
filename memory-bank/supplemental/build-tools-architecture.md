# Build Tools Architecture

**Created**: 2025-08-02
**Purpose**: Document the vintage_build_tools crate architecture and design decisions

## Overview

The `vintage_build_tools` crate encapsulates all build-time logic for fetching game data from external APIs, pre-computing similarity graphs, and generating Rust code. This separation provides a clean architecture where build concerns don't pollute the runtime application.

## Architecture

### Crate Structure
```
crates/vintage_build_tools/
├── Cargo.toml
└── src/
    ├── lib.rs        # Public API (VintageBuildTools)
    ├── types.rs      # GiantBomb API data structures
    ├── api.rs        # GiantBombClient implementation
    ├── images.rs     # ImageDownloader for game covers
    ├── graph.rs      # GraphBuilder for pre-computation
    ├── templates.rs  # TemplateProcessor for code generation
    └── generator.rs  # GameDataGenerator orchestrator
```

### Component Responsibilities

#### VintageBuildTools (lib.rs)
- Public facade for the entire build process
- Simple API: `new(api_key, start_year, end_year).build()`
- Delegates to GameDataGenerator

#### Types (types.rs)
- Strongly typed representations of GiantBomb API responses
- Includes: Game, PlatformInfo, Developer, Genre, ImageInfo
- Constants for filtering (VINTAGE_PLATFORMS, TOP_GENRES_PER_YEAR)

#### GiantBombClient (api.rs)
- Handles all API communication with GiantBomb
- Methods:
  - `fetch_platforms()` - Get vintage platform information
  - `fetch_timeline_games()` - Get games by year range
  - `enhance_games_with_images()` - Fetch detailed image URLs
- Built-in rate limiting to respect API limits
- Error handling with fallbacks

#### ImageDownloader (images.rs)
- Downloads game cover images to local assets
- Saves to `assets/wizard/game_covers/{game_id}.jpg`
- Skips already downloaded images
- Rate limiting between downloads

#### GraphBuilder (graph.rs)
- Pre-computes game similarity using vintage_blending_core
- Converts API data to GameMetadata
- Calculates similarity scores between all games
- Generates graph statistics (hubs, average similarity)
- Outputs JSON structure for template consumption

#### TemplateProcessor (templates.rs)
- Uses MinJinja to generate Rust modules
- Templates located in `templates/giantbomb/`
- Generates:
  - `mod.rs` - Module exports
  - `games.rs` - Game data and functions
  - `platforms.rs` - Platform information
  - `eras.rs` - Era definitions
  - `graph.rs` - Pre-computed similarity data

#### GameDataGenerator (generator.rs)
- Orchestrates the entire build pipeline
- Steps:
  1. Check if data already exists (skip if so)
  2. Create API client
  3. Fetch platforms
  4. Fetch games timeline
  5. Enhance with images
  6. Convert to JSON format
  7. Download cover images
  8. Build similarity graph
  9. Generate Rust modules

## Design Decisions

### Why a Separate Crate?
1. **Clean Separation**: Build logic doesn't belong in the main application
2. **Reusability**: Other crates could use the build tools
3. **Testing**: Easier to test build logic in isolation
4. **Dependencies**: Build-only dependencies don't affect runtime

### Why Generate Rust Code?
1. **Performance**: No runtime parsing of JSON
2. **Type Safety**: Compiler validates all data
3. **Size**: Data is optimized by the compiler
4. **Simplicity**: No need for runtime file loading

### Why Pre-compute Graphs?
1. **Performance**: Similarity calculation is O(n²) - expensive at runtime
2. **Consistency**: All users get the same graph
3. **Predictability**: No variations in blending results

### Template vs Direct Generation
Templates provide:
- Flexibility to change generated code structure
- Separation of data processing from code generation
- Easy to add new generated files
- Version control friendly (can review template changes)

## Usage

### In build.rs
```rust
use vintage_build_tools::VintageBuildTools;

fn main() -> Result<()> {
    let api_key = std::env::var("GIANTBOMB_API_KEY")?;
    let build_tools = VintageBuildTools::new(api_key, 1980, 1995);
    build_tools.build()?;
    Ok(())
}
```

### Generated Module Structure
```rust
// src/vintage_games/mod.rs
pub mod games;
pub mod platforms;
pub mod eras;
pub mod graph;

pub use games::{TimelineGame, TIMELINE_GAMES, games_by_year, search_games};
pub use platforms::{Platform, PLATFORMS};
pub use eras::{Era, GameEra, ERAS};
pub use graph::{SIMILARITY_GRAPH, find_similar_games};
```

## Data Flow

```
GiantBomb API
    ↓
API Client (fetch & parse)
    ↓
Data Enhancement (images, metadata)
    ↓
Image Downloader (cover art)
    ↓
Graph Builder (similarity computation)
    ↓
Template Processor (code generation)
    ↓
Generated Rust Modules
```

## Error Handling

The build tools use a graceful degradation approach:
- Missing games for a year: Continue with other years
- API errors: Log warning and continue
- Image download failures: Skip that image
- Critical errors: Fail the build with clear message

## Performance Considerations

1. **Rate Limiting**: All API calls respect rate limits
2. **Caching**: Skip generation if files already exist
3. **Batch Processing**: Fetch data in year batches
4. **Parallel Downloads**: Images downloaded concurrently (with limits)

## Future Enhancements

1. **Incremental Updates**: Only fetch new/changed data
2. **Multiple APIs**: Support IGDB, MobyGames as alternatives
3. **Custom Filters**: Allow filtering games by criteria
4. **Offline Mode**: Work with cached API responses
5. **Configuration**: TOML/JSON config for build parameters

## Testing

The crate includes:
- Unit tests for each component
- Integration tests with mock API responses
- VCR tests for recording real API interactions
- Property tests for graph algorithms

## Dependencies

Runtime dependencies kept minimal:
- `reqwest` - HTTP client
- `serde` - Serialization
- `minijinja` - Template engine
- `anyhow` - Error handling
- `vintage_blending_core` - Graph algorithms

## Security Considerations

1. **API Keys**: Never committed, loaded from environment
2. **Rate Limiting**: Prevents API abuse
3. **Input Validation**: All API responses validated
4. **File Paths**: Sanitized to prevent directory traversal
