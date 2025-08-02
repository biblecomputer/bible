#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    // Navigation
    NextVerse,
    PreviousVerse,
    NextChapter,
    PreviousChapter,
    NextBook,
    PreviousBook,
    NextInList,
    PreviousInList,

    // Chapter/Verse jumping
    BeginningOfChapter,
    EndOfChapter,
    GoToVerse(u32),

    // Special navigation
    SwitchToPreviousChapter,

    // Copy operations
    CopyRawVerse,
    CopyVerseWithReference,

    // UI toggles
    ToggleSidebar,
    ToggleCrossReferences,
    OpenCommandPalette,
}
