use anyhow::{Error, Result};
use reqwest::{Client, header};
use std::{collections::HashSet, path::Path};

pub mod models;

fn get_text_from_pdf(pdf_path: &str) -> String {
    let bytes = std::fs::read(pdf_path).unwrap();
    pdf_extract::extract_text_from_mem(&bytes).unwrap()
}

/// Extracts the content of a given section from a PDF’s text representation.
///
/// * `section_index` – index of the section in `ALL_SECTIONS` (0‑based).
/// * `pdf_text` – the full plain‑text of the PDF (passed by reference).
///
/// The function returns the section’s text **without** the section header itself.  
/// If the requested section is not found the returned string is empty.
pub fn extract_section(section_index: usize, pdf_text: &str) -> String {
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

// removes irrelevant details from a pdf
fn filter_irrelevant_details(pdf_text: String) -> String {
    pdf_text
}

// make an api call to ollama local model
fn text_to_structured_format() {
    let client: Client = Client::builder().build().unwrap();
}

fn main() {
    let pdf_text = get_text_from_pdf("pdf/16.pdf");
        // println!("{}\n\n", pdf_text);

    println!("{}", extract_section(5, &pdf_text));
}
