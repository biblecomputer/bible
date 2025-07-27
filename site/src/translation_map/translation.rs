use serde::{Deserialize, Serialize};
use crate::core::types::Language;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Translation<'a> {
    // Old Testament
    pub genesis: &'a str,
    pub exodus: &'a str,
    pub leviticus: &'a str,
    pub numbers: &'a str,
    pub deuteronomy: &'a str,
    pub joshua: &'a str,
    pub judges: &'a str,
    pub ruth: &'a str,
    pub first_samuel: &'a str,
    pub second_samuel: &'a str,
    pub first_kings: &'a str,
    pub second_kings: &'a str,
    pub first_chronicles: &'a str,
    pub second_chronicles: &'a str,
    pub ezra: &'a str,
    pub nehemiah: &'a str,
    pub esther: &'a str,
    pub job: &'a str,
    pub psalms: &'a str,
    pub proverbs: &'a str,
    pub ecclesiastes: &'a str,
    pub song_of_solomon: &'a str,
    pub isaiah: &'a str,
    pub jeremiah: &'a str,
    pub lamentations: &'a str,
    pub ezekiel: &'a str,
    pub daniel: &'a str,
    pub hosea: &'a str,
    pub joel: &'a str,
    pub amos: &'a str,
    pub obadiah: &'a str,
    pub jonah: &'a str,
    pub micah: &'a str,
    pub nahum: &'a str,
    pub habakkuk: &'a str,
    pub zephaniah: &'a str,
    pub haggai: &'a str,
    pub zechariah: &'a str,
    pub malachi: &'a str,
    // New Testament
    pub matthew: &'a str,
    pub mark: &'a str,
    pub luke: &'a str,
    pub john: &'a str,
    pub acts: &'a str,
    pub romans: &'a str,
    pub first_corinthians: &'a str,
    pub second_corinthians: &'a str,
    pub galatians: &'a str,
    pub ephesians: &'a str,
    pub philippians: &'a str,
    pub colossians: &'a str,
    pub first_thessalonians: &'a str,
    pub second_thessalonians: &'a str,
    pub first_timothy: &'a str,
    pub second_timothy: &'a str,
    pub titus: &'a str,
    pub philemon: &'a str,
    pub hebrews: &'a str,
    pub james: &'a str,
    pub first_peter: &'a str,
    pub second_peter: &'a str,
    pub first_john: &'a str,
    pub second_john: &'a str,
    pub third_john: &'a str,
    pub jude: &'a str,
    pub revelation: &'a str,
}

const DUTCH_JSON: &str = include_str!("dutch.json");
const ENGLISH_JSON: &str = include_str!("english.json");

use std::sync::LazyLock;

static DUTCH_TRANSLATION: LazyLock<Translation<'static>> = LazyLock::new(|| {
    serde_json::from_str(DUTCH_JSON).expect("Failed to parse dutch.json")
});

static ENGLISH_TRANSLATION: LazyLock<Translation<'static>> = LazyLock::new(|| {
    serde_json::from_str(ENGLISH_JSON).expect("Failed to parse english.json")
});

impl<'a> Translation<'a> {
    pub fn from_language(language: Language) -> &'static Translation<'static> {
        match language {
            Language::Dutch => &DUTCH_TRANSLATION,
            Language::English => &ENGLISH_TRANSLATION,
        }
    }

