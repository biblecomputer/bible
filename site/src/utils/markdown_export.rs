use crate::core::bible_core::Bible;
use crate::storage::translations::get_current_translation;
use crate::translation_map::translation::Translation;
use crate::core::types::Language;
use web_sys::console;

/// Convert a Language enum from storage to core::types::Language
fn convert_language(lang: &crate::storage::translation_storage::Language) -> Language {
    match lang {
        crate::storage::translation_storage::Language::Dutch => Language::Dutch,
        crate::storage::translation_storage::Language::English => Language::English,
    }
}

/// Get translated book name based on current translation
fn get_translated_book_name(book_name: &str) -> String {
    console::log_1(&format!("🔄 Translating book name for markdown: {}", book_name).into());
    
    if let Some(current_translation) = get_current_translation() {
        console::log_1(&format!("📚 Current translation: {} ({:?})", current_translation.name, current_translation.languages).into());
        
        if let Some(first_language) = current_translation.languages.first() {
            console::log_1(&format!("🌍 Using language: {:?}", first_language).into());
            let translation = Translation::from_language(convert_language(first_language));
            
            // Convert book name to lowercase for mapping
            let key = book_name.to_lowercase();
            console::log_1(&format!("🔑 Looking up key: '{}'", key).into());
            
            if let Some(translated_name) = translation.get(&key) {
                console::log_1(&format!("✅ Found translation: {} → {}", book_name, translated_name).into());
                return translated_name;
            } else {
                console::log_1(&format!("❌ No translation found for key: '{}'", key).into());
            }
        }
    } else {
        console::log_1(&"❌ No current translation available".into());
    }
    
    console::log_1(&format!("⏭️ Using original name: {}", book_name).into());
    // Return original name if no translation found
    book_name.to_string()
}

