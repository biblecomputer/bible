import { Data, Option, Schema } from "effect";

// BibleBook Schema using Effect Schema
export const BibleBookSchema = Schema.Literal(
    // Old Testament
    "Genesis", "Exodus", "Leviticus", "Numbers", "Deuteronomy", "Joshua", "Judges", "Ruth",
    "FirstSamuel", "SecondSamuel", "FirstKings", "SecondKings", "FirstChronicles", "SecondChronicles",
    "Ezra", "Nehemiah", "Esther", "Job", "Psalms", "Proverbs", "Ecclesiastes", "SongOfSongs",
    "Isaiah", "Jeremiah", "Lamentations", "Ezekiel", "Daniel", "Hosea", "Joel", "Amos",
    "Obadiah", "Jonah", "Micah", "Nahum", "Habakkuk", "Zephaniah", "Haggai", "Zechariah", "Malachi",
    // New Testament
    "Matthew", "Mark", "Luke", "John", "Acts", "Romans", "FirstCorinthians", "SecondCorinthians",
    "Galatians", "Ephesians", "Philippians", "Colossians", "FirstThessalonians", "SecondThessalonians",
    "FirstTimothy", "SecondTimothy", "Titus", "Philemon", "Hebrews", "James", "FirstPeter",
    "SecondPeter", "FirstJohn", "SecondJohn", "ThirdJohn", "Jude", "Revelation"
);

export type BibleBook = Schema.Schema.Type<typeof BibleBookSchema>;

export const BibleBook = {
    // Old Testament
    Genesis: "Genesis" as const,
    Exodus: "Exodus" as const,
    Leviticus: "Leviticus" as const,
    Numbers: "Numbers" as const,
    Deuteronomy: "Deuteronomy" as const,
    Joshua: "Joshua" as const,
    Judges: "Judges" as const,
    Ruth: "Ruth" as const,
    FirstSamuel: "FirstSamuel" as const,
    SecondSamuel: "SecondSamuel" as const,
    FirstKings: "FirstKings" as const,
    SecondKings: "SecondKings" as const,
    FirstChronicles: "FirstChronicles" as const,
    SecondChronicles: "SecondChronicles" as const,
    Ezra: "Ezra" as const,
    Nehemiah: "Nehemiah" as const,
    Esther: "Esther" as const,
    Job: "Job" as const,
    Psalms: "Psalms" as const,
    Proverbs: "Proverbs" as const,
    Ecclesiastes: "Ecclesiastes" as const,
    SongOfSongs: "SongOfSongs" as const,
    Isaiah: "Isaiah" as const,
    Jeremiah: "Jeremiah" as const,
    Lamentations: "Lamentations" as const,
    Ezekiel: "Ezekiel" as const,
    Daniel: "Daniel" as const,
    Hosea: "Hosea" as const,
    Joel: "Joel" as const,
    Amos: "Amos" as const,
    Obadiah: "Obadiah" as const,
    Jonah: "Jonah" as const,
    Micah: "Micah" as const,
    Nahum: "Nahum" as const,
    Habakkuk: "Habakkuk" as const,
    Zephaniah: "Zephaniah" as const,
    Haggai: "Haggai" as const,
    Zechariah: "Zechariah" as const,
    Malachi: "Malachi" as const,

    // New Testament
    Matthew: "Matthew" as const,
    Mark: "Mark" as const,
    Luke: "Luke" as const,
    John: "John" as const,
    Acts: "Acts" as const,
    Romans: "Romans" as const,
    FirstCorinthians: "FirstCorinthians" as const,
    SecondCorinthians: "SecondCorinthians" as const,
    Galatians: "Galatians" as const,
    Ephesians: "Ephesians" as const,
    Philippians: "Philippians" as const,
    Colossians: "Colossians" as const,
    FirstThessalonians: "FirstThessalonians" as const,
    SecondThessalonians: "SecondThessalonians" as const,
    FirstTimothy: "FirstTimothy" as const,
    SecondTimothy: "SecondTimothy" as const,
    Titus: "Titus" as const,
    Philemon: "Philemon" as const,
    Hebrews: "Hebrews" as const,
    James: "James" as const,
    FirstPeter: "FirstPeter" as const,
    SecondPeter: "SecondPeter" as const,
    FirstJohn: "FirstJohn" as const,
    SecondJohn: "SecondJohn" as const,
    ThirdJohn: "ThirdJohn" as const,
    Jude: "Jude" as const,
    Revelation: "Revelation" as const
} as const;

