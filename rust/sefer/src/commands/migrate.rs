use crate::file_ops;
use crate::metadata::create_translation_metadata;
use peter::translation::{
    Translation, 
    v0::translation_v0::TranslationV0, 
    v1::{Books, build_v1},
};
use std::process;

pub fn migrate_translation(input_path: &str) {
    // Read the JSON file
    let json_content = match file_ops::read_file(input_path) {
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
    let books = match Books::try_from(translation_v0) {
        Ok(books) => {
            println!("✓ Successfully converted v0 -> v1.books");
            books
        }
        Err(err) => {
            eprintln!("Error converting to v1 books: {}", err);
            process::exit(1);
        }
    };

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

    // Generate output filename
    let output_path = file_ops::generate_output_filename(input_path, "btrl");

    // Write to file
    match file_ops::write_file(&output_path, &btrl_content) {
        Ok(_) => {
            println!("✓ Successfully migrated '{}' to '{}'", input_path, output_path);
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