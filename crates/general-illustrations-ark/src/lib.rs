use base64::Engine;
use general_illustrations_core::{
    AspectRatio, GeneratedImage, ImageGenerationRequest, ImageProvider, ImageProviderError,
    ImageProviderId, OutputFormat,
};
use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::json;

const DEFAULT_ENDPOINT: &str = "https://ark.cn-beijing.volces.com/api/plan/v3/images/generations";
const DEFAULT_MODEL: &str = "doubao-seedream-5.0-lite";

#[derive(Debug, Clone)]
pub struct ArkImageProvider {
    api_key: String,
    endpoint: String,
    model: String,
    client: Client,
}

impl ArkImageProvider {
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

impl ImageProvider for ArkImageProvider {
    fn id(&self) -> ImageProviderId {
        ImageProviderId::Ark
    }

    fn generate(
        &self,
        request: &ImageGenerationRequest,
    ) -> Result<Vec<GeneratedImage>, ImageProviderError> {
        let payload = json!({
            "model": self.model,
            "prompt": prompt_with_aspect_ratio(request),
            "size": ark_size(&request.aspect_ratio),
            "output_format": ark_output_format(&request.output_format),
            "watermark": false,
            "response_format": "url",
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

        self.decode_ark_response(&body, request.output_format.clone())
    }
}

#[derive(Debug, Deserialize)]
struct ArkResponse {
    data: Vec<ArkImage>,
}

#[derive(Debug, Deserialize)]
struct ArkImage {
    b64_json: Option<String>,
    url: Option<String>,
}

fn prompt_with_aspect_ratio(request: &ImageGenerationRequest) -> String {
    format!(
        "{}\n\nCanvas: {} aspect ratio.",
        request.prompt,
        request.aspect_ratio.as_provider_value()
    )
}

fn ark_size(aspect_ratio: &AspectRatio) -> &'static str {
    match aspect_ratio {
        AspectRatio::Square => "2K",
        AspectRatio::Landscape16x9 | AspectRatio::Portrait9x16 | AspectRatio::Custom(_) => "2K",
    }
}

fn ark_output_format(output_format: &OutputFormat) -> &'static str {
    match output_format {
        OutputFormat::Png => "png",
        OutputFormat::Jpeg => "jpeg",
    }
}

impl ArkImageProvider {
    fn decode_ark_response(
        &self,
        body: &str,
        output_format: OutputFormat,
    ) -> Result<Vec<GeneratedImage>, ImageProviderError> {
        let response = parse_ark_response(body)?;

        response
            .data
            .into_iter()
            .map(|image| {
                let bytes = match (image.b64_json, image.url) {
                    (Some(b64_json), _) => decode_b64(&b64_json)?,
                    (None, Some(url)) => self.download_image(&url)?,
                    (None, None) => {
                        return Err(ImageProviderError::UnsupportedResponse {
                            provider: ImageProviderId::Ark,
                            message: "missing data[].b64_json and data[].url".to_string(),
                        });
                    }
                };
                Ok(GeneratedImage {
                    bytes,
                    output_format: output_format.clone(),
                    provider: ImageProviderId::Ark,
                })
            })
            .collect()
    }

    fn download_image(&self, url: &str) -> Result<Vec<u8>, ImageProviderError> {
        let response =
            self.client
                .get(url)
                .send()
                .map_err(|error| ImageProviderError::Transport {
                    provider: self.id(),
                    message: error.to_string(),
                })?;
        let status = response.status();
        if !status.is_success() {
            return Err(ImageProviderError::ProviderRejected {
                provider: self.id(),
                message: format!("image download returned HTTP {status}"),
            });
        }
        response
            .bytes()
            .map(Vec::from)
            .map_err(|error| ImageProviderError::Transport {
                provider: self.id(),
                message: error.to_string(),
            })
    }
}

fn parse_ark_response(body: &str) -> Result<ArkResponse, ImageProviderError> {
    serde_json::from_str(body).map_err(|error| ImageProviderError::UnsupportedResponse {
        provider: ImageProviderId::Ark,
        message: error.to_string(),
    })
}

fn decode_b64(value: &str) -> Result<Vec<u8>, ImageProviderError> {
    base64::engine::general_purpose::STANDARD
        .decode(value)
        .map_err(|error| ImageProviderError::UnsupportedResponse {
            provider: ImageProviderId::Ark,
            message: error.to_string(),
        })
}

#[cfg(test)]
fn decode_ark_b64_response(
    body: &str,
    output_format: OutputFormat,
) -> Result<Vec<GeneratedImage>, ImageProviderError> {
    let response: ArkResponse =
        serde_json::from_str(body).map_err(|error| ImageProviderError::UnsupportedResponse {
            provider: ImageProviderId::Ark,
            message: error.to_string(),
        })?;

    response
        .data
        .into_iter()
        .map(|image| {
            let b64_json =
                image
                    .b64_json
                    .ok_or_else(|| ImageProviderError::UnsupportedResponse {
                        provider: ImageProviderId::Ark,
                        message: format!(
                            "missing data[].b64_json; url response was {:?}",
                            image.url.as_deref()
                        ),
                    })?;
            let bytes = decode_b64(&b64_json)?;
            Ok(GeneratedImage {
                bytes,
                output_format: output_format.clone(),
                provider: ImageProviderId::Ark,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_b64_json_response() {
        let body = r#"{"data":[{"b64_json":"aGVsbG8="}]}"#;

        let images = decode_ark_b64_response(body, OutputFormat::Png).unwrap();

        assert_eq!(images.len(), 1);
        assert_eq!(images[0].bytes, b"hello");
        assert_eq!(images[0].provider, ImageProviderId::Ark);
    }

    #[test]
    fn parses_url_response() {
        let body = r#"{"data":[{"url":"https://example.com/image.png"}]}"#;

        let response = parse_ark_response(body).unwrap();

        assert_eq!(
            response.data[0].url.as_deref(),
            Some("https://example.com/image.png")
        );
    }
}
