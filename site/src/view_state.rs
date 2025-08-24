use crate::core::{get_bible, Bible, Chapter, VerseRange};
use crate::instructions::Instruction;
use crate::storage::{
    get_references_sidebar_open, get_sidebar_open, get_verse_visibility,
    save_references_sidebar_open, save_sidebar_open, save_verse_visibility,
};
use crate::storage::{get_selected_theme, get_selected_translation};
use crate::utils::is_mobile_screen;
use leptos::prelude::*;

/// Central state management for all UI view states
/// This replaces multiple individual signals with a single, cohesive state structure
#[derive(Debug, Clone)]
pub struct AppState {
    // Sidebar states
    pub is_left_sidebar_open: bool,
    pub is_right_sidebar_open: bool,
    pub is_theme_sidebar_open: bool,

    // Panel states
    pub is_translation_comparison_open: bool,
    pub is_command_palette_open: bool,

    // Feature toggles
    pub verse_visibility_enabled: bool,

    // Command palette navigation
    pub next_palette_result_trigger: bool,
    pub previous_palette_result_trigger: bool,
    pub initial_search_query: Option<String>,

    // Navigation context (formerly InstructionContext)
    pub current_chapter: Option<Chapter>,
    pub search_params: String,

    // Navigation history
    pub previous_chapter_path: Option<String>,

    // Export progress state
    pub export_progress: f32,
    pub export_status: String,
    pub is_exporting: bool,

    // Bible data - single source of truth
    pub current_bible: Option<Bible>,

    // Component-specific state
    pub selected_book: String,
    pub command_palette_input: String,
    pub command_palette_search_query: String,
    pub command_palette_selected_index: usize,
    pub reference_selected_index: Option<usize>,
    pub is_reference_navigating: bool,
    pub reference_sidebar_has_focus: bool,
    pub shortcuts_help_open: bool,
    pub theme_switcher_open: bool,
    pub theme_selected_index: Option<usize>,
    pub translation_switcher_open: bool,
    pub translation_switching: bool,

    // Theme and translation management
    pub current_theme_id: String,
    pub current_translation_short_name: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            // Initialize from localStorage where applicable
            is_left_sidebar_open: get_sidebar_open(),
            is_right_sidebar_open: get_references_sidebar_open(),
            is_theme_sidebar_open: false,
            is_translation_comparison_open: false,
            is_command_palette_open: false,
            verse_visibility_enabled: get_verse_visibility(),
            next_palette_result_trigger: false,
            previous_palette_result_trigger: false,
            initial_search_query: None,
            current_chapter: None,
            search_params: String::new(),
            previous_chapter_path: None,
            export_progress: 0.0,
            export_status: String::new(),
            is_exporting: false,
            current_bible: Some(get_bible().clone()),

            // Component-specific state
            selected_book: String::new(),
            command_palette_input: String::new(),
            command_palette_search_query: String::new(),
            command_palette_selected_index: 0,
            reference_selected_index: None,
            is_reference_navigating: false,
            reference_sidebar_has_focus: false,
            shortcuts_help_open: false,
            theme_switcher_open: false,
            theme_selected_index: None,
            translation_switcher_open: false,
            translation_switching: false,

            // Theme and translation management - initialize from storage
            current_theme_id: get_selected_theme(),
            current_translation_short_name: get_selected_translation()
                .unwrap_or_else(|| "nl_sv".to_string()),
        }
    }
}

impl AppState {
    /// Create a new ViewState with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Get a reference to the current Bible
    pub fn get_bible(&self) -> Option<&Bible> {
        self.current_bible.as_ref()
    }

    /// Set the current Bible
    pub fn set_bible(&mut self, bible: Bible) {
        self.current_bible = Some(bible);
    }

    // Component-specific state management

    /// Get selected book
    pub fn get_selected_book(&self) -> &str {
        &self.selected_book
    }

    /// Update command palette input
    pub fn set_command_palette_input(&mut self, input: String) {
        self.command_palette_input = input;
    }

    /// Get command palette input
    pub fn get_command_palette_input(&self) -> &str {
        &self.command_palette_input
    }

    /// Set command palette search query
    pub fn set_command_palette_search_query(&mut self, query: String) {
        self.command_palette_search_query = query;
    }

    /// Get command palette search query
    pub fn get_command_palette_search_query(&self) -> &str {
        &self.command_palette_search_query
    }

    /// Set command palette selected index
    pub fn set_command_palette_selected_index(&mut self, index: usize) {
        self.command_palette_selected_index = index;
    }

    /// Get command palette selected index
    pub fn get_command_palette_selected_index(&self) -> usize {
        self.command_palette_selected_index
    }

    /// Set reference selected index
    pub fn set_reference_selected_index(&mut self, index: Option<usize>) {
        self.reference_selected_index = index;
    }

    /// Get reference selected index
    pub fn get_reference_selected_index(&self) -> Option<usize> {
        self.reference_selected_index
    }

