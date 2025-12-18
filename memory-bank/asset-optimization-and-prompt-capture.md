# Asset Optimization and Prompt Capture System

## Overview

The vintage game generator presents unique opportunities for asset optimization through intelligent metaprompt design and post-processing techniques. By combining AI generation with traditional game asset optimization methods, we can achieve both cost efficiency and visual consistency.

## Asset Optimization Strategies

### 1. **Base + Variation Generation**

Instead of generating every asset from scratch, we can:

```python
class AssetVariationGenerator:
    """Generate base assets via AI, then create variations programmatically"""
    
    async def generate_character_set(self, character_concept: str) -> Dict[str, Image]:
        # Generate ONE base character sprite via AI
        base_sprite = await self.ai.generate_image(
            f"16-bit pixel art character sprite, {character_concept}, "
            f"neutral pose, facing right, transparent background"
        )
        
        # Create variations programmatically
        variations = {
            "idle": base_sprite,
            "walk_1": self.create_walk_frame(base_sprite, offset=0),
            "walk_2": self.create_walk_frame(base_sprite, offset=1),
            "jump": self.create_jump_frame(base_sprite),
            "facing_left": self.flip_horizontal(base_sprite),
            "damaged": self.apply_damage_effect(base_sprite),
            "powered_up": self.apply_glow_effect(base_sprite)
        }
        
        return variations
```

### 2. **Palette-Based Recoloring**

Generate one asset and create multiple variations through palette swapping:

```python
class PaletteSwapper:
    """Create asset variations through intelligent palette swapping"""
    
    def generate_enemy_variations(self, base_enemy: Image) -> Dict[str, Image]:
        palettes = {
            "normal": [(255, 0, 0), (128, 0, 0), (64, 0, 0)],  # Red
            "elite": [(255, 215, 0), (184, 134, 11), (139, 69, 19)],  # Gold
            "ice": [(173, 216, 230), (70, 130, 180), (25, 25, 112)],  # Blue
            "poison": [(0, 255, 0), (0, 128, 0), (0, 64, 0)]  # Green
        }
        
        variations = {}
        for name, new_palette in palettes.items():
            variations[name] = self.swap_palette(base_enemy, new_palette)
            
        return variations
```

### 3. **Sprite Sheet Optimization**

Intelligently batch sprites for efficient texture atlas generation:

```python
class SpriteSheetOptimizer:
    """Optimize sprite sheets for game engine performance"""
    
    def create_optimized_atlas(self, sprites: Dict[str, Image]) -> Tuple[Image, AtlasMetadata]:
        # Group sprites by animation set
        animation_groups = self.group_by_animation(sprites)
        
        # Pack sprites efficiently
        packer = RectanglePacker(max_width=1024, max_height=1024)
        
        for group_name, group_sprites in animation_groups.items():
            # Keep animation frames together for cache efficiency
            packer.pack_group(group_sprites, padding=2)
        
        # Generate power-of-2 texture atlas
        atlas = packer.generate_atlas()
        metadata = packer.get_metadata()
        
        return atlas, metadata
```

### 4. **Style Transfer Post-Processing**

Apply consistent visual style to all generated assets:

```python
class StyleConsistencyEnforcer:
    """Ensure visual consistency across all assets"""
    
    def __init__(self, style_guide: StyleGuide):
        self.palette = self.extract_palette(style_guide)
        self.outline_style = style_guide.visual_style.outline
        self.shading_technique = style_guide.visual_style.shading
        
    async def process_asset(self, asset: Image) -> Image:
        # Enforce palette
        asset = self.quantize_to_palette(asset, self.palette)
        
        # Apply consistent outline
        if self.outline_style == "black_1px":
            asset = self.add_pixel_perfect_outline(asset)
        
        # Apply shading
        if self.shading_technique == "dithering":
            asset = self.apply_dithering(asset)
            
        return asset
```

### 5. **Procedural Asset Generation**

Combine AI generation with procedural techniques:

