use clap::{Arg, Command};
use clap::ArgAction;
use std::process;
use std::env;
use crate::extractor::retrieve;

pub async fn run_cli() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("pdfx")
        .version("0.1.0")
        .author("José Díaz <jose@boquila.org> & Adesoji Alu <adesoji.alu@gmail.com>")
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
        .arg(
            Arg::new("openai")
                .long("openai")
                .action(ArgAction::SetTrue)
                .help("Use OpenAI GPT 4.1 API for extraction"),
        )
        .get_matches();

    // Check if the --openai flag is set
    let use_openai = matches.get_flag("openai");

    // Get API key from environment variable
    let api_key = if use_openai {
        env::var("OPENAI_API_KEY").unwrap_or_else(|_| {
            eprintln!("Error: OPENAI_API_KEY environment variable not set");
            process::exit(1);
        })
    } else {
        env::var("GROQ_API_KEY").unwrap_or_else(|_| {
            eprintln!("Error: GROQ_API_KEY environment variable not set");
            process::exit(1);
        })
    };

    // Extract arguments
    let pdf_file = matches.get_one::<String>("pdf-file").unwrap();
    let prompt = matches.get_one::<String>("prompt").unwrap();

    // Validate PDF file exists
    if !std::path::Path::new(pdf_file).exists() {
        eprintln!("Error: PDF file '{}' does not exist", pdf_file);
        process::exit(1);
    }

    match retrieve(api_key, pdf_file, prompt).await {
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