    /// Set reference navigation state
    pub fn set_reference_navigating(&mut self, navigating: bool) {
        self.is_reference_navigating = navigating;
    }

    /// Get reference navigation state
    pub fn is_reference_navigating(&self) -> bool {
        self.is_reference_navigating
    }

    /// Set reference sidebar focus state
    pub fn set_reference_sidebar_focus(&mut self, focused: bool) {
        self.reference_sidebar_has_focus = focused;
    }

    /// Get reference sidebar focus state
    pub fn has_reference_sidebar_focus(&self) -> bool {
        self.reference_sidebar_has_focus
    }

    /// Toggle shortcuts help visibility
    pub fn toggle_shortcuts_help(&mut self) {
        self.shortcuts_help_open = !self.shortcuts_help_open;
    }

    /// Set shortcuts help visibility
    pub fn set_shortcuts_help_open(&mut self, open: bool) {
        self.shortcuts_help_open = open;
    }

    /// Get shortcuts help visibility
    pub fn is_shortcuts_help_open(&self) -> bool {
        self.shortcuts_help_open
    }

    /// Toggle theme switcher dropdown
    pub fn toggle_theme_switcher(&mut self) {
        self.theme_switcher_open = !self.theme_switcher_open;
    }

    /// Set theme switcher dropdown state
    pub fn set_theme_switcher_open(&mut self, open: bool) {
        self.theme_switcher_open = open;
    }

    /// Get theme switcher dropdown state
    pub fn is_theme_switcher_open(&self) -> bool {
        self.theme_switcher_open
    }

    /// Set theme selected index
    pub fn set_theme_selected_index(&mut self, index: Option<usize>) {
        self.theme_selected_index = index;
    }

    /// Get theme selected index
    pub fn get_theme_selected_index(&self) -> Option<usize> {
        self.theme_selected_index
    }

    /// Toggle translation switcher dropdown
    pub fn toggle_translation_switcher(&mut self) {
        self.translation_switcher_open = !self.translation_switcher_open;
    }

    /// Set translation switcher dropdown state
    pub fn set_translation_switcher_open(&mut self, open: bool) {
        self.translation_switcher_open = open;
    }

    /// Get translation switcher dropdown state
    pub fn is_translation_switcher_open(&self) -> bool {
        self.translation_switcher_open
    }

    /// Set translation switching state
    pub fn set_translation_switching(&mut self, switching: bool) {
        self.translation_switching = switching;
    }

    /// Get translation switching state
    pub fn is_translation_switching(&self) -> bool {
        self.translation_switching
    }

    /// Set current theme and persist to storage
    pub fn set_current_theme(&mut self, theme_id: String) {
        use crate::storage::save_selected_theme;
        self.current_theme_id = theme_id.clone();
        save_selected_theme(&theme_id);
    }

    /// Get current theme
    pub fn get_current_theme(&self) -> &str {
        &self.current_theme_id
    }

    /// Set current translation and persist to storage
    pub fn set_current_translation(&mut self, short_name: String) {
        use crate::storage::set_selected_translation;
        self.current_translation_short_name = short_name.clone();
        let _ = set_selected_translation(&short_name);
    }

    /// Get current translation short name
    pub fn get_current_translation_short_name(&self) -> &str {
        &self.current_translation_short_name
    }

