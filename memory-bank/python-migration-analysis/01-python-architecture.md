# Python Architecture Design

## Overall Architecture

```
vintage_game_generator/
├── app.py                      # Main Streamlit/Gradio app
├── core/
│   ├── __init__.py
│   ├── config.py              # Configuration management
│   ├── models.py              # Pydantic models for data
│   └── state.py               # Application state management
├── ui/
│   ├── __init__.py
│   ├── welcome.py             # Welcome screen with visual paths
│   ├── guided_mode.py         # Game selection UI
│   ├── freeform_mode.py       # Text editor with AI assist
│   ├── generation_view.py     # Real-time generation progress
│   └── components/            # Reusable UI components
├── ai/
│   ├── __init__.py
│   ├── orchestrator.py        # Main AI orchestration
│   ├── providers/             # Multi-provider support
│   │   ├── openai_provider.py
│   │   ├── anthropic_provider.py
│   │   └── litellm_provider.py
│   ├── metaprompts/           # Metaprompt templates
│   │   ├── style_guide.jinja2
│   │   ├── architecture.jinja2
│   │   ├── assets.jinja2
│   │   └── features/          # Feature-specific prompts
│   └── chains.py              # LangChain integration
├── generation/
│   ├── __init__.py
│   ├── pipeline.py            # Generation pipeline
│   ├── assets/
│   │   ├── sprites.py         # Sprite generation
│   │   ├── audio.py           # Audio generation
│   │   └── tilesets.py        # Tileset generation
│   ├── code/
│   │   ├── rust_generator.py  # Rust/Bevy code gen
│   │   ├── python_generator.py # Python/Pygame code gen
│   │   └── ruby_generator.py  # Ruby code gen
│   └── validators.py          # Output validation
├── data/
│   ├── __init__.py
│   ├── giantbomb.py          # GiantBomb API integration
│   ├── game_database.py      # Local game metadata
│   └── cache.py              # Caching layer
├── utils/
│   ├── __init__.py
│   ├── image_processing.py   # PIL-based image tools
│   ├── style_transfer.py     # Neural style transfer
│   └── packaging.py          # Distribution utilities
└── tests/
    ├── test_metaprompts.py
    ├── test_generation.py
    └── fixtures/

```

## Core Components

### 1. Configuration Management

```python
# core/config.py
from pydantic import BaseModel, Field
from typing import List, Optional, Dict
import os
from pathlib import Path

class AIConfig(BaseModel):
    """AI provider configuration"""
    provider: str = "openai"
    api_key: Optional[str] = Field(default_factory=lambda: os.getenv("OPENAI_API_KEY"))
    model: str = "gpt-4"
    temperature: float = 0.8
    max_tokens: int = 2000
    
class GameConfig(BaseModel):
    """Game configuration from wizard"""
    name: str
    genre: str
    perspective: str  # top-down, side-view, 3/4
    art_style: str    # specific pixel art style
    features: List[str]
    inspirations: List[str]  # Games selected in guided mode
    custom_description: Optional[str]  # From freeform mode
    
class GenerationConfig(BaseModel):
    """Generation pipeline configuration"""
    output_dir: Path = Path("generated_games")
    target_language: str = "rust"  # rust, python, ruby
    enable_cache: bool = True
    parallel_assets: bool = True
    style_consistency_check: bool = True
```

### 2. State Management

```python
# core/state.py
from dataclasses import dataclass, field
from typing import Dict, List, Optional
from datetime import datetime
import json

@dataclass
class GenerationState:
    """Tracks generation progress"""
    phase: str = "idle"
    progress: float = 0.0
    current_task: str = ""
    completed_tasks: List[str] = field(default_factory=list)
    errors: List[str] = field(default_factory=list)
    generated_assets: Dict[str, str] = field(default_factory=dict)
    
class AppState:
    """Global application state"""
    def __init__(self):
        self.game_config: Optional[GameConfig] = None
        self.generation_state = GenerationState()
        self.conversation_history: List[Dict] = []
        self.style_guide: Optional[Dict] = None
        
    def save_checkpoint(self, path: Path):
        """Save state for resumption"""
        checkpoint = {
            "timestamp": datetime.now().isoformat(),
            "game_config": self.game_config.dict() if self.game_config else None,
            "generation_state": asdict(self.generation_state),
            "style_guide": self.style_guide
        }
        with open(path, 'w') as f:
            json.dump(checkpoint, f, indent=2)
```

### 3. AI Orchestration

