# Vintage Game Generator (Rust)

[![CI](https://github.com/agentic-dev-library/game-generator/workflows/Rust%20CI/badge.svg)](https://github.com/agentic-dev-library/game-generator/actions)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

## 🏢 Enterprise Context

**Agentic** is the AI & Agents division of the [jbcom enterprise](https://jbcom.github.io). This project is part of a coherent suite of specialized tools, sharing a unified design system and interconnected with sibling organizations like [Strata](https://strata.game) and [Extended Data](https://extendeddata.dev).

AI-powered retro RPG generator with multi-provider AI integration, built with the Bevy game engine.

## Features

- 🤖 **Multi-Provider AI**: OpenAI, Anthropic integration with intelligent caching
- 🎮 **Complete Game Generation**: Generates playable Bevy games
- 📝 **TOML Configuration**: Simple dev.toml-based project setup
- 🎨 **Asset Generation**: Sprites, audio, dialogs, maps
- 🧙‍♂️ **Interactive Wizard**: GUI for designing your RPG

## Crate Structure

| Crate | Description | Status |
|-------|-------------|--------|
| `vintage-ai-client` | AI service abstraction with caching | 🚧 Migration |
| `vintage-blending-core` | Game similarity algorithms | 🚧 Migration |
| `vintage-build-tools` | Build-time code generation | 🚧 Migration |
| `vintage-game-generator` | Main application with wizard | 🚧 Migration |

## Development

```bash
# Check all crates
cargo check --all

# Run tests
cargo test --all

# Run the generator
cargo run --bin vintage-game-generator
```

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