    pub fn execute(&mut self, instruction: &Instruction) -> InstructionResult {
        #[cfg(target_arch = "wasm32")]
        leptos::web_sys::console::log_1(
            &format!("🎮 Executing instruction: {:?}", instruction).into(),
        );

        match instruction {
            // UI Toggle instructions
            Instruction::ToggleCommandPallate => {
                self.toggle_command_palette();
                InstructionResult::Handled
            }
            Instruction::ToggleSidebar => {
                self.toggle_left_sidebar();
                InstructionResult::Handled
            }
            Instruction::ToggleCrossReferences => {
                self.toggle_right_sidebar();
                InstructionResult::Handled
            }
            Instruction::ToggleThemeSidebar => {
                self.toggle_theme_sidebar();
                InstructionResult::Handled
            }
            Instruction::ToggleTranslationComparison => {
                self.toggle_translation_comparison();
                InstructionResult::Handled
            }
            Instruction::ToggleVerseVisibility => {
                self.toggle_verse_visibility();
                InstructionResult::Handled
            }

            // Navigation instructions
            Instruction::NextVerse => {
                let result = self.handle_next_verse();
                #[cfg(target_arch = "wasm32")]
                leptos::web_sys::console::log_1(
                    &format!("📖 NextVerse result: {:?}", result).into(),
                );
                result
            }
            Instruction::PreviousVerse => {
                let result = self.handle_previous_verse();
                #[cfg(target_arch = "wasm32")]
                leptos::web_sys::console::log_1(
                    &format!("📖 PreviousVerse result: {:?}", result).into(),
                );
                result
            }
            Instruction::NextChapter => {
                let result = self.handle_next_chapter();
                #[cfg(target_arch = "wasm32")]
                leptos::web_sys::console::log_1(
                    &format!("📖 NextChapter result: {:?}", result).into(),
                );
                result
            }
            Instruction::PreviousChapter => {
                let result = self.handle_previous_chapter();
                #[cfg(target_arch = "wasm32")]
                leptos::web_sys::console::log_1(
                    &format!("📖 PreviousChapter result: {:?}", result).into(),
                );
                result
            }
            Instruction::NextBook => self.handle_next_book(),
            Instruction::PreviousBook => self.handle_previous_book(),
            Instruction::BeginningOfChapter => self.handle_beginning_of_chapter(),
            Instruction::EndOfChapter => self.handle_end_of_chapter(),
            Instruction::GoToVerse(verse_id) => self.handle_go_to_verse(*verse_id),
            Instruction::GoToChapter(chapter_path) => {
                // Handle mobile sidebar closing as a side effect
                if is_mobile_screen() {
                    self.execute(&Instruction::CloseLeftSidebar);
                }
                InstructionResult::Navigate(chapter_path.clone())
            }

            // Selection instructions
            Instruction::ExtendSelectionNextVerse => self.handle_extend_selection_next_verse(),
            Instruction::ExtendSelectionPreviousVerse => {
                self.handle_extend_selection_previous_verse()
            }

            // Previous chapter navigation
            Instruction::SwitchToPreviousChapter => self.handle_switch_to_previous_chapter(),

            // Palette navigation
            Instruction::NextPaletteResult => {
                self.trigger_next_palette_result();
                InstructionResult::Handled
            }
            Instruction::PreviousPaletteResult => {
                self.trigger_previous_palette_result();
                InstructionResult::Handled
            }

            // Instructions that involve external actions but can be handled here
            Instruction::OpenGithubRepository => {
                #[cfg(target_arch = "wasm32")]
                {
                    if let Some(window) = leptos::web_sys::window() {
                        let _ = window
                            .location()
                            .set_href("https://github.com/biblecomputer/bible");
                    }
                }
                InstructionResult::Handled
            }
            Instruction::OpenAboutPage => InstructionResult::Navigate("/about".to_string()),
            Instruction::ShowTranslations => {
                InstructionResult::Navigate("/?choose=true".to_string())
            }
            Instruction::RandomVerse => {
                if let Some(path) = self.get_random_verse_path() {
                    InstructionResult::Navigate(path)
                } else {
                    InstructionResult::Failed("No verses available".to_string())
                }
            }
            Instruction::RandomChapter => {
                if let Some(path) = self.get_random_chapter_path() {
                    InstructionResult::Navigate(path)
                } else {
                    InstructionResult::Failed("No chapters available".to_string())
                }
            }

            // UI Close instructions
            Instruction::CloseCommandPalette => {
                self.is_command_palette_open = false;
                InstructionResult::Handled
            }
            Instruction::CloseLeftSidebar => {
                self.is_left_sidebar_open = false;
                InstructionResult::Handled
            }
            Instruction::CloseRightSidebar => {
                self.is_right_sidebar_open = false;
                InstructionResult::Handled
            }
            Instruction::CloseThemeSidebar => {
                self.is_theme_sidebar_open = false;
                InstructionResult::Handled
            }
            Instruction::CloseTranslationComparison => {
                self.is_translation_comparison_open = false;
                InstructionResult::Handled
            }

            // Book Selection instructions
            Instruction::SelectBook(book_name) => {
                self.selected_book = book_name.clone();
                InstructionResult::Handled
            }
            Instruction::ClearSelectedBook => {
                self.selected_book = String::new();
                InstructionResult::Handled
            }

            // Instructions that still need external handling (exports, copy operations, palette toggles)
            Instruction::CopyRawVerse
            | Instruction::CopyVerseWithReference
            | Instruction::ExportToPDF
            | Instruction::ExportToMarkdown
            | Instruction::ExportLinkedMarkdown
            | Instruction::ToggleBiblePallate
            | Instruction::ToggleVersePallate
            | Instruction::NextReference
            | Instruction::PreviousReference => InstructionResult::NotHandled,
        }
    }

    pub fn execute_with_multiplier(
        &mut self,
        instruction: &Instruction,
        multiplier: u32,
    ) -> InstructionResult {
        #[cfg(target_arch = "wasm32")]
        leptos::web_sys::console::log_1(
            &format!(
                "🎮 Executing instruction with multiplier: {:?} x{}",
                instruction, multiplier
            )
            .into(),
        );

        match instruction {
            Instruction::NextVerse => self.handle_next_verse_with_multiplier(multiplier),
            Instruction::PreviousVerse => self.handle_previous_verse_with_multiplier(multiplier),
            Instruction::NextChapter => self.handle_next_chapter_with_multiplier(multiplier),
            Instruction::PreviousChapter => {
                self.handle_previous_chapter_with_multiplier(multiplier)
            }
            Instruction::NextBook => self.handle_next_book_with_multiplier(multiplier),
            Instruction::PreviousBook => self.handle_previous_book_with_multiplier(multiplier),
            _ => self.execute(instruction),
        }
    }

