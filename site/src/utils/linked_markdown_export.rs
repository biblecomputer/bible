use crate::core::bible_core::Bible;
use crate::storage::translations::get_current_translation;
use crate::translation_map::translation::Translation;
use crate::core::types::Language;
use web_sys::console;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

/// Convert a Language enum from storage to core::types::Language
fn convert_language(lang: &crate::storage::translation_storage::Language) -> Language {
    match lang {
        crate::storage::translation_storage::Language::Dutch => Language::Dutch,
        crate::storage::translation_storage::Language::English => Language::English,
    }
}

/// Get translated book name based on current translation
fn get_translated_book_name(book_name: &str) -> String {
    console::log_1(&format!("🔄 Translating book name for linked markdown: {}", book_name).into());
    
    if let Some(current_translation) = get_current_translation() {
        if let Some(first_language) = current_translation.languages.first() {
            let translation = Translation::from_language(convert_language(first_language));
            let key = book_name.to_lowercase();
            
            if let Some(translated_name) = translation.get(&key) {
                console::log_1(&format!("✅ Found translation: {} → {}", book_name, translated_name).into());
                return translated_name;
            }
        }
    }
    
    // Return original name if no translation found
    book_name.to_string()
}

/// Structure to hold all the files for the linked markdown export
#[derive(Debug)]
pub struct LinkedMarkdownExport {
    pub files: HashMap<String, String>,
    pub folder_name: String,
}

