use reqwest::Client;
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::{collections::HashSet, path::Path};

use crate::models::{OllamaRequest, OllamaResponse};
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

/// Extracts the content of a given section from a PDF’s text representation.
///
/// * `section_index` – index of the section in `ALL_SECTIONS` (0‑based).
/// * `pdf_text` – the full plain‑text of the PDF (passed by reference).
///
/// The function returns the section’s text **with** the section header itself.  
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
    // --------------------------------------------------------------
    // 1️⃣  Load the whole PDF as text
    // --------------------------------------------------------------
    let pdf_text = get_text_from_pdf(pdf_path);

    // --------------------------------------------------------------
    // 2️⃣  Build the Markdown content
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
    // 3️⃣  Write the Markdown file
    // --------------------------------------------------------------
    // Determine an output path: same folder, .md extension
    let mut out_path = PathBuf::from(pdf_path);
    out_path.set_extension("md");

    let mut file = File::create(&out_path)?;
    file.write_all(md.as_bytes())?;

    println!("Markdown written to {}", out_path.display());
    Ok(())
}

/// Sends section content to a local Ollama LLM and asks it to return JSON
pub async fn parse_section_with_ollama(
    client: &Client,
    section_content: &str,
) -> Result<Value, Box<dyn Error>> {
    let prompt = format!(
        r#"
You are a data extraction engine.
Parse the following section and return ONLY valid JSON.
Do not include explanations, markdown, or extra text.

Section:
{}
"#,
        section_content
    );

    let request = OllamaRequest {
        model: "qwen2.5:7b",
        prompt: &prompt,
        stream: false,
        format: "json"
    };

    let res = client
        .post("http://localhost:11434/api/generate")
        .json(&request)
        .send()
        .await?
        .error_for_status()?;

    let ollama_res: OllamaResponse = res.json().await?;

    // Ollama returns JSON *as text*, so parse again
    let parsed_json: Value = serde_json::from_str(ollama_res.response.trim())?;

    Ok(parsed_json)
}

// removes irrelevant details from a pdf
fn filter_irrelevant_details(pdf_text: String) -> String {
    pdf_text
}

// make an api call to ollama local model
fn text_to_structured_format() {
    let client: Client = Client::builder().build().unwrap();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let pdf_text = get_text_from_pdf("pdf/77.pdf");
    let section = extract_section(4, &pdf_text); // Office Bearers

    if section.trim().is_empty() {
        println!("Section empty");
        return Ok(());
    }

    let json = parse_section_with_ollama(&client, &section).await?;

    println!("Structured JSON:\n{}", serde_json::to_string_pretty(&json)?);

    Ok(())

    // test_section_extraction_to_markdown("pdf/77.pdf");
}