    /// Toggle the left sidebar and persist to storage
    pub fn toggle_left_sidebar(&mut self) {
        self.is_left_sidebar_open = !self.is_left_sidebar_open;
        save_sidebar_open(self.is_left_sidebar_open);
    }

    /// Set left sidebar state and persist to storage
    pub fn set_left_sidebar(&mut self, open: bool) {
        self.is_left_sidebar_open = open;
        save_sidebar_open(self.is_left_sidebar_open);
    }

    /// Toggle the right sidebar and persist to storage
    pub fn toggle_right_sidebar(&mut self) {
        self.is_right_sidebar_open = !self.is_right_sidebar_open;
        save_references_sidebar_open(self.is_right_sidebar_open);

        // Close theme sidebar if opening references sidebar
        if self.is_right_sidebar_open {
            self.is_theme_sidebar_open = false;
        }
    }

    /// Set right sidebar state and persist to storage
    pub fn set_right_sidebar(&mut self, open: bool) {
        self.is_right_sidebar_open = open;
        save_references_sidebar_open(self.is_right_sidebar_open);
    }

    /// Toggle the theme sidebar
    pub fn toggle_theme_sidebar(&mut self) {
        self.is_theme_sidebar_open = !self.is_theme_sidebar_open;

        // Close references sidebar if opening theme sidebar
        if self.is_theme_sidebar_open {
            self.is_right_sidebar_open = false;
            save_references_sidebar_open(false);
        }
    }

    /// Set theme sidebar state
    pub fn set_theme_sidebar(&mut self, open: bool) {
        self.is_theme_sidebar_open = open;
    }

    /// Toggle translation comparison panel
    pub fn toggle_translation_comparison(&mut self) {
        self.is_translation_comparison_open = !self.is_translation_comparison_open;
    }

    /// Set translation comparison panel state
    pub fn set_translation_comparison(&mut self, open: bool) {
        self.is_translation_comparison_open = open;
    }

    /// Toggle command palette
    pub fn toggle_command_palette(&mut self) {
        self.is_command_palette_open = !self.is_command_palette_open;
    }

    /// Set command palette state
    pub fn set_command_palette(&mut self, open: bool) {
        self.is_command_palette_open = open;

        // Clear initial search query after palette opens
        if open && self.initial_search_query.is_some() {
            // Note: In practice, this should be handled by an Effect in the component
            // self.initial_search_query = None;
        }
    }

    /// Toggle verse visibility and persist to storage
    pub fn toggle_verse_visibility(&mut self) {
        self.verse_visibility_enabled = !self.verse_visibility_enabled;
        save_verse_visibility(self.verse_visibility_enabled);
    }

    /// Set verse visibility and persist to storage
    pub fn set_verse_visibility(&mut self, enabled: bool) {
        self.verse_visibility_enabled = enabled;
        save_verse_visibility(self.verse_visibility_enabled);
    }

    /// Trigger next palette result navigation
    pub fn trigger_next_palette_result(&mut self) {
        self.next_palette_result_trigger = !self.next_palette_result_trigger;
    }

    /// Trigger previous palette result navigation
    pub fn trigger_previous_palette_result(&mut self) {
        self.previous_palette_result_trigger = !self.previous_palette_result_trigger;
    }

    /// Set initial search query for command palette
    pub fn set_initial_search_query(&mut self, query: Option<String>) {
        self.initial_search_query = query;
    }

    /// Clear initial search query
    pub fn clear_initial_search_query(&mut self) {
        self.initial_search_query = None;
    }

    // Navigation instruction handlers
    fn handle_next_verse(&mut self) -> InstructionResult {
        self.handle_next_verse_with_multiplier(1)
    }

    fn handle_previous_verse(&mut self) -> InstructionResult {
        self.handle_previous_verse_with_multiplier(1)
    }

    fn handle_next_chapter(&mut self) -> InstructionResult {
        self.handle_next_chapter_with_multiplier(1)
    }

    fn handle_previous_chapter(&mut self) -> InstructionResult {
        self.handle_previous_chapter_with_multiplier(1)
    }

    fn handle_next_book(&mut self) -> InstructionResult {
        self.handle_next_book_with_multiplier(1)
    }

    fn handle_previous_book(&mut self) -> InstructionResult {
        self.handle_previous_book_with_multiplier(1)
    }

