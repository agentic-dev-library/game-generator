# Complete Plan: Wizard + AI Conversation Hybrid Architecture

## Executive Summary
A two-phase game generation system that combines structured data collection (wizard) with creative AI exploration (conversation), resulting in a richer, more guided experience for developers creating 16-bit style RPGs.

## Phase 1: Wizard Data Collection

### Step 1: Welcome Screen
- Load existing projects from `~/.config/ai-rpg-generator/projects/`
- Option to resume existing project or start new
- Display saved projects with preview (name, genre, last modified)

### Step 2: Basic Information
**Collected Data:**
- Game name (required)
- Tagline (one-liner description)
- Genre (dropdown: Action RPG, Turn-Based RPG, Puzzle RPG, etc.)
- Target audience (Casual, Core, Hardcore)
- Full description (multiline)
- Inspiration notes (for AI context)

### Step 3: Gameplay Design
**Collected Data:**
- Core mechanics (multi-select: Combat, Exploration, Puzzle Solving, etc.)
- Progression type (Linear, Open World, Metroidvania, Hub-Based)
- Gameplay loop description
- Player motivation (what drives progression)
- Difficulty curve settings:
  - Starting difficulty (0.0-1.0)
  - Ramp speed
  - Maximum difficulty
  - Adaptive difficulty toggle

### Step 4: Visual Style
**Collected Data:**
- Reference games (multi-select: Secret of Mana, Chrono Trigger, etc.)
- Color mood (Vibrant, Pastel, Dark, Earthy, Neon)
- Sprite size (16-64 pixels, default 24)
- Outline toggle and style
- Shading technique
- Animation complexity
- UI theme preference

### Step 5: Game Features
**Collected Data:**
- Combat system (optional):
  - Type (Real-Time vs Turn-Based)
  - Damage numbers
  - Combo system
  - Special abilities (AI will design)
- Inventory system (optional):
  - Slot count
  - Stack size
  - Categories
- Dialogue system (optional):
  - Type (Linear, Branching)
  - Portrait style
  - Text speed
- Additional features:
  - Save system
  - Day/night cycle
  - Weather effects
  - Minimap
  - Achievements

### Step 6: Technical Settings
**Collected Data:**
- World size (Small, Medium, Large)
- Performance target
- Target platforms (multi-select: Windows, Mac, Linux, Web)
- Multiplayer settings (optional)

### Step 7: Review & Launch
- Display complete configuration summary
- Show what AI will use this for
- Save to TOML
- "Start AI Conversation" button

## Phase 2: AI Conversation Enhancement

### Initial Context Loading
The AI receives:
1. Complete TOML configuration from wizard
2. Formatted summary via `GameConfig::to_ai_summary()`
3. Any existing conversation history
4. Previous design decisions

### Conversation Flow

#### Opening
AI begins with context-aware greeting:
```
"I see you're creating Crystal Quest, a Metroidvania-style Adventure RPG 
inspired by Secret of Mana and Zelda. Let's dive deeper into what makes 
your game unique. You mentioned wanting more puzzle focus - can you tell 
me about the types of puzzles you envision?"
```

#### Topic Progression
1. **Unique Mechanics** - Based on selected core mechanics
2. **World Building** - Informed by genre and references
3. **Character Design** - Shaped by visual style choices
4. **Level Themes** - Guided by progression type
5. **Story Integration** - Woven through features selected

#### AI Capabilities
- Ask clarifying questions about wizard choices
- Suggest complementary features
- Design detailed implementations
- Create lore that fits the style
- Generate specific asset concepts
- Plan technical architecture

### Conversation Storage
All exchanges saved to TOML:
```toml
[[ai_context.conversation_history]]
timestamp = "2024-01-15T10:30:00Z"
role = "assistant"
content = "Let's design your crystal-based puzzle mechanics..."
phase = "mechanics_design"

[[ai_context.design_decisions]]
category = "puzzle_mechanics"
decision = "Crystals have color-based properties"
rationale = "Matches vibrant art style, enables visual puzzles"
alternatives_considered = ["Shape-based", "Sound-based"]
```

## Phase 3: Generation Pipeline

