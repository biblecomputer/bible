mod metadata;

use metadata::create_translation_metadata;

fn main() {
    let metadata = create_translation_metadata();
    println!("\nCreated metadata: {:#?}", metadata);
}