```python
class ProceduralAssetGenerator:
    """Generate assets using AI + procedural techniques"""
    
    async def generate_tileset(self, theme: str) -> Dict[str, Image]:
        # Generate key tiles via AI
        base_tiles = {
            "ground": await self.ai.generate_tile(f"{theme} ground tile"),
            "wall": await self.ai.generate_tile(f"{theme} wall tile"),
            "decoration": await self.ai.generate_tile(f"{theme} decoration")
        }
        
        # Generate variations procedurally
        tileset = {}
        
        # Create edge tiles
        tileset["ground_left"] = self.create_edge_tile(base_tiles["ground"], "left")
        tileset["ground_right"] = self.create_edge_tile(base_tiles["ground"], "right")
        tileset["ground_corner"] = self.create_corner_tile(base_tiles["ground"])
        
        # Create damaged variations
        for damage_level in [0.2, 0.4, 0.6]:
            tileset[f"wall_damaged_{int(damage_level*100)}"] = \
                self.apply_damage(base_tiles["wall"], damage_level)
        
        return tileset
```

## Metaprompt Intelligence

### 1. **Cost-Aware Metaprompt Generation**

Design metaprompts that understand asset generation costs:

```python
class CostAwareMetapromptGenerator:
    """Generate metaprompts that optimize for cost and reusability"""
    
    async def generate_asset_metaprompts(self, game_concept: GameConcept) -> AssetMetaprompts:
        # Analyze what can be generated once and reused
        reusable_assets = self.identify_reusable_assets(game_concept)
        
        metaprompts = {
            "base_character": self.create_base_prompt(
                "Generate ONE base character sprite that can be modified for animations"
            ),
            "tileset_foundation": self.create_base_prompt(
                "Generate 3-4 foundational tiles that can be procedurally varied"
            ),
            "ui_elements": self.create_batch_prompt(
                "Generate a sprite sheet with all UI elements in one image"
            )
        }
        
        # Add variation instructions
        metaprompts["variations"] = self.create_variation_instructions(reusable_assets)
        
        return AssetMetaprompts(metaprompts)
```

### 2. **Batch Generation Prompts**

Create prompts that generate multiple assets in one API call:

```python
class BatchAssetPromptGenerator:
    """Generate prompts for batch asset creation"""
    
    def create_sprite_sheet_prompt(self, asset_type: str, style: StyleGuide) -> str:
        return f"""
        Create a 16-bit sprite sheet with the following {asset_type}:
        - Arranged in a grid format
        - Each sprite: {style.sprite_dimensions}
        - Transparent background
        - Consistent {style.color_palette} palette
        - Include these variations: {', '.join(self.get_required_variations(asset_type))}
        
        Layout sprites in a 4x4 grid with 2px padding.
        """
```

## Prompt Capture and History System

### 1. **Comprehensive Prompt Tracking**

```python
@dataclass
class PromptHistory:
    """Track all prompts in the generation cascade"""
    
    id: str
    timestamp: datetime
    phase: str
    level: PromptLevel  # META, PROMPT, GENERATION
    parent_id: Optional[str]
    
    # The actual prompts
    input_prompt: str
    generated_output: Optional[str]  # For metaprompts that generate prompts
    
    # Execution details
    model_used: str
    tokens_used: int
    cost: float
    duration: float
    
    # Results
    success: bool
    validation_score: Optional[float]
    retry_count: int
    error_message: Optional[str]

class PromptCaptureSystem:
    """Capture and store all prompts in the generation cascade"""
    
    def __init__(self):
        self.history: List[PromptHistory] = []
        self.prompt_tree: Dict[str, List[str]] = {}  # parent_id -> child_ids
        
    async def capture_metaprompt_execution(
        self,
        phase: str,
        input_prompt: str,
        parent_id: Optional[str] = None
    ) -> str:
        """Capture a metaprompt execution"""
        prompt_id = str(uuid.uuid4())
        
        # Start tracking
        entry = PromptHistory(
            id=prompt_id,
            timestamp=datetime.now(),
            phase=phase,
            level=PromptLevel.META,
            parent_id=parent_id,
            input_prompt=input_prompt,
            model_used="gpt-4",
            # ... other fields
        )
        
        return prompt_id
        
    async def capture_generated_prompt(
        self,
        parent_id: str,
        generated_prompt: str,
        purpose: str
    ) -> str:
        """Capture a prompt generated by a metaprompt"""
        prompt_id = str(uuid.uuid4())
        
        entry = PromptHistory(
            id=prompt_id,
            timestamp=datetime.now(),
            phase=purpose,
            level=PromptLevel.PROMPT,
            parent_id=parent_id,
            input_prompt=generated_prompt,
            # ...
        )
        
        # Update tree
        self.prompt_tree.setdefault(parent_id, []).append(prompt_id)
        
        return prompt_id
```