### Input Sources
1. **Wizard Data** - Structured configuration
2. **AI Decisions** - Creative enhancements
3. **Style Guide** - Generated from both sources
4. **Asset Concepts** - Designed during conversation

### Generation Sequence

#### 1. Style Guide Generation
- Combines wizard visual preferences with AI artistic decisions
- Creates comprehensive visual rulebook
- Includes validation rules

#### 2. Architecture Generation
- Uses wizard technical settings
- Implements AI-suggested features
- Follows selected performance targets

#### 3. Asset Generation
- Sprites match wizard-selected size and style
- Incorporates AI-designed characters
- Follows reference game aesthetics

#### 4. Code Generation
- Implements all wizard-selected features
- Includes AI-designed unique mechanics
- Targets specified platforms

#### 5. Integration & Polish
- Connects all systems
- Implements AI story elements
- Adds wizard-configured features

## Implementation Architecture

### Module Structure
```
src/app/
├── config.rs          # TOML game configuration
├── wizard.rs          # Wizard UI component
├── conversation.rs    # AI conversation interface
├── ui.rs             # Integrated UI with flow control
├── state.rs          # Unified app state
└── pipeline.rs       # Generation orchestration

src/metaprompts/
├── conversation.rs    # AI conversation logic
├── generator.rs      # Metaprompt generation
├── types.rs         # Shared types
└── validation.rs    # Prompt validation
```

### Data Flow
1. Wizard → `GameConfig` TOML
2. TOML → AI Context
3. AI Conversation → Enhanced TOML
4. Enhanced TOML → Generation Pipeline
5. Pipeline → Complete Game

### State Management
```rust
pub struct AppState {
    // Wizard state
    wizard_ui: Option<WizardUI>,
    
    // Conversation state
    conversation: ConversationState,
    
    // Generation state
    pipeline: GenerationPipeline,
    
    // Shared config
    game_config: Option<GameConfig>,
}
```

## User Experience Flow

### First-Time User
1. Launch app → Welcome screen
2. "Create New Game" → Wizard Step 1
3. Progress through wizard (auto-saves each step)
4. Review configuration
5. "Start AI Conversation"
6. AI asks clarifying questions
7. Develop unique features together
8. "Generate Game" when ready
9. Monitor progress in UI
10. Play generated game!

### Returning User
1. Launch app → Welcome shows saved projects
2. Select "Crystal Quest (In Progress)"
3. If wizard incomplete → Resume at last step
4. If in conversation → Continue discussion
5. If generation started → View progress

### Advanced User
1. Can edit TOML directly
2. Skip wizard, go straight to conversation
3. Modify AI suggestions in TOML
4. Re-run specific generation phases

## Benefits Over Pure Approaches

### vs Pure Wizard
- Not limited to predefined options
- AI can suggest novel features
- Richer, more detailed output
- Personalized to developer's vision

### vs Pure Conversation
- Structured starting point
- No blank page problem
- Consistent data collection
- Resumable progress

### Hybrid Advantages
- Progressive complexity
- Natural handoff points
- Best of both worlds
- Clear progress tracking

## Technical Considerations

### Performance
- Wizard is lightweight Bevy UI
- TOML files are small (<10KB)
- AI calls are async/non-blocking
- Generation runs in background

### Error Handling
- Wizard validates at each step
- TOML corruption recovery
- AI timeout handling
- Generation failure recovery

### Extensibility
- New wizard steps easy to add
- AI prompts in Jinja templates
- Generation phases modular
- Plugin architecture ready

## Future Enhancements

### Version 2.0
- Multiple AI providers (Anthropic, Cohere)
- Collaborative multiplayer wizard
- Real-time preview during conversation
- Voice input for conversation

### Version 3.0
- AI learns from successful games
- Community feature templates
- Cross-game asset sharing
- Steam Workshop integration

## Success Metrics
- Time from launch to playable game
- User satisfaction with AI suggestions
- Completion rate of wizard
- Quality of generated games
- Community engagement

## Conclusion
This hybrid approach provides the optimal balance between structure and creativity, giving developers a guided yet flexible path to creating their dream 16-bit RPG. The wizard ensures we collect all necessary information while the AI conversation adds the creative spark that makes each game unique.
