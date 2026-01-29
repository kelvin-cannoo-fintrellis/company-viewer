use reqwest::Client;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::error::Error;

use crate::company::{
    AnnualReturnList, BusinessDetailsList, CertificateList, CompanyDetails, OfficeBearerList,
    RegistrationFee, ShareHolderList, StatedCapitalList,
};
use crate::financial::{BalanceSheet, ProfitAndLoss};
use crate::models::api::{JsonSchema, Message, OllamaChatRequest, OllamaChatResponse};

/// Parse a section with structured output using Ollama's chat API
///
/// # Type Parameters
/// * `T` - The type to deserialize the response into. Must implement DeserializeOwned and JsonSchema
///
/// # Arguments
/// * `client` - HTTP client for making requests
/// * `section_content` - The text content of the section to parse
/// * `section_name` - Human-readable name of the section for context
///
/// # Returns
/// * `Result<T, Box<dyn Error>>` - Parsed and typed response or error
pub async fn parse_section_with_structured_output<T>(
    client: &Client,
    section_content: &str,
    section_name: &str,
) -> Result<T, Box<dyn Error>>
where
    T: DeserializeOwned + JsonSchema,
{
    let prompt = if section_name == "Business Details" {
        format!(
            r#"You are a data extraction engine.

IMPORTANT RULES:
- If a value is missing, unknown, or unclear, return an EMPTY STRING "".
- DO NOT use placeholder text ("Not provided", "Unknown", etc).
- DO NOT include explanations as values.
- Return ONLY valid JSON.

The following section represents a TABLE with these columns:
1. Business Name
2. Nature of Business
3. Principal Place of Business

Table rules:
- Each logical row starts with either a Business Name or a single "." character.
- If a row starts with ".", the Business Name is empty. Store it EXACTLY as ".".
- Rows may span multiple lines; merge wrapped lines into one row.
- Ignore headers, repeated titles, registration numbers, page numbers, dates, and footers.
- Do not invent or infer data.

Section:
{}"#,
            section_content
        )
    } else if section_name == "Office Bearers" {
        format!(
            r#"You are a data extraction engine.

IMPORTANT RULES:
- If a value is missing, unknown, or unclear, return an EMPTY STRING "".
- The country is usually the last word in the address.
- DO NOT use placeholder text ("Not provided", "Unknown", etc).
- DO NOT include explanations as values.
- Return ONLY valid JSON.

Extract information from the following "{}" section.

Section:
{}"#,
            section_name, section_content
        )
    } else {
        format!(
            r#"You are a data extraction engine.

IMPORTANT RULES:
- If a value is missing, unknown, or unclear, return an EMPTY STRING "".
- DO NOT use placeholder text ("Not provided", "Unknown", etc).
- DO NOT include explanations as values.
- Return ONLY valid JSON.

Extract information from the following "{}" section.

Section:
{}"#,
            section_name, section_content
        )
    };

    let schema = T::schema();

    let request = OllamaChatRequest {
        model: "qwen2.5:3b",
        messages: vec![Message {
            role: "user",
            content: &prompt,
        }],
        stream: false,
        format: schema,
    };

    let res = client
        .post("http://localhost:11434/api/chat")
        .json(&request)
        .send()
        .await?
        .error_for_status()?;

    let chat_response: OllamaChatResponse = res.json().await?;

    // Parse the content into the target type
    let parsed: T = serde_json::from_str(&chat_response.message.content)?;

    Ok(parsed)
}

/// Section parser enum to dispatch parsing based on section type
pub enum SectionParser {
    CompanyDetails,
    BusinessDetails,
    StatedCapital,
    Certificates,
    OfficeBearers,
    ShareHolders,
    AnnualReturns,
    RegistrationFee,
    BalanceSheet,
    ProfitAndLoss,
}

impl SectionParser {
    /// Get the appropriate parser for a section index
    pub fn from_section_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(SectionParser::CompanyDetails),
            1 => Some(SectionParser::BusinessDetails),
            2 => Some(SectionParser::StatedCapital),
            3 => Some(SectionParser::Certificates),
            4 => Some(SectionParser::OfficeBearers),
            5 => Some(SectionParser::ShareHolders),
            // Note: index 6 belongs to the Members section and is always empty. We ignore it.
            7 => Some(SectionParser::AnnualReturns),
            10 => Some(SectionParser::ProfitAndLoss),
            11 => Some(SectionParser::BalanceSheet),
            15 => Some(SectionParser::RegistrationFee),
            _ => None,
        }
    }

    /// Get the section name for a section index
    pub fn section_name(index: usize) -> &'static str {
        crate::ALL_SECTIONS.get(index).unwrap_or(&"Unknown Section")
    }

    /// Parse the section content using the appropriate parser
    pub async fn parse(
        &self,
        client: &Client,
        section_content: &str,
        section_name: &str,
    ) -> Result<Value, Box<dyn Error>> {
        match self {
            SectionParser::CompanyDetails => {
                let result: CompanyDetails =
                    parse_section_with_structured_output(client, section_content, section_name)
                        .await?;
                Ok(serde_json::to_value(result)?)
            }
            SectionParser::BusinessDetails => {
                let result: BusinessDetailsList =
                    parse_section_with_structured_output(client, section_content, section_name)
                        .await?;
                Ok(serde_json::to_value(result)?)
            }
            SectionParser::StatedCapital => {
                let result: StatedCapitalList =
                    parse_section_with_structured_output(client, section_content, section_name)
                        .await?;
                Ok(serde_json::to_value(result)?)
            }
            SectionParser::Certificates => {
                let result: CertificateList =
                    parse_section_with_structured_output(client, section_content, section_name)
                        .await?;
                Ok(serde_json::to_value(result)?)
            }
            SectionParser::OfficeBearers => {
                let result: OfficeBearerList =
                    parse_section_with_structured_output(client, section_content, section_name)
                        .await?;
                Ok(serde_json::to_value(result)?)
            }
            SectionParser::ShareHolders => {
                let result: ShareHolderList =
                    parse_section_with_structured_output(client, section_content, section_name)
                        .await?;
                Ok(serde_json::to_value(result)?)
            }
            SectionParser::AnnualReturns => {
                let result: AnnualReturnList =
                    parse_section_with_structured_output(client, section_content, section_name)
                        .await?;
                Ok(serde_json::to_value(result)?)
            }
            SectionParser::RegistrationFee => {
                let result: RegistrationFee =
                    parse_section_with_structured_output(client, section_content, section_name)
                        .await?;
                Ok(serde_json::to_value(result)?)
            }
            SectionParser::BalanceSheet => {
                let result: BalanceSheet =
                    parse_section_with_structured_output(client, section_content, section_name)
                        .await?;
                Ok(serde_json::to_value(result)?)
            }
            SectionParser::ProfitAndLoss => {
                let result: ProfitAndLoss =
                    parse_section_with_structured_output(client, section_content, section_name)
                        .await?;
                Ok(serde_json::to_value(result)?)
            }
        }
    }
}
