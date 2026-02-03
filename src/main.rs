use std::error::Error;

use company_pdf_viewer::processor::batch::process_pdfs_in_directory;

use std::fs::File;
use tracing_subscriber::EnvFilter;

pub fn init_logging(log_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(log_path)?;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .with_writer(file)
        .with_ansi(false)
        .init();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();

    init_logging("processing.log")?;

    // Usage example: process all PDFs in the 'pdf' directory
    // and save results to 'output_json'
    process_pdfs_in_directory("pdf", "output_json").await?;

    Ok(())
}
