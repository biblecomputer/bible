use super::types::Instruction;
use crate::core::{get_bible, VerseRange};
use crate::storage::translations::get_current_translation;
use crate::translation_map::translation::Translation;
use crate::view_state::AppState;
use leptos_router::NavigateOptions;
use std::sync::atomic::{AtomicUsize, Ordering};
use wasm_bindgen_futures::{spawn_local, JsFuture};

// Global counter for pseudo-random verse selection
static RANDOM_COUNTER: AtomicUsize = AtomicUsize::new(1);

// Helper function to get a random chapter path (can be used from anywhere)
pub fn get_random_chapter_path() -> Option<String> {
    let bible = get_bible();

    // Calculate total number of chapters
    let mut total_chapters = 0;
    let mut chapter_locations = Vec::new();

    for book in &bible.books {
        for chapter in &book.chapters {
            chapter_locations.push(chapter.clone());
            total_chapters += 1;
        }
    }

    if total_chapters == 0 {
        return None; // No chapters found
    }

    // Get the current counter value and increment it for next time
    let counter = RANDOM_COUNTER.fetch_add(1, Ordering::Relaxed);

    // Use a simple linear congruential generator with the counter as seed
    let mut rng_state = counter.wrapping_mul(1103515245).wrapping_add(12345);
    rng_state = rng_state.wrapping_mul(1664525).wrapping_add(1013904223);

    let random_index = rng_state % total_chapters;

    // Ensure the index is within bounds
    let safe_index = random_index.min(total_chapters - 1);

    chapter_locations
        .get(safe_index)
        .map(|chapter| chapter.to_path())
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

    pub fn process(&self, instruction: Instruction, context: &AppState) -> bool {
        self.process_with_multiplier(instruction, context, 1)
    }

    pub fn process_with_multiplier(
        &self,
        instruction: Instruction,
        context: &AppState,
        multiplier: u32,
    ) -> bool {
        match instruction {
            Instruction::NextVerse => self.handle_next_verse_with_multiplier(context, multiplier),
            Instruction::PreviousVerse => {
                self.handle_previous_verse_with_multiplier(context, multiplier)
            }
            Instruction::ExtendSelectionNextVerse => {
                self.handle_extend_selection_next_verse_with_multiplier(context, multiplier)
            }
            Instruction::ExtendSelectionPreviousVerse => {
                self.handle_extend_selection_previous_verse_with_multiplier(context, multiplier)
            }
            Instruction::NextChapter => {
                self.handle_next_chapter_with_multiplier(context, multiplier)
            }
            Instruction::PreviousChapter => {
                self.handle_previous_chapter_with_multiplier(context, multiplier)
            }
            Instruction::NextBook => self.handle_next_book_with_multiplier(context, multiplier),
            Instruction::PreviousBook => {
                self.handle_previous_book_with_multiplier(context, multiplier)
            }
            Instruction::BeginningOfChapter => self.handle_beginning_of_chapter(context),
            Instruction::EndOfChapter => self.handle_end_of_chapter(context),
            Instruction::GoToVerse(verse_id) => self.handle_go_to_verse(context, verse_id),
            Instruction::CopyRawVerse => self.handle_copy_raw_verse(context),
            Instruction::CopyVerseWithReference => self.handle_copy_verse_with_reference(context),
            Instruction::OpenGithubRepository => self.handle_open_github_repository(),
            Instruction::RandomVerse => self.handle_random_verse(),
            Instruction::RandomChapter => self.handle_random_chapter(),
            Instruction::OpenAboutPage => self.handle_open_about_page(),
            Instruction::ShowTranslations => self.handle_show_translations(),
            _ => {
                // Other instructions need to be handled by the UI components
                // Return false to indicate this processor didn't handle it
                false
            }
        }
    }

    fn handle_beginning_of_chapter(&self, context: &AppState) -> bool {
        if let Some(ref current_chapter) = context.current_chapter {
            let new_path = current_chapter.to_path();
            (self.navigate)(
                &new_path,
                NavigateOptions {
                    scroll: false,
                    ..Default::default()
                },
            );
            true
        } else {
            false
        }
    }

    fn handle_end_of_chapter(&self, context: &AppState) -> bool {
        if let Some(ref current_chapter) = context.current_chapter {
            let last_verse = current_chapter.verses.len() as u32;
            if last_verse > 0 {
                let verse_range = VerseRange {
                    start: last_verse,
                    end: last_verse,
                };
                let new_path = current_chapter.to_path_with_verses(&[verse_range]);
                (self.navigate)(
                    &new_path,
                    NavigateOptions {
                        scroll: false,
                        ..Default::default()
                    },
                );
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn handle_go_to_verse(&self, context: &AppState, verse_id: crate::core::types::VerseId) -> bool {
        if let Some(ref current_chapter) = context.current_chapter {
            let verse_num = verse_id.verse();
            if verse_num > 0 && verse_num <= current_chapter.verses.len() as u32 {
                let verse_range = VerseRange {
                    start: verse_num,
                    end: verse_num,
                };
                let new_path = current_chapter.to_path_with_verses(&[verse_range]);
                (self.navigate)(
                    &new_path,
                    NavigateOptions {
                        scroll: false,
                        ..Default::default()
                    },
                );
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn handle_copy_raw_verse(&self, context: &AppState) -> bool {
        use leptos::web_sys::console;

        let verse_ranges = context.get_verse_ranges();
        console::log_1(
            &format!(
                "📖 Copy raw verse - Found {} verse ranges",
                verse_ranges.len()
            )
            .into(),
        );

        let copy_text = if let Some(ref current_chapter) = context.current_chapter {
            if !verse_ranges.is_empty() {
                let mut verses_to_copy = Vec::new();

                // Collect verses in selected ranges
                for verse in &current_chapter.verses {
                    for range in &verse_ranges {
                        if range.contains(verse.verse) {
                            console::log_1(
                                &format!(
                                    "📝 Including verse {}: {}",
                                    verse.verse,
                                    &verse.text[..verse.text.len().min(50)]
                                )
                                .into(),
                            );
                            verses_to_copy.push(verse);
                            break;
                        }
                    }
                }

                if !verses_to_copy.is_empty() {
                    let text = verses_to_copy
                        .iter()
                        .map(|verse| verse.text.clone())
                        .collect::<Vec<_>>()
                        .join("\n");
                    console::log_1(
                        &format!(
                            "📝 Raw copy: {} verses, {} chars",
                            verses_to_copy.len(),
                            text.len()
                        )
                        .into(),
                    );
                    text
                } else {
                    console::log_1(&"📖 No verses in ranges, copying chapter name".into());
                    current_chapter.name.clone()
                }
            } else {
                console::log_1(&"📖 No verse ranges selected, copying chapter name".into());
                current_chapter.name.clone()
            }
        } else {
            console::log_1(&"❌ No current chapter available for copy".into());
            return false;
        };

        self.copy_to_clipboard(copy_text);
        true
    }

    fn handle_copy_verse_with_reference(&self, context: &AppState) -> bool {
        use leptos::web_sys::console;

        let verse_ranges = context.get_verse_ranges();
        console::log_1(
            &format!(
                "📖 Copy with reference - Found {} verse ranges",
                verse_ranges.len()
            )
            .into(),
        );

        let mut copy_text = String::new();

        if let Some(ref current_chapter) = context.current_chapter {
            if !verse_ranges.is_empty() {
                let mut verses_to_copy = Vec::new();

                // Collect verses in selected ranges
                for verse in &current_chapter.verses {
                    for range in &verse_ranges {
                        if range.contains(verse.verse) {
                            console::log_1(
                                &format!("📝 Including verse {} for reference copy", verse.verse)
                                    .into(),
                            );
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
                        .join("\n");

                    // Add reference and link
                    copy_text.push_str("\n\n");

                    // Extract book name from chapter name (everything except the last word which is the chapter number)
                    let name_parts: Vec<&str> = current_chapter.name.split_whitespace().collect();
                    let book_name = if name_parts.len() > 1 {
                        name_parts[..name_parts.len() - 1].join(" ")
                    } else {
                        current_chapter.name.clone()
                    };
                    let translated_book_name = self.get_translated_book_name(&book_name);
                    let chapter_num = current_chapter.chapter.to_string();

                    console::log_1(
                        &format!(
                            "📚 Book: {}, Chapter: {}",
                            translated_book_name, chapter_num
                        )
                        .into(),
                    );

                    // Format reference
                    if verse_ranges.len() == 1 && verse_ranges[0].start == verse_ranges[0].end {
                        copy_text.push_str(&format!(
                            "{} {}:{}",
                            translated_book_name, chapter_num, verse_ranges[0].start
                        ));
                    } else {
                        let mut range_strs = Vec::new();
                        for range in &verse_ranges {
                            if range.start == range.end {
                                range_strs.push(range.start.to_string());
                            } else {
                                range_strs.push(format!("{}-{}", range.start, range.end));
                            }
                        }
                        copy_text.push_str(&format!(
                            "{} {}:{}",
                            translated_book_name,
                            chapter_num,
                            range_strs.join(",")
                        ));
                    }

                    // Add link
                    copy_text.push('\n');
                    let book_name_url = urlencoding::encode(&book_name);
                    let verses_param = context
                        .search_params
                        .split("verses=")
                        .nth(1)
                        .unwrap_or("")
                        .split('&')
                        .next()
                        .unwrap_or("");
                    copy_text.push_str(&format!(
                        "https://bible.computer/{}/{}?verses={}",
                        book_name_url, chapter_num, verses_param
                    ));

                    console::log_1(
                        &format!(
                            "📝 Reference copy: {} verses, complete with reference and link",
                            verses_to_copy.len()
                        )
                        .into(),
                    );
                }
            } else {
                console::log_1(
                    &"📖 No verse ranges selected, copying chapter name for reference".into(),
                );
                copy_text = current_chapter.name.clone();
            }
        } else {
            console::log_1(&"❌ No current chapter available for copy with reference".into());
            return false;
        }

        self.copy_to_clipboard(copy_text);
        true
    }

    fn copy_to_clipboard(&self, text: String) {
        use leptos::web_sys::{console, window};

        console::log_1(
            &format!(
                "📋 Attempting to copy text: '{}'",
                &text[..text.len().min(100)]
            )
            .into(),
        );

        spawn_local(async move {
            if let Some(window) = window() {
                // Try modern Clipboard API
                let navigator = window.navigator();
                let clipboard = navigator.clipboard();

                console::log_1(&"📋 Using modern Clipboard API".into());
                match JsFuture::from(clipboard.write_text(&text)).await {
                    Ok(_) => {
                        console::log_1(&"✅ Successfully copied to clipboard!".into());
                    }
                    Err(e) => {
                        console::log_1(&format!("❌ Clipboard API failed: {:?}", e).into());
                        console::log_1(
                            &"💡 Make sure you're using HTTPS and the page is focused".into(),
                        );
                    }
                }
            } else {
                console::log_1(&"❌ No window object available".into());
            }
        });
    }

    fn get_translated_book_name(&self, book_name: &str) -> String {
        if let Some(current_translation) = get_current_translation() {
            if let Some(first_language) = current_translation.languages.first() {
                let translation = Translation::from_language(*first_language);
                if let Some(translated_name) = translation.get_book(&book_name.to_lowercase()) {
                    return translated_name.to_string();
                }
            }
        }
        book_name.to_string()
    }

    // Multiplier versions of navigation methods
    fn handle_next_verse_with_multiplier(&self, context: &AppState, multiplier: u32) -> bool {
        if let Some(ref chapter) = context.current_chapter {
            let mut current_verse = context.get_current_verse();
            let mut current_chapter = chapter.clone();

            for _ in 0..multiplier {
                if current_verse == 0 {
                    // Currently on chapter heading, navigate to first verse
                    current_verse = 1;
                } else if let Some(next_verse) = current_chapter.get_next_verse(current_verse) {
                    // Move to next verse in current chapter
                    current_verse = next_verse;
                } else if let Some(next_chapter) = get_bible().get_next_chapter(&current_chapter) {
                    // Reached end of chapter, move to first verse of next chapter
                    current_chapter = next_chapter;
                    current_verse = 1;
                } else {
                    // Reached the end of the Bible
                    break;
                }
            }

            // Navigate to final position
            if current_verse == 0 {
                (self.navigate)(
                    &current_chapter.to_path(),
                    NavigateOptions {
                        scroll: false,
                        ..Default::default()
                    },
                );
            } else {
                let verse_range = VerseRange {
                    start: current_verse,
                    end: current_verse,
                };
                let new_path = current_chapter.to_path_with_verses(&[verse_range]);
                (self.navigate)(
                    &new_path,
                    NavigateOptions {
                        scroll: false,
                        ..Default::default()
                    },
                );
            }
            true
        } else {
            false
        }
    }

    fn handle_previous_verse_with_multiplier(&self, context: &AppState, multiplier: u32) -> bool {
        if let Some(ref chapter) = context.current_chapter {
            let mut current_verse = context.get_current_verse();
            let mut current_chapter = chapter.clone();

            for _ in 0..multiplier {
                if current_verse == 0 {
                    // Currently on chapter heading, navigate to last verse of previous chapter
                    if let Some(prev_chapter) = get_bible().get_previous_chapter(&current_chapter) {
                        current_chapter = prev_chapter;
                        current_verse = current_chapter.verses.len() as u32;
                    } else {
                        // Reached the beginning of the Bible
                        break;
                    }
                } else if current_verse == 1 {
                    // Currently on first verse, navigate to last verse of previous chapter
                    if let Some(prev_chapter) = get_bible().get_previous_chapter(&current_chapter) {
                        current_chapter = prev_chapter;
                        current_verse = current_chapter.verses.len() as u32;
                    } else {
                        // No previous chapter, go to chapter heading
                        current_verse = 0;
                    }
                } else if let Some(prev_verse) = current_chapter.get_previous_verse(current_verse) {
                    // Move to previous verse in current chapter
                    current_verse = prev_verse;
                } else {
                    // This shouldn't happen, but handle it gracefully
                    break;
                }
            }

            // Navigate to final position
            if current_verse == 0 {
                (self.navigate)(
                    &current_chapter.to_path(),
                    NavigateOptions {
                        scroll: false,
                        ..Default::default()
                    },
                );
            } else {
                let verse_range = VerseRange {
                    start: current_verse,
                    end: current_verse,
                };
                let new_path = current_chapter.to_path_with_verses(&[verse_range]);
                (self.navigate)(
                    &new_path,
                    NavigateOptions {
                        scroll: false,
                        ..Default::default()
                    },
                );
            }
            true
        } else {
            false
        }
    }

    fn handle_extend_selection_next_verse_with_multiplier(
        &self,
        context: &AppState,
        multiplier: u32,
    ) -> bool {
        let current_ranges = context.get_verse_ranges();

        if let Some(ref chapter) = context.current_chapter {
            // Determine the anchor point for the selection
            let (anchor_verse, mut target_verse, mut target_chapter) = if current_ranges.is_empty()
            {
                // No current selection, start from current verse or beginning of chapter
                let current_verse = context.get_current_verse();
                if current_verse == 0 {
                    (1, 1, chapter.clone())
                } else {
                    (current_verse, current_verse, chapter.clone())
                }
            } else {
                // Find the rightmost (highest) verse in current selection as anchor
                let last_range = current_ranges.iter().max_by_key(|r| r.end).unwrap();
                (
                    current_ranges.iter().min_by_key(|r| r.start).unwrap().start,
                    last_range.end,
                    chapter.clone(),
                )
            };

            // Move target verse forward by multiplier
            for _ in 0..multiplier {
                if let Some(next_verse) = target_chapter.get_next_verse(target_verse) {
                    target_verse = next_verse;
                } else if let Some(next_chapter) = get_bible().get_next_chapter(&target_chapter) {
                    // Cross chapter boundary
                    target_chapter = next_chapter;
                    target_verse = 1;
                } else {
                    // Reached end of Bible
                    break;
                }
            }

            // Create new selection range from anchor to target
            let new_range = if target_chapter.name == chapter.name {
                // Same chapter - create single range
                VerseRange {
                    start: anchor_verse.min(target_verse),
                    end: anchor_verse.max(target_verse),
                }
            } else {
                // Cross-chapter selection not supported for now, just select the target verse
                target_verse = 1; // Reset to first verse of new chapter
                VerseRange {
                    start: target_verse,
                    end: target_verse,
                }
            };

            // Navigate to new selection
            let new_path = if target_chapter.name == chapter.name {
                chapter.to_path_with_verses(&[new_range])
            } else {
                target_chapter.to_path_with_verses(&[new_range])
            };

            (self.navigate)(
                &new_path,
                NavigateOptions {
                    scroll: false,
                    ..Default::default()
                },
            );
            true
        } else {
            false
        }
    }

    fn handle_extend_selection_previous_verse_with_multiplier(
        &self,
        context: &AppState,
        multiplier: u32,
    ) -> bool {
        let current_ranges = context.get_verse_ranges();

        if let Some(ref chapter) = context.current_chapter {
            // Determine the anchor point for the selection
            let (anchor_verse, mut target_verse, mut target_chapter) = if current_ranges.is_empty()
            {
                // No current selection, start from current verse or end of chapter
                let current_verse = context.get_current_verse();
                if current_verse == 0 {
                    let last_verse = chapter.verses.len() as u32;
                    (last_verse, last_verse, chapter.clone())
                } else {
                    (current_verse, current_verse, chapter.clone())
                }
            } else {
                // Find the leftmost (lowest) verse in current selection as anchor
                let first_range = current_ranges.iter().min_by_key(|r| r.start).unwrap();
                (
                    current_ranges.iter().max_by_key(|r| r.end).unwrap().end,
                    first_range.start,
                    chapter.clone(),
                )
            };

            // Move target verse backward by multiplier
            for _ in 0..multiplier {
                if target_verse == 1 {
                    // At first verse, try to go to previous chapter
                    if let Some(prev_chapter) = get_bible().get_previous_chapter(&target_chapter) {
                        target_chapter = prev_chapter;
                        target_verse = target_chapter.verses.len() as u32;
                    } else {
                        // Reached beginning of Bible
                        target_verse = 1;
                        break;
                    }
                } else if let Some(prev_verse) = target_chapter.get_previous_verse(target_verse) {
                    target_verse = prev_verse;
                } else {
                    // Shouldn't happen, but break to be safe
                    break;
                }
            }

            // Create new selection range from target to anchor
            let new_range = if target_chapter.name == chapter.name {
                // Same chapter - create single range
                VerseRange {
                    start: anchor_verse.min(target_verse),
                    end: anchor_verse.max(target_verse),
                }
            } else {
                // Cross-chapter selection not supported for now, just select the target verse
                VerseRange {
                    start: target_verse,
                    end: target_verse,
                }
            };

            // Navigate to new selection
            let new_path = if target_chapter.name == chapter.name {
                chapter.to_path_with_verses(&[new_range])
            } else {
                target_chapter.to_path_with_verses(&[new_range])
            };

            (self.navigate)(
                &new_path,
                NavigateOptions {
                    scroll: false,
                    ..Default::default()
                },
            );
            true
        } else {
            false
        }
    }

    fn handle_next_chapter_with_multiplier(&self, context: &AppState, multiplier: u32) -> bool {
        if let Some(ref current_chapter) = context.current_chapter {
            if let Some(target_path) =
                get_bible().get_nth_next_chapter_path(current_chapter, multiplier)
            {
                (self.navigate)(
                    &target_path,
                    NavigateOptions {
                        scroll: false,
                        ..Default::default()
                    },
                );
            }
            true
        } else {
            false
        }
    }

    fn handle_previous_chapter_with_multiplier(&self, context: &AppState, multiplier: u32) -> bool {
        if let Some(ref current_chapter) = context.current_chapter {
            if let Some(target_path) =
                get_bible().get_nth_previous_chapter_path(current_chapter, multiplier)
            {
                (self.navigate)(
                    &target_path,
                    NavigateOptions {
                        scroll: false,
                        ..Default::default()
                    },
                );
            }
            true
        } else {
            false
        }
    }

    fn handle_next_book_with_multiplier(&self, context: &AppState, multiplier: u32) -> bool {
        if let Some(ref chapter) = context.current_chapter {
            let mut current_chapter = chapter.clone();

            for _ in 0..multiplier {
                if let Some(next_book_chapter) = get_bible().get_next_book(&current_chapter) {
                    current_chapter = next_book_chapter;
                } else {
                    // Reached the end
                    break;
                }
            }

            (self.navigate)(
                &current_chapter.to_path(),
                NavigateOptions {
                    scroll: false,
                    ..Default::default()
                },
            );
            true
        } else {
            false
        }
    }

    fn handle_previous_book_with_multiplier(&self, context: &AppState, multiplier: u32) -> bool {
        if let Some(ref chapter) = context.current_chapter {
            let mut current_chapter = chapter.clone();

            for _ in 0..multiplier {
                if let Some(prev_book_chapter) = get_bible().get_previous_book(&current_chapter) {
                    current_chapter = prev_book_chapter;
                } else {
                    // Reached the beginning
                    break;
                }
            }

            (self.navigate)(
                &current_chapter.to_path(),
                NavigateOptions {
                    scroll: false,
                    ..Default::default()
                },
            );
            true
        } else {
            false
        }
    }

    fn handle_open_github_repository(&self) -> bool {
        if let Some(window) = leptos::web_sys::window() {
            let _ = window
                .location()
                .set_href("https://github.com/biblecomputer/bible");
            true
        } else {
            false
        }
    }

    fn handle_random_verse(&self) -> bool {
        let bible = get_bible();

        // Calculate total number of verses in the entire Bible
        let mut total_verses = 0;
        let mut verse_locations = Vec::new();

        for book in &bible.books {
            for chapter in &book.chapters {
                for verse in &chapter.verses {
                    verse_locations.push((chapter.clone(), verse.verse));
                    total_verses += 1;
                }
            }
        }

        if total_verses == 0 {
            return false; // No verses found
        }

        // Get the current counter value and increment it for next time
        let counter = RANDOM_COUNTER.fetch_add(1, Ordering::Relaxed);

        // Use a simple linear congruential generator with the counter as seed
        let mut rng_state = counter.wrapping_mul(1103515245).wrapping_add(12345);
        rng_state = rng_state.wrapping_mul(1664525).wrapping_add(1013904223);

        let random_index = rng_state % total_verses;

        // Ensure the index is within bounds
        let safe_index = random_index.min(total_verses - 1);

        if let Some((chapter, verse_num)) = verse_locations.get(safe_index) {
            let verse_range = VerseRange {
                start: *verse_num,
                end: *verse_num,
            };
            let verse_ranges: &[VerseRange] = &[verse_range];
            let new_path = chapter.to_path_with_verses(verse_ranges);
            (self.navigate)(
                &new_path,
                NavigateOptions {
                    scroll: false,
                    ..Default::default()
                },
            );
            true
        } else {
            false
        }
    }

    fn handle_random_chapter(&self) -> bool {
        if let Some(random_path) = get_random_chapter_path() {
            (self.navigate)(
                &random_path,
                NavigateOptions {
                    scroll: false,
                    ..Default::default()
                },
            );
            true
        } else {
            false
        }
    }

    fn handle_open_about_page(&self) -> bool {
        (self.navigate)(
            "/about",
            NavigateOptions {
                scroll: false,
                ..Default::default()
            },
        );
        true
    }

    fn handle_show_translations(&self) -> bool {
        (self.navigate)(
            "/?choose=true",
            NavigateOptions {
                scroll: false,
                ..Default::default()
            },
        );
        true
    }
}