### 2. **Prompt Visualization**

```python
class PromptChainVisualizer:
    """Visualize the prompt generation cascade"""
    
    def create_prompt_tree(self, history: PromptCaptureSystem) -> nx.DiGraph:
        """Create a directed graph of the prompt cascade"""
        G = nx.DiGraph()
        
        for entry in history.history:
            # Add node with metadata
            G.add_node(
                entry.id,
                label=f"{entry.phase}\n{entry.level.value}",
                phase=entry.phase,
                level=entry.level,
                cost=entry.cost,
                success=entry.success
            )
            
            # Add edge from parent
            if entry.parent_id:
                G.add_edge(entry.parent_id, entry.id)
                
        return G
        
    def render_interactive_tree(self, G: nx.DiGraph) -> str:
        """Render an interactive HTML visualization"""
        net = Network(height="800px", width="100%", directed=True)
        
        # Color nodes by level
        colors = {
            PromptLevel.META: "#ff6b6b",
            PromptLevel.PROMPT: "#4ecdc4",
            PromptLevel.GENERATION: "#45b7d1"
        }
        
        for node, attrs in G.nodes(data=True):
            net.add_node(
                node,
                label=attrs['label'],
                color=colors[attrs['level']],
                size=20 + attrs['cost'] * 100  # Size by cost
            )
            
        # Add edges
        for source, target in G.edges():
            net.add_edge(source, target)
            
        return net.generate_html()
```

### 3. **Prompt Analytics**

```python
class PromptAnalytics:
    """Analyze prompt effectiveness and costs"""
    
    def analyze_generation_efficiency(self, history: List[PromptHistory]) -> Dict:
        """Analyze which prompts are most efficient"""
        
        # Group by phase
        phase_stats = defaultdict(lambda: {
            'total_cost': 0,
            'success_rate': 0,
            'avg_retries': 0,
            'total_prompts': 0
        })
        
        for entry in history:
            stats = phase_stats[entry.phase]
            stats['total_cost'] += entry.cost
            stats['total_prompts'] += 1
            if entry.success:
                stats['success_rate'] += 1
            stats['avg_retries'] += entry.retry_count
            
        # Calculate averages
        for phase, stats in phase_stats.items():
            count = stats['total_prompts']
            if count > 0:
                stats['success_rate'] /= count
                stats['avg_retries'] /= count
                stats['avg_cost'] = stats['total_cost'] / count
                
        return dict(phase_stats)
```

## Integration with Review System

### 1. **Cascade Modification Tracking**

```python
class CascadeModificationTracker:
    """Track modifications through the prompt cascade"""
    
    async def flag_prompt_issue(
        self,
        prompt_id: str,
        issue_type: str,
        regenerate_children: bool = True
    ):
        """Flag an issue with a prompt and optionally regenerate children"""
        
        # Find all affected children
        affected = self.find_all_descendants(prompt_id)
        
        if regenerate_children:
            # Mark all children for regeneration
            for child_id in affected:
                await self.mark_for_regeneration(child_id)
                
        # Create modification record
        modification = PromptModification(
            prompt_id=prompt_id,
            timestamp=datetime.now(),
            issue_type=issue_type,
            affected_prompts=affected,
            status="pending_regeneration"
        )
        
        return modification
```

## Implementation Priority

1. **Phase 1: Basic Capture** - Implement PromptCaptureSystem
2. **Phase 2: Asset Optimization** - Implement variation generators and palette swapping
3. **Phase 3: Visualization** - Build prompt tree visualizer
4. **Phase 4: Analytics** - Add cost and efficiency tracking
5. **Phase 5: Integration** - Connect to review systems

## Benefits

1. **Cost Reduction**: Generate fewer base assets, create variations programmatically
2. **Consistency**: Enforce style through post-processing
3. **Transparency**: Full visibility into prompt cascade
4. **Optimization**: Learn from prompt performance
5. **Control**: Ability to modify and regenerate specific branches