    fn handle_beginning_of_chapter(&mut self) -> InstructionResult {
        if let Some(ref current_chapter) = self.current_chapter {
            let new_path = current_chapter.to_path();
            InstructionResult::Navigate(new_path)
        } else {
            InstructionResult::Failed("No current chapter".to_string())
        }
    }

    fn handle_end_of_chapter(&mut self) -> InstructionResult {
        if let Some(ref current_chapter) = self.current_chapter {
            let last_verse = current_chapter.verses.len() as u32;
            if last_verse > 0 {
                let verse_range = VerseRange {
                    start: last_verse,
                    end: last_verse,
                };
                let new_path = current_chapter.to_path_with_verses(&[verse_range]);
                InstructionResult::Navigate(new_path)
            } else {
                InstructionResult::Failed("Chapter has no verses".to_string())
            }
        } else {
            InstructionResult::Failed("No current chapter".to_string())
        }
    }

    fn handle_go_to_verse(&mut self, verse_id: crate::core::types::VerseId) -> InstructionResult {
        if let Some(ref current_chapter) = self.current_chapter {
            let verse_num = verse_id.verse();
            if verse_num > 0 && verse_num <= current_chapter.verses.len() as u32 {
                let verse_range = VerseRange {
                    start: verse_num,
                    end: verse_num,
                };
                let new_path = current_chapter.to_path_with_verses(&[verse_range]);
                InstructionResult::Navigate(new_path)
            } else {
                InstructionResult::Failed(format!("Invalid verse number: {}", verse_num))
            }
        } else {
            InstructionResult::Failed("No current chapter".to_string())
        }
    }

    fn handle_switch_to_previous_chapter(&mut self) -> InstructionResult {
        if let Some(ref prev_path) = self.previous_chapter_path.clone() {
            // Update previous chapter to current path before switching
            if let Some(ref current_chapter) = self.current_chapter {
                self.previous_chapter_path = Some(format!(
                    "/{}?{}",
                    current_chapter.to_path().trim_start_matches('/'),
                    self.search_params
                ));
            }
            InstructionResult::Navigate(prev_path.clone())
        } else {
            InstructionResult::Failed("No previous chapter available".to_string())
        }
    }

    // Navigation methods with Bible core integration
    fn handle_next_verse_with_multiplier(&mut self, multiplier: u32) -> InstructionResult {
        if let Some(ref current_chapter) = self.current_chapter {
            let mut current_verse = self.get_current_verse();
            let mut target_chapter = current_chapter.clone();

            for _ in 0..multiplier {
                if current_verse == 0 {
                    // Currently on chapter heading, navigate to first verse
                    current_verse = 1;
                } else if let Some(next_verse) = target_chapter.get_next_verse(current_verse) {
                    // Move to next verse in current chapter
                    current_verse = next_verse;
                } else if let Some(next_chapter) = self
                    .get_bible()
                    .and_then(|bible| bible.get_next_chapter(&target_chapter))
                {
                    // Reached end of chapter, move to first verse of next chapter
                    target_chapter = next_chapter;
                    current_verse = 1;
                } else {
                    // Reached the end of the Bible
                    break;
                }
            }

            // Generate navigation path
            if current_verse == 0 {
                InstructionResult::Navigate(target_chapter.to_path())
            } else {
                let verse_range = VerseRange {
                    start: current_verse,
                    end: current_verse,
                };
                let new_path = target_chapter.to_path_with_verses(&[verse_range]);
                InstructionResult::Navigate(new_path)
            }
        } else {
            InstructionResult::Failed("No current chapter".to_string())
        }
    }

    fn handle_previous_verse_with_multiplier(&mut self, multiplier: u32) -> InstructionResult {
        if let Some(ref current_chapter) = self.current_chapter {
            let mut current_verse = self.get_current_verse();
            let mut target_chapter = current_chapter.clone();

            for _ in 0..multiplier {
                if current_verse == 0 {
                    // Currently on chapter heading, navigate to last verse of previous chapter
                    if let Some(prev_chapter) = self
                        .get_bible()
                        .and_then(|bible| bible.get_previous_chapter(&target_chapter))
                    {
                        target_chapter = prev_chapter;
                        current_verse = target_chapter.verses.len() as u32;
                    } else {
                        // Reached the beginning of the Bible
                        break;
                    }
                } else if current_verse == 1 {
                    // Currently on first verse, navigate to last verse of previous chapter
                    if let Some(prev_chapter) = self
                        .get_bible()
                        .and_then(|bible| bible.get_previous_chapter(&target_chapter))
                    {
                        target_chapter = prev_chapter;
                        current_verse = target_chapter.verses.len() as u32;
                    } else {
                        // No previous chapter, go to chapter heading
                        current_verse = 0;
                    }
                } else if let Some(prev_verse) = target_chapter.get_previous_verse(current_verse) {
                    // Move to previous verse in current chapter
                    current_verse = prev_verse;
                } else {
                    // This shouldn't happen, but handle it gracefully
                    break;
                }
            }

            // Generate navigation path
            if current_verse == 0 {
                InstructionResult::Navigate(target_chapter.to_path())
            } else {
                let verse_range = VerseRange {
                    start: current_verse,
                    end: current_verse,
                };
                let new_path = target_chapter.to_path_with_verses(&[verse_range]);
                InstructionResult::Navigate(new_path)
            }
        } else {
            InstructionResult::Failed("No current chapter".to_string())
        }
    }

