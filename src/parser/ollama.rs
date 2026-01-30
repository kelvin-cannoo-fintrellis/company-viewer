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

/// Shared base prompt for all sections
#[rustfmt::skip]
const COMMON_PROMPT: &str = r#"
You are a data extraction engine.

IMPORTANT RULES:
- If a value is missing, unknown, or unclear, return an EMPTY STRING "".
- DO NOT use placeholder text ("Not provided", "Unknown", etc).
- DO NOT include explanations as values.
- Return ONLY valid JSON.
"#;

/// Build the final prompt using base + section rules
fn build_prompt(parser: &SectionParser, section_name: &str, section_content: &str) -> String {
    format!(
        r#"{common}

{rules}

Extract information from the "{section_name}" section.

Section:
{content}
"#,
        common = COMMON_PROMPT,
        rules = parser.prompt_rules(),
        section_name = section_name,
        content = section_content,
    )
}

/// Parse a section with structured output using Ollama's chat API
pub async fn parse_section_with_structured_output<T>(
    client: &Client,
    prompt: String,
) -> Result<T, Box<dyn Error>>
where
    T: DeserializeOwned + JsonSchema,
{
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
    /// Map index → section parser
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

    /// Get section name by index
    pub fn section_name(index: usize) -> &'static str {
        crate::ALL_SECTIONS.get(index).unwrap_or(&"Unknown Section")
    }

    /// Section-specific prompt rules
    pub fn prompt_rules(&self) -> &'static str {
        match self {
            #[rustfmt::skip]
            SectionParser::BusinessDetails => {
                r#"
The following section represents a TABLE with these columns:
1. Business Name
2. Nature of Business
3. Principal Place of Business

Table rules:
- Each logical row starts with either a Business Name or a single "." character.
- If a row starts with ".", the Business Name is empty. Store it EXACTLY as ".".
- Rows may span multiple lines; merge wrapped lines into one row.
- Ignore headers, repeated titles, page numbers, footers.
- Do not invent or infer data.
"#
            },
            #[rustfmt::skip]
            SectionParser::OfficeBearers => {
                r#"
The following section represents a TABLE with these columns:
1. Position
2. Name
3. Service Address
4. Appointed Date

CRITICAL EXTRACTION RULES (MUST FOLLOW):
- Position MUST contain only the role (e.g. DIRECTOR, SECRETARY, CHAIRMAN).
- Name MUST contain ONLY the entity name (person OR company).
- REMOVE the position title if it appears inside the Name.
  Example:
  Input: "DIRECTOR BEDEUX JEAN ALAIN"
  Output:
    Position = "DIRECTOR"
    Name = "BEDEUX JEAN ALAIN"

- Service Address may include street, city, and country.
- Country MUST be the LAST word in the address field.
- entityType MUST ALWAYS be an EMPTY STRING "".
- If a value is missing, return "" (empty string).
- DO NOT invent, infer, or normalize names.

Return ONLY valid JSON.
"#
            },

            _ => "",
        }
    }

    /// Parse section content using correct structured output type
    pub async fn parse(
        &self,
        client: &Client,
        section_content: &str,
        section_name: &str,
    ) -> Result<Value, Box<dyn Error>> {
        let prompt = build_prompt(self, section_name, section_content);

        match self {
            SectionParser::CompanyDetails => {
                let result: CompanyDetails =
                    parse_section_with_structured_output(client, prompt).await?;
                Ok(serde_json::to_value(result)?)
            }

            SectionParser::BusinessDetails => {
                let result: BusinessDetailsList =
                    parse_section_with_structured_output(client, prompt).await?;
                Ok(serde_json::to_value(result)?)
            }

            SectionParser::StatedCapital => {
                let result: StatedCapitalList =
                    parse_section_with_structured_output(client, prompt).await?;
                Ok(serde_json::to_value(result)?)
            }

            SectionParser::Certificates => {
                let result: CertificateList =
                    parse_section_with_structured_output(client, prompt).await?;
                Ok(serde_json::to_value(result)?)
            }

            SectionParser::OfficeBearers => {
                let result: OfficeBearerList =
                    parse_section_with_structured_output(client, prompt).await?;
                Ok(serde_json::to_value(result)?)
            }

            SectionParser::ShareHolders => {
                let result: ShareHolderList =
                    parse_section_with_structured_output(client, prompt).await?;
                Ok(serde_json::to_value(result)?)
            }

            SectionParser::AnnualReturns => {
                let result: AnnualReturnList =
                    parse_section_with_structured_output(client, prompt).await?;
                Ok(serde_json::to_value(result)?)
            }

            SectionParser::RegistrationFee => {
                let result: RegistrationFee =
                    parse_section_with_structured_output(client, prompt).await?;
                Ok(serde_json::to_value(result)?)
            }

            SectionParser::BalanceSheet => {
                let result: BalanceSheet =
                    parse_section_with_structured_output(client, prompt).await?;
                Ok(serde_json::to_value(result)?)
            }

            SectionParser::ProfitAndLoss => {
                let result: ProfitAndLoss =
                    parse_section_with_structured_output(client, prompt).await?;
                Ok(serde_json::to_value(result)?)
            }
        }
    }
}
