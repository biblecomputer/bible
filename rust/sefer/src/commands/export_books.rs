use crate::file_ops;
use peter::translation::v0::translation_v0::TranslationV0;
use peter::translation::v1::Books;
use std::process;

pub fn export_books_json(input_path: &str) {
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

    // Convert to Books
    let books = match Books::try_from(translation_v0) {
        Ok(books) => {
            println!("✓ Successfully converted v0 -> Books");
            books
        }
        Err(err) => {
            eprintln!("Error converting to Books: {}", err);
            process::exit(1);
        }
    };

    // Serialize Books to JSON
    let books_json = match serde_json::to_string_pretty(&books) {
        Ok(json) => json,
        Err(err) => {
            eprintln!("Error serializing Books to JSON: {}", err);
            process::exit(1);
        }
    };

    // Generate output filename
    let output_path = file_ops::generate_output_filename(input_path, "books.json");

    // Write to file
    match file_ops::write_file(&output_path, &books_json) {
        Ok(_) => {
            println!("✓ Successfully exported books to '{}'", output_path);
            println!("\nExport complete:");
            println!("  - Converted from .json to TranslationV0");
            println!("  - Extracted and converted books data");
            println!("  - Saved as {}", output_path);
            println!("\nThis books.json file can now be uploaded to Iagon and referenced");
            println!("in a TranslationV1 using the 'create-iagon' command.");
        }
        Err(err) => {
            eprintln!("Error writing to file '{}': {}", output_path, err);
            process::exit(1);
        }
    }
}