use leptos::prelude::*;
use crate::storage::{get_sidebar_open, save_sidebar_open, get_references_sidebar_open, save_references_sidebar_open, get_verse_visibility, save_verse_visibility};
use crate::core::{Chapter, VerseRange};

/// Central state management for all UI view states
/// This replaces multiple individual signals with a single, cohesive state structure
#[derive(Debug, Clone)]
pub struct ViewState {
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
}

impl Default for ViewState {
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
        }
    }
}

impl ViewState {
    /// Create a new ViewState with default values
    pub fn new() -> Self {
        Self::default()
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
    
    /// Close all sidebars and panels
    pub fn close_all_sidebars(&mut self) {
        self.set_left_sidebar(false);
        self.set_right_sidebar(false);
        self.set_theme_sidebar(false);
        self.set_translation_comparison(false);
    }
    
    /// Close all overlays (useful for mobile)
    pub fn close_all_overlays(&mut self) {
        self.set_command_palette(false);
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
}

/// Leptos signal wrapper for ViewState
pub type ViewStateSignal = RwSignal<ViewState>;

/// Create a new ViewState signal with default values
pub fn create_view_state() -> ViewStateSignal {
    RwSignal::new(ViewState::new())
}

/// Helper functions for common view state operations
pub trait ViewStateSignalExt {
    /// Update the view state with a closure
    fn update_state<F>(&self, f: F) 
    where 
        F: FnOnce(&mut ViewState);
    
    /// Get a specific boolean state value
    fn get_bool<F>(&self, f: F) -> bool 
    where 
        F: Fn(&ViewState) -> bool;
}

impl ViewStateSignalExt for ViewStateSignal {
    fn update_state<F>(&self, f: F) 
    where 
        F: FnOnce(&mut ViewState)
    {
        self.update(f);
    }
    
    fn get_bool<F>(&self, f: F) -> bool 
    where 
        F: Fn(&ViewState) -> bool
    {
        self.with(f)
    }
}