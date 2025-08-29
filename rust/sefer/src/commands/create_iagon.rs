use crate::metadata::create_translation_metadata;
use peter::storage::Storage;
use peter::translation::{Translation, v1::{Books, TranslationV1}};
use std::process;
use url::Url;

pub fn create_iagon_translation(iagon_url: &str) {
    // Validate the URL
    let url = match Url::parse(iagon_url) {
        Ok(url) => url,
        Err(err) => {
            eprintln!("Error: Invalid URL '{}': {}", iagon_url, err);
            process::exit(1);
        }
    };

    println!("Creating new TranslationV1 with Iagon storage...");
    println!("Iagon URL: {}", url);

    // Create metadata for this translation
    println!("\nLet's create metadata for this translation:");
    let meta = create_translation_metadata();

    // Create Storage pointing to Iagon
    let books_storage: Storage<Books> = Storage::iagon(url);

    // Store metadata for later use before moving
    let meta_clone = meta.clone();
    
    // Create TranslationV1 with Iagon storage
    let translation_v1 = TranslationV1 {
        meta,
        books: books_storage,
    };

    let translation = Translation::V1(translation_v1);

    // Export as BTRL
    let btrl_content = match translation.export_as_btrl() {
        Ok(output) => output,
        Err(err) => {
            eprintln!("Error exporting translation: {}", err);
            process::exit(1);
        }
    };

    // Generate output filename based on translation name
    let safe_name = meta_clone.short_name
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
        .collect::<String>()
        .to_lowercase();
    
    let output_path = format!("{}_iagon.btrl", safe_name);

    // Write to file
    match std::fs::write(&output_path, btrl_content) {
        Ok(_) => {
            println!("✓ Successfully created Iagon translation: '{}'", output_path);
            println!("\nTranslation details:");
            println!("  - Type: TranslationV1 with Iagon storage");
            println!("  - Books URL: {}", iagon_url);
            println!("  - Metadata: {} ({})", meta_clone.full_name, meta_clone.short_name);
            println!("  - File: {}", output_path);
            println!("\nNote: The books data will be fetched from Iagon when this translation is used.");
        }
        Err(err) => {
            eprintln!("Error writing to file '{}': {}", output_path, err);
            process::exit(1);
        }
    }
}