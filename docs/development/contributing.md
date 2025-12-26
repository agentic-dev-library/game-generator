# Contributing

Thank you for your interest in contributing to **Agentic**, the AI & Agents division of the [jbcom enterprise](https://jbcom.github.io). This guide will help you get started with the `game-generator` repository.

## 🏢 Enterprise Context

All Agentic repositories follow unified enterprise standards for documentation, branding, and cross-organization interconnections.

## Development Setup

```bash
# Clone the repository
git clone https://github.com/agentic-dev-library/game-generator.git
cd game-generator
```

## Running Tests

```bash
# Run tests
uv run pytest

# Run with coverage
uv run pytest --cov=game_generator
```

## Code Style

This project uses:
- [Ruff](https://docs.astral.sh/ruff/) for linting and formatting
- Type hints throughout

```bash
# Check code style
uv run ruff check .
uv run ruff format --check .

# Auto-fix issues
uv run ruff check --fix .
uv run ruff format .
```

## Building Documentation

```bash
# Install docs dependencies
uv sync --extra docs

# Build docs
cd docs
uv run sphinx-build -b html . _build/html

# Or use make
make html
```

## Pull Request Process

1. Create a feature branch from `main`
2. Make your changes with tests
3. Ensure CI passes (lint + tests)
4. Submit PR - an AI agent will review and merge

## Commit Messages

Use conventional commits:
- `feat:` New features
- `fix:` Bug fixes
- `docs:` Documentation changes
- `refactor:` Code refactoring
- `test:` Test changes
- `chore:` Maintenance tasks
