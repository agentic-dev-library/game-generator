# Metaprompt System Design

## Core Philosophy

The metaprompt system is the heart of the generator. It implements the cascading prompt architecture that makes this project unique:

```
User Intent → Style Guide → Metaprompts → Child Prompts → Final Assets
```

## 1. Style Guide Generation (Phase 1)

### Template: `ai/metaprompts/style_guide.jinja2`
```jinja2
You are creating a comprehensive style guide for a vintage 16-bit game.

Game Configuration:
- Name: {{ config.name }}
- Genre: {{ config.genre }}
- Perspective: {{ config.perspective }}
- Art Style: {{ config.art_style }}
{% if config.inspirations %}
- Inspired by: {{ config.inspirations | join(", ") }}
{% endif %}
{% if config.custom_description %}
- Description: {{ config.custom_description }}
{% endif %}

Generate a DETAILED style guide that will ensure ALL game assets are consistent.
The style guide MUST include:

1. COLOR PALETTE
   - EXACTLY 16-32 colors with RGB values
   - Primary colors (4-6)
   - Secondary colors (8-12)
   - UI colors (4-6)
   - Special effect colors (4-6)
   Format: "Sky Blue: RGB(135, 206, 235)"

2. SPRITE SPECIFICATIONS
   - Character sprite dimensions (e.g., 24x24, 32x32)
   - Tile dimensions (must be power of 2)
   - Animation frame counts
   - Outline style (thickness, color)
   
3. VISUAL RULES
   - Light source direction
   - Shadow style (dithered, solid, none)
   - Perspective rules for {{ config.perspective }}
   - Pixel density constraints

4. ARTISTIC CONSTRAINTS
   DO:
   - Use dithering for gradients
   - Maintain consistent pixel sizes
   - Follow era-appropriate limitations
   
   DON'T:
   - Use anti-aliasing
   - Include modern effects (bloom, particles)
   - Exceed color palette

5. TECHNICAL SPECIFICATIONS
   - Screen resolution: 320x240 or 256x224
   - Sprite limit per screen: 64-128
   - Background layers: 2-3
   - Tilemap chunk size: 16x16

6. AUDIO STYLE
   - Music: chiptune, {{ config.genre }}-appropriate
   - SFX: 8-bit style, short clips
   - Voice: None or synthesized

7. UI/UX PATTERNS
   - Menu style
   - Dialog box appearance
   - HUD layout
   - Font specifications

Output as structured JSON for parsing.
```

### Implementation
```python
# ai/orchestrator.py
class StyleGuideGenerator:
    def __init__(self, template_env, ai_client):
        self.template_env = template_env
        self.ai_client = ai_client
    
    async def generate(self, game_config: GameConfig) -> StyleGuide:
        """Generate comprehensive style guide"""
        template = self.template_env.get_template("style_guide.jinja2")
        prompt = template.render(config=game_config)
        
        # Use lower temperature for consistency
        response = await self.ai_client.complete(
            prompt=prompt,
            temperature=0.3,
            response_format={"type": "json_object"}
        )
        
        style_guide = StyleGuide.parse_raw(response)
        
        # Generate preview image
        preview_prompt = self._create_preview_prompt(style_guide)
        preview_image = await self.ai_client.generate_image(
            prompt=preview_prompt,
            size="1024x1024"
        )
        
        style_guide.preview_image = preview_image
        return style_guide
```

## 2. Metaprompt Generation (Phase 2)

### Architecture Metaprompt
```jinja2
{# ai/metaprompts/architecture.jinja2 #}
You are generating the code architecture for a {{ config.target_language }} game.

Style Guide Summary:
{{ style_guide | tojson(indent=2) }}

Game Features:
{% for feature in config.features %}
- {{ feature }}
{% endfor %}

Generate a metaprompt that will create the COMPLETE game architecture.
The metaprompt should instruct the AI to:

1. Create the project structure
2. Set up the game engine ({{ engine_for_language(config.target_language) }})
3. Implement the ECS (Entity Component System) architecture
4. Create base classes for all game systems
5. Set up asset loading pipeline
6. Implement the game loop

The metaprompt must be self-contained and include:
- All necessary context from the style guide
- Specific technical requirements
- Code style preferences
- Error handling patterns

Output the metaprompt as a single string.
```

