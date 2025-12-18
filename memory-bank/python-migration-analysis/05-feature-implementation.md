# Feature Implementation: Classic RPG Systems

## Overview

This document details how each classic RPG feature will be implemented in the Python version, with a focus on AI-driven generation through metaprompts.

## 1. Dialogue System

### Metaprompt Template
```jinja2
{# ai/metaprompts/features/dialogue_system.jinja2 #}
Create a dialogue system for a {{ config.genre }} game with these requirements:

Style Guide:
- Text box style: {{ style_guide.ui_patterns.dialog_box }}
- Font: {{ style_guide.ui_patterns.font }}
- Character portraits: {{ style_guide.sprite_dimensions }}

Features needed:
- Branching conversations with choices
- Character emotions/expressions
- Quest-giving functionality
- Shop integration for merchant NPCs
{% if "party" in config.features %}
- Party member interjections
{% endif %}
{% if "romance" in config.features %}
- Relationship tracking
{% endif %}

Generate:
1. Dialogue tree structure
2. NPC personality templates
3. Conversation flow patterns
4. Integration with other systems
```

### Implementation
```python
# generation/features/dialogue.py
class DialogueSystemGenerator:
    async def generate(self, config: GameConfig, style_guide: StyleGuide):
        # Generate dialogue structure
        dialogue_structure = await self._generate_structure(config)
        
        # Generate NPC templates
        npc_templates = await self._generate_npc_templates(
            config.inspirations,
            style_guide
        )
        
        # Generate sample dialogues
        sample_dialogues = await self._generate_samples(
            npc_templates,
            config.genre
        )
        
        # Generate UI components
        ui_components = await self._generate_dialogue_ui(style_guide)
        
        return {
            "structure": dialogue_structure,
            "npcs": npc_templates,
            "samples": sample_dialogues,
            "ui": ui_components
        }
    
    async def _generate_npc_templates(self, inspirations, style_guide):
        """Generate NPC archetypes based on game inspirations"""
        
        prompt = f"""
        Based on these classic games: {inspirations}
        Generate NPC personality archetypes that would fit this style.
        
        For each archetype, provide:
        - Name pattern (e.g., "Wise Elder", "Mysterious Merchant")
        - Speaking style and vocabulary
        - Common dialogue patterns
        - Emotional range (happy, sad, angry expressions)
        - Quest types they might offer
        """
        
        archetypes = await self.ai_client.complete(prompt)
        
        # Generate portrait variations for each
        portraits = {}
        for archetype in archetypes:
            portraits[archetype.id] = await self._generate_portraits(
                archetype,
                style_guide
            )
        
        return {"archetypes": archetypes, "portraits": portraits}
```

## 2. Inventory System

### Metaprompt Template
```jinja2
{# ai/metaprompts/features/inventory_system.jinja2 #}
Design an inventory system inspired by {{ config.inspirations | join(", ") }}.

Requirements:
- Grid size: Calculate based on {{ style_guide.screen_resolution }}
- Item categories: Weapons, Armor, Consumables, Key Items, Materials
- Visual style: {{ style_guide.ui_patterns.menu_style }}

{% if "crafting" in config.features %}
- Crafting material organization
- Recipe discovery tracking
{% endif %}

{% if "shops" in config.features %}
- Buy/sell value display
- Merchant inventory integration
{% endif %}

Generate:
1. Inventory data structure
2. Item categorization system
3. UI layout and interaction
4. Integration points with other systems
```

### Implementation
```python
# generation/features/inventory.py
class InventorySystemGenerator:
    async def generate(self, config: GameConfig, style_guide: StyleGuide):
        # Calculate optimal grid size
        grid_size = self._calculate_grid(style_guide.screen_resolution)
        
        # Generate item categories and types
        item_system = await self._generate_item_system(config)
        
        # Generate inventory UI
        ui_layout = await self._generate_inventory_ui(
            grid_size,
            style_guide,
            config.inspirations
        )
        
        # Generate item sprites
        item_sprites = await self._generate_item_sprites(
            item_system.categories,
            style_guide
        )
        
        return {
            "grid_size": grid_size,
            "item_system": item_system,
            "ui_layout": ui_layout,
            "sprites": item_sprites
        }
    
    def _calculate_grid(self, resolution):
        """Calculate optimal inventory grid based on resolution"""
        # 16-bit era typically used 16x16 or 32x32 item sprites
        item_size = 32 if resolution[0] >= 320 else 16
        
        # Leave room for UI elements
        usable_width = resolution[0] * 0.7
        usable_height = resolution[1] * 0.6
        
        cols = int(usable_width // item_size)
        rows = int(usable_height // item_size)
        
        return {"cols": cols, "rows": rows, "item_size": item_size}
```

## 3. Combat System

