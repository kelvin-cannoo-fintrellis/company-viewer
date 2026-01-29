/// Extracts text content from a PDF file
///
/// # Arguments
/// * `pdf_path` - Path to the PDF file
///
/// # Returns
/// * Extracted text content as a String
///
/// # Panics
/// * If the file cannot be read or PDF extraction fails
pub fn get_text_from_pdf(pdf_path: &str) -> String {
    let bytes = std::fs::read(pdf_path).unwrap();
    pdf_extract::extract_text_from_mem(&bytes).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Requires actual PDF file
    fn test_get_text_from_pdf() {
        // This test requires a real PDF file to work
        // Run with: cargo test -- --ignored
        let text = get_text_from_pdf("test.pdf");
        assert!(!text.is_empty());
    }
}