pub fn export_bible_to_linked_markdown<F>(bible: &Bible, progress_callback: Option<F>) -> Result<LinkedMarkdownExport, Box<dyn std::error::Error>>
where
    F: Fn(f32, String) + Clone + 'static,
{
    console::log_1(&"🚀 Starting Linked Markdown export process".into());
    console::log_1(&format!("📖 Bible has {} books", bible.books.len()).into());
    
    // Report initial progress
    if let Some(ref callback) = progress_callback {
        callback(0.0, "Initializing Linked Markdown export...".to_string());
    }
    
    let translation_info = get_current_translation().unwrap_or_else(|| {
        console::log_1(&"⚠️ No current translation found, using default".into());
        crate::storage::translation_storage::BibleTranslation {
            name: "Unknown Bible".to_string(),
            short_name: "unknown".to_string(), 
            description: "".to_string(),
            wikipedia: "".to_string(),
            release_year: 2024,
            languages: vec![],
            iagon: "".to_string(),
        }
    });
    
    console::log_1(&format!("📚 Using translation: {}", translation_info.name).into());
    
    let folder_name = translation_info.name.to_lowercase().replace(" ", "_");
    let mut files = HashMap::new();
    
    if let Some(ref callback) = progress_callback {
        callback(0.05, "Creating index file...".to_string());
    }
    
    // Create main index file (e.g., "statenvertaling.md")
    let main_filename = format!("{}.md", folder_name);
    let mut main_content = String::new();
    
    // Add metadata
    main_content.push_str(&format!("# {}\n\n", translation_info.name));
    main_content.push_str(&format!("**Published:** {}\n", translation_info.release_year));
    if !translation_info.description.is_empty() {
        main_content.push_str(&format!("**Description:** {}\n", translation_info.description));
    }
    if !translation_info.wikipedia.is_empty() {
        main_content.push_str(&format!("**Wikipedia:** {}\n", translation_info.wikipedia));
    }
    main_content.push_str("\n## Book Index\n\n");
    
    // Add book index with links
    for book in &bible.books {
        let translated_book_name = get_translated_book_name(&book.name);
        let book_filename = translated_book_name.to_lowercase().replace(" ", "_");
        main_content.push_str(&format!("- [[{}]]\n", book_filename));
    }
    
    files.insert(main_filename, main_content);
    
    if let Some(ref callback) = progress_callback {
        callback(0.1, "Processing books and chapters...".to_string());
    }
    
    let mut book_count = 0;
    let mut chapter_count = 0;
    let mut verse_count = 0;
    
    // Process each book
    for book in &bible.books {
        book_count += 1;
        console::log_1(&format!("📖 Processing book {}/{}: {}", book_count, bible.books.len(), book.name).into());
        
        // Report progress for current book (10% to 90% of total progress)
        let book_progress = 0.1 + (book_count as f32 / bible.books.len() as f32) * 0.8;
        if let Some(ref callback) = progress_callback {
            callback(book_progress, format!("Processing {} ({}/{})", book.name, book_count, bible.books.len()));
        }
        
        let translated_book_name = get_translated_book_name(&book.name);
        let book_filename = format!("{}.md", translated_book_name.to_lowercase().replace(" ", "_"));
        
        // Create book index file
        let mut book_content = String::new();
        book_content.push_str(&format!("# {}\n\n", translated_book_name));
        book_content.push_str("## Chapter Index\n\n");
        
        // Add chapter links
        for chapter in &book.chapters {
            let chapter_filename = format!("{} {}", translated_book_name.to_lowercase().replace(" ", "_"), chapter.chapter);
            book_content.push_str(&format!("- [[{}]]\n", chapter_filename));
        }
        
        files.insert(book_filename, book_content);
        
        // Process each chapter
        for (chapter_index, chapter) in book.chapters.iter().enumerate() {
            chapter_count += 1;
            
            let chapter_filename = format!("{} {}.md", translated_book_name.to_lowercase().replace(" ", "_"), chapter.chapter);
            let mut chapter_content = String::new();
            
            // Chapter title
            chapter_content.push_str(&format!("# {} {}\n\n", translated_book_name, chapter.chapter));
            
            // Navigation links
            // Handle previous link
            if chapter_index > 0 {
                // Previous chapter in same book
                let prev_chapter_filename = format!("{} {}", translated_book_name.to_lowercase().replace(" ", "_"), book.chapters[chapter_index - 1].chapter);
                chapter_content.push_str(&format!("previous: [[{}]]\n", prev_chapter_filename));
            } else {
                // First chapter of this book, try to link to last chapter of previous book
                if book_count > 1 { // book_count is 1-based, so > 1 means there's a previous book
                    let prev_book = &bible.books[book_count - 2]; // book_count - 2 gets previous book (0-indexed)
                    let prev_book_translated = get_translated_book_name(&prev_book.name);
                    if !prev_book.chapters.is_empty() {
                        let last_chapter = prev_book.chapters.last().unwrap();
                        let prev_chapter_filename = format!("{} {}", prev_book_translated.to_lowercase().replace(" ", "_"), last_chapter.chapter);
                        chapter_content.push_str(&format!("previous: [[{}]]\n", prev_chapter_filename));
                    }
                }
            }
            
            // Handle next link
            if chapter_index < book.chapters.len() - 1 {
                // Next chapter in same book
                let next_chapter_filename = format!("{} {}", translated_book_name.to_lowercase().replace(" ", "_"), book.chapters[chapter_index + 1].chapter);
                chapter_content.push_str(&format!("next: [[{}]]\n", next_chapter_filename));
            } else {
                // Last chapter of this book, try to link to first chapter of next book
                if book_count < bible.books.len() {
                    let next_book = &bible.books[book_count]; // book_count is 1-based, so this gets next book
                    let next_book_translated = get_translated_book_name(&next_book.name);
                    if !next_book.chapters.is_empty() {
                        let next_chapter_filename = format!("{} {}", next_book_translated.to_lowercase().replace(" ", "_"), next_book.chapters[0].chapter);
                        chapter_content.push_str(&format!("next: [[{}]]\n", next_chapter_filename));
                    }
                }
            }
            
            chapter_content.push_str("\n");
            
            // Add verses
            for verse in &chapter.verses {
                verse_count += 1;
                chapter_content.push_str(&format!("{} {}\n", verse.verse, verse.text));
            }
            
            files.insert(chapter_filename, chapter_content);
        }
    }
    
    console::log_1(&format!("📊 Linked Markdown export complete! Processed {} books, {} chapters, {} verses", book_count, chapter_count, verse_count).into());
    console::log_1(&format!("📁 Created {} files", files.len()).into());
    
    if let Some(ref callback) = progress_callback {
        callback(1.0, "Linked Markdown export complete!".to_string());
    }
    
    Ok(LinkedMarkdownExport {
        files,
        folder_name,
    })
}

