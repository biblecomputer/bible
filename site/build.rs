use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};

// Mirror the structures from the main code
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Reference {
    to_book_name: String,
    to_chapter: u32,
    to_verse_start: u32,
    to_verse_end: Option<u32>,
    votes: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct VerseId(u32);

impl VerseId {
    fn new(book_id: u8, chapter: u32, verse: u32) -> Self {
        let packed = ((book_id as u32) << 24) | ((chapter & 0xFFF) << 12) | (verse & 0xFFF);
        VerseId(packed)
    }
    
    fn from_book_name(book_name: &str, chapter: u32, verse: u32) -> Option<Self> {
        let book_id = book_name_to_id(book_name)?;
        Some(Self::new(book_id, chapter, verse))
    }
}

fn book_name_to_id(book_name: &str) -> Option<u8> {
    match book_name {
        // Old Testament
        "Genesis" => Some(1),
        "Exodus" => Some(2),
        "Leviticus" => Some(3),
        "Numbers" => Some(4),
        "Deuteronomy" => Some(5),
        "Joshua" => Some(6),
        "Judges" => Some(7),
        "Ruth" => Some(8),
        "1 Samuel" => Some(9),
        "2 Samuel" => Some(10),
        "1 Kings" => Some(11),
        "2 Kings" => Some(12),
        "1 Chronicles" => Some(13),
        "2 Chronicles" => Some(14),
        "Ezra" => Some(15),
        "Nehemiah" => Some(16),
        "Esther" => Some(17),
        "Job" => Some(18),
        "Psalms" => Some(19),
        "Proverbs" => Some(20),
        "Ecclesiastes" => Some(21),
        "Song of Solomon" => Some(22),
        "Isaiah" => Some(23),
        "Jeremiah" => Some(24),
        "Lamentations" => Some(25),
        "Ezekiel" => Some(26),
        "Daniel" => Some(27),
        "Hosea" => Some(28),
        "Joel" => Some(29),
        "Amos" => Some(30),
        "Obadiah" => Some(31),
        "Jonah" => Some(32),
        "Micah" => Some(33),
        "Nahum" => Some(34),
        "Habakkuk" => Some(35),
        "Zephaniah" => Some(36),
        "Haggai" => Some(37),
        "Zechariah" => Some(38),
        "Malachi" => Some(39),
        
        // New Testament
        "Matthew" => Some(40),
        "Mark" => Some(41),
        "Luke" => Some(42),
        "John" => Some(43),
        "Acts" => Some(44),
        "Romans" => Some(45),
        "1 Corinthians" => Some(46),
        "2 Corinthians" => Some(47),
        "Galatians" => Some(48),
        "Ephesians" => Some(49),
        "Philippians" => Some(50),
        "Colossians" => Some(51),
        "1 Thessalonians" => Some(52),
        "2 Thessalonians" => Some(53),
        "1 Timothy" => Some(54),
        "2 Timothy" => Some(55),
        "Titus" => Some(56),
        "Philemon" => Some(57),
        "Hebrews" => Some(58),
        "James" => Some(59),
        "1 Peter" => Some(60),
        "2 Peter" => Some(61),
        "1 John" => Some(62),
        "2 John" => Some(63),
        "3 John" => Some(64),
        "Jude" => Some(65),
        "Revelation" => Some(66),
        
        _ => None,
    }
}

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

fn parse_single_verse_reference(verse_ref: &str) -> Result<(String, u32, u32, Option<u32>), String> {
    let parts: Vec<&str> = verse_ref.split('.').collect();
    
    if parts.len() != 3 {
        return Err("Invalid verse reference format".to_string());
    }
    
    let book_name = expand_book_abbreviation(parts[0]);
    let chapter = parts[1].parse::<u32>()
        .map_err(|_| "Invalid chapter number".to_string())?;
    let verse = parts[2].parse::<u32>()
        .map_err(|_| "Invalid verse number".to_string())?;
    
    Ok((book_name, chapter, verse, None))
}

fn parse_verse_reference(verse_ref: &str) -> Result<(String, u32, u32, Option<u32>), String> {
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

fn parse_cross_references(content: &str) -> Result<HashMap<VerseId, Vec<Reference>>, String> {
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
            .map_err(|_| "Invalid votes".to_string())?;
        
        // Parse from verse
        let (from_book, from_chapter, from_verse, _) = parse_single_verse_reference(from_verse_ref)?;
        let from_key = match VerseId::from_book_name(&from_book, from_chapter, from_verse) {
            Some(id) => id,
            None => continue, // Skip unknown books
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
    
    Ok(references_map)
}

fn main() {
    println!("cargo:rerun-if-changed=src/storage/cross_references.txt");
    
    let cross_references_path = "src/storage/cross_references.txt";
    
    // Read the cross-references file
    let content = fs::read_to_string(cross_references_path)
        .expect("Failed to read cross_references.txt");
    
    // Parse the cross-references
    let references = parse_cross_references(&content)
        .expect("Failed to parse cross-references");
    
    println!("Parsed {} verses with cross-references", references.len());
    
    // Convert to simpler format for binary serialization
    let simplified_map: HashMap<u32, Vec<(String, u32, u32, Option<u32>, i32)>> = references
        .into_iter()
        .map(|(verse_id, refs)| {
            let simplified_refs = refs.into_iter()
                .map(|r| (r.to_book_name, r.to_chapter, r.to_verse_start, r.to_verse_end, r.votes))
                .collect();
            (verse_id.0, simplified_refs)
        })
        .collect();
    
    // Serialize to binary format for faster loading
    let binary_data = bincode::serialize(&simplified_map)
        .expect("Failed to serialize cross-references");
    
    // Write binary data to output directory
    let out_dir = env::var("OUT_DIR").unwrap();
    let binary_path = Path::new(&out_dir).join("cross_references.bin");
    
    fs::write(&binary_path, &binary_data)
        .expect("Failed to write cross_references.bin");
    
    // Generate simple Rust code that loads the binary at runtime
    let code = r#"// Auto-generated cross-references loader at compile time
use crate::core::types::{References, Reference, VerseId};
use std::collections::HashMap;
use std::sync::OnceLock;

static COMPILED_CROSS_REFERENCES: OnceLock<References> = OnceLock::new();

pub fn get_compiled_cross_references() -> &'static References {
    COMPILED_CROSS_REFERENCES.get_or_init(|| {
        // Load binary data embedded at compile time
        let binary_data = include_bytes!(concat!(env!("OUT_DIR"), "/cross_references.bin"));
        
        // Deserialize using a simple format
        let parsed_map: HashMap<u32, Vec<(String, u32, u32, Option<u32>, i32)>> = 
            bincode::deserialize(binary_data).expect("Failed to deserialize cross-references");
        
        // Convert to runtime types
        let mut runtime_map = HashMap::new();
        for (verse_id_raw, refs) in parsed_map {
            let verse_id = VerseId(verse_id_raw);
            let runtime_refs: Vec<Reference> = refs.into_iter().map(|(book, chapter, start, end, votes)| {
                Reference {
                    to_book_name: book,
                    to_chapter: chapter,
                    to_verse_start: start,
                    to_verse_end: end,
                    votes,
                }
            }).collect();
            runtime_map.insert(verse_id, runtime_refs);
        }
        
        References(runtime_map)
    })
}
"#;
    
    // Write the Rust code
    let dest_path = Path::new(&out_dir).join("compiled_cross_references.rs");
    fs::write(&dest_path, code)
        .expect("Failed to write compiled_cross_references.rs");
    
    println!("Generated compiled cross-references with {} bytes of binary data", binary_data.len());
    println!("Generated at: {}", dest_path.display());
}