    fn handle_next_chapter_with_multiplier(&mut self, multiplier: u32) -> InstructionResult {
        if let Some(ref current_chapter) = self.current_chapter {
            if let Some(target_path) = self
                .get_bible()
                .and_then(|bible| bible.get_nth_next_chapter_path(current_chapter, multiplier))
            {
                InstructionResult::Navigate(target_path)
            } else {
                InstructionResult::Failed("No next chapter available".to_string())
            }
        } else {
            InstructionResult::Failed("No current chapter".to_string())
        }
    }

    fn handle_previous_chapter_with_multiplier(&mut self, multiplier: u32) -> InstructionResult {
        if let Some(ref current_chapter) = self.current_chapter {
            if let Some(target_path) = self
                .get_bible()
                .and_then(|bible| bible.get_nth_previous_chapter_path(current_chapter, multiplier))
            {
                InstructionResult::Navigate(target_path)
            } else {
                InstructionResult::Failed("No previous chapter available".to_string())
            }
        } else {
            InstructionResult::Failed("No current chapter".to_string())
        }
    }

    fn handle_next_book_with_multiplier(&mut self, multiplier: u32) -> InstructionResult {
        if let Some(ref current_chapter) = self.current_chapter {
            let mut target_chapter = current_chapter.clone();

            for _ in 0..multiplier {
                if let Some(next_book_chapter) = self
                    .get_bible()
                    .and_then(|bible| bible.get_next_book(&target_chapter))
                {
                    target_chapter = next_book_chapter;
                } else {
                    // Reached the end
                    break;
                }
            }

            InstructionResult::Navigate(target_chapter.to_path())
        } else {
            InstructionResult::Failed("No current chapter".to_string())
        }
    }

    fn handle_previous_book_with_multiplier(&mut self, multiplier: u32) -> InstructionResult {
        if let Some(ref current_chapter) = self.current_chapter {
            let mut target_chapter = current_chapter.clone();

            for _ in 0..multiplier {
                if let Some(prev_book_chapter) = self
                    .get_bible()
                    .and_then(|bible| bible.get_previous_book(&target_chapter))
                {
                    target_chapter = prev_book_chapter;
                } else {
                    // Reached the beginning
                    break;
                }
            }

            InstructionResult::Navigate(target_chapter.to_path())
        } else {
            InstructionResult::Failed("No current chapter".to_string())
        }
    }

    fn handle_extend_selection_next_verse(&mut self) -> InstructionResult {
        if let Some(ref current_chapter) = self.current_chapter {
            let current_ranges = self.get_verse_ranges();

            // Determine the anchor point for the selection
            let (anchor_verse, mut target_verse) = if current_ranges.is_empty() {
                // No current selection, start from current verse or beginning of chapter
                let current_verse = self.get_current_verse();
                if current_verse == 0 {
                    (1, 1)
                } else {
                    (current_verse, current_verse)
                }
            } else {
                // Find the rightmost (highest) verse in current selection as anchor
                let last_range = current_ranges.iter().max_by_key(|r| r.end).unwrap();
                (
                    current_ranges.iter().min_by_key(|r| r.start).unwrap().start,
                    last_range.end,
                )
            };

            // Move target verse forward by 1
            if let Some(next_verse) = current_chapter.get_next_verse(target_verse) {
                target_verse = next_verse;
            } else {
                // At end of chapter, can't extend further
                return InstructionResult::Failed(
                    "Cannot extend selection beyond chapter".to_string(),
                );
            }

            // Create new selection range from anchor to target
            let new_range = VerseRange {
                start: anchor_verse.min(target_verse),
                end: anchor_verse.max(target_verse),
            };

            let new_path = current_chapter.to_path_with_verses(&[new_range]);
            InstructionResult::Navigate(new_path)
        } else {
            InstructionResult::Failed("No current chapter".to_string())
        }
    }

