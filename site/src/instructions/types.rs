/*!
 * Instruction Types
 *
 * This module defines all user instructions that can be performed
 * in the Bible application. Instructions are triggered by keyboard
 * shortcuts, command palette, or programmatic actions.
 */

use crate::core::types::VerseId;

/// Enum representing all possible user instructions
///
/// Instructions are organized by category to improve maintainability.
/// Each instruction should have a corresponding handler in the processor.
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    // === Navigation Instructions ===
    // Basic movement through Bible content
    NextVerse,
    PreviousVerse,
    NextChapter,
    PreviousChapter,
    NextBook,
    PreviousBook,
    NextReference,
    PreviousReference,
    NextPaletteResult,
    PreviousPaletteResult,

    // === Range Selection Instructions ===
    // For selecting multiple verses at once
    ExtendSelectionNextVerse,
    ExtendSelectionPreviousVerse,

    // === Chapter/Verse Jumping Instructions ===
    // Direct navigation to specific locations
    BeginningOfChapter,
    EndOfChapter,
    /// Navigate to a specific verse number
    GoToVerse(VerseId),
    /// Navigate to a specific chapter (handles all side effects like mobile sidebar closing)
    GoToChapter(String), // chapter path

    // === Special Navigation Instructions ===
    // Advanced navigation features
    SwitchToPreviousChapter,

    // === Copy Operations Instructions ===
    // Text copying functionality
    CopyRawVerse,
    CopyVerseWithReference,

    // === UI Toggle Instructions ===
    // Interface visibility controls
    ToggleSidebar,
    ToggleCrossReferences,
    ToggleThemeSidebar,
    ToggleBiblePallate,
    ToggleCommandPallate,
    ToggleTranslationComparison, // Added: Toggle translation comparison panel
    ToggleVerseVisibility,
    ToggleVersePallate,
    
    // === UI Close Instructions ===
    // Direct close actions for specific UI elements
    CloseCommandPalette,
    CloseLeftSidebar,
    CloseRightSidebar,
    CloseThemeSidebar,
    CloseTranslationComparison,
    
    // === Book Selection Instructions ===
    // Book navigation and selection
    SelectBook(String),
    ClearSelectedBook,

    // === External Actions ===
    // Actions that interact with external systems
    OpenGithubRepository,

    // === Random Navigation ===
    // Serendipitous discovery features
    RandomVerse,
    RandomChapter,

    // === Information & Settings ===
    // Application information and configuration
    OpenAboutPage,
    ShowTranslations,

    // === Export Instructions ===
    // Data export functionality
    ExportToPDF,
    ExportToMarkdown,
    ExportLinkedMarkdown,
}
