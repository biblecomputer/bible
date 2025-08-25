// Include the compile-time generated cross-references
include!(concat!(env!("OUT_DIR"), "/compiled_cross_references.rs"));

/// Loads cross-references from compile-time parsed data (no runtime parsing needed!)
pub fn load_cross_references() -> Result<crate::core::types::References, &'static str> {
    Ok(get_compiled_cross_references().clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::VerseId;

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
            let has_john_ref = gen_1_1_refs
                .iter()
                .any(|r| r.to_book_name == "John" && r.to_chapter == 1 && r.to_verse_start == 1);

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
            assert!(
                has_john_ref || has_hebrews_ref,
                "Missing expected cross-references for Genesis 1:1"
            );
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
            println!(
                "Found {} cross-references for 1 Samuel 1:10:",
                samuel_refs.len()
            );

            for (i, ref_) in samuel_refs.iter().enumerate() {
                println!(
                    "  {}. {} {}:{}",
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

            let has_job_9_18 = samuel_refs
                .iter()
                .any(|r| r.to_book_name == "Job" && r.to_chapter == 9 && r.to_verse_start == 18);

            let has_luke_22_44 = samuel_refs
                .iter()
                .any(|r| r.to_book_name == "Luke" && r.to_chapter == 22 && r.to_verse_start == 44);

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
            println!(
                "Found {} cross-references for 1 Samuel 1:11:",
                samuel_refs_11.len()
            );

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
            assert!(
                !samuel_refs_11.is_empty(),
                "1 Samuel 1:11 should have cross-references"
            );
        } else {
            println!("⚠ No cross-references found for 1 Samuel 1:11 (this might be expected if not in the data)");
        }
    }
}
