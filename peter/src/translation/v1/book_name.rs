use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum BookName {
    // Old Testament - Torah
    Genesis,
    Exodus,
    Leviticus,
    Numbers,
    Deuteronomy,

    // Historical books
    Joshua,
    Judges,
    Ruth,
    FirstSamuel,
    SecondSamuel,
    FirstKings,
    SecondKings,
    FirstChronicles,
    SecondChronicles,
    Ezra,
    Nehemiah,

    // Catholic deuterocanonical historical additions
    Tobit,  // deuterocanonical
    Judith, // deuterocanonical

    Esther,
    AdditionsToEsther, // deuterocanonical (additions/chapters in some editions)

    FirstMaccabees,  // deuterocanonical
    SecondMaccabees, // deuterocanonical

    // Wisdom / Poetry
    Job,
    Psalms,
    Proverbs,
    Ecclesiastes,
    SongOfSongs,
    Wisdom, // Wisdom of Solomon (deuterocanonical)
    Sirach, // Ecclesiasticus (deuterocanonical)

    // Major Prophets
    Isaiah,
    Jeremiah,
    Lamentations,
    Baruch,           // deuterocanonical (often with Letter of Jeremiah)
    LetterOfJeremiah, // sometimes treated as part of Baruch

    Ezekiel,
    Daniel,
    // Daniel additions (deuterocanonical)
    PrayerOfAzariah, // "Song of the Three Holy Children" / Prayer of Azariah
    Susanna,
    BelAndTheDragon,

    // Minor Prophets
    Hosea,
    Joel,
    Amos,
    Obadiah,
    Jonah,
    Micah,
    Nahum,
    Habakkuk,
    Zephaniah,
    Haggai,
    Zechariah,
    Malachi,

    // New Testament - Gospels
    Matthew,
    Mark,
    Luke,
    John,

    // History
    Acts,

    // Pauline Epistles
    Romans,
    FirstCorinthians,
    SecondCorinthians,
    Galatians,
    Ephesians,
    Philippians,
    Colossians,
    FirstThessalonians,
    SecondThessalonians,
    FirstTimothy,
    SecondTimothy,
    Titus,
    Philemon,

    // General Epistles
    Hebrews,
    James,
    FirstPeter,
    SecondPeter,
    FirstJohn,
    SecondJohn,
    ThirdJohn,
    Jude,

    // Prophecy
    Revelation,
}
