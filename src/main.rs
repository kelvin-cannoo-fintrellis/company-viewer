use std::error::Error;

use company_pdf_viewer::processor::batch::process_pdfs_in_directory;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Usage example: process all PDFs in the 'pdf' directory
    // and save results to 'output_json' and 'output_markdown' directories
    process_pdfs_in_directory("pdf", "output_json", "output_markdown").await?;

    Ok(())
}
