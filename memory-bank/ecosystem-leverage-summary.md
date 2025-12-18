# Ecosystem Leverage Summary

## Key Decisions Made (2025-07-31)

### 1. Multi-Provider AI Strategy
- **async-openai** (REQUIRED) - For DALL-E images and TTS audio
- **allms** (OPTIONAL) - For text generation across providers
- Support OpenRouter, Anthropic, Google Gemini, etc.
- Store multiple API keys in database per provider/model type

### 2. Testing Infrastructure Fix
- Replace custom VCR with **reqwest-vcr v0.3.0**
- This is what we should have been using all along
- Enables deterministic API testing without burning tokens

### 3. Audio Enhancement
- **freesound-rs** (OPTIONAL) - Access to Freesound.org
- Store Freesound API key in database
- Supplement AI-generated audio with real samples

### 4. Asset Pipeline Simplification
- Replace custom build.rs with **bevy_asset_loader**
- Use **bevy_embedded_assets** for wizard UI only
- Keep dynamic generation for game assets

### 5. Metaprompt Alignment
All chosen libraries must support our core principle:
- Dynamic AI generation, not static templates
- Runtime asset creation, not predefined files
- Flexible provider switching

## Immediate Action Items

1. **Add reqwest-vcr** to dev dependencies NOW
2. **Create database migration** for multi-provider API keys
3. **Integrate allms** as optional feature
4. **Replace TODOs** in generator/mod.rs with ecosystem libraries

## Benefits
- Reduce 500+ lines of custom code to ~50 lines of library usage
- Get battle-tested implementations
- Focus on our unique value: metaprompt generation
- Faster development velocity

This strategy maintains our vision while leveraging community work.
