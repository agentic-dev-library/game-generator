# Vintage Game Generator Vision

## Core Philosophy
Transform from ai_rpg_generator to vintage_game_generator - targeting all vintage games, not just RPGs. The key insight is that starting with constraints helps, but we've been too binary in our approach.

### Production-First Development
**NO PROTOTYPES. NO TODOS. NO PARTIAL IMPLEMENTATIONS.**
- Every file must be 100% production-ready before moving to the next
- No "in a full implementation..." comments
- No placeholder code or stub functions
- Focus on ONE file at a time until it's completely done
- Quality over quantity - better to have 3 perfect files than 10 incomplete ones

## Major Problems with Current Approach
1. **Too Text-Heavy**: Basic info step is overwhelmingly textual - developers get lost
2. **AI-Generated Lists**: Features/gameplay steps are just AI-derived lists that may not match what developers want
3. **Binary Flow**: ALL wizard → ALL config → ALL metaprompts → ALL conversations
4. **Not Scalable**: Fixed feature lists don't adapt to different game types
5. **Not Fun**: The process should be engaging from the start!

## New Vision: Two Paths

### Path 1: Guided Experience (Brainstorming Mode)
- Visual grid of vintage game box art
- Mouse over to expand and see game details
- Pick multiple games to create a "melange" 
- Show how each selection influences the final design
- Use GiantBomb API data for rich game metadata

### Path 2: Freeform Experience (Concrete Plan Mode)
- Large notepad editor on left side
- Real-time AI assistance using async-openai
- NLTK dictionary with gaming semantics
- Auto-complete and suggestions as they type
- Right panel shows AI suggestions based on what they're writing

## Technical Implementation

### Build Script with GiantBomb API
- Use GIANTBOMB_API_KEY from env
- Fetch vintage game data (30+ year old games)
- Store processed data in src/ (not build output)
- Key endpoints:
  - `/games` - Full game catalog
  - `/search` - Search functionality
  - `/themes` - Game themes
  - `/user_review` - User reviews

### Visual First Design
- Generate welcome images with DALL-E for path selection
- Use game cover art for guided mode
- Language selection as visual panels (Rust | Python | Ruby)
- "Coming Soon" overlays for unsupported features
- No more text dropdowns!

### Critical vs Nice-to-Have
**Critical (must have for compilation):**
- Perspective (top-down, side-view, etc.)
- Art style (pixel variations)
- Display mode (windowed, fullscreen, etc.)
- Language selection
- ECS architecture

**Everything Else:**
- Theme, tone, inventory, shops, quests, leveling
- All can be inferred from guided selections or freeform text

### Language Support Rules
1. ALL languages must support ECS (Entity Component System)
2. Maximum 1-2 core libraries per language:
   - Rust: Bevy (includes ECS)
   - Python: pygame + pygame-ecs
   - Ruby: TBD game library + ECS library
3. Plus standard testing, linting, formatting, packaging libraries

### Non-Binary Flow
Instead of sequential phases, we merge:
- Brainstorm with developer
- Build ECS world in chosen language
- Merge concepts from game design and style guide prompts
- ECS world dictates game structure and tone

## Success Metrics
- Developer engagement from first screen
- No overwhelming text fields
- Visual choices wherever possible
- Only ask for missing critical pieces
- Fun and inspiring process!

## The Hybrid Approach: Visual World Building

### Core Insight: Games are Formed by Playing Them
The current binary approach (ALL wizard → ALL config → ALL metaprompts → ALL conversations) misses the fundamental nature of game development. We need a MIX of:

1. **TOML-driven generation** (structured, consistent)
2. **Conversational refinement** (flexible, creative)
3. **Visual world interaction** (immediate, tangible)

### After Guided/Freeform + Language Selection
Once the initial direction is set and language chosen:

1. **Generate ECS World Structure**
   - Use metaprompt system to build initial ECS world
   - Create project structure and foundational code
   - Generate based on game "melange" selections

2. **Visual ECS World Viewer**
   - Dynamically load the generated ECS world
   - Present interactive tree view of:
     - Entities (player, enemies, items)
     - Components (health, position, inventory)
     - Systems (movement, combat, rendering)
   - Show relationships and dependencies
   - Allow real-time modification

3. **Feature Inference from Game Combinations**
   - FF7 + Zelda = Turn-based combat + Exploration puzzles
   - Pokemon + Chrono Trigger = Monster collection + Time mechanics
   - No need for manual feature selection!
   - The "melange" already contains all feature information
   - GiantBomb data provides detailed game mechanics

