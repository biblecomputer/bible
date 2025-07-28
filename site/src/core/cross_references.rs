// Include the compile-time generated cross-references
include!(concat!(env!("OUT_DIR"), "/compiled_cross_references.rs"));

/// Loads cross-references from compile-time parsed data (no runtime parsing needed!)
pub fn load_cross_references() -> Result<crate::core::types::References, &'static str> {
    Ok(get_compiled_cross_references().clone())
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
}