### Feature-Specific Metaprompts
```python
# ai/metaprompts/features/
class FeatureMetapromptGenerator:
    """Generate metaprompts for specific game features"""
    
    FEATURE_TEMPLATES = {
        "combat": "combat_system.jinja2",
        "inventory": "inventory_system.jinja2",
        "dialogue": "dialogue_system.jinja2",
        "quests": "quest_system.jinja2",
        "shops": "shop_system.jinja2",
        "crafting": "crafting_system.jinja2",
        "magic": "magic_system.jinja2",
        "party": "party_system.jinja2",
    }
    
    async def generate_for_feature(
        self, 
        feature: str, 
        game_config: GameConfig,
        style_guide: StyleGuide
    ) -> str:
        """Generate metaprompt for a specific feature"""
        template_name = self.FEATURE_TEMPLATES.get(
            feature, 
            "generic_feature.jinja2"
        )
        
        template = self.template_env.get_template(
            f"features/{template_name}"
        )
        
        return template.render(
            feature=feature,
            config=game_config,
            style_guide=style_guide,
            related_features=self._get_related_features(feature)
        )
```

## 3. Asset Generation Metaprompts

### Sprite Generation Chain
```jinja2
{# ai/metaprompts/assets/sprite_generator.jinja2 #}
You will generate prompts for creating game sprites.

Style Guide:
- Dimensions: {{ style_guide.sprite_dimensions }}
- Colors: {{ style_guide.color_palette | join(", ") }}
- Outline: {{ style_guide.outline_style }}
- Perspective: {{ style_guide.perspective }}

For the {{ asset_type }} sprite, create a prompt that:
1. Describes the character/object clearly
2. Specifies EXACT pixel dimensions
3. Lists ONLY allowed colors by RGB value
4. Defines animation frames needed
5. Includes style constraints

The prompt must ensure the generated sprite:
- Matches the established art style exactly
- Uses only colors from the palette
- Maintains consistent perspective
- Includes proper outlines

Generate 3 variations of the prompt for variety.
```

### Implementation
```python
class AssetMetapromptGenerator:
    async def generate_asset_prompts(
        self,
        asset_type: str,
        style_guide: StyleGuide,
        context: Dict
    ) -> List[str]:
        """Generate multiple prompts for asset variety"""
        
        # First, generate the metaprompt
        metaprompt = self.template_env.get_template(
            "assets/sprite_generator.jinja2"
        ).render(
            asset_type=asset_type,
            style_guide=style_guide,
            context=context
        )
        
        # Use metaprompt to generate actual prompts
        response = await self.ai_client.complete(
            prompt=metaprompt,
            temperature=0.7  # More variety
        )
        
        # Parse and validate prompts
        prompts = self._parse_prompt_variations(response)
        
        # Ensure style guide compliance
        validated_prompts = []
        for prompt in prompts:
            validated = self._inject_style_constraints(
                prompt, 
                style_guide
            )
            validated_prompts.append(validated)
        
        return validated_prompts
    
    def _inject_style_constraints(
        self, 
        prompt: str, 
        style_guide: StyleGuide
    ) -> str:
        """Ensure every prompt includes style constraints"""
        
        constraints = f"""
MANDATORY CONSTRAINTS:
- Use ONLY these RGB colors: {style_guide.color_list}
- Size MUST be exactly {style_guide.sprite_dimensions}
- Include {style_guide.outline_style} outline
- Perspective: {style_guide.perspective_rules}
- NO anti-aliasing, NO gradients, NO transparency
"""
        return prompt + "\n\n" + constraints
```

## 4. Code Generation Metaprompts

### Dynamic Code Generation
```python
class CodeMetapromptGenerator:
    """Generate code based on architecture and features"""
    
    async def generate_system_code(
        self,
        system_name: str,
        architecture: Dict,
        dependencies: List[str]
    ) -> str:
        """Generate metaprompt for system implementation"""
        
        template = self.template_env.get_template(
            "code/system_generator.jinja2"
        )
        
        metaprompt = template.render(
            system=system_name,
            architecture=architecture,
            dependencies=dependencies,
            language=self.target_language,
            style_guide=self.style_guide
        )
        
        # Generate the actual code
        code = await self.ai_client.complete(
            prompt=metaprompt,
            temperature=0.2,  # Low for consistency
            max_tokens=4000
        )
        
        # Validate and format
        return self._validate_and_format_code(code)
```

## 5. Cascading Prompt Management

