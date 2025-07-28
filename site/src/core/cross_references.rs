use crate::core::types::{References, Reference, VerseKey};
use std::collections::HashMap;

#[derive(Debug)]
pub enum CrossReferenceParseError {
    InvalidFormat,
    InvalidVerseReference,
    InvalidVotes,
}

/// Parses a verse reference like "Gen.1.1" or "Rom.1.19-Rom.1.20"
fn parse_verse_reference(verse_ref: &str) -> Result<(String, u32, u32, Option<u32>), CrossReferenceParseError> {
    // Handle ranges like "Rom.1.19-Rom.1.20"
    if let Some((start_ref, end_ref)) = verse_ref.split_once('-') {
        let (start_book, start_chapter, start_verse, _) = parse_single_verse_reference(start_ref)?;
        let (_, _, end_verse, _) = parse_single_verse_reference(end_ref)?;
        Ok((start_book, start_chapter, start_verse, Some(end_verse)))
    } else {
        let (book, chapter, verse, _) = parse_single_verse_reference(verse_ref)?;
        Ok((book, chapter, verse, None))
    }
}

/// Parses a single verse reference like "Gen.1.1"
fn parse_single_verse_reference(verse_ref: &str) -> Result<(String, u32, u32, Option<u32>), CrossReferenceParseError> {
    let parts: Vec<&str> = verse_ref.split('.').collect();
    
    if parts.len() != 3 {
        return Err(CrossReferenceParseError::InvalidVerseReference);
    }
    
    let book_name = expand_book_abbreviation(parts[0]);
    let chapter = parts[1].parse::<u32>()
        .map_err(|_| CrossReferenceParseError::InvalidVerseReference)?;
    let verse = parts[2].parse::<u32>()
        .map_err(|_| CrossReferenceParseError::InvalidVerseReference)?;
    
    Ok((book_name, chapter, verse, None))
}

/// Expands book abbreviations to full names
fn expand_book_abbreviation(abbrev: &str) -> String {
    match abbrev {
        // Old Testament
        "Gen" => "Genesis",
        "Exod" => "Exodus", 
        "Lev" => "Leviticus",
        "Num" => "Numbers",
        "Deut" => "Deuteronomy",
        "Josh" => "Joshua",
        "Judg" => "Judges",
        "Ruth" => "Ruth",
        "1Sam" => "1 Samuel",
        "2Sam" => "2 Samuel",
        "1Kgs" => "1 Kings",
        "2Kgs" => "2 Kings",
        "1Chr" => "1 Chronicles",
        "2Chr" => "2 Chronicles",
        "Ezra" => "Ezra",
        "Neh" => "Nehemiah",
        "Esth" => "Esther",
        "Job" => "Job",
        "Ps" => "Psalms",
        "Prov" => "Proverbs",
        "Eccl" => "Ecclesiastes",
        "Song" => "Song of Solomon",
        "Isa" => "Isaiah",
        "Jer" => "Jeremiah",
        "Lam" => "Lamentations",
        "Ezek" => "Ezekiel",
        "Dan" => "Daniel",
        "Hos" => "Hosea",
        "Joel" => "Joel",
        "Amos" => "Amos",
        "Obad" => "Obadiah",
        "Jonah" => "Jonah",
        "Mic" => "Micah",
        "Nah" => "Nahum",
        "Hab" => "Habakkuk",
        "Zeph" => "Zephaniah",
        "Hag" => "Haggai",
        "Zech" => "Zechariah",
        "Mal" => "Malachi",
        
        // New Testament
        "Matt" => "Matthew",
        "Mark" => "Mark",
        "Luke" => "Luke",
        "John" => "John",
        "Acts" => "Acts",
        "Rom" => "Romans",
        "1Cor" => "1 Corinthians",
        "2Cor" => "2 Corinthians",
        "Gal" => "Galatians",
        "Eph" => "Ephesians",
        "Phil" => "Philippians",
        "Col" => "Colossians",
        "1Thess" => "1 Thessalonians",
        "2Thess" => "2 Thessalonians",
        "1Tim" => "1 Timothy",
        "2Tim" => "2 Timothy",
        "Titus" => "Titus",
        "Phlm" => "Philemon",
        "Heb" => "Hebrews",
        "Jas" => "James",
        "1Pet" => "1 Peter",
        "2Pet" => "2 Peter",
        "1John" => "1 John",
        "2John" => "2 John",
        "3John" => "3 John",
        "Jude" => "Jude",
        "Rev" => "Revelation",
        
        // Return as-is if not found
        _ => abbrev,
    }.to_string()
}

/// Loads cross-references from the embedded file
pub fn load_cross_references() -> Result<References, CrossReferenceParseError> {
    let content = include_str!("../storage/cross_references.txt");
    parse_cross_references(content)
}

