use inquire::{CustomType, MultiSelect, Text};
use peter::language::Language;
use peter::translation::v1::{EquivalenceLevel, TranslationMetaData, Year};
use url::Url;

pub fn create_translation_metadata() -> TranslationMetaData {
    println!("=== Bible Translation Metadata Creator ===\n");

    let full_name = Text::new("Enter full translation name (e.g., 'King James Version'):")
        .prompt()
        .expect("Failed to get full translation name");

    let short_name = Text::new("Enter short name/abbreviation (e.g., 'KJV'):")
        .prompt()
        .expect("Failed to get short name");

    let description = Text::new("Enter description:")
        .prompt()
        .expect("Failed to get description");

    let link = CustomType::<String>::new("Enter website link (such as Wikipedia):")
        .with_parser(&|input| {
            Url::parse(input)
                .map(|url| url.to_string())
                .map_err(|_| ())
        })
        .with_error_message("Invalid URL. Please enter a valid URL (e.g., https://example.com)")
        .prompt()
        .expect("Failed to get URL");
    let link = Url::parse(&link).unwrap();

    let release_year = CustomType::<i32>::new("Enter release year:")
        .with_error_message("Please enter a valid year")
        .prompt()
        .expect("Failed to get release year");

    let languages = select_languages();

    let equivalence_level = CustomType::<u8>::new("Enter equivalence level (0-255):")
        .with_help_message(
            "0 = Extremely formal (word-for-word)\n\
             128 = Balanced\n\
             255 = Extremely functional (meaning-based)",
        )
        .with_error_message("Please enter a number between 0 and 255")
        .with_parser(&|input| {
            input
                .parse::<u8>()
                .map_err(|_| ())
        })
        .prompt()
        .expect("Failed to get equivalence level");

    let funded_by = Text::new("Enter funding organization/person (optional, press Enter to skip):")
        .prompt()
        .ok()
        .and_then(|s| if s.is_empty() { None } else { Some(s) });

    TranslationMetaData {
        full_name,
        short_name,
        description,
        link,
        release: Year::new(release_year),
        languages,
        equivalence_level: EquivalenceLevel::new(equivalence_level),
        funded_by,
    }
}

fn select_languages() -> Vec<Language> {
    let options = vec![Language::English, Language::Dutch];

    let languages = MultiSelect::new("Select languages:", options)
        .with_help_message("Use space to select, enter to confirm")
        .prompt();

    match languages {
        Ok(langs) if !langs.is_empty() => langs,
        _ => {
            println!("No languages selected, defaulting to English.");
            vec![Language::English]
        }
    }
}