### Prompt Hierarchy Manager
```python
class PromptHierarchy:
    """Manage the cascading prompt structure"""
    
    def __init__(self):
        self.hierarchy = {
            "root": None,
            "style_guide": None,
            "metaprompts": {},
            "child_prompts": {},
            "outputs": {}
        }
    
    async def execute_cascade(
        self, 
        game_config: GameConfig,
        callback: Callable = None
    ):
        """Execute the full prompt cascade"""
        
        # Level 1: Style Guide
        if callback:
            callback("Generating style guide...", 0.1)
        
        style_guide = await self.generate_style_guide(game_config)
        self.hierarchy["style_guide"] = style_guide
        
        # Level 2: Metaprompts
        if callback:
            callback("Creating metaprompts...", 0.3)
        
        metaprompts = await self.generate_metaprompts(
            game_config, 
            style_guide
        )
        self.hierarchy["metaprompts"] = metaprompts
        
        # Level 3: Child Prompts
        if callback:
            callback("Generating prompts...", 0.5)
        
        child_prompts = await self.generate_child_prompts(
            metaprompts,
            style_guide
        )
        self.hierarchy["child_prompts"] = child_prompts
        
        # Level 4: Final Outputs
        if callback:
            callback("Creating game assets...", 0.7)
        
        outputs = await self.generate_outputs(child_prompts)
        self.hierarchy["outputs"] = outputs
        
        return self.hierarchy
```

## 6. Prompt Optimization

### Token-Efficient Prompting
```python
class PromptOptimizer:
    """Optimize prompts for token efficiency"""
    
    def optimize(self, prompt: str, max_tokens: int = 2000) -> str:
        """Compress prompt while maintaining effectiveness"""
        
        # Remove redundancy
        prompt = self._remove_redundancy(prompt)
        
        # Use references instead of repetition
        prompt = self._use_references(prompt)
        
        # Compress style guide
        prompt = self._compress_style_guide(prompt)
        
        # Ensure under token limit
        tokens = self.count_tokens(prompt)
        if tokens > max_tokens:
            prompt = self._truncate_smartly(prompt, max_tokens)
        
        return prompt
    
    def _compress_style_guide(self, prompt: str) -> str:
        """Compress style guide to essential elements"""
        # Instead of full style guide, use summary
        return prompt.replace(
            "{{ style_guide | tojson(indent=2) }}",
            "{{ style_guide_summary }}"
        )
```

## 7. Validation and Consistency

### Style Compliance Validator
```python
class StyleComplianceValidator:
    """Ensure all outputs comply with style guide"""
    
    async def validate_asset(
        self, 
        asset: Any, 
        style_guide: StyleGuide
    ) -> ValidationResult:
        """Validate asset against style guide"""
        
        if isinstance(asset, Image):
            return await self._validate_image(asset, style_guide)
        elif isinstance(asset, str):  # Code
            return await self._validate_code(asset, style_guide)
        elif isinstance(asset, AudioClip):
            return await self._validate_audio(asset, style_guide)
    
    async def _validate_image(
        self, 
        image: Image, 
        style_guide: StyleGuide
    ) -> ValidationResult:
        """Validate image compliance"""
        
        issues = []
        
        # Check dimensions
        if image.size != style_guide.sprite_dimensions:
            issues.append(f"Wrong dimensions: {image.size}")
        
        # Check colors
        used_colors = self._extract_colors(image)
        invalid_colors = used_colors - set(style_guide.color_palette)
        if invalid_colors:
            issues.append(f"Invalid colors: {invalid_colors}")
        
        # Check style
        if self._has_antialiasing(image):
            issues.append("Contains anti-aliasing")
        
        return ValidationResult(
            valid=len(issues) == 0,
            issues=issues,
            suggestions=self._generate_fix_suggestions(issues)
        )
```

## Key Benefits of Python Implementation

1. **Dynamic Templates**: Jinja2 allows runtime template modification
2. **Easy Debugging**: Print prompts, inspect at each level
3. **Flexible Validation**: Rich image/audio libraries for validation
4. **Rapid Iteration**: Change prompts without recompiling
5. **Better Error Handling**: Python's exceptions are more flexible
6. **Library Support**: LangChain provides prompt management tools
7. **Cost Tracking**: Easy to implement token counting and optimization

## Migration Strategy

1. Port all `.jinja` templates directly (same syntax)
2. Implement cascade management with async/await
3. Add rich validation using PIL/scikit-image
4. Implement caching at each level
5. Add prompt versioning for A/B testing