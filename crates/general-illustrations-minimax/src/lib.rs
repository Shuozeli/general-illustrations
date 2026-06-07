use base64::Engine;
use general_illustrations_core::{
    GeneratedImage, ImageGenerationRequest, ImageProvider, ImageProviderError, ImageProviderId,
    OutputFormat,
};
use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::json;

const DEFAULT_ENDPOINT: &str = "https://api.minimaxi.com/v1/image_generation";
const DEFAULT_MODEL: &str = "image-01";

#[derive(Debug, Clone)]
pub struct MinimaxImageProvider {
    api_key: String,
    endpoint: String,
    model: String,
    client: Client,
}

impl MinimaxImageProvider {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            endpoint: DEFAULT_ENDPOINT.to_string(),
            model: DEFAULT_MODEL.to_string(),
            client: Client::new(),
        }
    }

    pub fn with_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = endpoint.into();
        self
    }

    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }
}

impl ImageProvider for MinimaxImageProvider {
    fn id(&self) -> ImageProviderId {
        ImageProviderId::Minimax
    }

    fn generate(
        &self,
        request: &ImageGenerationRequest,
    ) -> Result<Vec<GeneratedImage>, ImageProviderError> {
        let payload = json!({
            "model": self.model,
            "prompt": request.prompt,
            "aspect_ratio": request.aspect_ratio.as_provider_value(),
            "response_format": "base64",
        });

        let response = self
            .client
            .post(&self.endpoint)
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .map_err(|error| ImageProviderError::Transport {
                provider: self.id(),
                message: error.to_string(),
            })?;

        let http_status = response.status();
        let body = response
            .text()
            .map_err(|error| ImageProviderError::Transport {
                provider: self.id(),
                message: error.to_string(),
            })?;

        if !http_status.is_success() {
            return Err(ImageProviderError::ProviderRejected {
                provider: self.id(),
                message: format!("HTTP {http_status}: {body}"),
            });
        }

        decode_minimax_response(&body, request.output_format.clone())
    }
}

#[derive(Debug, Deserialize)]
struct MinimaxResponse {
    data: Option<MinimaxData>,
    base_resp: Option<MinimaxBaseResp>,
}

#[derive(Debug, Deserialize)]
struct MinimaxData {
    image_base64: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct MinimaxBaseResp {
    status_code: i64,
    status_msg: String,
}

fn decode_minimax_response(
    body: &str,
    output_format: OutputFormat,
) -> Result<Vec<GeneratedImage>, ImageProviderError> {
    let response: MinimaxResponse =
        serde_json::from_str(body).map_err(|error| ImageProviderError::UnsupportedResponse {
            provider: ImageProviderId::Minimax,
            message: error.to_string(),
        })?;

    if let Some(base_resp) = response.base_resp
        && base_resp.status_code != 0
    {
        return Err(ImageProviderError::ProviderRejected {
            provider: ImageProviderId::Minimax,
            message: format!("{} ({})", base_resp.status_msg, base_resp.status_code),
        });
    }

    let images = response
        .data
        .and_then(|data| data.image_base64)
        .ok_or_else(|| ImageProviderError::UnsupportedResponse {
            provider: ImageProviderId::Minimax,
            message: "missing data.image_base64".to_string(),
        })?;

    images
        .into_iter()
        .map(|image| {
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(image)
                .map_err(|error| ImageProviderError::UnsupportedResponse {
                    provider: ImageProviderId::Minimax,
                    message: error.to_string(),
                })?;

            Ok(GeneratedImage {
                bytes,
                output_format: output_format.clone(),
                provider: ImageProviderId::Minimax,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use general_illustrations_core::OutputFormat;

    use super::*;

    #[test]
    fn decodes_successful_response() {
        // Arrange
        let body = r#"{
            "data": { "image_base64": ["aGVsbG8="] },
            "base_resp": { "status_code": 0, "status_msg": "success" }
        }"#;

        // Act
        let images = decode_minimax_response(body, OutputFormat::Jpeg).unwrap();

        // Assert
        assert_eq!(images.len(), 1);
        assert_eq!(images[0].bytes, b"hello");
        assert_eq!(images[0].output_format, OutputFormat::Jpeg);
        assert_eq!(images[0].provider, ImageProviderId::Minimax);
    }

    #[test]
    fn reports_provider_rejection() {
        // Arrange
        let body = r#"{
            "base_resp": { "status_code": 1008, "status_msg": "insufficient balance" }
        }"#;

        // Act
        let error = decode_minimax_response(body, OutputFormat::Png).unwrap_err();

        // Assert
        assert_eq!(
            error.to_string(),
            "provider minimax rejected request: insufficient balance (1008)"
        );
    }
}
