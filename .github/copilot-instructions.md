# Copilot Instructions

## Organization Context

This repository is part of the **jbcom** ecosystem:

- **Organization**: [jbcom](https://github.com/jbcom)
- **Control Center**: [jbcom/control-center](https://github.com/jbcom/control-center)
- **Portfolio**: [jbcom.github.io](https://jbcom.github.io)

## Project Management

Link issues and PRs to organization projects:

| Project | Purpose |
|---------|---------|
| **Roadmap** | Features and milestones |
| **Backlog** | Bug fixes, tech debt |
| **Sprint** | Active development |

## Repository Settings

This organization uses the [Settings GitHub App](https://github.com/apps/settings):

- **Org settings**: `jbcom/control-center/.github/settings.yml`
- **Repo settings**: `.github/settings.yml` (extends org settings)

## Code Standards

- Check existing patterns before creating new ones
- Run tests before AND after changes
- Follow established conventions

### Commit Format
```
<type>(<scope>): <description>

Types: feat, fix, docs, style, refactor, test, chore
```

---

## Repository-Specific Instructions

### Vintage Game Generator - AI-Powered RPG Creation

**Language**: Rust (Edition 2021, targeting 2024)
**Engine**: Bevy with egui for UI
**AI**: OpenAI and Anthropic integration

### Development Commands
```bash
# Run the generator
cargo run -p vintage_game_generator

# Check all crates
cargo check --all

# Run tests
cargo test --all

# Lint and format
cargo clippy -- -D warnings
cargo fmt

# Build release
cargo build --release

# Generate documentation
cargo doc --open
```

### Crate Structure
| Crate | Purpose |
|-------|---------|
| `vintage_ai_client` | Multi-provider AI abstraction (OpenAI, Anthropic) |
| `vintage_blending_core` | Game similarity algorithms, feature vectors |
| `vintage_build_tools` | Build-time code generation, GiantBomb integration |
| `vintage_game_generator` | Main application with Bevy/egui UI |

### Crates.io Candidates
These crates are being prepared for public release:
- `ai-client-rs` - Generic multi-provider AI client
- `game-blending` - Game similarity and blending algorithms

### Architecture Notes
- Template-based code generation with MinJinja
- Async-first with Tokio runtime
- Caching layer for AI responses
- Graph-based game relationship modeling

### Key Dependencies
- `async-openai` - OpenAI API client
- `minijinja` - Template rendering
- `bevy` + `bevy_egui` - UI framework
- `tokio` - Async runtime
- `serde` - Serialization

### Memory Bank
Always read `memory-bank/` files at session start for project context.
