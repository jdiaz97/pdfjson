use clap::{Arg, Command};
use std::process;
use std::env;
use crate::extractor::extract_contact_info;

pub async fn run_cli() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("pdfjson")
        .version("0.1.0")
        .author("José Díaz <jose@boquila.org>")
        .about("Extract information from PDF files using AI")
        .arg(
            Arg::new("pdf-file")
                .help("Path to the PDF file to process")
                .required(true)
                .index(1)
        )
        .arg(
            Arg::new("prompt")
                .help("Prompt for extraction")
                .required(true)
                .index(2)
        )
        .get_matches();

    // Get API key from environment variable
    let api_key = match env::var("GROQ_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Error: GROQ_API_KEY environment variable not set");
            process::exit(1);
        }
    };

    // Extract arguments
    let pdf_file = matches.get_one::<String>("pdf-file").unwrap();
    let prompt = matches.get_one::<String>("prompt").unwrap();

    // Validate PDF file exists
    if !std::path::Path::new(pdf_file).exists() {
        eprintln!("Error: PDF file '{}' does not exist", pdf_file);
        process::exit(1);
    }

    // Call the extraction function
    match extract_contact_info(api_key, pdf_file, prompt).await {
        Ok(result) => {
            println!("{}", result);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }

    Ok(())
}