### Metaprompt Template
```jinja2
{# ai/metaprompts/features/combat_system.jinja2 #}
Create a combat system that blends elements from:
{% for game in config.inspirations %}
- {{ game }}
{% endfor %}

Combat style: {{ "Turn-based" if "turn-based" in config.tags else "Real-time" }}
Perspective: {{ config.perspective }}

Core mechanics needed:
- Damage calculation formula
- Status effects (poison, sleep, etc.)
- Critical hit system
- Elemental weaknesses
{% if "magic" in config.features %}
- Spell casting and MP system
{% endif %}
{% if "party" in config.features %}
- Party formation and tactics
{% endif %}

Generate complete combat system including:
1. Core combat loop
2. Damage formulas
3. Status effect definitions
4. UI layout for battles
5. Victory/defeat conditions
```

### Implementation
```python
# generation/features/combat.py
class CombatSystemGenerator:
    async def generate(self, config: GameConfig, style_guide: StyleGuide):
        combat_style = self._determine_combat_style(config)
        
        # Generate core mechanics
        mechanics = await self._generate_mechanics(
            combat_style,
            config.inspirations
        )
        
        # Generate combat UI
        if combat_style == "turn-based":
            ui = await self._generate_turn_based_ui(style_guide)
        else:
            ui = await self._generate_realtime_ui(style_guide)
        
        # Generate combat animations
        animations = await self._generate_combat_animations(
            mechanics.actions,
            style_guide
        )
        
        # Generate battle backgrounds
        backgrounds = await self._generate_battle_backgrounds(
            config.genre,
            style_guide
        )
        
        return {
            "style": combat_style,
            "mechanics": mechanics,
            "ui": ui,
            "animations": animations,
            "backgrounds": backgrounds
        }
```

## 4. Quest System

### Metaprompt Template
```jinja2
{# ai/metaprompts/features/quest_system.jinja2 #}
Design a quest system for a {{ config.genre }} game.

Quest types needed:
- Main story quests
- Side quests
- Daily/repeatable quests
{% if "guilds" in config.tags %}
- Guild missions
{% endif %}
{% if "romance" in config.features %}
- Relationship quests
{% endif %}

Quest structure:
- Objectives tracking
- Reward system
- Quest journal UI
- Map markers/indicators

Generate:
1. Quest data structure
2. Objective types and tracking
3. Quest generation templates
4. UI components
5. Integration with dialogue and combat
```

### Implementation
```python
# generation/features/quests.py
class QuestSystemGenerator:
    async def generate(self, config: GameConfig, style_guide: StyleGuide):
        # Generate quest archetypes
        archetypes = await self._generate_quest_archetypes(config)
        
        # Generate quest journal UI
        journal_ui = await self._generate_journal_ui(style_guide)
        
        # Generate quest marker system
        markers = await self._generate_marker_system(style_guide)
        
        # Generate sample quests
        samples = await self._generate_sample_quests(
            archetypes,
            config.inspirations
        )
        
        return {
            "archetypes": archetypes,
            "journal": journal_ui,
            "markers": markers,
            "samples": samples
        }
```

## 5. Shop System

### Metaprompt Template
```jinja2
{# ai/metaprompts/features/shop_system.jinja2 #}
Create a shop system inspired by classic RPG merchants.

Shop types:
- General goods
- Weapon smith
- Armor shop
- Magic/potion shop
{% if "crafting" in config.features %}
- Material supplier
{% endif %}

Features:
- Buy/sell interface
- Price scaling with player level
- Rare item availability
- Merchant personalities
- Shop-specific music/ambience

Generate complete shop system with UI and merchant NPCs.
```

### Implementation
```python
# generation/features/shops.py
class ShopSystemGenerator:
    async def generate(self, config: GameConfig, style_guide: StyleGuide):
        # Generate shop types and inventories
        shop_types = await self._generate_shop_types(config)
        
        # Generate merchant NPCs
        merchants = await self._generate_merchants(
            shop_types,
            style_guide
        )
        
        # Generate shop UI
        shop_ui = await self._generate_shop_ui(style_guide)
        
        # Generate shop interiors
        interiors = await self._generate_shop_interiors(
            shop_types,
            style_guide
        )
        
        return {
            "shops": shop_types,
            "merchants": merchants,
            "ui": shop_ui,
            "interiors": interiors
        }
```

## 6. Map System

