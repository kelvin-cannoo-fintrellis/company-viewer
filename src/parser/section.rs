use std::collections::HashSet;

use crate::ALL_SECTIONS;

/// Extracts the content of a given section from a PDF's text representation.
///
/// # Arguments
/// * `section_index` – index of the section in `ALL_SECTIONS` (0‑based).
/// * `pdf_text` – the full plain‑text of the PDF (passed by reference).
///
/// # Returns
/// The section's text **with** the section header itself.  
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
    // of them is encountered after the start line.
    let header_set: HashSet<&str> = ALL_SECTIONS.iter().copied().collect();

    // Find the index of the next header (if any) that appears after
    // `start_idx`.  If none is found we simply read to the end.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_section_invalid_index() {
        let text = "Some text";
        let result = extract_section(999, text);
        assert_eq!(result, "");
    }

    #[test]
    fn test_extract_section_not_found() {
        let text = "Some random text\nwithout any sections";
        let result = extract_section(0, text);
        assert_eq!(result, "");
    }

    #[test]
    fn test_extract_section_basic() {
        let text = "Company Details\nSome company info\nBusiness Details\nSome business info";
        let result = extract_section(0, text);
        assert!(result.contains("Company Details"));
        assert!(result.contains("Some company info"));
        assert!(!result.contains("Business Details"));
    }
}
