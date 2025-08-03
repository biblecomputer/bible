#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    // Navigation
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
    ToggleBiblePallate,
    ToggleCommandPallate,

    OpenGithubRepository,

    RandomVerse,
    OpenAboutPage,
}
