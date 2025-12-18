# Cascade Review System Design

## Core Principle: "See, Understand, Control"

Every generated element must be reviewable, traceable, and modifiable at any level of the cascade.

## 1. Asset Review & Prompt Chain Navigation

### Visual Asset Review Interface

```
┌─────────────────────────────────────────────────────────────┐
│  Asset Generation Review                              [Pause] │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Current Batch: Character Sprites (3/12)                    │
│                                                             │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐                      │
│  │ Player  │ │ Goblin  │ │ Wizard  │  → → → [Queue: 9]   │
│  │ Sprite  │ │ Enemy   │ │  NPC    │                      │
│  └─────────┘ └─────────┘ └─────────┘                      │
│   ✓ Approve   ✗ Reject   🔄 Regen                          │
│                                                             │
│  [View Prompt Chain ▼]                                      │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Prompt Chain Visualization

```
┌─────────────────────────────────────────────────────────────┐
│  Prompt Hierarchy for "Player Sprite"                       │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Level 1: Style Guide (Phase 1)                      [Edit] │
│  └─ "16-bit pixel art, 24x32 sprites, outlined..."         │
│                                                             │
│  Level 2: Asset Metaprompt (Phase 2)                [Edit] │
│  └─ "Generate character sprites following style..."         │
│                                                             │
│  Level 3: Character Category Prompt                 [Edit] │
│  └─ "For player characters, emphasize heroic..."           │
│                                                             │
│  Level 4: Specific Asset Prompt              [Edit] [Flag] │
│  └─ "Create a 24x32 pixel sprite of the main..."          │
│                                                             │
│  [🚨 Flag Issue at Higher Level]                            │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Cascade Modification Flow

When developer flags a metaprompt issue:

```python
class CascadeModification:
    def flag_metaprompt_issue(self, asset_id: str, level: int, issue: str):
        # 1. Pause all child generation
        self.pause_children(level)
        
        # 2. Show modification interface
        new_prompt = self.show_prompt_editor(level, issue)
        
        # 3. Calculate impact
        affected_assets = self.calculate_cascade_impact(level)
        
        # 4. Confirm with developer
        if self.confirm_regeneration(affected_assets):
            # 5. Regenerate from that level down
            self.regenerate_cascade(level, new_prompt)
```

### Implementation in Streamlit

```python
class AssetReviewInterface:
    def render(self):
        st.title("🎨 Asset Generation Review")
        
        # Asset grid with real-time updates
        cols = st.columns(4)
        for idx, asset in enumerate(st.session_state.asset_queue[:4]):
            with cols[idx]:
                st.image(asset.preview)
                st.text(asset.name)
                
                col1, col2, col3 = st.columns(3)
                with col1:
                    if st.button("✓", key=f"approve_{idx}"):
                        self.approve_asset(asset)
                with col2:
                    if st.button("✗", key=f"reject_{idx}"):
                        self.reject_asset(asset)
                with col3:
                    if st.button("🔄", key=f"regen_{idx}"):
                        self.regenerate_asset(asset)
                
                if st.button("View Chain", key=f"chain_{idx}"):
                    self.show_prompt_chain(asset)
        
        # Prompt chain viewer
        if st.session_state.viewing_chain:
            self.render_prompt_chain()
```

## 2. Code Review & Understanding System

### Progressive Code Revelation

Instead of dumping all code at once, reveal it progressively:

```
┌─────────────────────────────────────────────────────────────┐
│  Code Generation Progress                                    │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Stage 1: Components ████████░░ 80%                        │
│  Stage 2: Systems    ████░░░░░░ 40%                        │
│  Stage 3: Core Loop  ░░░░░░░░░░ 0%                         │
│  Stage 4: Integration ░░░░░░░░░░ 0%                        │
│                                                             │
│  Currently Generating: Movement System                       │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ class MovementSystem:                                │   │
│  │     """Handles player and entity movement"""        │   │
│  │     def __init__(self, world):                      │   │
│  │         self.world = world                          │   │
│  │         self.speed = 100  # pixels per second       │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│  [🤖 Explain This] [📝 Modify] [👁️ See Usage] [▶️ Test]    │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Interactive Code Explanation

```python
class CodeExplainer:
    async def explain_code(self, code_snippet: str, context: str):
        explanation = await self.ai.explain(
            code=code_snippet,
            context=context,
            level="beginner"  # Adjustable
        )
        
        # Show explanation in sidebar
        with st.sidebar:
            st.markdown("### 🤖 AI Explanation")
            st.write(explanation)
            
            # Interactive Q&A
            question = st.text_input("Ask about this code:")
            if question:
                answer = await self.ai.answer_code_question(
                    code_snippet, question
                )
                st.info(answer)
```

### Component/System Visualization

```
┌─────────────────────────────────────────────────────────────┐
│  ECS Architecture Visualization                              │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Components:                    Systems:                    │
│  ┌─────────┐                   ┌─────────────┐            │
│  │Position │ ←───uses───────── │MovementSystem│            │
│  └─────────┘                   └─────────────┘            │
│       ↑                               ↓                     │
│  ┌─────────┐                   ┌─────────────┐            │
│  │Velocity │ ←───updates────── │PhysicsSystem │            │
│  └─────────┘                   └─────────────┘            │
│                                                             │
│  [Highlight Active] [Show Data Flow] [Test Interaction]    │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Live Component Testing

```python
class ComponentTester:
    def render_test_interface(self, component: Type):
        st.subheader(f"Test {component.__name__}")
        
        # Create test instance
        test_data = {}
        for field in component.__dataclass_fields__:
            test_data[field] = st.number_input(
                f"{field}:", 
                value=0.0
            )
        
        # Live preview
        instance = component(**test_data)
        
        # Show how systems would process it
        affected_systems = self.find_systems_using(component)
        for system in affected_systems:
            st.write(f"**{system.__name__}** would:")
            behavior = self.simulate_system_behavior(
                system, instance
            )
            st.code(behavior)
```

