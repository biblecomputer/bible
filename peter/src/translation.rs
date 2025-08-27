use crate::language::Language;

struct Translation {
    meta: TranslationMetaData,
    old_testament: Option<OldTestament>,
    new_testament: Option<NewTestament>,
    deuterocanonical: Option<Deuterocanonical>,
}

/// More used in Catholic and Orthodox
struct Deuterocanonical {
    tobit: Book,
    judith: Book,
    wisdom: Book, // Wisdom of Solomon
    sirach: Book, // Ecclesiasticus
    baruch: Book,
    letter_of_jeremiah: Book, // sometimes counted as part of Baruch
    first_maccabees: Book,
    second_maccabees: Book,

    // Additions
    additions_to_esther: Book,
    prayer_of_azariah: Book, // also called Song of the Three Holy Children
    susanna: Book,
    bel_and_the_dragon: Book,
}

struct OldTestament {
    genesis: Book,
    exodus: Book,
    leviticus: Book,
    numbers: Book,
    deuteronomy: Book,

    joshua: Book,
    judges: Book,
    ruth: Book,
    first_samuel: Book,
    second_samuel: Book,
    first_kings: Book,
    second_kings: Book,
    first_chronicles: Book,
    second_chronicles: Book,
    ezra: Book,
    nehemiah: Book,
    esther: Book,

    job: Book,
    psalms: Book,
    proverbs: Book,
    ecclesiastes: Book,
    song_of_songs: Book,

    isaiah: Book,
    jeremiah: Book,
    lamentations: Book,
    ezekiel: Book,
    daniel: Book,

    hosea: Book,
    joel: Book,
    amos: Book,
    obadiah: Book,
    jonah: Book,
    micah: Book,
    nahum: Book,
    habakkuk: Book,
    zephaniah: Book,
    haggai: Book,
    zechariah: Book,
    malachi: Book,
}

struct NewTestament {
    // Gospels
    matthew: Book,
    mark: Book,
    luke: Book,
    john: Book,
    acts: Book,
    romans: Book,
    first_corinthians: Book,
    second_corinthians: Book,
    galatians: Book,
    ephesians: Book,
    philippians: Book,
    colossians: Book,
    first_thessalonians: Book,
    second_thessalonians: Book,
    first_timothy: Book,
    second_timothy: Book,
    titus: Book,
    philemon: Book,
    hebrews: Book,
    james: Book,
    first_peter: Book,
    second_peter: Book,
    first_john: Book,
    second_john: Book,
    third_john: Book,
    jude: Book,
    revelation: Book,
}

struct TranslationMetaData {
    name: String,
    description: String,
    link: Url,
    release: Year,
    languages: Vec<Language>,
    equivalence_level: EquivalenceLevel,
    /// Describes the organisation or person who funded it.
    funded_by: String,
}

struct Url(String);

/// describes equivulance of a translation
/// 0 means extreamly formal - word for word
/// 255 means extreamly functional - meaning
struct EquivalenceLevel(u8);

struct Year(u16);

struct Book {
    name: String,
    chapters: Vec<Chapter>,
}

struct Chapter {
    id: u32,
    verses: Vec<Verse>,
}

struct Verse {
    verse_id: VerseID,
    content: String,
    footnotes: Option<String>,
}

enum VerseID {
    Single(u32),
    // right should be greater then left.
    Range(u32, u32),
}
