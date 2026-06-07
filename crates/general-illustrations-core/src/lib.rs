use std::fmt;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ImageProviderId {
    Codex,
    Minimax,
    Gemini,
    Custom,
}

impl fmt::Display for ImageProviderId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            ImageProviderId::Codex => "codex",
            ImageProviderId::Minimax => "minimax",
            ImageProviderId::Gemini => "gemini",
            ImageProviderId::Custom => "custom",
        };
        f.write_str(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AspectRatio {
    Square,
    #[default]
    Landscape16x9,
    Portrait9x16,
    Custom(String),
}

impl AspectRatio {
    pub fn as_provider_value(&self) -> &str {
        match self {
            AspectRatio::Square => "1:1",
            AspectRatio::Landscape16x9 => "16:9",
            AspectRatio::Portrait9x16 => "9:16",
            AspectRatio::Custom(value) => value.as_str(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OutputFormat {
    #[default]
    Png,
    Jpeg,
}

impl OutputFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            OutputFormat::Png => "png",
            OutputFormat::Jpeg => "jpeg",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageGenerationRequest {
    pub prompt: String,
    pub aspect_ratio: AspectRatio,
    pub output_format: OutputFormat,
    pub n: u8,
}

impl ImageGenerationRequest {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            aspect_ratio: AspectRatio::default(),
            output_format: OutputFormat::default(),
            n: 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedImage {
    pub bytes: Vec<u8>,
    pub output_format: OutputFormat,
    pub provider: ImageProviderId,
}

#[derive(Debug, Error)]
pub enum ImageProviderError {
    #[error("provider {provider} rejected request: {message}")]
    ProviderRejected {
        provider: ImageProviderId,
        message: String,
    },
    #[error("provider {provider} returned unsupported response: {message}")]
    UnsupportedResponse {
        provider: ImageProviderId,
        message: String,
    },
    #[error("transport error from provider {provider}: {message}")]
    Transport {
        provider: ImageProviderId,
        message: String,
    },
}

pub trait ImageProvider {
    fn id(&self) -> ImageProviderId;

    fn generate(
        &self,
        request: &ImageGenerationRequest,
    ) -> Result<Vec<GeneratedImage>, ImageProviderError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aspect_ratio_formats_provider_values() {
        // Arrange
        let landscape = AspectRatio::Landscape16x9;
        let portrait = AspectRatio::Portrait9x16;
        let custom = AspectRatio::Custom("4:3".to_string());

        // Act
        let values = [
            landscape.as_provider_value(),
            portrait.as_provider_value(),
            custom.as_provider_value(),
        ];

        // Assert
        assert_eq!(values, ["16:9", "9:16", "4:3"]);
    }

    #[test]
    fn request_defaults_to_single_landscape_png() {
        // Arrange
        let prompt = "draw an LSM tree";

        // Act
        let request = ImageGenerationRequest::new(prompt);

        // Assert
        assert_eq!(request.prompt, prompt);
        assert_eq!(request.aspect_ratio, AspectRatio::Landscape16x9);
        assert_eq!(request.output_format, OutputFormat::Png);
        assert_eq!(request.n, 1);
    }
}
