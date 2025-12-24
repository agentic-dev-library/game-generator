//! Voice synthesis module using ElevenLabs
//!
//! This module provides text-to-speech capabilities for game dialogue and narration.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::cache::{AiCache, CachedData};
use super::tokens::TokenCounter;

#[cfg(feature = "voice")]
use sha2::{Digest, Sha256};

#[cfg(feature = "voice")]
use llm::tts::{TtsProvider, Voice, ElevenLabsConfig};

/// Voice generator for game dialogue and narration
#[derive(Clone)]
pub struct VoiceGenerator {
    cache: Arc<Mutex<AiCache>>,
    token_counter: Arc<Mutex<TokenCounter>>,
    api_key: String,
}

/// Configuration for voice synthesis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceConfig {
    /// Voice ID or name
    pub voice_id: String,
    /// Model to use (e.g., "eleven_multilingual_v2")
    pub model: String,
    /// Stability (0.0 - 1.0)
    pub stability: f32,
    /// Similarity boost (0.0 - 1.0)
    pub similarity_boost: f32,
    /// Style (0.0 - 1.0)
    pub style: f32,
    /// Use speaker boost
    pub use_speaker_boost: bool,
    /// Speaking rate adjustment
    pub rate: f32,
    /// Pitch adjustment
    pub pitch: f32,
    /// Output format (mp3, wav)
    pub format: String,
}

impl Default for VoiceConfig {
    fn default() -> Self {
        Self {
            voice_id: "21m00Tcm4TlvDq8ikWAM".to_string(), // Rachel
            model: "eleven_multilingual_v2".to_string(),
            stability: 0.5,
            similarity_boost: 0.75,
            style: 0.0,
            use_speaker_boost: true,
            rate: 1.0,
            pitch: 1.0,
            format: "mp3".to_string(),
        }
    }
}

impl VoiceGenerator {
    /// Create a new voice generator
    pub fn new(
        cache: Arc<Mutex<AiCache>>,
        token_counter: Arc<Mutex<TokenCounter>>,
    ) -> Self {
        let api_key = std::env::var("ELEVENLABS_API_KEY").unwrap_or_default();
        Self {
            cache,
            token_counter,
            api_key,
        }
    }

    /// Generate voice audio for a piece of text
    pub async fn generate_voice(
        &self,
        text: &str,
        config: &VoiceConfig,
    ) -> Result<Vec<u8>> {
        #[cfg(not(feature = "voice"))]
        {
            anyhow::bail!("Voice feature is not enabled. Enable 'voice' feature to use ElevenLabs.")
        }

        #[cfg(feature = "voice")]
        {
            if self.api_key.is_empty() {
                anyhow::bail!("ELEVENLABS_API_KEY environment variable is not set");
            }

            // Generate cache key
            let mut params = HashMap::new();
            params.insert("voice_id".to_string(), config.voice_id.clone());
            params.insert("model".to_string(), config.model.clone());
            params.insert("text_hash".to_string(), format!("{:x}", sha2::Sha256::digest(text.as_bytes())));

            let cache_key = self
                .cache
                .lock()
                .await
                .generate_key("voice", "elevenlabs", &params);

            // Check cache
            if let Some(cached) = self.cache.lock().await.get(&cache_key).await {
                if let CachedData::Binary(data) = cached.data {
                    return Ok(data);
                }
            }

            // Prepare ElevenLabs config
            let tts = TtsProvider::ElevenLabs(ElevenLabsConfig {
                api_key: self.api_key.clone(),
                voice_id: config.voice_id.clone(),
                model: Some(config.model.clone()),
                stability: Some(config.stability),
                similarity_boost: Some(config.similarity_boost),
                style: Some(config.style),
                use_speaker_boost: Some(config.use_speaker_boost),
            });

            // Generate audio
            let audio_data = tts.generate(text).await
                .context("Failed to generate voice audio from ElevenLabs")?;

            // Cache result
            let mut cache_params = HashMap::new();
            for (k, v) in params {
                cache_params.insert(k, serde_json::Value::String(v));
            }
            
            self.cache
                .lock()
                .await
                .put(cache_key, CachedData::Binary(audio_data.clone()), cache_params)
                .await?;

            // Record usage (simplified token count for voice)
            self.token_counter
                .lock()
                .await
                .record_usage("elevenlabs", text.len() / 4, 0)
                .await?;

            Ok(audio_data)
        }
    }

    /// Save generated voice to a file in the asset directory
    pub async fn save_voice_to_file(
        &self,
        text: &str,
        config: &VoiceConfig,
        output_path: &Path,
    ) -> Result<()> {
        let audio_data = self.generate_voice(text, config).await?;
        
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        std::fs::write(output_path, audio_data)?;
        Ok(())
    }

    /// Get available voices
    pub async fn get_available_voices(&self) -> Result<Vec<VoiceInfo>> {
        #[cfg(not(feature = "voice"))]
        {
            anyhow::bail!("Voice feature is not enabled")
        }

        #[cfg(feature = "voice")]
        {
            // This would normally call an ElevenLabs API to list voices
            // For now, returning some defaults
            Ok(vec![
                VoiceInfo {
                    id: "21m00Tcm4TlvDq8ikWAM".to_string(),
                    name: "Rachel".to_string(),
                    category: "premade".to_string(),
                    description: "Female, soft, American".to_string(),
                },
                VoiceInfo {
                    id: "AZnzlk1Xhk6s7t6p32M5".to_string(),
                    name: "Nicole".to_string(),
                    category: "premade".to_string(),
                    description: "Female, energetic, American".to_string(),
                },
                VoiceInfo {
                    id: "EXAVITQu4vr4xn7AYnmo".to_string(),
                    name: "Bella".to_string(),
                    category: "premade".to_string(),
                    description: "Female, soft, American".to_string(),
                },
            ])
        }
    }
}

/// Information about a voice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceInfo {
    pub id: String,
    pub name: String,
    pub category: String,
    pub description: String,
}

#[async_trait::async_trait]
impl super::AiGenerator for VoiceGenerator {
    async fn estimate_tokens(&self, request: &str) -> Result<usize> {
        Ok(request.len() / 4)
    }

    async fn estimate_cost(&self, request: &str) -> Result<f64> {
        // ElevenLabs cost is roughly $0.0003 per character for higher tiers
        Ok(request.len() as f64 * 0.0003)
    }

    async fn is_cached(&self, key: &str) -> bool {
        self.cache.lock().await.get(key).await.is_some()
    }

    async fn clear_cache(&self, key: &str) -> Result<()> {
        self.cache.lock().await.clear(key).await
    }
}