    fn handle_extend_selection_previous_verse(&mut self) -> InstructionResult {
        if let Some(ref current_chapter) = self.current_chapter {
            let current_ranges = self.get_verse_ranges();

            // Determine the anchor point for the selection
            let (anchor_verse, mut target_verse) = if current_ranges.is_empty() {
                // No current selection, start from current verse or end of chapter
                let current_verse = self.get_current_verse();
                if current_verse == 0 {
                    let last_verse = current_chapter.verses.len() as u32;
                    (last_verse, last_verse)
                } else {
                    (current_verse, current_verse)
                }
            } else {
                // Find the leftmost (lowest) verse in current selection as anchor
                let first_range = current_ranges.iter().min_by_key(|r| r.start).unwrap();
                (
                    current_ranges.iter().max_by_key(|r| r.end).unwrap().end,
                    first_range.start,
                )
            };

            // Move target verse backward by 1
            if target_verse == 1 {
                // At beginning of chapter, can't extend further
                return InstructionResult::Failed(
                    "Cannot extend selection beyond chapter".to_string(),
                );
            } else if let Some(prev_verse) = current_chapter.get_previous_verse(target_verse) {
                target_verse = prev_verse;
            } else {
                // Shouldn't happen, but break to be safe
                return InstructionResult::Failed("Invalid verse navigation".to_string());
            }

            // Create new selection range from target to anchor
            let new_range = VerseRange {
                start: anchor_verse.min(target_verse),
                end: anchor_verse.max(target_verse),
            };

            let new_path = current_chapter.to_path_with_verses(&[new_range]);
            InstructionResult::Navigate(new_path)
        } else {
            InstructionResult::Failed("No current chapter".to_string())
        }
    }

    /// Close all sidebars and panels
    pub fn close_all_sidebars(&mut self) {
        self.execute(&Instruction::CloseLeftSidebar);
        self.execute(&Instruction::CloseRightSidebar);
        self.execute(&Instruction::CloseThemeSidebar);
        self.execute(&Instruction::CloseTranslationComparison);
    }

    /// Close all overlays (useful for mobile)
    pub fn close_all_overlays(&mut self) {
        self.execute(&Instruction::CloseCommandPalette);
        self.close_all_sidebars();
    }

    /// Set the current chapter
    pub fn set_current_chapter(&mut self, chapter: Option<Chapter>) {
        self.current_chapter = chapter;
    }

    /// Set the search parameters
    pub fn set_search_params(&mut self, search_params: String) {
        self.search_params = search_params;
    }

    /// Get current verse from search params (formerly from InstructionContext)
    pub fn get_current_verse(&self) -> u32 {
        if self.search_params.contains("verses=") {
            let verse_param = self
                .search_params
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

    /// Get verse ranges from search params (formerly from InstructionContext)
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

    /// Set previous chapter path for navigation history
    pub fn set_previous_chapter_path(&mut self, path: Option<String>) {
        self.previous_chapter_path = path;
    }

    /// Update export progress
    pub fn set_export_progress(&mut self, progress: f32, status: String) {
        self.export_progress = progress;
        self.export_status = status;
    }

    /// Set export state
    pub fn set_exporting(&mut self, exporting: bool) {
        self.is_exporting = exporting;
        if !exporting {
            self.export_progress = 0.0;
            self.export_status.clear();
        }
    }

    /// Get a random verse path
    fn get_random_verse_path(&self) -> Option<String> {
        use std::sync::atomic::{AtomicUsize, Ordering};
        static RANDOM_COUNTER: AtomicUsize = AtomicUsize::new(1);

        let bible = self.get_bible()?;
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
            return None;
        }

        let counter = RANDOM_COUNTER.fetch_add(1, Ordering::Relaxed);
        let mut rng_state = counter.wrapping_mul(1103515245).wrapping_add(12345);
        rng_state = rng_state.wrapping_mul(1664525).wrapping_add(1013904223);
        let random_index = rng_state % total_verses;
        let safe_index = random_index.min(total_verses - 1);

        if let Some((chapter, verse_num)) = verse_locations.get(safe_index) {
            let verse_range = VerseRange {
                start: *verse_num,
                end: *verse_num,
            };
            Some(chapter.to_path_with_verses(&[verse_range]))
        } else {
            None
        }
    }

    /// Get a random chapter path
    fn get_random_chapter_path(&self) -> Option<String> {
        use std::sync::atomic::{AtomicUsize, Ordering};
        static RANDOM_COUNTER: AtomicUsize = AtomicUsize::new(1);

        let bible = self.get_bible()?;
        let mut total_chapters = 0;
        let mut chapter_locations = Vec::new();

        for book in &bible.books {
            for chapter in &book.chapters {
                chapter_locations.push(chapter.clone());
                total_chapters += 1;
            }
        }

        if total_chapters == 0 {
            return None;
        }

        let counter = RANDOM_COUNTER.fetch_add(1, Ordering::Relaxed);
        let mut rng_state = counter.wrapping_mul(1103515245).wrapping_add(12345);
        rng_state = rng_state.wrapping_mul(1664525).wrapping_add(1013904223);
        let random_index = rng_state % total_chapters;
        let safe_index = random_index.min(total_chapters - 1);

        chapter_locations
            .get(safe_index)
            .map(|chapter| chapter.to_path())
    }
}

/// Leptos signal wrapper for ViewState
pub type ViewStateSignal = RwSignal<AppState>;

/// Create a new ViewState signal with default values
pub fn create_view_state() -> ViewStateSignal {
    RwSignal::new(AppState::new())
}

/// Helper functions for common view state operations
pub trait ViewStateSignalExt {
    /// Update the view state with a closure
    fn update_state<F>(&self, f: F)
    where
        F: FnOnce(&mut AppState);

