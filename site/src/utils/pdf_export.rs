use crate::core::bible_core::Bible;
use crate::storage::translations::get_current_translation;
use printpdf::*;
use std::io::BufWriter;
use web_sys::console;

pub fn export_bible_to_pdf(bible: &Bible) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    console::log_1(&"🚀 Starting PDF export process".into());
    console::log_1(&format!("📖 Bible has {} books", bible.books.len()).into());
    
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
    let (doc, page1, layer1) = PdfDocument::new("Bible Export", Mm(210.0), Mm(297.0), "Layer 1");
    
    // Load font
    console::log_1(&"🔤 Loading fonts...".into());
    let font = doc.add_builtin_font(BuiltinFont::TimesRoman)?;
    let bold_font = doc.add_builtin_font(BuiltinFont::TimesBold)?;
    let italic_font = doc.add_builtin_font(BuiltinFont::TimesItalic)?;
    console::log_1(&"✅ Fonts loaded successfully".into());
    
    let current_layer = doc.get_page(page1).get_layer(layer1);
    
    let mut current_y = Mm(270.0);
    let margin_left = Mm(20.0);
    let margin_right = Mm(190.0);
    let line_height = Mm(5.0);
    let page_height = Mm(297.0);
    let page_bottom_margin = Mm(20.0);
    
    // Add title page
    console::log_1(&"📝 Adding title page...".into());
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
    
    let mut current_page = page1;
    let mut current_layer_ref = current_layer;
    
    // Export all books, chapters, and verses
    console::log_1(&format!("📖 Processing {} books for PDF export...", bible.books.len()).into());
    
    let mut book_count = 0;
    let mut chapter_count = 0;
    let mut verse_count = 0;
    
    for book in &bible.books {
        book_count += 1;
        console::log_1(&format!("📖 Processing book {}/{}: {}", book_count, bible.books.len(), book.name).into());
        
        // Check if we need a new page for the book
        if current_y < page_bottom_margin + line_height * 10.0 {
            console::log_1(&"📄 Adding new page for book".into());
            let (new_page, new_layer) = doc.add_page(Mm(210.0), Mm(297.0), "Layer 1");
            current_page = new_page;
            current_layer_ref = doc.get_page(current_page).get_layer(new_layer);
            current_y = Mm(270.0);
        }
        
        // Book title
        current_layer_ref.use_text(
            &book.name,
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
            }
            
            // Chapter title
            current_layer_ref.use_text(
                &format!("Chapter {}", chapter.chapter),
                14.0,
                margin_left,
                current_y,
                &bold_font,
            );
            
            current_y -= line_height * 1.5;
            
            for verse in &chapter.verses {
                verse_count += 1;
                
                // Check if we need a new page for the verse
                if current_y < page_bottom_margin + line_height * 3.0 {
                    console::log_1(&"📄 Adding new page for verse".into());
                    let (new_page, new_layer) = doc.add_page(Mm(210.0), Mm(297.0), "Layer 1");
                    current_page = new_page;
                    current_layer_ref = doc.get_page(current_page).get_layer(new_layer);
                    current_y = Mm(270.0);
                }
                
                // Verse number and text
                let verse_text = format!("{} {}", verse.verse, verse.text);
                
                // Simple text wrapping (split long verses into multiple lines)
                let max_chars_per_line = 80;
                let words: Vec<&str> = verse_text.split_whitespace().collect();
                let mut current_line = String::new();
                
                for word in words {
                    if current_line.len() + word.len() + 1 > max_chars_per_line {
                        if !current_line.is_empty() {
                            current_layer_ref.use_text(
                                &current_line,
                                11.0,
                                margin_left,
                                current_y,
                                &font,
                            );
                            current_y -= line_height;
                            current_line.clear();
                            
                            // Check if we need a new page
                            if current_y < page_bottom_margin + line_height {
                                console::log_1(&"📄 Adding new page for text wrap".into());
                                let (new_page, new_layer) = doc.add_page(Mm(210.0), Mm(297.0), "Layer 1");
                                current_page = new_page;
                                current_layer_ref = doc.get_page(current_page).get_layer(new_layer);
                                current_y = Mm(270.0);
                            }
                        }
                    }
                    
                    if !current_line.is_empty() {
                        current_line.push(' ');
                    }
                    current_line.push_str(word);
                }
                
                // Print remaining text
                if !current_line.is_empty() {
                    current_layer_ref.use_text(
                        &current_line,
                        11.0,
                        margin_left,
                        current_y,
                        &font,
                    );
                    current_y -= line_height;
                }
                
                current_y -= line_height * 0.3; // Small spacing between verses
            }
            
            current_y -= line_height; // Extra spacing between chapters
        }
        
        current_y -= line_height * 2.0; // Extra spacing between books
    }
    
    console::log_1(&format!("📊 PDF content complete! Processed {} books, {} chapters, {} verses", book_count, chapter_count, verse_count).into());
    
    // Save PDF to bytes
    console::log_1(&"💾 Converting PDF to bytes...".into());
    let mut buf = Vec::new();
    {
        let mut writer = BufWriter::new(&mut buf);
        doc.save(&mut writer)?;
    }
    
    console::log_1(&format!("✅ PDF export successful! Generated {} bytes", buf.len()).into());
    Ok(buf)
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