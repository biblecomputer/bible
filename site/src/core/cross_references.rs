// Include the compile-time generated cross-references
include!(concat!(env!("OUT_DIR"), "/compiled_cross_references.rs"));

/// Loads cross-references from compile-time parsed data (no runtime parsing needed!)
pub fn load_cross_references() -> Result<crate::core::types::References, &'static str> {
    Ok(get_compiled_cross_references().clone())
}

/// Load compact cross-references for lazy parsing (memory efficient)
pub fn load_compact_cross_references() -> Result<crate::core::types::CompactCrossReferences, Box<dyn std::error::Error>> {
    let json_str = include_str!("../../data/cross_references.json");
    let cross_refs: crate::core::types::CompactCrossReferences = serde_json::from_str(json_str)?;
    Ok(cross_refs)
}

/// Parse cross-references for a specific chapter only (minimal memory footprint)
pub fn parse_chapter_compact_refs(
    compact_refs: &crate::core::types::CompactCrossReferences,
    book_name: &str,
    chapter: u32
) -> crate::core::types::ChapterCrossReferences {
    use crate::core::types::{ChapterCrossReferences, CompactReference};
    use std::collections::HashMap;
    
    let mut verse_refs = HashMap::new();
    let book_abbrev = book_name_to_abbreviation(book_name);
    
    // Only check verses that exist in this chapter (much more efficient)
    for verse in 1..=200 {  // Conservative upper bound
        let verse_key = format!("{}.{}.{}", book_abbrev, chapter, verse);
        
        if let Some(raw_references) = compact_refs.0.get(&verse_key) {
            let mut compact_references = Vec::new();
            
            for raw_ref in raw_references {
                if let Some(compact_ref) = CompactReference::from_raw(raw_ref) {
                    compact_references.push(compact_ref);
                }
            }
            
            if !compact_references.is_empty() {
                verse_refs.insert(verse, compact_references);
            }
        }
    }
    
    ChapterCrossReferences { verse_refs }
}