## 3. Progressive Game Assembly

### Stage-by-Stage Preview

```
┌─────────────────────────────────────────────────────────────┐
│  Game Assembly Progress                                      │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Stage 1: Core Loop ✓                                       │
│  ┌─────────────────┐                                       │
│  │                 │ Basic movement and rendering           │
│  │  [Play Core]    │ works. Try moving around!             │
│  └─────────────────┘                                       │
│                                                             │
│  Stage 2: Combat System (Building...)                       │
│  ┌─────────────────┐                                       │
│  │                 │ Adding combat mechanics...             │
│  │  [Preview]      │ 60% complete                          │
│  └─────────────────┘                                       │
│                                                             │
│  Stage 3: UI Layer (Queued)                                │
│  Stage 4: Content (Queued)                                 │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Implementation

```python
class ProgressiveGameBuilder:
    def __init__(self):
        self.stages = [
            "core_loop",
            "combat_system", 
            "ui_layer",
            "content_integration",
            "polish_pass"
        ]
        self.current_stage = 0
        
    async def build_stage(self, stage_name: str):
        # Generate code for this stage
        code = await self.generate_stage_code(stage_name)
        
        # Let developer review
        if self.review_interface.show_code(code):
            # Build incremental game
            self.game_preview.add_stage(stage_name, code)
            
            # Allow testing
            self.show_stage_preview(stage_name)
            
    def show_stage_preview(self, stage: str):
        """Show embedded pygame with just this stage active"""
        with st.container():
            # Embedded pygame preview
            game_surface = self.game_preview.get_surface(
                active_stages=self.stages[:self.current_stage+1]
            )
            st.pyplot(game_surface)
            
            # Stage-specific controls
            if stage == "core_loop":
                st.info("Use arrow keys to move!")
            elif stage == "combat_system":
                st.info("Press SPACE to attack!")
```

## 4. Full Game Playtesting with AI Observer

### Playtest Environment

```python
class PlaytestEnvironment:
    def __init__(self):
        self.ai_observer = AIGameObserver()
        self.input_recorder = InputRecorder()
        self.screen_recorder = ScreenRecorder()
        
    async def start_playtest(self):
        st.title("🎮 Playtest Your Game")
        
        col1, col2 = st.columns([2, 1])
        
        with col1:
            # Embedded game
            game_container = st.container()
            
        with col2:
            # AI observations
            st.subheader("🤖 AI Observer")
            observation_container = st.empty()
            
            # Developer notes
            st.subheader("📝 Your Thoughts")
            thought = st.text_area(
                "What are you thinking?",
                key="developer_thoughts"
            )
            if st.button("Save Thought"):
                self.save_thought(thought)
        
        # Run game with observers
        async for frame in self.game_loop():
            # Capture screen
            screenshot = self.screen_recorder.capture(frame)
            
            # AI analysis every N frames
            if frame % 30 == 0:  # Every second at 30fps
                observation = await self.ai_observer.analyze(
                    screenshot=screenshot,
                    recent_inputs=self.input_recorder.get_recent(),
                    game_state=self.get_game_state()
                )
                observation_container.write(observation)
```

### AI Observer Analysis

```python
class AIGameObserver:
    async def analyze(self, screenshot, recent_inputs, game_state):
        # Analyze what's happening
        analysis = await self.ai.analyze_gameplay(
            image=screenshot,
            inputs=recent_inputs,
            state=game_state
        )
        
        # Generate insights
        insights = {
            "current_action": analysis.what_player_doing,
            "difficulty": analysis.perceived_difficulty,
            "engagement": analysis.engagement_level,
            "suggestions": analysis.improvement_suggestions,
            "bugs_noticed": analysis.potential_bugs
        }
        
        return self.format_insights(insights)
```

## 5. Critical Features for Success

### 1. Pause & Resume
- Pause generation at ANY point
- Resume from saved state
- No lost work

### 2. Cascade Impact Preview
```python
def preview_cascade_impact(self, level: int, change: str):
    """Show what will be regenerated if we change this"""
    affected = []
    
    # Calculate downstream impact
    if level == "style_guide":
        affected = ["all_metaprompts", "all_assets", "all_code"]
    elif level == "asset_metaprompt":
        affected = ["all_assets_below", "related_code"]
    
    # Visual representation
    st.warning(f"⚠️ Changing this will regenerate: {affected}")
    st.info("Previously approved assets will be preserved")
```

### 3. Intelligent Batching
- Group related assets for review
- Show code in logical chunks
- Test features incrementally

### 4. Context Preservation
```python
class ReviewContext:
    """Maintains context across review sessions"""
    
    def save_state(self):
        return {
            "approved_assets": self.approved_assets,
            "rejected_assets": self.rejected_assets,
            "code_modifications": self.code_modifications,
            "developer_notes": self.developer_notes,
            "playtest_sessions": self.playtest_sessions
        }
```

## Implementation Priority

1. **Asset Review with Prompt Chain** (Week 1)
   - Visual grid interface
   - Prompt hierarchy viewer
   - Cascade modification system

2. **Progressive Code Review** (Week 2)
   - Stage-by-stage revelation
   - Interactive explanations
   - Component testing

3. **Incremental Game Preview** (Week 3)
   - Embedded pygame stages
   - Feature-by-feature testing
   - Live modification

4. **AI Playtest Observer** (Week 4)
   - Screen/input capture
   - Real-time analysis
   - Insight generation

This system ensures developers maintain control and understanding throughout the entire generation process, making it truly collaborative rather than just generative.