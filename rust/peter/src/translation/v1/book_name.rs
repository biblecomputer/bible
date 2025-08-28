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

#[derive(Debug, thiserror::Error)]
pub enum BookNameParseError {
    #[error("Unknown book name: {0}")]
    UnknownName(String),
}

impl TryFrom<&str> for BookName {
    type Error = BookNameParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use BookName::*;

        match value.trim() {
            // Old Testament - Torah
            "Genesis" => Ok(Genesis),
            "Exodus" => Ok(Exodus),
            "Leviticus" => Ok(Leviticus),
            "Numbers" => Ok(Numbers),
            "Deuteronomy" => Ok(Deuteronomy),

            // Historical books
            "Joshua" => Ok(Joshua),
            "Judges" => Ok(Judges),
            "Ruth" => Ok(Ruth),
            "1Samuel" | "I Samuel" | "FirstSamuel" => Ok(FirstSamuel),
            "2Samuel" | "II Samuel" | "SecondSamuel" => Ok(SecondSamuel),
            "1Kings" | "I Kings" | "FirstKings" => Ok(FirstKings),
            "2Kings" | "II Kings" | "SecondKings" => Ok(SecondKings),
            "1Chronicles" | "I Chronicles" | "FirstChronicles" => Ok(FirstChronicles),
            "2Chronicles" | "II Chronicles" | "SecondChronicles" => Ok(SecondChronicles),
            "Ezra" => Ok(Ezra),
            "Nehemiah" => Ok(Nehemiah),

            // Catholic deuterocanonical historical additions
            "Tobit" => Ok(Tobit),
            "Judith" => Ok(Judith),
            "Esther" => Ok(Esther),
            "AdditionsToEsther" => Ok(AdditionsToEsther),
            "1Maccabees" | "I Maccabees" | "FirstMaccabees" => Ok(FirstMaccabees),
            "2Maccabees" | "II Maccabees" | "SecondMaccabees" => Ok(SecondMaccabees),

            // Wisdom / Poetry
            "Job" => Ok(Job),
            "Psalms" => Ok(Psalms),
            "Proverbs" => Ok(Proverbs),
            "Ecclesiastes" => Ok(Ecclesiastes),
            "SongOfSongs" => Ok(SongOfSongs),
            "Wisdom" => Ok(Wisdom),
            "Sirach" => Ok(Sirach),

            // Major Prophets
            "Isaiah" => Ok(Isaiah),
            "Jeremiah" => Ok(Jeremiah),
            "Lamentations" => Ok(Lamentations),
            "Baruch" => Ok(Baruch),
            "LetterOfJeremiah" => Ok(LetterOfJeremiah),
            "Ezekiel" => Ok(Ezekiel),
            "Daniel" => Ok(Daniel),
            "PrayerOfAzariah" => Ok(PrayerOfAzariah),
            "Susanna" => Ok(Susanna),
            "BelAndTheDragon" => Ok(BelAndTheDragon),

            // Minor Prophets
            "Hosea" => Ok(Hosea),
            "Joel" => Ok(Joel),
            "Amos" => Ok(Amos),
            "Obadiah" => Ok(Obadiah),
            "Jonah" => Ok(Jonah),
            "Micah" => Ok(Micah),
            "Nahum" => Ok(Nahum),
            "Habakkuk" => Ok(Habakkuk),
            "Zephaniah" => Ok(Zephaniah),
            "Haggai" => Ok(Haggai),
            "Zechariah" => Ok(Zechariah),
            "Malachi" => Ok(Malachi),

            // New Testament - Gospels
            "Matthew" => Ok(Matthew),
            "Mark" => Ok(Mark),
            "Luke" => Ok(Luke),
            "John" => Ok(John),

            // History
            "Acts" => Ok(Acts),

            // Pauline Epistles
            "Romans" => Ok(Romans),
            "1Corinthians" | "I Corinthians" | "FirstCorinthians" => Ok(FirstCorinthians),
            "2Corinthians" | "II Corinthians" | "SecondCorinthians" => Ok(SecondCorinthians),
            "Galatians" => Ok(Galatians),
            "Ephesians" => Ok(Ephesians),
            "Philippians" => Ok(Philippians),
            "Colossians" => Ok(Colossians),
            "1Thessalonians" | "I Thessalonians" | "FirstThessalonians" => Ok(FirstThessalonians),
            "2Thessalonians" | "II Thessalonians" | "SecondThessalonians" => {
                Ok(SecondThessalonians)
            }
            "1Timothy" | "I Timothy" | "FirstTimothy" => Ok(FirstTimothy),
            "2Timothy" | "II Timothy" | "SecondTimothy" => Ok(SecondTimothy),
            "Titus" => Ok(Titus),
            "Philemon" => Ok(Philemon),

            // General Epistles
            "Hebrews" => Ok(Hebrews),
            "James" => Ok(James),
            "1Peter" | "I Peter" | "FirstPeter" => Ok(FirstPeter),
            "2Peter" | "II Peter" | "SecondPeter" => Ok(SecondPeter),
            "1John" | "I John" | "FirstJohn" => Ok(FirstJohn),
            "2John" | "II John" | "SecondJohn" => Ok(SecondJohn),
            "3John" | "III John" | "ThirdJohn" => Ok(ThirdJohn),
            "Jude" => Ok(Jude),

            // Prophecy
            "Revelation" => Ok(Revelation),

            // fallback
            other => Err(BookNameParseError::UnknownName(other.to_string())),
        }
    }
}
