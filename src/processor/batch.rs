use reqwest::Client;
use serde_json::Value;
use std::error::Error;
use std::path::Path;

use crate::parser::ollama::SectionParser;
use crate::parser::pdf::get_text_from_pdf;
use crate::parser::section::extract_section;

/// Helper function to convert parsed JSON to the appropriate output key and value
fn output_key_and_value(section_index: usize, json: Value) -> Option<(String, Value)> {
    let key = match section_index {
        0 => "companyDetails",
        1 => "businessDetails",
        4 => "officeBearers",
        5 => "shareHolders",
        _ => return None,
    };

    let value = match json {
        Value::Object(obj) if obj.len() == 1 => {
            // Extract the only value safely
            let (_, v) = obj.into_iter().next().unwrap();
            v
        }
        other => other,
    };

    Some((key.to_string(), value))
}

/// Build markdown representation of extracted sections
fn build_markdown_for_pdf(pdf_name: &str, pdf_text: &str, sections_to_parse: &[usize]) -> String {
    let mut md = String::new();

    md.push_str(&format!("# Extracted Sections from `{}`\n\n", pdf_name));

    for &idx in sections_to_parse {
        let section_name = crate::ALL_SECTIONS.get(idx).unwrap_or(&"Unknown Section");

        let section = extract_section(idx, pdf_text);

        if section.trim().is_empty() {
            continue;
        }

        md.push_str(&format!("## {}\n\n", section_name));
        md.push_str(&section);
        md.push_str("\n\n---\n\n");
    }

    md
}

/// Process all PDFs in a directory and save parsed sections to JSON files
///
/// # Arguments
/// * `input_dir` - Directory containing PDF files to process
/// * `output_dir` - Directory where JSON output files will be saved
/// * `markdown_dir` - Directory where markdown files will be saved
///
/// # Returns
/// * `Result<(), Box<dyn Error>>` - Success or error
pub async fn process_pdfs_in_directory(
    input_dir: &str,
    output_dir: &str,
) -> Result<(), Box<dyn Error>> {
    let debugging = true;
    let debug_markdown_dir = "output_markdown";
    let client = Client::new();

    std::fs::create_dir_all(output_dir)?;

    if debugging {
        std::fs::create_dir_all(debug_markdown_dir)?;
    }

    let sections_to_parse = [0, 1, 4];

    for entry in std::fs::read_dir(input_dir)? {
        let path = entry?.path();

        if path.extension().map_or(true, |e| e != "pdf") {
            continue;
        }

        let pdf_path = path.to_str().unwrap();
        let pdf_filename = path.file_stem().unwrap().to_str().unwrap();

        println!("Processing: {}", pdf_filename);

        let pdf_text = get_text_from_pdf(pdf_path);

        // ---------------- JSON ----------------
        let mut pdf_data = serde_json::Map::new();
        pdf_data.insert(
            "filename".to_string(),
            Value::String(pdf_filename.to_string()),
        );

        for &section_index in &sections_to_parse {
            let section_text = extract_section(section_index, &pdf_text);
            let section_name = SectionParser::section_name(section_index);

            if section_text.trim().is_empty() {
                continue;
            }

            println!("  - Parsing {}", section_name);

            if let Some(parser) = SectionParser::from_section_index(section_index) {
                match parser.parse(&client, &section_text, section_name).await {
                    Ok(json) => {
                        if let Some((key, value)) = output_key_and_value(section_index, json) {
                            pdf_data.insert(key, value);
                        }
                    }
                    Err(e) => {
                        println!("  ✗ {} parse error: {}", section_name, e);
                    }
                }
            }
        }

        let json_path = format!("{}/{}.json", output_dir, pdf_filename);
        std::fs::write(&json_path, serde_json::to_string_pretty(&pdf_data)?)?;

        // ---------------- MARKDOWN ----------------
        if debugging {
            let markdown = build_markdown_for_pdf(pdf_filename, &pdf_text, &sections_to_parse);
            let md_path = format!("{}/{}.md", debug_markdown_dir, pdf_filename);
            std::fs::write(&md_path, markdown)?;
        }
    }

    Ok(())
}

/// Process a single PDF file and return the parsed data
///
/// # Arguments
/// * `pdf_path` - Path to the PDF file
/// * `sections_to_parse` - Optional list of section indices to parse. If None, parses default sections.
///
/// # Returns
/// * `Result<serde_json::Map<String, Value>, Box<dyn Error>>` - Parsed data or error
pub async fn process_single_pdf(
    pdf_path: &str,
    sections_to_parse: Option<&[usize]>,
) -> Result<serde_json::Map<String, Value>, Box<dyn Error>> {
    let client = Client::new();
    let sections = sections_to_parse.unwrap_or(&[0, 1, 4, 5]);

    let pdf_filename = Path::new(pdf_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");

    let pdf_text = get_text_from_pdf(pdf_path);

    let mut pdf_data = serde_json::Map::new();
    pdf_data.insert(
        "filename".to_string(),
        Value::String(pdf_filename.to_string()),
    );

    for &section_index in sections {
        let section_text = extract_section(section_index, &pdf_text);
        let section_name = SectionParser::section_name(section_index);

        if section_text.trim().is_empty() {
            continue;
        }

        if let Some(parser) = SectionParser::from_section_index(section_index) {
            match parser.parse(&client, &section_text, section_name).await {
                Ok(json) => {
                    if let Some((key, value)) = output_key_and_value(section_index, json) {
                        pdf_data.insert(key, value);
                    }
                }
                Err(e) => {
                    eprintln!("Warning: {} parse error: {}", section_name, e);
                }
            }
        }
    }

    Ok(pdf_data)
}