pub fn trigger_linked_markdown_download(export: LinkedMarkdownExport, filename: &str) {
    use web_sys::window;
    
    console::log_1(&"🔽 Starting Linked Markdown ZIP export process".into());
    console::log_1(&format!("📁 Folder name: {}", export.folder_name).into());
    console::log_1(&format!("📄 Number of files: {}", export.files.len()).into());
    console::log_1(&format!("📦 ZIP filename: {}", filename).into());
    
    let window = match window() {
        Some(w) => {
            console::log_1(&"✅ Window object obtained".into());
            w
        }
        None => {
            console::log_1(&"❌ Failed to get window object".into());
            return;
        }
    };
    
    
    // Sort files for consistent ordering
    let mut sorted_files: Vec<_> = export.files.iter().collect();
    sorted_files.sort_by_key(|(filename, _)| *filename);
    
    // Create a JavaScript object with all file data
    console::log_1(&"🗜️ Preparing files for ZIP creation...".into());
    console::log_1(&format!("🔍 Total files to process: {}", sorted_files.len()).into());
    
    let files_obj = js_sys::Object::new();
    
    for (i, (filename, content)) in sorted_files.iter().enumerate() {
        let file_path = format!("{}/{}", export.folder_name, filename);
        console::log_1(&format!("📄 [{}/{}] Processing file: {}", i + 1, sorted_files.len(), file_path).into());
        let preview = if content.len() > 50 {
            match content.char_indices().nth(50) {
                Some((idx, _)) => &content[..idx],
                None => content
            }
        } else {
            content
        };
        console::log_1(&format!("📄 Content preview: {}...", preview).into());
        
        // Convert content to Uint8Array
        let content_bytes = content.as_bytes();
        let uint8_array = js_sys::Uint8Array::new_with_length(content_bytes.len() as u32);
        uint8_array.copy_from(content_bytes);
        
        match js_sys::Reflect::set(&files_obj, &JsValue::from_str(&file_path), &uint8_array.into()) {
            Ok(_) => {
                console::log_1(&format!("✅ Added file: {} ({} bytes)", file_path, content.len()).into());
            }
            Err(e) => {
                console::log_1(&format!("❌ Failed to add file {}: {:?}", file_path, e).into());
            }
        }
    }
    
    console::log_1(&format!("📦 Prepared {} files for ZIP creation", sorted_files.len()).into());
    
    console::log_1(&"📦 Creating ZIP using JavaScript...".into());
    
    // Call JavaScript function to create ZIP and download
    match create_and_download_zip(files_obj, filename) {
        Ok(_) => {
            console::log_1(&format!("🎉 ZIP creation successful! File: {}", filename).into());
            console::log_1(&"📝 Extract the ZIP file to get individual markdown files for Obsidian.".into());
        }
        Err(e) => {
            console::log_1(&format!("❌ Failed to create ZIP: {:?}", e).into());
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch)]
    fn create_and_download_zip(files: js_sys::Object, filename: &str) -> Result<(), JsValue>;
}

/// Convenience function for exporting linked markdown without progress tracking
pub fn export_bible_to_linked_markdown_simple(bible: &Bible) -> Result<LinkedMarkdownExport, Box<dyn std::error::Error>> {
    export_bible_to_linked_markdown(bible, None::<fn(f32, String)>)
}