// Helper function for book name to abbreviation
fn book_name_to_abbreviation(book_name: &str) -> String {
    match book_name {
        "Genesis" => "Gen".to_string(),
        "Exodus" => "Exod".to_string(),
        "Leviticus" => "Lev".to_string(),
        "Numbers" => "Num".to_string(),
        "Deuteronomy" => "Deut".to_string(),
        "Joshua" => "Josh".to_string(),
        "Judges" => "Judg".to_string(),
        "Ruth" => "Ruth".to_string(),
        "I Samuel" | "1 Samuel" => "1Sam".to_string(),
        "II Samuel" | "2 Samuel" => "2Sam".to_string(),
        "I Kings" | "1 Kings" => "1Kgs".to_string(),
        "II Kings" | "2 Kings" => "2Kgs".to_string(),
        "I Chronicles" | "1 Chronicles" => "1Chr".to_string(),
        "II Chronicles" | "2 Chronicles" => "2Chr".to_string(),
        "Ezra" => "Ezra".to_string(),
        "Nehemiah" => "Neh".to_string(),
        "Esther" => "Esth".to_string(),
        "Job" => "Job".to_string(),
        "Psalms" => "Ps".to_string(),
        "Proverbs" => "Prov".to_string(),
        "Ecclesiastes" => "Eccl".to_string(),
        "Song of Solomon" => "Song".to_string(),
        "Isaiah" => "Isa".to_string(),
        "Jeremiah" => "Jer".to_string(),
        "Lamentations" => "Lam".to_string(),
        "Ezekiel" => "Ezek".to_string(),
        "Daniel" => "Dan".to_string(),
        "Hosea" => "Hos".to_string(),
        "Joel" => "Joel".to_string(),
        "Amos" => "Amos".to_string(),
        "Obadiah" => "Obad".to_string(),
        "Jonah" => "Jonah".to_string(),
        "Micah" => "Mic".to_string(),
        "Nahum" => "Nah".to_string(),
        "Habakkuk" => "Hab".to_string(),
        "Zephaniah" => "Zeph".to_string(),
        "Haggai" => "Hag".to_string(),
        "Zechariah" => "Zech".to_string(),
        "Malachi" => "Mal".to_string(),
        "Matthew" => "Matt".to_string(),
        "Mark" => "Mark".to_string(),
        "Luke" => "Luke".to_string(),
        "John" => "John".to_string(),
        "Acts" => "Acts".to_string(),
        "Romans" => "Rom".to_string(),
        "I Corinthians" | "1 Corinthians" => "1Cor".to_string(),
        "II Corinthians" | "2 Corinthians" => "2Cor".to_string(),
        "Galatians" => "Gal".to_string(),
        "Ephesians" => "Eph".to_string(),
        "Philippians" => "Phil".to_string(),
        "Colossians" => "Col".to_string(),
        "I Thessalonians" | "1 Thessalonians" => "1Thess".to_string(),
        "II Thessalonians" | "2 Thessalonians" => "2Thess".to_string(),
        "I Timothy" | "1 Timothy" => "1Tim".to_string(),
        "II Timothy" | "2 Timothy" => "2Tim".to_string(),
        "Titus" => "Titus".to_string(),
        "Philemon" => "Phlm".to_string(),
        "Hebrews" => "Heb".to_string(),
        "James" => "Jas".to_string(),
        "I Peter" | "1 Peter" => "1Pet".to_string(),
        "II Peter" | "2 Peter" => "2Pet".to_string(),
        "I John" | "1 John" => "1John".to_string(),
        "II John" | "2 John" => "2John".to_string(),
        "III John" | "3 John" => "3John".to_string(),
        "Jude" => "Jude".to_string(),
        "Revelation of John" | "Revelation" => "Rev".to_string(),
        _ => book_name.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::{VerseId};

    #[test]
    fn test_load_compiled_cross_references() {
        // This test verifies we can load the compile-time parsed cross-references
        let references = load_cross_references().unwrap();
        
        // The file should have many references
        assert!(!references.0.is_empty());
        
        // Check that Genesis 1:1 has references (should be one of the most referenced verses)
        let gen_1_1_key = VerseId::from_book_name("Genesis", 1, 1).unwrap();
        
        if let Some(gen_1_1_refs) = references.0.get(&gen_1_1_key) {
            assert!(!gen_1_1_refs.is_empty());
            
            // Verify at least one reference has proper structure
            let first_ref = &gen_1_1_refs[0];
            assert!(!first_ref.to_book_name.is_empty());
            assert!(first_ref.to_chapter > 0);
            assert!(first_ref.to_verse_start > 0);
            
            // Genesis 1:1 should have many popular cross-references
            println!("Genesis 1:1 has {} cross-references", gen_1_1_refs.len());
            assert!(gen_1_1_refs.len() > 10); // Should have many references
        } else {
            panic!("Genesis 1:1 should have cross-references");
        }
    }

    #[test]
    fn test_compile_time_performance() {
        // Test that accessing cross-references is fast (binary deserialization)
        use std::time::Instant;
        
        let start = Instant::now();
        let references = load_cross_references().unwrap();
        let duration = start.elapsed();
        
        // Should be much faster than text parsing (binary deserialization vs text parsing)
        assert!(duration.as_millis() < 500); // Should take less than 500ms
        assert!(!references.0.is_empty());
        
        println!("Binary cross-references loaded in {:?}", duration);
        
        // Test subsequent access should be even faster (cached)
        let start2 = Instant::now();
        let _references2 = load_cross_references().unwrap();
        let duration2 = start2.elapsed();
        
        // Subsequent accesses should be faster due to OnceLock caching
        assert!(duration2.as_millis() < 50);
        println!("Cached cross-references loaded in {:?}", duration2);
    }

    #[test] 
    fn test_cross_references_data_integrity() {
        let references = load_cross_references().unwrap();
        
        // Test a few known cross-references to ensure data integrity
        let gen_1_1_key = VerseId::from_book_name("Genesis", 1, 1).unwrap();
        
        if let Some(gen_1_1_refs) = references.0.get(&gen_1_1_key) {
            // Should contain references to John 1:1-3 (one of the most popular)
            let has_john_ref = gen_1_1_refs.iter().any(|r| {
                r.to_book_name == "John" && r.to_chapter == 1 && r.to_verse_start == 1
            });
            
            // Should contain references to Hebrews 11:3
            let has_hebrews_ref = gen_1_1_refs.iter().any(|r| {
                r.to_book_name == "Hebrews" && r.to_chapter == 11 && r.to_verse_start == 3
            });
            
            if has_john_ref {
                println!("✓ Found John 1:1 cross-reference for Genesis 1:1");
            }
            if has_hebrews_ref {
                println!("✓ Found Hebrews 11:3 cross-reference for Genesis 1:1");
            }
            
            // At least one of these popular cross-references should exist
            assert!(has_john_ref || has_hebrews_ref, "Missing expected cross-references for Genesis 1:1");
        }
    }

    #[test]
    fn test_numbered_books_cross_references() {
        let references = load_cross_references().unwrap();
        
        // Test 1 Samuel 1:10 specifically (from the example data provided)
        // 1Sam.1.10    2Kgs.20.3    3
        // 1Sam.1.10    Job.9.18    3
        // 1Sam.1.10    Luke.22.44    10
        // 1Sam.1.10    Gen.50.10    1
        // 1Sam.1.10    Job.10.1    10
        let samuel_1_1_10_key = VerseId::from_book_name("1 Samuel", 1, 10).unwrap();
        
        println!("Testing cross-references for 1 Samuel 1:10...");
        println!("VerseId key: {:?}", samuel_1_1_10_key);
        
        if let Some(samuel_refs) = references.0.get(&samuel_1_1_10_key) {
            println!("Found {} cross-references for 1 Samuel 1:10:", samuel_refs.len());
            
            for (i, ref_) in samuel_refs.iter().enumerate() {
                println!("  {}. {} {}:{}", 
                    i + 1, 
                    ref_.to_book_name, 
                    ref_.to_chapter, 
                    ref_.to_verse_start
                );
            }
            
            // Check for expected cross-references from the example data
            let has_2_kings_20_3 = samuel_refs.iter().any(|r| {
                r.to_book_name == "2 Kings" && r.to_chapter == 20 && r.to_verse_start == 3
            });
            
            let has_job_9_18 = samuel_refs.iter().any(|r| {
                r.to_book_name == "Job" && r.to_chapter == 9 && r.to_verse_start == 18
            });
            
            let has_luke_22_44 = samuel_refs.iter().any(|r| {
                r.to_book_name == "Luke" && r.to_chapter == 22 && r.to_verse_start == 44
            });
            
            let has_genesis_50_10 = samuel_refs.iter().any(|r| {
                r.to_book_name == "Genesis" && r.to_chapter == 50 && r.to_verse_start == 10
            });
            
            if has_2_kings_20_3 {
                println!("✓ Found 2 Kings 20:3 cross-reference");
            }
            if has_job_9_18 {
                println!("✓ Found Job 9:18 cross-reference");
            }
            if has_luke_22_44 {
                println!("✓ Found Luke 22:44 cross-reference");
            }
            if has_genesis_50_10 {
                println!("✓ Found Genesis 50:10 cross-reference");
            }
            
            // At least one of these expected cross-references should exist
            assert!(
                has_2_kings_20_3 || has_job_9_18 || has_luke_22_44 || has_genesis_50_10,
                "Missing expected cross-references for 1 Samuel 1:10. Expected at least one of: 2 Kings 20:3, Job 9:18, Luke 22:44, Genesis 50:10"
            );
            
        } else {
            panic!("No cross-references found for 1 Samuel 1:10! This indicates the numbered book parsing is broken.");
        }
        
        // Also test 1 Samuel 1:11 from the example data
        // 1Sam.1.11    Ps.25.18    5
        // 1Sam.1.11    Eccl.5.4    8
        // 1Sam.1.11    Gen.28.20    5
        let samuel_1_1_11_key = VerseId::from_book_name("1 Samuel", 1, 11).unwrap();
        
        println!("\nTesting cross-references for 1 Samuel 1:11...");
        
        if let Some(samuel_refs_11) = references.0.get(&samuel_1_1_11_key) {
            println!("Found {} cross-references for 1 Samuel 1:11:", samuel_refs_11.len());
            
            let has_psalm_25_18 = samuel_refs_11.iter().any(|r| {
                r.to_book_name == "Psalms" && r.to_chapter == 25 && r.to_verse_start == 18
            });
            
            let has_ecclesiastes_5_4 = samuel_refs_11.iter().any(|r| {
                r.to_book_name == "Ecclesiastes" && r.to_chapter == 5 && r.to_verse_start == 4
            });
            
            if has_psalm_25_18 {
                println!("✓ Found Psalms 25:18 cross-reference");
            }
            if has_ecclesiastes_5_4 {
                println!("✓ Found Ecclesiastes 5:4 cross-reference");
            }
            
            // Should have at least some cross-references
            assert!(!samuel_refs_11.is_empty(), "1 Samuel 1:11 should have cross-references");
            
        } else {
            println!("⚠ No cross-references found for 1 Samuel 1:11 (this might be expected if not in the data)");
        }
    }
}