use std::path::Path;
use anyhow::{Error, Result};

pub mod models;

fn main() {
let bytes = std::fs::read("pdf/51.pdf").unwrap();
let out = pdf_extract::extract_text_from_mem(&bytes).unwrap();
println!("{}",out);
}