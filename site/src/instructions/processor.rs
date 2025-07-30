use super::types::Instruction;
use crate::core::{get_bible, Chapter, VerseRange};
use crate::storage::translations::get_current_translation;
use crate::translation_map::translation::Translation;
use crate::core::types::Language;
use leptos_router::NavigateOptions;
use wasm_bindgen_futures::{spawn_local, JsFuture};

pub struct InstructionContext {
    pub current_chapter: Chapter,
    pub search_params: String,
    pub pathname: String,
}

impl InstructionContext {
    pub fn new(current_chapter: Chapter, search_params: String, pathname: String) -> Self {
        Self {
            current_chapter,
            search_params,
            pathname,
        }
    }
    
    pub fn get_current_verse(&self) -> u32 {
        if self.search_params.contains("verses=") {
            let verse_param = self.search_params
                .split("verses=")
                .nth(1)
                .unwrap_or("1")
                .split('&')
                .next()
                .unwrap_or("1");
            verse_param
                .split(',')
                .next()
                .unwrap_or("1")
                .split('-')
                .next()
                .unwrap_or("1")
                .parse()
                .unwrap_or(1)
        } else {
            0 // No verse selected = chapter heading is selected
        }
    }
    
    pub fn get_verse_ranges(&self) -> Vec<VerseRange> {
        if self.search_params.contains("verses=") {
            self.search_params
                .split('&')
                .find_map(|param| {
                    let mut parts = param.split('=');
                    if parts.next()? == "verses" {
                        parts.next()
                    } else {
                        None
                    }
                })
                .map(|verses_param| {
                    verses_param
                        .split(',')
                        .filter_map(|range_str| VerseRange::from_string(range_str))
                        .collect()
                })
                .unwrap_or_else(Vec::new)
        } else {
            Vec::new()
        }
    }
}

pub struct InstructionProcessor<F>
where
    F: Fn(&str, NavigateOptions),
{
    pub navigate: F,
}

