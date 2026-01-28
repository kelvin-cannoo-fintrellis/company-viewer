use reqwest::Client;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::{collections::HashSet, path::Path};

use crate::models::*;
pub mod models;

// All possible section headers (index in the array = section_index).
// Keep the order, because the function uses it to find the next header.
const ALL_SECTIONS: [&str; 17] = [
    "Company Details",
    "Business Details",
    "Particulars of Stated Capital",
    "Certificate (Issued by Other Institutions)",
    "Office Bearers",
    "Shareholders",
    "Members (Applicable for Company Limited by Guarantee or Shares and Guarantee)",
    "Annual Return filed for last 3 years",
    "Financial Summary/Statements filed for last 3 years",
    "Last Financial Summary Filed",
    "Profit and Loss Statement",
    "Balance Sheet",
    "Charges",
    "Removal/Winding Up Details",
    "Objections",
    "Last Annual Registration Fee Paid",
    "Extract of file with additional comments",
];

fn get_text_from_pdf(pdf_path: &str) -> String {
    let bytes = std::fs::read(pdf_path).unwrap();
    pdf_extract::extract_text_from_mem(&bytes).unwrap()
}

/// Extracts the content of a given section from a PDF's text representation.
///
/// * `section_index` – index of the section in `ALL_SECTIONS` (0‑based).
/// * `pdf_text` – the full plain‑text of the PDF (passed by reference).
///
/// The function returns the section's text **with** the section header itself.  
/// If the requested section is not found the returned string is empty.
pub fn extract_section(section_index: usize, pdf_text: &str) -> String {
    // Guard against out‑of‑range indices; if the caller passes an
    // invalid index we just return an empty string.
    if section_index >= ALL_SECTIONS.len() {
        return String::new();
    }

    // Split the PDF text into lines.
    let all_lines: Vec<&str> = pdf_text.lines().collect();

    // Find the first line that matches the requested section header.
    // If not found we return an empty string.
    let start_idx = match all_lines
        .iter()
        .position(|l| l.trim() == ALL_SECTIONS[section_index])
    {
        Some(idx) => idx,
        None => return String::new(),
    };

    // Build a quick‑lookup set of *all* headers so we can stop when any
    //    of them is encountered after the start line.
    let header_set: HashSet<&str> = ALL_SECTIONS.iter().copied().collect();

    // Find the index of the next header (if any) that appears after
    //    `start_idx`.  If none is found we simply read to the end.
    let mut end_idx = all_lines.len();
    for (i, line) in all_lines.iter().enumerate().skip(start_idx + 1) {
        if header_set.contains(line.trim()) {
            end_idx = i; // stop before this next header
            break;
        }
    }

    // Join all lines that belong to the section, re‑adding the line breaks.
    let mut result = String::new();
    for line in &all_lines[start_idx..end_idx] {
        result.push_str(line);
        result.push('\n');
    }

    // Trim a trailing newline (if the section was empty it will just be "").
    result.trim_end().to_string()
}

/// Extracts each section from the PDF and writes the result to a Markdown file.
///
/// # Arguments
/// * `pdf_path` – Path to the source PDF.
///
/// # Returns
/// * `Ok(())` on success; otherwise the I/O error that occurred.
///
/// # Example
/// ```ignore
/// test_section_extraction_to_markdown("my‑company.pdf")?;
/// ```
pub fn test_section_extraction_to_markdown(pdf_path: &str) -> std::io::Result<()> {
    let pdf_text = get_text_from_pdf(pdf_path);

    // --------------------------------------------------------------
    // Build the Markdown content
    // --------------------------------------------------------------
    let mut md = String::new();

    // File‑name for the table of contents / header
    let pdf_name = Path::new(pdf_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");

    md.push_str(&format!("# Extracted Sections from `{}`\n\n", pdf_name));

    for (idx, _) in ALL_SECTIONS.iter().enumerate() {
        // `extract_section` expects a `u8` – cast safely
        let section_body = extract_section(idx, &pdf_text);

        // Skip empty sections to keep the Markdown tidy
        if section_body.trim().is_empty() {
            continue;
        }

        // Level‑2 heading
        md.push_str(&format!("## {}\n\n", ALL_SECTIONS[idx]));

        // Body – preserve line breaks
        md.push_str(&section_body);
        md.push('\n');
        md.push_str("\n---\n\n"); // horizontal rule for visual separation
    }

    // --------------------------------------------------------------
    // Write the Markdown file
    // --------------------------------------------------------------
    // Determine an output path: same folder, .md extension
    let mut out_path = PathBuf::from(pdf_path);
    out_path.set_extension("md");

    let mut file = File::create(&out_path)?;
    file.write_all(md.as_bytes())?;

    println!("Markdown written to {}", out_path.display());
    Ok(())
}

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
    let prompt = format!(
        r#"You are a data extraction engine.
Extract information from the following "{}" section and return it in the specified JSON format.
Be precise and extract all available information.

Section:
{}"#,
        section_name, section_content
    );

    let schema = T::schema();

    let request = OllamaChatRequest {
        model: "qwen2.5:7b",
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
        ALL_SECTIONS.get(index).unwrap_or(&"Unknown Section")
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let pdf_text = get_text_from_pdf("pdf/77.pdf");

    // Example 1: Parse Office Bearers section
    let section_index = 4; // Office Bearers
    let section = extract_section(section_index, &pdf_text);

    if section.trim().is_empty() {
        println!("Section {} is empty", SectionParser::section_name(section_index));
    } else {
        println!("Parsing section: {}", SectionParser::section_name(section_index));
        
        if let Some(parser) = SectionParser::from_section_index(section_index) {
            let json = parser
                .parse(&client, &section, SectionParser::section_name(section_index))
                .await?;

            println!("Structured JSON:\n{}", serde_json::to_string_pretty(&json)?);
        } else {
            println!("No parser available for section index {}", section_index);
        }
    }

    // Example 2: Parse multiple sections
    println!("\n\n=== Parsing Multiple Sections ===\n");
    
    let sections_to_parse = vec![0, 4, 5]; // Company Details, Office Bearers, ShareHolders
    
    for section_index in sections_to_parse {
        let section = extract_section(section_index, &pdf_text);
        
        if section.trim().is_empty() {
            println!("Section {} is empty\n", SectionParser::section_name(section_index));
            continue;
        }
        
        println!("Parsing section: {}", SectionParser::section_name(section_index));
        
        if let Some(parser) = SectionParser::from_section_index(section_index) {
            match parser
                .parse(&client, &section, SectionParser::section_name(section_index))
                .await
            {
                Ok(json) => {
                    println!("Success! Preview:\n{}\n", 
                        serde_json::to_string_pretty(&json)
                            .unwrap_or_else(|_| "Error formatting JSON".to_string())
                            .lines()
                            .take(10)
                            .collect::<Vec<_>>()
                            .join("\n")
                    );
                }
                Err(e) => {
                    println!("Error parsing section: {}\n", e);
                }
            }
        }
    }

    Ok(())
}