use anyhow::{Result, anyhow};
use serde_json::{json, Value};
use reqwest;
use pdf_extract;

#[derive(Debug)]
pub struct PdfExtractor {
    groq_api_key: String,
}

impl PdfExtractor {
    pub fn new(groq_api_key: String) -> Self {
        Self { groq_api_key }
    }

    /// Extract information from a PDF using GROQ API based on a prompt
    pub async fn extract_pdf_to_json(
        &self,
        pdf_path: &str,
        prompt: &str,
    ) -> Result<Value> {
        // Extract text from PDF
        let text = self.extract_pdf_text(pdf_path)?;
        
        // Make API call to GROQ
        let result = self.call_groq_api(&text, prompt).await?;
        
        Ok(result)
    }

    /// Extract text from PDF file
    fn extract_pdf_text(&self, pdf_path: &str) -> Result<String> {
        // let bytes = fs::read(pdf_path)?;
        
        match pdf_extract::extract_text(&pdf_path) {
            Ok(text) => Ok(text),
            Err(e) => Err(anyhow!("Failed to extract PDF text: {}", e)),
        }
    }

    /// Make API call to GROQ
    async fn call_groq_api(&self, text: &str, prompt: &str) -> Result<Value> {
        let client = reqwest::Client::new();
        let url = "https://api.groq.com/openai/v1/chat/completions";

        let request_body = json!({
            "model": "meta-llama/llama-4-scout-17b-16e-instruct",
            "response_format": {"type": "json_object"},
            "messages": [
                {
                    "role": "system",
                    "content": format!(
                        "You are a helpful assistant that extracts specific information from documents. The user wants you to: {}\n\nPlease return your response as valid JSON only, with no additional text or formatting.",
                        prompt
                    )
                },
                {
                    "role": "user",
                    "content": format!("Here is the document text:\n\n{}", text)
                }
            ],
            "temperature": 0.1,
            "max_tokens": 2000
        });

        let response = client
            .post(url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.groq_api_key))
            .json(&request_body)
            .send()
            .await?;

        if response.status().is_success() {
            let response_json: Value = response.json().await?;
            
            // Extract content from response
            if let Some(content) = response_json
                .get("choices")
                .and_then(|choices| choices.get(0))
                .and_then(|choice| choice.get("message"))
                .and_then(|message| message.get("content"))
                .and_then(|content| content.as_str())
            {
                // Parse the content as JSON
                match serde_json::from_str::<Value>(content) {
                    Ok(parsed_json) => Ok(parsed_json),
                    Err(e) => Ok(json!({
                        "error": format!("Failed to parse response: {}", e),
                        "raw_response": response_json
                    }))
                }
            } else {
                Ok(json!({
                    "error": "Invalid response format",
                    "raw_response": response_json
                }))
            }
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Ok(json!({
                "error": format!("API call failed: {}", status),
                "message": error_text
            }))
        }
    }
}

pub async fn extract_contact_info(api_key: String, pdf_file: &str, prompt: &str) -> Result<String> {
    let extractor = PdfExtractor::new(api_key);
    let result: Value = extractor.extract_pdf_to_json(pdf_file, prompt).await?;
    let formatted = serde_json::to_string_pretty(&result)?;
    Ok(formatted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pdf_extractor_creation() {
        let extractor = PdfExtractor::new("test_key".to_string());
        assert_eq!(extractor.groq_api_key, "test_key");
    }

    #[test]
    fn test_extract_pdf_text_nonexistent_file() {
        let extractor = PdfExtractor::new("test_key".to_string());
        let result = extractor.extract_pdf_text("nonexistent.pdf");
        assert!(result.is_err());
    }
}