impl<F> InstructionProcessor<F>
where
    F: Fn(&str, NavigateOptions),
{
    pub fn new(navigate: F) -> Self {
        Self { navigate }
    }
    
    pub fn process(&self, instruction: Instruction, context: &InstructionContext) -> bool {
        match instruction {
            Instruction::NextVerse => self.handle_next_verse(context),
            Instruction::PreviousVerse => self.handle_previous_verse(context),
            Instruction::NextChapter => self.handle_next_chapter(context),
            Instruction::PreviousChapter => self.handle_previous_chapter(context),
            Instruction::NextBook => self.handle_next_book(context),
            Instruction::PreviousBook => self.handle_previous_book(context),
            Instruction::BeginningOfChapter => self.handle_beginning_of_chapter(context),
            Instruction::EndOfChapter => self.handle_end_of_chapter(context),
            Instruction::GoToVerse(verse_num) => self.handle_go_to_verse(context, verse_num),
            Instruction::CopyRawVerse => self.handle_copy_raw_verse(context),
            Instruction::CopyVerseWithReference => self.handle_copy_verse_with_reference(context),
            Instruction::NoOp => false,
            _ => {
                // Other instructions need to be handled by the UI components
                // Return false to indicate this processor didn't handle it
                false
            }
        }
    }
    
    fn handle_next_verse(&self, context: &InstructionContext) -> bool {
        let current_verse = context.get_current_verse();
        
        if current_verse == 0 {
            // Currently on chapter heading, navigate to first verse
            let verse_range = VerseRange { start: 1, end: 1 };
            let new_path = context.current_chapter.to_path_with_verses(&[verse_range]);
            (self.navigate)(&new_path, NavigateOptions { scroll: false, ..Default::default() });
        } else if let Some(next_verse) = context.current_chapter.get_next_verse(current_verse) {
            // Navigate to next verse in current chapter
            let verse_range = VerseRange { start: next_verse, end: next_verse };
            let new_path = context.current_chapter.to_path_with_verses(&[verse_range]);
            (self.navigate)(&new_path, NavigateOptions { scroll: false, ..Default::default() });
        } else if let Some(next_chapter) = get_bible().get_next_chapter(&context.current_chapter) {
            // Navigate to chapter heading of next chapter
            (self.navigate)(&next_chapter.to_path(), NavigateOptions { scroll: false, ..Default::default() });
        }
        true
    }
    
    fn handle_previous_verse(&self, context: &InstructionContext) -> bool {
        let current_verse = context.get_current_verse();
        
        if current_verse == 0 {
            // Currently on chapter heading, navigate to previous chapter heading
            if let Some(prev_chapter) = get_bible().get_previous_chapter(&context.current_chapter) {
                (self.navigate)(&prev_chapter.to_path(), NavigateOptions { scroll: false, ..Default::default() });
            }
        } else if current_verse == 1 {
            // Currently on first verse, navigate to chapter heading
            let new_path = context.current_chapter.to_path();
            (self.navigate)(&new_path, NavigateOptions { scroll: false, ..Default::default() });
        } else if let Some(prev_verse) = context.current_chapter.get_previous_verse(current_verse) {
            // Navigate to previous verse in current chapter
            let verse_range = VerseRange { start: prev_verse, end: prev_verse };
            let new_path = context.current_chapter.to_path_with_verses(&[verse_range]);
            (self.navigate)(&new_path, NavigateOptions { scroll: false, ..Default::default() });
        }
        true
    }
    
    fn handle_next_chapter(&self, context: &InstructionContext) -> bool {
        if let Some(next_chapter) = get_bible().get_next_chapter(&context.current_chapter) {
            (self.navigate)(&next_chapter.to_path(), NavigateOptions { scroll: false, ..Default::default() });
            true
        } else {
            false
        }
    }
    
    fn handle_previous_chapter(&self, context: &InstructionContext) -> bool {
        if let Some(prev_chapter) = get_bible().get_previous_chapter(&context.current_chapter) {
            (self.navigate)(&prev_chapter.to_path(), NavigateOptions { scroll: false, ..Default::default() });
            true
        } else {
            false
        }
    }
    
    fn handle_next_book(&self, context: &InstructionContext) -> bool {
        if let Some(next_book_chapter) = get_bible().get_next_book(&context.current_chapter) {
            (self.navigate)(&next_book_chapter.to_path(), NavigateOptions { scroll: false, ..Default::default() });
            true
        } else {
            false
        }
    }
    
    fn handle_previous_book(&self, context: &InstructionContext) -> bool {
        if let Some(prev_book_chapter) = get_bible().get_previous_book(&context.current_chapter) {
            (self.navigate)(&prev_book_chapter.to_path(), NavigateOptions { scroll: false, ..Default::default() });
            true
        } else {
            false
        }
    }
    
    fn handle_beginning_of_chapter(&self, context: &InstructionContext) -> bool {
        let new_path = context.current_chapter.to_path();
        (self.navigate)(&new_path, NavigateOptions { scroll: false, ..Default::default() });
        true
    }
    
    fn handle_end_of_chapter(&self, context: &InstructionContext) -> bool {
        let last_verse = context.current_chapter.verses.len() as u32;
        if last_verse > 0 {
            let verse_range = VerseRange { start: last_verse, end: last_verse };
            let new_path = context.current_chapter.to_path_with_verses(&[verse_range]);
            (self.navigate)(&new_path, NavigateOptions { scroll: false, ..Default::default() });
            true
        } else {
            false
        }
    }
    
    fn handle_go_to_verse(&self, context: &InstructionContext, verse_num: u32) -> bool {
        if verse_num > 0 && verse_num <= context.current_chapter.verses.len() as u32 {
            let verse_range = VerseRange { start: verse_num, end: verse_num };
            let new_path = context.current_chapter.to_path_with_verses(&[verse_range]);
            (self.navigate)(&new_path, NavigateOptions { scroll: false, ..Default::default() });
            true
        } else {
            false
        }
    }
    
    fn handle_copy_raw_verse(&self, context: &InstructionContext) -> bool {
        let verse_ranges = context.get_verse_ranges();
        let copy_text = if !verse_ranges.is_empty() {
            let mut verses_to_copy = Vec::new();
            
            // Collect verses in selected ranges
            for verse in &context.current_chapter.verses {
                for range in &verse_ranges {
                    if range.contains(verse.verse) {
                        verses_to_copy.push(verse);
                        break;
                    }
                }
            }
            
            if !verses_to_copy.is_empty() {
                verses_to_copy
                    .iter()
                    .map(|verse| verse.text.clone())
                    .collect::<Vec<_>>()
                    .join(" ")
            } else {
                context.current_chapter.name.clone()
            }
        } else {
            context.current_chapter.name.clone()
        };
        
        self.copy_to_clipboard(copy_text);
        true
    }
    
    fn handle_copy_verse_with_reference(&self, context: &InstructionContext) -> bool {
        let verse_ranges = context.get_verse_ranges();
        let mut copy_text = String::new();
        
        if !verse_ranges.is_empty() {
            let mut verses_to_copy = Vec::new();
            
            // Collect verses in selected ranges
            for verse in &context.current_chapter.verses {
                for range in &verse_ranges {
                    if range.contains(verse.verse) {
                        verses_to_copy.push(verse);
                        break;
                    }
                }
            }
            
            if !verses_to_copy.is_empty() {
                // Add verse text
                copy_text = verses_to_copy
                    .iter()
                    .map(|verse| verse.text.clone())
                    .collect::<Vec<_>>()
                    .join(" ");
                
                // Add reference and link
                copy_text.push_str("\n\n");
                
                let book_name = context.current_chapter.name
                    .split_whitespace()
                    .next()
                    .unwrap_or("");
                let translated_book_name = self.get_translated_book_name(book_name);
                let chapter_num = context.current_chapter.name
                    .split_whitespace()
                    .nth(1)
                    .unwrap_or("1");
                
                // Format reference
                if verse_ranges.len() == 1 && verse_ranges[0].start == verse_ranges[0].end {
                    copy_text.push_str(&format!("{} {}:{}", translated_book_name, chapter_num, verse_ranges[0].start));
                } else {
                    let mut range_strs = Vec::new();
                    for range in &verse_ranges {
                        if range.start == range.end {
                            range_strs.push(range.start.to_string());
                        } else {
                            range_strs.push(format!("{}-{}", range.start, range.end));
                        }
                    }
                    copy_text.push_str(&format!("{} {}:{}", translated_book_name, chapter_num, range_strs.join(",")));
                }
                
                // Add link
                copy_text.push('\n');
                let book_name_url = book_name.replace(' ', "_").to_lowercase();
                let verses_param = context.search_params
                    .split("verses=")
                    .nth(1)
                    .unwrap_or("")
                    .split('&')
                    .next()
                    .unwrap_or("");
                copy_text.push_str(&format!("https://bible.pruijs.net/{}/{}?verses={}", book_name_url, chapter_num, verses_param));
            }
        } else {
            copy_text = context.current_chapter.name.clone();
        }
        
        self.copy_to_clipboard(copy_text);
        true
    }
    
    fn copy_to_clipboard(&self, text: String) {
        spawn_local(async move {
            if let Some(window) = leptos::web_sys::window() {
                let clipboard = window.navigator().clipboard();
                match JsFuture::from(clipboard.write_text(&text)).await {
                    Ok(_) => {
                        leptos::web_sys::console::log_1(&"Copied to clipboard!".into());
                    }
                    Err(_) => {
                        leptos::web_sys::console::log_1(&"Failed to copy to clipboard".into());
                    }
                }
            }
        });
    }
    
    fn get_translated_book_name(&self, book_name: &str) -> String {
        fn convert_language(storage_lang: &crate::storage::translation_storage::Language) -> Language {
            match storage_lang {
                crate::storage::translation_storage::Language::Dutch => Language::Dutch,
                crate::storage::translation_storage::Language::English => Language::English,
            }
        }
        
        if let Some(current_translation) = get_current_translation() {
            if let Some(first_language) = current_translation.languages.first() {
                let translation = Translation::from_language(convert_language(first_language));
                if let Some(translated_name) = translation.get_book(&book_name.to_lowercase()) {
                    return translated_name.to_string();
                }
            }
        }
        book_name.to_string()
    }
}