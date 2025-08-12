use crate::core::bible_core::Bible;
use crate::storage::translations::get_current_translation;
use crate::translation_map::translation::Translation;
use crate::core::types::Language;
use printpdf::*;
use std::io::BufWriter;
use web_sys::console;

/// Convert a Language enum from storage to core::types::Language
fn convert_language(lang: &crate::storage::translation_storage::Language) -> Language {
    match lang {
        crate::storage::translation_storage::Language::Dutch => Language::Dutch,
        crate::storage::translation_storage::Language::English => Language::English,
    }
}

/// Add book header to the top of a page
fn add_book_header_to_page(layer: &PdfLayerReference, book_name: &str, margin_left: Mm, italic_font: &IndirectFontRef) {
    layer.use_text(
        book_name,
        9.0,
        margin_left,
        Mm(285.0), // Near top of page
        italic_font,
    );
}

/// Get translated book name based on current translation
fn get_translated_book_name(book_name: &str) -> String {
    console::log_1(&format!("🔄 Translating book name: {}", book_name).into());
    
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

pub fn export_bible_to_pdf<F>(bible: &Bible, progress_callback: Option<F>) -> Result<Vec<u8>, Box<dyn std::error::Error>>
where
    F: Fn(f32, String) + Clone + 'static,
{
    console::log_1(&"🚀 Starting PDF export process".into());
    console::log_1(&format!("📖 Bible has {} books", bible.books.len()).into());
    
    // Report initial progress
    if let Some(ref callback) = progress_callback {
        callback(0.0, "Initializing PDF export...".to_string());
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
    
    // Create PDF document
    console::log_1(&"📄 Creating PDF document...".into());
    if let Some(ref callback) = progress_callback {
        callback(0.05, "Creating PDF document...".to_string());
    }
    let (doc, page1, layer1) = PdfDocument::new("Bible Export", Mm(210.0), Mm(297.0), "Layer 1");
    
    // Load font
    console::log_1(&"🔤 Loading fonts...".into());
    if let Some(ref callback) = progress_callback {
        callback(0.1, "Loading fonts...".to_string());
    }
    let font = doc.add_builtin_font(BuiltinFont::TimesRoman)?;
    let bold_font = doc.add_builtin_font(BuiltinFont::TimesBold)?;
    let italic_font = doc.add_builtin_font(BuiltinFont::TimesItalic)?;
    console::log_1(&"✅ Fonts loaded successfully".into());
    
    let current_layer = doc.get_page(page1).get_layer(layer1);
    
    let mut current_y = Mm(270.0);
    let margin_left = Mm(20.0);
    let _margin_right = Mm(190.0);
    let line_height = Mm(5.0);
    let _page_height = Mm(297.0);
    let page_bottom_margin = Mm(20.0);
    
    // Add title page
    console::log_1(&"📝 Adding title page...".into());
    if let Some(ref callback) = progress_callback {
        callback(0.15, "Creating title page...".to_string());
    }
    current_layer.use_text(
        format!("{}", translation_info.name),
        24.0,
        margin_left,
        current_y,
        &bold_font,
    );
    
    current_y -= line_height * 2.0;
    
    current_layer.use_text(
        format!("Published: {}", translation_info.release_year),
        12.0,
        margin_left,
        current_y,
        &italic_font,
    );
    
    current_y -= line_height * 4.0;
    
    let mut current_page;
    let mut current_layer_ref = current_layer;
    let mut current_book_name; // Track current book for headers
    
    // Export all books, chapters, and verses
    console::log_1(&format!("📖 Processing {} books for PDF export...", bible.books.len()).into());
    
    let mut book_count = 0;
    let mut chapter_count = 0;
    let mut verse_count = 0;
    
    for book in &bible.books {
        book_count += 1;
        console::log_1(&format!("📖 Processing book {}/{}: {}", book_count, bible.books.len(), book.name).into());
        
        // Report progress for current book (20% to 90% of total progress)
        let book_progress = 0.2 + (book_count as f32 / bible.books.len() as f32) * 0.7;
        if let Some(ref callback) = progress_callback {
            callback(book_progress, format!("Processing {} ({}/{})", book.name, book_count, bible.books.len()));
        }
        
        // Book title - use translated name
        let translated_book_name = get_translated_book_name(&book.name);
        console::log_1(&format!("📖 Book: {} → {}", book.name, translated_book_name).into());
        current_book_name = translated_book_name.clone(); // Update current book for headers
        
        // Check if we need a new page for the book
        if current_y < page_bottom_margin + line_height * 10.0 {
            console::log_1(&"📄 Adding new page for book".into());
            let (new_page, new_layer) = doc.add_page(Mm(210.0), Mm(297.0), "Layer 1");
            current_page = new_page;
            current_layer_ref = doc.get_page(current_page).get_layer(new_layer);
            current_y = Mm(270.0);
        }
        
        // Add book header to current page
        add_book_header_to_page(&current_layer_ref, &translated_book_name, margin_left, &italic_font);
        
        current_layer_ref.use_text(
            &translated_book_name,
            18.0,
            margin_left,
            current_y,
            &bold_font,
        );
        
        current_y -= line_height * 2.0;
        
        for chapter in &book.chapters {
            chapter_count += 1;
            
            // Check if we need a new page for the chapter
            if current_y < page_bottom_margin + line_height * 20.0 {
                console::log_1(&"📄 Adding new page for chapter".into());
                let (new_page, new_layer) = doc.add_page(Mm(210.0), Mm(297.0), "Layer 1");
                current_page = new_page;
                current_layer_ref = doc.get_page(current_page).get_layer(new_layer);
                current_y = Mm(270.0);
                // Add book header to the new page
                add_book_header_to_page(&current_layer_ref, &current_book_name, margin_left, &italic_font);
            }
            
            // Chapter title - centered format like "-- 1 --"
            let chapter_title = format!("-- {} --", chapter.chapter);
            let title_width = 14.0 * chapter_title.len() as f32 * 0.6; // Rough character width estimation
            let centered_x = (margin_left + Mm(170.0)) / 2.0 - Mm(title_width / 2.0); // Center between margins
            
            current_layer_ref.use_text(
                &chapter_title,
                14.0,
                centered_x,
                current_y,
                &bold_font,
            );
            
            current_y -= line_height * 2.0;
            
            // Render verses with subscript verse numbers and continuous flow
            let max_chars_per_line = 85;
            let mut current_line = String::new();
            let mut first_verse_in_chapter = true;
            
            for verse in &chapter.verses {
                verse_count += 1;
                
                // Add space before verse number if not first verse in chapter
                if !first_verse_in_chapter {
                    current_line.push(' ');
                }
                
                // Add verse number (we'll handle subscript rendering separately)
                let verse_marker = format!("⁽{}⁾ ", verse.verse); // Use superscript-like characters
                current_line.push_str(&verse_marker);
                
                first_verse_in_chapter = false;
                
                // Add verse text
                current_line.push_str(&verse.text);
                current_line.push(' '); // Space between verses
            }
            
            // Split the continuous text into lines that fit the page width
            let words: Vec<&str> = current_line.split_whitespace().collect();
            let mut line_buffer = String::new();
            
            for word in words {
                let test_line = if line_buffer.is_empty() {
                    word.to_string()
                } else {
                    format!("{} {}", line_buffer, word)
                };
                
                // Check if this line would be too long
                if test_line.len() > max_chars_per_line && !line_buffer.is_empty() {
                    // Render the current line and start a new one
                    current_layer_ref.use_text(
                        &line_buffer,
                        11.0,
                        margin_left,
                        current_y,
                        &font,
                    );
                    current_y -= line_height;
                    
                    // Check if we need a new page
                    if current_y < page_bottom_margin + line_height {
                        console::log_1(&"📄 Adding new page for chapter text".into());
                        let (new_page, new_layer) = doc.add_page(Mm(210.0), Mm(297.0), "Layer 1");
                        current_page = new_page;
                        current_layer_ref = doc.get_page(current_page).get_layer(new_layer);
                        current_y = Mm(270.0);
                        
                        // Add book header to new page
                        add_book_header_to_page(&current_layer_ref, &current_book_name, margin_left, &italic_font);
                    }
                    
                    // Start new line with current word
                    line_buffer = word.to_string();
                } else {
                    // Add word to current line
                    line_buffer = test_line;
                }
            }
            
            // Render any remaining text
            if !line_buffer.is_empty() {
                current_layer_ref.use_text(
                    &line_buffer,
                    11.0,
                    margin_left,
                    current_y,
                    &font,
                );
                current_y -= line_height;
            }
            
            // Move to next line after chapter content
            current_y -= line_height;
            
            current_y -= line_height; // Extra spacing between chapters
        }
        
        current_y -= line_height * 2.0; // Extra spacing between books
    }
    
    console::log_1(&format!("📊 PDF content complete! Processed {} books, {} chapters, {} verses", book_count, chapter_count, verse_count).into());
    
    // Save PDF to bytes
    console::log_1(&"💾 Converting PDF to bytes...".into());
    if let Some(ref callback) = progress_callback {
        callback(0.9, "Finalizing PDF...".to_string());
    }
    let mut buf = Vec::new();
    {
        let mut writer = BufWriter::new(&mut buf);
        doc.save(&mut writer)?;
    }
    
    if let Some(ref callback) = progress_callback {
        callback(1.0, "PDF export complete!".to_string());
    }
    console::log_1(&format!("✅ PDF export successful! Generated {} bytes", buf.len()).into());
    Ok(buf)
}

/// Convenience function for exporting PDF without progress tracking
#[allow(dead_code)]
pub fn export_bible_to_pdf_simple(bible: &Bible) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    export_bible_to_pdf(bible, None::<fn(f32, String)>)
}

pub fn trigger_pdf_download(pdf_bytes: Vec<u8>, filename: &str) {
    use web_sys::{window, Blob, Url, HtmlAnchorElement};
    use wasm_bindgen::JsCast;
    
    console::log_1(&"🔽 Starting PDF download process".into());
    console::log_1(&format!("📄 PDF size: {} bytes", pdf_bytes.len()).into());
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
    
    // Create blob from PDF bytes
    console::log_1(&"🔄 Creating Uint8Array from PDF bytes...".into());
    let uint8_array = js_sys::Uint8Array::new_with_length(pdf_bytes.len() as u32);
    uint8_array.copy_from(&pdf_bytes);
    console::log_1(&format!("✅ Uint8Array created with length: {}", uint8_array.length()).into());
    
    console::log_1(&"📦 Creating JS Array for blob...".into());
    let array = js_sys::Array::new();
    array.push(&uint8_array);
    
    console::log_1(&"🔄 Creating PDF blob...".into());
    let blob = match Blob::new_with_u8_array_sequence(&array) {
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
    
    // Create anchor element for download to Downloads folder
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
    
    console::log_1(&format!("🎉 PDF download process completed! File: {}", filename).into());
}