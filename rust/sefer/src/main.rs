mod commands;
mod file_ops;
mod interactive;
mod metadata;

use clap::{Arg, Command};
use commands::{create_iagon_translation, export_books_json, migrate_translation};
use metadata::create_translation_metadata;
use std::process;

fn main() {
    let matches = Command::new("sefer")
        .version("0.1.0")
        .author("Bible Computer")
        .about("A tool for performing tasks related to the bible computer. Sefer is Hebrew for book.")
        .subcommand(
            Command::new("migrate")
                .about("Migrate a TranslationV0 JSON file to btrl format.")
                .arg(
                    Arg::new("input")
                        .help("Path to the JSON file to migrate")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            Command::new("export-books")
                .about("Export books from a TranslationV0 JSON file to books.json format for Iagon upload.")
                .arg(
                    Arg::new("input")
                        .help("Path to the JSON file to export books from")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            Command::new("create-iagon")
                .about("Create a new TranslationV1 with Iagon storage pointing to a remote books.json file.")
                .arg(
                    Arg::new("url")
                        .help("URL of the books.json file on Iagon (optional - will prompt if not provided)")
                        .required(false)
                        .index(1),
                ),
        )
        .subcommand(
            Command::new("metadata")
                .about("Create translation metadata interactively")
        )
        .get_matches();

    match matches.subcommand() {
        Some(("migrate", sub_matches)) => {
            let input_file = sub_matches.get_one::<String>("input").unwrap();
            migrate_translation(input_file);
        }
        Some(("export-books", sub_matches)) => {
            let input_file = sub_matches.get_one::<String>("input").unwrap();
            export_books_json(input_file);
        }
        Some(("create-iagon", sub_matches)) => {
            let iagon_url = sub_matches.get_one::<String>("url").map(|s| s.as_str());
            create_iagon_translation(iagon_url);
        }
        Some(("metadata", _)) => {
            let metadata = create_translation_metadata();
            println!("\nCreated metadata: {:#?}", metadata);
        }
        _ => {
            eprintln!("No command specified. Use --help for usage information.");
            process::exit(1);
        }
    }
}