    /// Get a specific boolean state value
    fn get_bool<F>(&self, f: F) -> bool
    where
        F: Fn(&AppState) -> bool;
}

impl ViewStateSignalExt for ViewStateSignal {
    fn update_state<F>(&self, f: F)
    where
        F: FnOnce(&mut AppState),
    {
        self.update(f);
    }

    fn get_bool<F>(&self, f: F) -> bool
    where
        F: Fn(&AppState) -> bool,
    {
        self.with(f)
    }
}

/// Result of applying an instruction to ViewState
#[derive(Debug, Clone, PartialEq)]
pub enum InstructionResult {
    /// Instruction was handled and ViewState was updated
    Handled,
    /// Instruction was handled but requires navigation
    Navigate(String),
    /// Instruction was not handled (e.g., needs external processing)
    NotHandled,
    /// Instruction failed due to invalid state
    Failed(String),
}

/// ViewState instruction application methods
impl AppState {
    /// Apply an instruction to the ViewState
    pub fn apply_instruction(&mut self, instruction: Instruction) -> InstructionResult {
        match instruction {
            // UI Toggle instructions
            Instruction::ToggleCommandPallate => {
                self.toggle_command_palette();
                InstructionResult::Handled
            }
            Instruction::ToggleSidebar => {
                self.toggle_left_sidebar();
                InstructionResult::Handled
            }
            Instruction::ToggleCrossReferences => {
                self.toggle_right_sidebar();
                InstructionResult::Handled
            }
            Instruction::ToggleThemeSidebar => {
                self.toggle_theme_sidebar();
                InstructionResult::Handled
            }
            Instruction::ToggleTranslationComparison => {
                self.toggle_translation_comparison();
                InstructionResult::Handled
            }
            Instruction::ToggleVerseVisibility => {
                self.toggle_verse_visibility();
                InstructionResult::Handled
            }

            // Navigation instructions that require current chapter
            Instruction::NextVerse => self.handle_next_verse(),
            Instruction::PreviousVerse => self.handle_previous_verse(),
            Instruction::NextChapter => self.handle_next_chapter(),
            Instruction::PreviousChapter => self.handle_previous_chapter(),
            Instruction::NextBook => self.handle_next_book(),
            Instruction::PreviousBook => self.handle_previous_book(),
            Instruction::BeginningOfChapter => self.handle_beginning_of_chapter(),
            Instruction::EndOfChapter => self.handle_end_of_chapter(),
            Instruction::GoToVerse(verse_num) => self.handle_go_to_verse(verse_num),

            // Selection instructions
            Instruction::ExtendSelectionNextVerse => self.handle_extend_selection_next_verse(),
            Instruction::ExtendSelectionPreviousVerse => {
                self.handle_extend_selection_previous_verse()
            }

            // Previous chapter navigation
            Instruction::SwitchToPreviousChapter => self.handle_switch_to_previous_chapter(),

            // Palette navigation
            Instruction::NextPaletteResult => {
                self.trigger_next_palette_result();
                InstructionResult::Handled
            }
            Instruction::PreviousPaletteResult => {
                self.trigger_previous_palette_result();
                InstructionResult::Handled
            }

            // Instructions that need external handling
            Instruction::CopyRawVerse
            | Instruction::CopyVerseWithReference
            | Instruction::ExportToPDF
            | Instruction::ExportToMarkdown
            | Instruction::ExportLinkedMarkdown
            | Instruction::OpenGithubRepository
            | Instruction::RandomVerse
            | Instruction::RandomChapter
            | Instruction::OpenAboutPage
            | Instruction::ShowTranslations => InstructionResult::NotHandled,

            // Other instructions
            _ => InstructionResult::NotHandled,
        }
    }

    /// Apply instruction with multiplier
    pub fn apply_instruction_with_multiplier(
        &mut self,
        instruction: Instruction,
        multiplier: u32,
    ) -> InstructionResult {
        match instruction {
            Instruction::NextVerse => self.handle_next_verse_with_multiplier(multiplier),
            Instruction::PreviousVerse => self.handle_previous_verse_with_multiplier(multiplier),
            Instruction::NextChapter => self.handle_next_chapter_with_multiplier(multiplier),
            Instruction::PreviousChapter => {
                self.handle_previous_chapter_with_multiplier(multiplier)
            }
            Instruction::NextBook => self.handle_next_book_with_multiplier(multiplier),
            Instruction::PreviousBook => self.handle_previous_book_with_multiplier(multiplier),
            _ => self.apply_instruction(instruction),
        }
    }
}
