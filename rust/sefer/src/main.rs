mod metadata;

use clap::{Arg, Command};
use metadata::create_translation_metadata;
use peter::translation::{
    Translation, 
    v0::translation_v0::TranslationV0, 
    v1::{Books, build_v1},
};
use std::fs;
use std::process;

fn main() {
    let matches = Command::new("sefer")
        .version("0.1.0")
        .author("Bible Computer")
        .about(
            "A tool for performing tasks related to the bible computer. Sefer is Hebrew for book.",
        )
        .subcommand(
            Command::new("migrate")
                .about("Migrate a TranslationV0 JSON file to btrs or upgrade old btrs files.")
                .arg(
                    Arg::new("input")
                        .help("Path to the JSON file to migrate")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(Command::new("metadata").about("Create translation metadata interactively"))
        .get_matches();

    match matches.subcommand() {
        Some(("migrate", sub_matches)) => {
            let input_file = sub_matches.get_one::<String>("input").unwrap();
            migrate_translation(input_file);
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

fn migrate_translation(input_path: &str) {
    // Read the JSON file
    let json_content = match fs::read_to_string(input_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", input_path, err);
            process::exit(1);
        }
    };

    // Parse the JSON as TranslationV0
    let translation_v0 = match TranslationV0::try_from(json_content.as_str()) {
        Ok(v0) => {
            println!("✓ Successfully parsed .json file as TranslationV0");
            v0
        }
        Err(err) => {
            eprintln!("Error parsing JSON: {}", err);
            process::exit(1);
        }
    };

    // v1 parsing
    let books = Books::try_from(translation_v0).unwrap();
    println!("Successfully parsed. v0 -> v1.books");

    println!("\nNow let's add metadata for this translation:");
    let meta = create_translation_metadata();

    let v1 = build_v1(books, meta);
    let translation = Translation::V1(v1);

    let btrl_content = match translation.export_as_btrl() {
        Ok(output) => output,
        Err(err) => {
            eprintln!("Error exporting translation: {}", err);
            process::exit(1);
        }
    };

    // Generate output filename by replacing extension with .btrl
    let output_path = if input_path.contains('.') {
        let base = input_path.rsplit_once('.').unwrap().0;
        format!("{}.btrl", base)
    } else {
        format!("{}.btrl", input_path)
    };

    // Write to file
    match fs::write(&output_path, btrl_content) {
        Ok(_) => {
            println!(
                "✓ Successfully migrated '{}' to '{}'",
                input_path, output_path
            );
            println!("\nMigration complete:");
            println!("  - Converted from .json to TranslationV0");
            println!("  - Added metadata");
            println!("  - Upgraded to TranslationV1");
            println!("  - Saved as {}", output_path);
        }
        Err(err) => {
            eprintln!("Error writing to file '{}': {}", output_path, err);
            process::exit(1);
        }
    }
}
