use serde::{Deserialize, Serialize};
use std::fmt;

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

impl fmt::Display for BookName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use BookName::*;
        let name = match self {
            Genesis => "Genesis",
            Exodus => "Exodus",
            Leviticus => "Leviticus",
            Numbers => "Numbers",
            Deuteronomy => "Deuteronomy",
            Joshua => "Joshua",
            Judges => "Judges",
            Ruth => "Ruth",
            FirstSamuel => "FirstSamuel",
            SecondSamuel => "SecondSamuel",
            FirstKings => "FirstKings",
            SecondKings => "SecondKings",
            FirstChronicles => "FirstChronicles",
            SecondChronicles => "SecondChronicles",
            Ezra => "Ezra",
            Nehemiah => "Nehemiah",
            Tobit => "Tobit",
            Judith => "Judith",
            Esther => "Esther",
            AdditionsToEsther => "AdditionsToEsther",
            FirstMaccabees => "FirstMaccabees",
            SecondMaccabees => "SecondMaccabees",
            Job => "Job",
            Psalms => "Psalms",
            Proverbs => "Proverbs",
            Ecclesiastes => "Ecclesiastes",
            SongOfSongs => "SongOfSongs",
            Wisdom => "Wisdom",
            Sirach => "Sirach",
            Isaiah => "Isaiah",
            Jeremiah => "Jeremiah",
            Lamentations => "Lamentations",
            Baruch => "Baruch",
            LetterOfJeremiah => "LetterOfJeremiah",
            Ezekiel => "Ezekiel",
            Daniel => "Daniel",
            PrayerOfAzariah => "PrayerOfAzariah",
            Susanna => "Susanna",
            BelAndTheDragon => "BelAndTheDragon",
            Hosea => "Hosea",
            Joel => "Joel",
            Amos => "Amos",
            Obadiah => "Obadiah",
            Jonah => "Jonah",
            Micah => "Micah",
            Nahum => "Nahum",
            Habakkuk => "Habakkuk",
            Zephaniah => "Zephaniah",
            Haggai => "Haggai",
            Zechariah => "Zechariah",
            Malachi => "Malachi",
            Matthew => "Matthew",
            Mark => "Mark",
            Luke => "Luke",
            John => "John",
            Acts => "Acts",
            Romans => "Romans",
            FirstCorinthians => "FirstCorinthians",
            SecondCorinthians => "SecondCorinthians",
            Galatians => "Galatians",
            Ephesians => "Ephesians",
            Philippians => "Philippians",
            Colossians => "Colossians",
            FirstThessalonians => "FirstThessalonians",
            SecondThessalonians => "SecondThessalonians",
            FirstTimothy => "FirstTimothy",
            SecondTimothy => "SecondTimothy",
            Titus => "Titus",
            Philemon => "Philemon",
            Hebrews => "Hebrews",
            James => "James",
            FirstPeter => "FirstPeter",
            SecondPeter => "SecondPeter",
            FirstJohn => "FirstJohn",
            SecondJohn => "SecondJohn",
            ThirdJohn => "ThirdJohn",
            Jude => "Jude",
            Revelation => "Revelation",
        };
        write!(f, "{}", name)
    }
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

        match value.trim().to_lowercase().as_str() {
            // Old Testament - Torah
            "genesis" => Ok(Genesis),
            "exodus" => Ok(Exodus),
            "leviticus" => Ok(Leviticus),
            "numbers" => Ok(Numbers),
            "deuteronomy" => Ok(Deuteronomy),

            // Historical books
            "joshua" => Ok(Joshua),
            "judges" => Ok(Judges),
            "ruth" => Ok(Ruth),
            "1samuel" | "i samuel" | "firstsamuel" => Ok(FirstSamuel),
            "2samuel" | "ii samuel" | "secondsamuel" => Ok(SecondSamuel),
            "1kings" | "i kings" | "firstkings" => Ok(FirstKings),
            "2kings" | "ii kings" | "secondkings" => Ok(SecondKings),
            "1chronicles" | "i chronicles" | "firstchronicles" => Ok(FirstChronicles),
            "2chronicles" | "ii chronicles" | "secondchronicles" => Ok(SecondChronicles),
            "ezra" => Ok(Ezra),
            "nehemiah" => Ok(Nehemiah),

            // Catholic deuterocanonical historical additions
            "tobit" => Ok(Tobit),
            "judith" => Ok(Judith),
            "esther" => Ok(Esther),
            "additionstoesther" => Ok(AdditionsToEsther),
            "1maccabees" | "i maccabees" | "firstmaccabees" => Ok(FirstMaccabees),
            "2maccabees" | "ii maccabees" | "secondmaccabees" => Ok(SecondMaccabees),

            // Wisdom / Poetry
            "job" => Ok(Job),
            "psalms" => Ok(Psalms),
            "proverbs" => Ok(Proverbs),
            "ecclesiastes" => Ok(Ecclesiastes),
            "songofsongs" | "song of solomon" | "song of songs" => Ok(SongOfSongs),
            "wisdom" => Ok(Wisdom),
            "sirach" => Ok(Sirach),

            // Major Prophets
            "isaiah" => Ok(Isaiah),
            "jeremiah" => Ok(Jeremiah),
            "lamentations" => Ok(Lamentations),
            "baruch" => Ok(Baruch),
            "letterofjeremiah" => Ok(LetterOfJeremiah),
            "ezekiel" => Ok(Ezekiel),
            "daniel" => Ok(Daniel),
            "prayerofazariah" => Ok(PrayerOfAzariah),
            "susanna" => Ok(Susanna),
            "belandthedragon" => Ok(BelAndTheDragon),

            // Minor Prophets
            "hosea" => Ok(Hosea),
            "joel" => Ok(Joel),
            "amos" => Ok(Amos),
            "obadiah" => Ok(Obadiah),
            "jonah" => Ok(Jonah),
            "micah" => Ok(Micah),
            "nahum" => Ok(Nahum),
            "habakkuk" => Ok(Habakkuk),
            "zephaniah" => Ok(Zephaniah),
            "haggai" => Ok(Haggai),
            "zechariah" => Ok(Zechariah),
            "malachi" => Ok(Malachi),

            // New Testament - Gospels
            "matthew" => Ok(Matthew),
            "mark" => Ok(Mark),
            "luke" => Ok(Luke),
            "john" => Ok(John),

            // History
            "acts" => Ok(Acts),

            // Pauline Epistles
            "romans" => Ok(Romans),
            "1corinthians" | "i corinthians" | "firstcorinthians" => Ok(FirstCorinthians),
            "2corinthians" | "ii corinthians" | "secondcorinthians" => Ok(SecondCorinthians),
            "galatians" => Ok(Galatians),
            "ephesians" => Ok(Ephesians),
            "philippians" => Ok(Philippians),
            "colossians" => Ok(Colossians),
            "1thessalonians" | "i thessalonians" | "firstthessalonians" => Ok(FirstThessalonians),
            "2thessalonians" | "ii thessalonians" | "secondthessalonians" => {
                Ok(SecondThessalonians)
            }
            "1timothy" | "i timothy" | "firsttimothy" => Ok(FirstTimothy),
            "2timothy" | "ii timothy" | "secondtimothy" => Ok(SecondTimothy),
            "titus" => Ok(Titus),
            "philemon" => Ok(Philemon),

            // General Epistles
            "hebrews" => Ok(Hebrews),
            "james" => Ok(James),
            "1peter" | "i peter" | "firstpeter" => Ok(FirstPeter),
            "2peter" | "ii peter" | "secondpeter" => Ok(SecondPeter),
            "1john" | "i john" | "firstjohn" => Ok(FirstJohn),
            "2john" | "ii john" | "secondjohn" => Ok(SecondJohn),
            "3john" | "iii john" | "thirdjohn" => Ok(ThirdJohn),
            "jude" => Ok(Jude),

            // Prophecy
            "revelation" | "revelation of john" => Ok(Revelation),

            // fallback
            other => Err(BookNameParseError::UnknownName(other.to_string())),
        }
    }
}