### Town Map Implementation
```python
# generation/features/maps/town.py
class TownMapGenerator:
    async def generate(self, config: GameConfig, style_guide: StyleGuide):
        # Generate town layout
        layout = await self._generate_town_layout(config)
        
        # Generate building types
        buildings = await self._generate_buildings(
            layout,
            config.features
        )
        
        # Generate NPCs for town
        npcs = await self._generate_town_npcs(
            buildings,
            config.inspirations
        )
        
        # Generate tileset
        tileset = await self._generate_town_tileset(style_guide)
        
        return {
            "layout": layout,
            "buildings": buildings,
            "npcs": npcs,
            "tileset": tileset
        }
    
    async def _generate_buildings(self, layout, features):
        """Generate buildings based on enabled features"""
        
        building_types = ["Inn", "Town Square"]
        
        if "shops" in features:
            building_types.extend(["General Store", "Blacksmith", "Armory"])
        if "magic" in features:
            building_types.append("Magic Shop")
        if "guilds" in features:
            building_types.append("Adventurer's Guild")
        if "crafting" in features:
            building_types.append("Crafting Workshop")
            
        # Generate building layouts and interiors
        buildings = {}
        for building_type in building_types:
            buildings[building_type] = await self._generate_building(
                building_type,
                layout
            )
            
        return buildings
```

### Dungeon Map Implementation
```python
# generation/features/maps/dungeon.py
class DungeonMapGenerator:
    async def generate(self, config: GameConfig, style_guide: StyleGuide):
        # Generate dungeon themes
        themes = await self._generate_dungeon_themes(config)
        
        # Generate room types
        room_types = await self._generate_room_types(themes)
        
        # Generate procedural generation rules
        proc_rules = await self._generate_procedural_rules(
            config.inspirations
        )
        
        # Generate dungeon tilesets
        tilesets = {}
        for theme in themes:
            tilesets[theme.id] = await self._generate_dungeon_tileset(
                theme,
                style_guide
            )
        
        # Generate sample layouts
        samples = await self._generate_sample_dungeons(
            themes,
            room_types,
            proc_rules
        )
        
        return {
            "themes": themes,
            "room_types": room_types,
            "generation_rules": proc_rules,
            "tilesets": tilesets,
            "samples": samples
        }
```

## 7. Party System

### Implementation
```python
# generation/features/party.py
class PartySystemGenerator:
    async def generate(self, config: GameConfig, style_guide: StyleGuide):
        # Generate party member archetypes
        archetypes = await self._generate_party_archetypes(
            config.inspirations
        )
        
        # Generate party UI
        party_ui = await self._generate_party_ui(style_guide)
        
        # Generate relationship system
        relationships = await self._generate_relationship_system(
            archetypes,
            config.features
        )
        
        # Generate party banter
        banter = await self._generate_party_banter(
            archetypes,
            relationships
        )
        
        return {
            "archetypes": archetypes,
            "ui": party_ui,
            "relationships": relationships,
            "banter": banter
        }
```

## 8. Magic System

### Implementation
```python
# generation/features/magic.py
class MagicSystemGenerator:
    async def generate(self, config: GameConfig, style_guide: StyleGuide):
        # Determine magic system type
        magic_type = await self._determine_magic_type(
            config.inspirations
        )
        
        # Generate spell categories
        categories = await self._generate_spell_categories(
            magic_type,
            config.genre
        )
        
        # Generate individual spells
        spells = {}
        for category in categories:
            spells[category.id] = await self._generate_spells(
                category,
                style_guide
            )
        
        # Generate spell effects
        effects = await self._generate_spell_effects(
            spells,
            style_guide
        )
        
        # Generate magic UI
        magic_ui = await self._generate_magic_ui(
            magic_type,
            style_guide
        )
        
        return {
            "system_type": magic_type,
            "categories": categories,
            "spells": spells,
            "effects": effects,
            "ui": magic_ui
        }
```

## Integration Points

### Feature Dependencies
```python
class FeatureIntegrator:
    """Ensures features work together seamlessly"""
    
    DEPENDENCIES = {
        "shops": ["inventory", "dialogue"],
        "crafting": ["inventory"],
        "party": ["dialogue", "combat"],
        "romance": ["dialogue", "party", "quests"],
        "guilds": ["quests", "combat"],
        "magic": ["combat"],
    }
    
    async def integrate_features(self, features: Dict[str, Any]):
        """Ensure all features work together"""
        
        # Check dependencies
        for feature, deps in self.DEPENDENCIES.items():
            if feature in features:
                for dep in deps:
                    if dep not in features:
                        # Generate minimal version of dependency
                        features[dep] = await self._generate_minimal(dep)
        
        # Create integration points
        integrations = await self._create_integrations(features)
        
        return integrations
```

## Key Advantages in Python

1. **Rapid Prototyping**: Test features immediately without compilation
2. **Rich Libraries**: Use NetworkX for quest graphs, NumPy for combat math
3. **Easy Serialization**: JSON/YAML for all game data
4. **Dynamic Systems**: Runtime feature addition/modification
5. **Better Debugging**: Print game state at any point
6. **Mod Support**: Python scripts can be modded easily
7. **Cross-Platform**: Same code works everywhere