pub fn export_bible_to_markdown<F>(bible: &Bible, progress_callback: Option<F>) -> Result<String, Box<dyn std::error::Error>>
where
    F: Fn(f32, String) + Clone + 'static,
{
    console::log_1(&"🚀 Starting Markdown export process".into());
    console::log_1(&format!("📖 Bible has {} books", bible.books.len()).into());
    
    // Report initial progress
    if let Some(ref callback) = progress_callback {
        callback(0.0, "Initializing Markdown export...".to_string());
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
    
    if let Some(ref callback) = progress_callback {
        callback(0.05, "Creating document structure...".to_string());
    }
    
    let mut markdown = String::new();
    
    if let Some(ref callback) = progress_callback {
        callback(0.1, "Processing books...".to_string());
    }
    
    let mut book_count = 0;
    let mut chapter_count = 0;
    let mut verse_count = 0;
    
    // Export all books, chapters, and verses
    console::log_1(&format!("📖 Processing {} books for Markdown export...", bible.books.len()).into());
    
    for book in &bible.books {
        book_count += 1;
        console::log_1(&format!("📖 Processing book {}/{}: {}", book_count, bible.books.len(), book.name).into());
        
        // Report progress for current book (10% to 90% of total progress)
        let book_progress = 0.1 + (book_count as f32 / bible.books.len() as f32) * 0.8;
        if let Some(ref callback) = progress_callback {
            callback(book_progress, format!("Processing {} ({}/{})", book.name, book_count, bible.books.len()));
        }
        
        // Book title - use translated name as # heading
        let translated_book_name = get_translated_book_name(&book.name);
        console::log_1(&format!("📖 Book: {} → {}", book.name, translated_book_name).into());
        
        markdown.push_str(&format!("# {}\n\n", translated_book_name.to_lowercase()));
        
        for chapter in &book.chapters {
            chapter_count += 1;
            
            // Chapter title as ## heading with book name
            markdown.push_str(&format!("## {} {}\n\n", translated_book_name.to_lowercase(), chapter.chapter));
            
            // Render verses with verse numbers at the start
            for verse in &chapter.verses {
                verse_count += 1;
                
                // Simple format: verse number followed by verse text, single line break
                markdown.push_str(&format!("{} {}\n", verse.verse, verse.text));
            }
            
            // Add double line break after chapter
            markdown.push_str("\n");
        }
    }
    
    console::log_1(&format!("📊 Markdown content complete! Processed {} books, {} chapters, {} verses", book_count, chapter_count, verse_count).into());
    
    if let Some(ref callback) = progress_callback {
        callback(1.0, "Markdown export complete!".to_string());
    }
    
    console::log_1(&format!("✅ Markdown export successful! Generated {} characters", markdown.len()).into());
    Ok(markdown)
}

/// Convenience function for exporting Markdown without progress tracking
#[allow(dead_code)]
pub fn export_bible_to_markdown_simple(bible: &Bible) -> Result<String, Box<dyn std::error::Error>> {
    export_bible_to_markdown(bible, None::<fn(f32, String)>)
}

pub fn trigger_markdown_download(markdown_content: String, filename: &str) {
    use web_sys::{window, Blob, Url, HtmlAnchorElement};
    use wasm_bindgen::JsCast;
    
    console::log_1(&"🔽 Starting Markdown download process".into());
    console::log_1(&format!("📄 Markdown size: {} characters", markdown_content.len()).into());
    console::log_1(&format!("📝 Filename: {}", filename).into());
    
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
    
    let document = match window.document() {
        Some(d) => {
            console::log_1(&"✅ Document object obtained".into());
            d
        }
        None => {
            console::log_1(&"❌ Failed to get document object".into());
            return;
        }
    };
    
    // Create blob from markdown content
    console::log_1(&"🔄 Creating Uint8Array from Markdown content...".into());
    let content_bytes = markdown_content.as_bytes();
    let uint8_array = js_sys::Uint8Array::new_with_length(content_bytes.len() as u32);
    uint8_array.copy_from(content_bytes);
    console::log_1(&format!("✅ Uint8Array created with length: {}", uint8_array.length()).into());
    
    console::log_1(&"📦 Creating JS Array for blob...".into());
    let array = js_sys::Array::new();
    array.push(&uint8_array);
    
    console::log_1(&"🔄 Creating Markdown blob...".into());
    let blob_options = web_sys::BlobPropertyBag::new();
    blob_options.set_type("text/markdown");
    
    let blob = match Blob::new_with_u8_array_sequence_and_options(&array, &blob_options) {
        Ok(b) => {
            console::log_1(&format!("✅ Blob created successfully, size: {}", b.size()).into());
            b
        }
        Err(e) => {
            console::log_1(&format!("❌ Failed to create blob: {:?}", e).into());
            return;
        }
    };
    
    // Create download URL
    console::log_1(&"🔗 Creating object URL for blob...".into());
    let url = match Url::create_object_url_with_blob(&blob) {
        Ok(u) => {
            console::log_1(&format!("✅ Object URL created: {}", u).into());
            u
        }
        Err(e) => {
            console::log_1(&format!("❌ Failed to create object URL: {:?}", e).into());
            return;
        }
    };
    
    // Create anchor element for download
    console::log_1(&"⚓ Creating anchor element...".into());
    let anchor = match document
        .create_element("a")
        .map_err(|e| e)
        .and_then(|elem| elem.dyn_into::<HtmlAnchorElement>().map_err(|_| wasm_bindgen::JsValue::from_str("Failed to cast to HtmlAnchorElement")))
    {
        Ok(a) => {
            console::log_1(&"✅ Anchor element created".into());
            a
        }
        Err(e) => {
            console::log_1(&format!("❌ Failed to create anchor element: {:?}", e).into());
            return;
        }
    };
    
    console::log_1(&"🔧 Setting anchor attributes...".into());
    anchor.set_href(&url);
    anchor.set_download(filename);
    console::log_1(&format!("✅ Anchor configured - href: {}, download: {}", anchor.href(), anchor.download()).into());
    
    // Add the anchor to the document and trigger download
    console::log_1(&"📎 Adding anchor to document body...".into());
    let body = match document.body() {
        Some(b) => {
            console::log_1(&"✅ Document body obtained".into());
            b
        }
        None => {
            console::log_1(&"❌ Failed to get document body".into());
            return;
        }
    };
    
    if let Err(e) = body.append_child(&anchor) {
        console::log_1(&format!("❌ Failed to append anchor to body: {:?}", e).into());
        return;
    }
    console::log_1(&"✅ Anchor added to document".into());
    
    console::log_1(&"🖱️ Triggering download click...".into());
    anchor.click();
    console::log_1(&"✅ Download click triggered".into());
    
    console::log_1(&"🧹 Cleaning up anchor element...".into());
    if let Err(e) = body.remove_child(&anchor) {
        console::log_1(&format!("⚠️ Failed to remove anchor from body: {:?}", e).into());
    } else {
        console::log_1(&"✅ Anchor removed from document".into());
    }
    
    // Clean up
    console::log_1(&"🗑️ Revoking object URL...".into());
    if let Err(e) = Url::revoke_object_url(&url) {
        console::log_1(&format!("⚠️ Failed to revoke object URL: {:?}", e).into());
    } else {
        console::log_1(&"✅ Object URL revoked".into());
    }
    
    console::log_1(&format!("🎉 Markdown download process completed! File: {}", filename).into());
}