    pub fn get_book(&self, s: &str) -> Option<&'a str> {
        match s {
            "genesis" => Some(self.genesis),
            "exodus" => Some(self.exodus),
            "leviticus" => Some(self.leviticus),
            "numbers" => Some(self.numbers),
            "deuteronomy" => Some(self.deuteronomy),
            "joshua" => Some(self.joshua),
            "judges" => Some(self.judges),
            "ruth" => Some(self.ruth),
            "first_samuel" | "1_samuel" | "1samuel" | "i_samuel" | "isamuel" => Some(self.first_samuel),
            "second_samuel" | "2_samuel" | "2samuel" | "ii_samuel" | "iisamuel" => Some(self.second_samuel),
            "first_kings" | "1_kings" | "1kings" | "i_kings" | "ikings" => Some(self.first_kings),
            "second_kings" | "2_kings" | "2kings" | "ii_kings" | "iikings" => Some(self.second_kings),
            "first_chronicles" | "1_chronicles" | "1chronicles" | "i_chronicles" | "ichronicles" => Some(self.first_chronicles),
            "second_chronicles" | "2_chronicles" | "2chronicles" | "ii_chronicles" | "iichronicles" => Some(self.second_chronicles),
            "ezra" => Some(self.ezra),
            "nehemiah" => Some(self.nehemiah),
            "esther" => Some(self.esther),
            "job" => Some(self.job),
            "psalms" => Some(self.psalms),
            "proverbs" => Some(self.proverbs),
            "ecclesiastes" => Some(self.ecclesiastes),
            "song_of_solomon" => Some(self.song_of_solomon),
            "isaiah" => Some(self.isaiah),
            "jeremiah" => Some(self.jeremiah),
            "lamentations" => Some(self.lamentations),
            "ezekiel" => Some(self.ezekiel),
            "daniel" => Some(self.daniel),
            "hosea" => Some(self.hosea),
            "joel" => Some(self.joel),
            "amos" => Some(self.amos),
            "obadiah" => Some(self.obadiah),
            "jonah" => Some(self.jonah),
            "micah" => Some(self.micah),
            "nahum" => Some(self.nahum),
            "habakkuk" => Some(self.habakkuk),
            "zephaniah" => Some(self.zephaniah),
            "haggai" => Some(self.haggai),
            "zechariah" => Some(self.zechariah),
            "malachi" => Some(self.malachi),
            "matthew" => Some(self.matthew),
            "mark" => Some(self.mark),
            "luke" => Some(self.luke),
            "john" => Some(self.john),
            "acts" => Some(self.acts),
            "romans" => Some(self.romans),
            "first_corinthians" | "1_corinthians" | "1corinthians" | "i_corinthians" | "icorinthians" => Some(self.first_corinthians),
            "second_corinthians" | "2_corinthians" | "2corinthians" | "ii_corinthians" | "iicorinthians" => {
                Some(self.second_corinthians)
            }
            "galatians" => Some(self.galatians),
            "ephesians" => Some(self.ephesians),
            "philippians" => Some(self.philippians),
            "colossians" => Some(self.colossians),
            "first_thessalonians" | "1_thessalonians" | "1thessalonians" | "i_thessalonians" | "ithessalonians" => {
                Some(self.first_thessalonians)
            }
            "second_thessalonians" | "2_thessalonians" | "2thessalonians" | "ii_thessalonians" | "iithessalonians" => {
                Some(self.second_thessalonians)
            }
            "first_timothy" | "1_timothy" | "1timothy" | "i_timothy" | "itimothy" => Some(self.first_timothy),
            "second_timothy" | "2_timothy" | "2timothy" | "ii_timothy" | "iitimothy" => Some(self.second_timothy),
            "titus" => Some(self.titus),
            "philemon" => Some(self.philemon),
            "hebrews" => Some(self.hebrews),
            "james" => Some(self.james),
            "first_peter" | "1_peter" | "1peter" | "i_peter" | "ipeter" => Some(self.first_peter),
            "second_peter" | "2_peter" | "2peter" | "ii_peter" | "iipeter" => Some(self.second_peter),
            "first_john" | "1_john" | "1john" | "i_john" | "ijohn" => Some(self.first_john),
            "second_john" | "2_john" | "2john" | "ii_john" | "iijohn" => Some(self.second_john),
            "third_john" | "3_john" | "3john" | "iii_john" | "iiijohn" => Some(self.third_john),
            "jude" => Some(self.jude),
            "revelation" => Some(self.revelation),
            _ => None,
        }
    }

    pub fn get(&self, s: &str) -> Option<String> {
        let input_lower = s.to_lowercase();
        
        // Check if it's a chapter reference by looking for the last space followed by a number
        let words: Vec<&str> = input_lower.split_whitespace().collect();
        if words.len() >= 2 {
            // Check if the last word is a number (chapter number)
            if let Ok(_) = words.last().unwrap().parse::<u32>() {
                // Everything except the last word is the book name
                let book_part = words[..words.len()-1].join(" ");
                let chapter_part = words.last().unwrap();
                
                // Convert book part to lookup format (replace spaces with underscores)
                let book_lookup_key = book_part.replace(' ', "_");
                
                // Try to translate the book part
                if let Some(translated_book) = self.get_book(&book_lookup_key) {
                    return Some(format!("{} {}", translated_book, chapter_part));
                }
            }
        }
        
        // If no chapter number found or book not found, try to translate as a book name only
        // Convert spaces to underscores for lookup
        let book_lookup_key = input_lower.replace(' ', "_");
        self.get_book(&book_lookup_key).map(|book| book.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Language;

    #[test]
    fn test_chapter_translation() {
        let dutch_translation = Translation::from_language(Language::Dutch);
        
        // Test book name only
        assert_eq!(dutch_translation.get("matthew"), Some("Matteüs".to_string()));
        assert_eq!(dutch_translation.get("john"), Some("Johannes".to_string()));
        assert_eq!(dutch_translation.get("numbers"), Some("Numeri".to_string()));
        
        // Test chapter references
        assert_eq!(dutch_translation.get("matthew 7"), Some("Matteüs 7".to_string()));
        assert_eq!(dutch_translation.get("john 3"), Some("Johannes 3".to_string()));
        assert_eq!(dutch_translation.get("numbers 7"), Some("Numeri 7".to_string()));
        assert_eq!(dutch_translation.get("first_john 2"), Some("I Johannes 2".to_string()));
        
        // Test non-existent book
        assert_eq!(dutch_translation.get("nonexistent"), None);
        assert_eq!(dutch_translation.get("nonexistent 5"), None);
    }
    
    #[test]
    fn test_english_translation() {
        let english_translation = Translation::from_language(Language::English);
        
        // Test book name only
        assert_eq!(english_translation.get("matthew"), Some("Matthew".to_string()));
        assert_eq!(english_translation.get("john"), Some("John".to_string()));
        
        // Test chapter references
        assert_eq!(english_translation.get("matthew 7"), Some("Matthew 7".to_string()));
        assert_eq!(english_translation.get("john 3"), Some("John 3".to_string()));
    }

    #[test]
    fn test_various_formats() {
        let dutch_translation = Translation::from_language(Language::Dutch);
        
        // Test different input formats that might come from the data
        assert_eq!(dutch_translation.get("Numbers"), Some("Numeri".to_string()));
        assert_eq!(dutch_translation.get("numbers"), Some("Numeri".to_string()));
        assert_eq!(dutch_translation.get("NUMBERS"), Some("Numeri".to_string()));
        assert_eq!(dutch_translation.get("Numbers 7"), Some("Numeri 7".to_string()));
        assert_eq!(dutch_translation.get("numbers 7"), Some("Numeri 7".to_string()));
        
        // Test what happens with unknown format
        assert_eq!(dutch_translation.get("SomeUnknownBook"), None);
        assert_eq!(dutch_translation.get("SomeUnknownBook 5"), None);
    }

    #[test]
    fn test_command_palette_search_scenario() {
        let dutch_translation = Translation::from_language(Language::Dutch);
        
        // Simulate the command palette scenario
        let chapter_name = "Numbers 1"; // This is what's stored in the Bible data
        let translated_name = dutch_translation.get(chapter_name).unwrap(); // Should be "Numeri 1"
        assert_eq!(translated_name, "Numeri 1");
        
        // Now test if searching for "numeri" would find this
        let query = "numeri";
        let translated_lower = translated_name.to_lowercase(); // "numeri 1"
        
        // This should return a score > 0 since "numeri" is in "numeri 1" 
        assert!(translated_lower.contains(query), "Translated name '{}' should contain query '{}'", translated_lower, query);
    }

    #[test]
    fn test_roman_numeral_translations() {
        let dutch_translation = Translation::from_language(Language::Dutch);
        
        // Test Roman numeral formats that might appear in Bible data
        assert_eq!(dutch_translation.get("I Samuel"), Some("I Samuël".to_string()));
        assert_eq!(dutch_translation.get("II Samuel"), Some("II Samuël".to_string()));
        assert_eq!(dutch_translation.get("I Kings"), Some("I Koningen".to_string()));
        assert_eq!(dutch_translation.get("II Kings"), Some("II Koningen".to_string()));
        assert_eq!(dutch_translation.get("I Chronicles"), Some("I Kronieken".to_string()));
        assert_eq!(dutch_translation.get("II Chronicles"), Some("II Kronieken".to_string()));
        
        // Test chapter references with Roman numerals
        assert_eq!(dutch_translation.get("II Kings 7"), Some("II Koningen 7".to_string()));
        assert_eq!(dutch_translation.get("I Samuel 3"), Some("I Samuël 3".to_string()));
        
        // Test New Testament books
        assert_eq!(dutch_translation.get("I Corinthians"), Some("I Korintiërs".to_string()));
        assert_eq!(dutch_translation.get("II Corinthians"), Some("II Korintiërs".to_string()));
        assert_eq!(dutch_translation.get("I Timothy"), Some("I Timoteüs".to_string()));
        assert_eq!(dutch_translation.get("II Timothy"), Some("II Timoteüs".to_string()));
        assert_eq!(dutch_translation.get("I Peter"), Some("I Petrus".to_string()));
        assert_eq!(dutch_translation.get("II Peter"), Some("II Petrus".to_string()));
        assert_eq!(dutch_translation.get("I John"), Some("I Johannes".to_string()));
        assert_eq!(dutch_translation.get("II John"), Some("II Johannes".to_string()));
        assert_eq!(dutch_translation.get("III John"), Some("III Johannes".to_string()));
    }
}