### Language Adapters for Universal ECS Viewing
- Rust: Native Bevy ECS inspection
- Python: pygame-ecs adapter
- Ruby: ECS adapter for chosen library
- All languages present the same visual interface

### The Power of Escalating Features
- Small set of game selections → Massive feature set
- Metaprompts generating metaprompts
- Each game brings its entire ecosystem of mechanics
- Combinations create emergent gameplay patterns

## Iterative ECS World Building

### Not a One-Time View
The ECS world tree is NOT a single-pass interface:
1. Developer views initial ECS world
2. Makes modifications in the tree viewer
3. AI asks follow-up questions based on changes
4. Developer responds, triggering more adjustments
5. Cycle continues until both AI and developer are satisfied
6. Final world state is saved

### Rust-First Development Strategy
A revolutionary simplification - ALWAYS develop in Rust, regardless of target language:
- **Development Phase**: Everything happens in Rust/Bevy
- **Final Phase**: Transpile to Python/Ruby/etc only at the end
- **Benefits**:
  - Full Rust capabilities throughout development
  - No adapters or dual-language complexity
  - Developer gets immediate visual feedback
  - AI only needs to understand one development environment
- **Directory Structure**:
  - Internal Rust UUID project (for development)
  - Target language UUID project (generated at the end)

## Asset Generation Architecture

### Rust-Based Asset Pipeline
Regardless of game language, asset generation is ALWAYS in Rust:
- PNG sprite generation and manipulation
- OGG audio rendering
- Sprite sheet assembly
- Tile map processing
- All heavy asset work stays in Rust

### Three-Way Interoperability
Asset prompts must coordinate between:
1. **Generator System** (Rust-based tool)
2. **Game ECS Models** (in target language)
3. **Asset Destination Management** (file paths, loading)

### No Forced Structure
- Let asset needs emerge from the ECS world
- Don't impose categories like "sprites", "audio", "maps"
- The game design dictates what assets are needed
- Metaprompts generate child prompts based on actual needs

### Prompt Hierarchy
1. **Parent Generation**: Base style guide enforcement
2. **Grandchild Prompts**: Specific asset generation
   - Sprite variations
   - Audio themes
   - Dialog generation
   - Map layouts
   - Quest structures
3. **Assignment Logic**: AI determines where assets belong in ECS

## Implementation Priority
1. Guided/Freeform welcome with DALL-E visuals
2. GiantBomb data ingestion pipeline
3. Visual language selection panels (but develop in Rust regardless)
4. Rust-only ECS generation and iteration
5. Iterative ECS world viewer with AI Q&A
6. Rust-based asset generation pipeline
7. Final transpilation system (Rust → Python/Ruby/etc)
8. Emergent asset structure from ECS world

## The Simplified Flow
1. **Choose target language** (but develop in Rust)
2. **Guided/Freeform game design**
3. **Build ECS world in Rust/Bevy**
4. **Iterate with visual feedback** (all in Rust)
5. **Generate and assign assets** (all in Rust)
6. **Test and refine** (all in Rust)
7. **Final transpilation** to chosen language using fixed assets and ECS design

## Real-Time Asset Review Process

### Asset Generation with Full Transparency
As soon as assets are generated:
1. **Real-time preview** of each asset
2. **Prompt chain visualization** showing:
   - Original metaprompt
   - Generated child prompts
   - Final asset prompt
3. **Interactive review** allowing developer to:
   - Approve/reject individual assets
   - Modify prompts at any level
   - Trace back to parent metaprompts
   - Change entire asset generation direction

### Prompt Chain Navigation
- Visual tree showing prompt hierarchy
- Click any node to see/edit that prompt
- Changes cascade down to child prompts
- Can pause generation and redirect at any point

## Interactive Game Development

### Code Generation Phase
After asset confirmation:
1. **Generate game code** using metaprompts
2. **bevy-inspector-egui integration** for:
   - Real-time world section previews
   - Interactive component editing
   - System behavior testing
   - Visual debugging

### Prototype Testing Loop
1. **Section-by-section development**:
   - Generate code for one game section
   - Preview in editor view
   - Test mechanics interactively
   - Get developer feedback
   - Iterate before moving to next section

2. **Progressive refinement**:
   - Start with core mechanics
   - Layer in features incrementally
   - Test each addition in real-time
   - Maintain playable prototype throughout

3. **Full game preview**:
   - Integrated playable prototype
   - Real gameplay testing
   - Performance monitoring
   - Final adjustments before transpilation