/// Parses the cross-references text file into a References struct
pub fn parse_cross_references(content: &str) -> Result<References, CrossReferenceParseError> {
    let mut references_map = HashMap::new();
    
    let lines = content.lines();
    
    for (line_num, line) in lines.enumerate() {
        // Skip header line
        if line_num == 0 {
            continue;
        }
        
        // Skip empty lines
        if line.trim().is_empty() {
            continue;
        }
        
        // Split by tabs
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() != 3 {
            continue; // Skip malformed lines
        }
        
        let from_verse_ref = parts[0].trim();
        let to_verse_ref = parts[1].trim();
        let votes_str = parts[2].trim();
        
        // Parse votes
        let votes = votes_str.parse::<i32>()
            .map_err(|_| CrossReferenceParseError::InvalidVotes)?;
        
        // Parse from verse
        let (from_book, from_chapter, from_verse, _) = parse_single_verse_reference(from_verse_ref)?;
        let from_key = VerseKey {
            book_name: from_book,
            chapter: from_chapter,
            verse: from_verse,
        };
        
        // Parse to verse (can be range)
        let (to_book, to_chapter, to_verse_start, to_verse_end) = parse_verse_reference(to_verse_ref)?;
        
        let reference = Reference {
            to_book_name: to_book,
            to_chapter,
            to_verse_start,
            to_verse_end,
            votes,
        };
        
        // Add to map
        references_map
            .entry(from_key)
            .or_insert_with(Vec::new)
            .push(reference);
    }
    
    Ok(References(references_map))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_verse_reference() {
        let (book, chapter, verse, _) = parse_single_verse_reference("Gen.1.1").unwrap();
        assert_eq!(book, "Genesis");
        assert_eq!(chapter, 1);
        assert_eq!(verse, 1);
        
        let (book, chapter, verse, _) = parse_single_verse_reference("1John.2.3").unwrap();
        assert_eq!(book, "1 John");
        assert_eq!(chapter, 2);
        assert_eq!(verse, 3);
    }

    #[test]
    fn test_parse_verse_reference_single() {
        let (book, chapter, start, end) = parse_verse_reference("Gen.1.1").unwrap();
        assert_eq!(book, "Genesis");
        assert_eq!(chapter, 1);
        assert_eq!(start, 1);
        assert_eq!(end, None);
    }

    #[test]
    fn test_parse_verse_reference_range() {
        let (book, chapter, start, end) = parse_verse_reference("Rom.1.19-Rom.1.20").unwrap();
        assert_eq!(book, "Romans");
        assert_eq!(chapter, 1);
        assert_eq!(start, 19);
        assert_eq!(end, Some(20));
    }

    #[test]
    fn test_expand_book_abbreviation() {
        assert_eq!(expand_book_abbreviation("Gen"), "Genesis");
        assert_eq!(expand_book_abbreviation("1John"), "1 John");
        assert_eq!(expand_book_abbreviation("Rev"), "Revelation");
        assert_eq!(expand_book_abbreviation("Unknown"), "Unknown");
    }

    #[test]
    fn test_parse_cross_references() {
        let content = r#"From Verse	To Verse	Votes	#www.openbible.info CC-BY 2025-07-21
Gen.1.1	Isa.51.16	51
Gen.1.1	Rom.1.19-Rom.1.20	50
Gen.1.2	Job.26.7	40"#;

        let references = parse_cross_references(content).unwrap();
        
        // Check Genesis 1:1 references
        let gen_1_1_key = VerseKey {
            book_name: "Genesis".to_string(),
            chapter: 1,
            verse: 1,
        };
        
        let gen_1_1_refs = references.0.get(&gen_1_1_key).unwrap();
        assert_eq!(gen_1_1_refs.len(), 2);
        
        // Check first reference (single verse)
        assert_eq!(gen_1_1_refs[0].to_book_name, "Isaiah");
        assert_eq!(gen_1_1_refs[0].to_chapter, 51);
        assert_eq!(gen_1_1_refs[0].to_verse_start, 16);
        assert_eq!(gen_1_1_refs[0].to_verse_end, None);
        assert_eq!(gen_1_1_refs[0].votes, 51);
        
        // Check second reference (verse range)
        assert_eq!(gen_1_1_refs[1].to_book_name, "Romans");
        assert_eq!(gen_1_1_refs[1].to_chapter, 1);
        assert_eq!(gen_1_1_refs[1].to_verse_start, 19);
        assert_eq!(gen_1_1_refs[1].to_verse_end, Some(20));
        assert_eq!(gen_1_1_refs[1].votes, 50);
        
        // Check Genesis 1:2 reference
        let gen_1_2_key = VerseKey {
            book_name: "Genesis".to_string(),
            chapter: 1,
            verse: 2,
        };
        
        let gen_1_2_refs = references.0.get(&gen_1_2_key).unwrap();
        assert_eq!(gen_1_2_refs.len(), 1);
        assert_eq!(gen_1_2_refs[0].to_book_name, "Job");
        assert_eq!(gen_1_2_refs[0].votes, 40);
    }

    #[test]
    fn test_load_actual_cross_references_file() {
        // This test verifies we can load the actual cross-references file
        let references = load_cross_references().unwrap();
        
        // The file should have many references
        assert!(!references.0.is_empty());
        
        // Check that Genesis 1:1 has references (should be one of the most referenced verses)
        let gen_1_1_key = VerseKey {
            book_name: "Genesis".to_string(),
            chapter: 1,
            verse: 1,
        };
        
        if let Some(gen_1_1_refs) = references.0.get(&gen_1_1_key) {
            assert!(!gen_1_1_refs.is_empty());
            
            // Verify at least one reference has proper structure
            let first_ref = &gen_1_1_refs[0];
            assert!(!first_ref.to_book_name.is_empty());
            assert!(first_ref.to_chapter > 0);
            assert!(first_ref.to_verse_start > 0);
        }
    }
}