// https://guide.unwsp.edu/SBL_guide/abbreviations/bible
const BOOK_TAG_TO_SHORT_NAME: Record<BibleBook, string> = {
    // Old Testament - Using SBL standard abbreviations (lowercase for URLs)
    Genesis: "gen",
    Exodus: "exod",
    Leviticus: "lev",
    Numbers: "num",
    Deuteronomy: "deut",
    Joshua: "josh",
    Judges: "judg",
    Ruth: "ruth",
    FirstSamuel: "1sam",
    SecondSamuel: "2sam",
    FirstKings: "1kgs",
    SecondKings: "2kgs",
    FirstChronicles: "1chr",
    SecondChronicles: "2chr",
    Ezra: "ezra",
    Nehemiah: "neh",
    Esther: "esth",
    Job: "job",
    Psalms: "ps",
    Proverbs: "prov",
    Ecclesiastes: "eccl",
    SongOfSongs: "song",
    Isaiah: "isa",
    Jeremiah: "jer",
    Lamentations: "lam",
    Ezekiel: "ezek",
    Daniel: "dan",
    Hosea: "hos",
    Joel: "joel",
    Amos: "amos",
    Obadiah: "obad",
    Jonah: "jonah",
    Micah: "mic",
    Nahum: "nah",
    Habakkuk: "hab",
    Zephaniah: "zeph",
    Haggai: "hag",
    Zechariah: "zech",
    Malachi: "mal",

    // New Testament - Using SBL standard abbreviations (lowercase for URLs)
    Matthew: "matt",
    Mark: "mark",
    Luke: "luke",
    John: "john",
    Acts: "acts",
    Romans: "rom",
    FirstCorinthians: "1cor",
    SecondCorinthians: "2cor",
    Galatians: "gal",
    Ephesians: "eph",
    Philippians: "phil",
    Colossians: "col",
    FirstThessalonians: "1thess",
    SecondThessalonians: "2thess",
    FirstTimothy: "1tim",
    SecondTimothy: "2tim",
    Titus: "titus",
    Philemon: "phlm",
    Hebrews: "heb",
    James: "jas",
    FirstPeter: "1pet",
    SecondPeter: "2pet",
    FirstJohn: "1john",
    SecondJohn: "2john",
    ThirdJohn: "3john",
    Jude: "jude",
    Revelation: "rev"
};

const SHORT_NAME_TO_BOOK_TAG: Record<string, BibleBook> = Object.fromEntries(
    Object.entries(BOOK_TAG_TO_SHORT_NAME).map(([tag, shortName]) => [shortName, tag as BibleBook])
);

export const toBibleBook = (value: string): Option.Option<BibleBook> => {
    const bookTag = SHORT_NAME_TO_BOOK_TAG[value];
    if (!bookTag) {
        return Option.none();
    }
    return Option.some(bookTag);
};

export const getShortName = (book: BibleBook): string => {
    return BOOK_TAG_TO_SHORT_NAME[book];
};

// Short names sorted longest-first so a book like "1john" is matched before "john"
const BOOK_SHORT_NAMES_BY_LENGTH = Object.keys(SHORT_NAME_TO_BOOK_TAG).sort(
    (a, b) => b.length - a.length
);

/**
 * Match a book abbreviation at the start of a string (case-insensitive),
 * e.g. "john1v1" -> { book: "John", rest: "1v1" }.
 * Used to split URL segments like "john1v1" without a separator between
 * the book and the chapter.
 */
export const matchBookPrefix = (value: string): { book: BibleBook; rest: string } | null => {
    const lower = value.toLowerCase();
    for (const shortName of BOOK_SHORT_NAMES_BY_LENGTH) {
        if (lower.startsWith(shortName)) {
            return { book: SHORT_NAME_TO_BOOK_TAG[shortName], rest: value.slice(shortName.length) };
        }
    }
    return null;
};

const BOOK_TAG_TO_DISPLAY_NAME: Record<BibleBook, string> = {
    // Old Testament
    Genesis: "Genesis",
    Exodus: "Exodus",
    Leviticus: "Leviticus",
    Numbers: "Numbers",
    Deuteronomy: "Deuteronomy",
    Joshua: "Joshua",
    Judges: "Judges",
    Ruth: "Ruth",
    FirstSamuel: "1 Samuel",
    SecondSamuel: "2 Samuel",
    FirstKings: "1 Kings",
    SecondKings: "2 Kings",
    FirstChronicles: "1 Chronicles",
    SecondChronicles: "2 Chronicles",
    Ezra: "Ezra",
    Nehemiah: "Nehemiah",
    Esther: "Esther",
    Job: "Job",
    Psalms: "Psalms",
    Proverbs: "Proverbs",
    Ecclesiastes: "Ecclesiastes",
    SongOfSongs: "Song of Songs",
    Isaiah: "Isaiah",
    Jeremiah: "Jeremiah",
    Lamentations: "Lamentations",
    Ezekiel: "Ezekiel",
    Daniel: "Daniel",
    Hosea: "Hosea",
    Joel: "Joel",
    Amos: "Amos",
    Obadiah: "Obadiah",
    Jonah: "Jonah",
    Micah: "Micah",
    Nahum: "Nahum",
    Habakkuk: "Habakkuk",
    Zephaniah: "Zephaniah",
    Haggai: "Haggai",
    Zechariah: "Zechariah",
    Malachi: "Malachi",

    // New Testament
    Matthew: "Matthew",
    Mark: "Mark",
    Luke: "Luke",
    John: "John",
    Acts: "Acts",
    Romans: "Romans",
    FirstCorinthians: "1 Corinthians",
    SecondCorinthians: "2 Corinthians",
    Galatians: "Galatians",
    Ephesians: "Ephesians",
    Philippians: "Philippians",
    Colossians: "Colossians",
    FirstThessalonians: "1 Thessalonians",
    SecondThessalonians: "2 Thessalonians",
    FirstTimothy: "1 Timothy",
    SecondTimothy: "2 Timothy",
    Titus: "Titus",
    Philemon: "Philemon",
    Hebrews: "Hebrews",
    James: "James",
    FirstPeter: "1 Peter",
    SecondPeter: "2 Peter",
    FirstJohn: "1 John",
    SecondJohn: "2 John",
    ThirdJohn: "3 John",
    Jude: "Jude",
    Revelation: "Revelation"
};

export const getDisplayName = (book: BibleBook): string => {
    return BOOK_TAG_TO_DISPLAY_NAME[book];
};