```python
# ai/orchestrator.py
from langchain.chains import LLMChain
from langchain.prompts import PromptTemplate
from litellm import completion
import asyncio
from typing import AsyncGenerator

class AIOrchestrator:
    """Main AI orchestration engine"""
    
    def __init__(self, config: AIConfig):
        self.config = config
        self.template_env = self._setup_templates()
        
    async def generate_style_guide(self, game_config: GameConfig) -> Dict:
        """Phase 1: Generate comprehensive style guide"""
        template = self.template_env.get_template("style_guide.jinja2")
        prompt = template.render(config=game_config)
        
        response = await completion(
            model=self.config.model,
            messages=[{"role": "user", "content": prompt}],
            temperature=0.3,  # Lower for consistency
        )
        
        return self._parse_style_guide(response)
        
    async def generate_metaprompts(self, game_config: GameConfig, style_guide: Dict) -> List[str]:
        """Generate feature-specific metaprompts"""
        metaprompts = []
        
        for feature in game_config.features:
            template = self.template_env.get_template(f"features/{feature}.jinja2")
            prompt = template.render(
                config=game_config,
                style_guide=style_guide
            )
            metaprompts.append(prompt)
            
        return metaprompts
        
    async def stream_generation(self, prompts: List[str]) -> AsyncGenerator[Dict, None]:
        """Stream generation progress"""
        for i, prompt in enumerate(prompts):
            async for chunk in self._generate_with_progress(prompt):
                yield {
                    "phase": f"prompt_{i}",
                    "progress": (i + chunk['progress']) / len(prompts),
                    "content": chunk['content']
                }
```

### 4. UI Implementation

```python
# ui/welcome.py
import streamlit as st
from PIL import Image
import asyncio

async def generate_welcome_images():
    """Generate DALL-E images for path selection"""
    # Implementation details...
    pass

def render_welcome():
    """Render the welcome screen"""
    st.set_page_config(page_title="Vintage Game Generator", layout="wide")
    
    st.title("🎮 Vintage Game Generator")
    st.markdown("### Choose Your Path")
    
    col1, col2 = st.columns(2)
    
    with col1:
        st.image("assets/guided_path.png", use_column_width=True)
        if st.button("🎯 Guided Experience", key="guided"):
            st.session_state.mode = "guided"
            st.rerun()
            
        st.caption("Select from classic games to create your unique blend")
        
    with col2:
        st.image("assets/freeform_path.png", use_column_width=True)
        if st.button("✏️ Freeform Creation", key="freeform"):
            st.session_state.mode = "freeform"
            st.rerun()
            
        st.caption("Describe your vision with AI assistance")
```

### 5. Generation Pipeline

```python
# generation/pipeline.py
from typing import Dict, List
import asyncio
from concurrent.futures import ThreadPoolExecutor

class GenerationPipeline:
    """Main generation pipeline"""
    
    def __init__(self, orchestrator: AIOrchestrator):
        self.orchestrator = orchestrator
        self.executor = ThreadPoolExecutor(max_workers=4)
        
    async def generate_game(self, config: GameConfig, callback=None):
        """Generate complete game"""
        
        # Phase 1: Style Guide
        if callback:
            callback({"phase": "style_guide", "progress": 0.1})
        style_guide = await self.orchestrator.generate_style_guide(config)
        
        # Phase 2: Architecture
        if callback:
            callback({"phase": "architecture", "progress": 0.2})
        architecture = await self.generate_architecture(config, style_guide)
        
        # Phase 3: Assets (Parallel)
        if callback:
            callback({"phase": "assets", "progress": 0.3})
        assets = await self.generate_assets_parallel(config, style_guide)
        
        # Phase 4: Code Generation
        if callback:
            callback({"phase": "code_generation", "progress": 0.7})
        code = await self.generate_code(config, architecture, assets)
        
        # Phase 5: Integration
        if callback:
            callback({"phase": "integration", "progress": 0.9})
        project = await self.integrate_project(code, assets)
        
        if callback:
            callback({"phase": "complete", "progress": 1.0})
            
        return project
```

## Key Design Decisions

### 1. Streamlit vs Gradio
- **Streamlit**: Better for complex, multi-page apps
- **Gradio**: Better for simple, single-page interfaces
- **Recommendation**: Start with Streamlit for flexibility

### 2. Async Architecture
- Use asyncio throughout for non-blocking operations
- LangChain's async support for AI calls
- Concurrent asset generation

### 3. Template Engine
- Jinja2 for all prompt templates
- Same syntax as MinJinja in Rust
- Easy migration of existing templates

### 4. State Persistence
- JSON-based checkpoints
- Resume interrupted generations
- Share configurations

### 5. Multi-Provider Support
- LiteLLM for provider abstraction
- Easy switching between OpenAI/Anthropic/etc
- Cost optimization

## Migration Path from Rust

1. **Port Templates**: Direct translation of .jinja files
2. **Port Models**: GameConfig → Pydantic models
3. **Port UI Logic**: Bevy systems → Streamlit components
4. **New Features**: Add Python-specific enhancements