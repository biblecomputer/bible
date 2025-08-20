use crate::storage::translation_storage::{get_selected_translation, BibleTranslation, Language};
use gloo_storage::{LocalStorage, Storage};

const CUSTOM_TRANSLATIONS_KEY: &str = "custom_translations";

pub fn get_custom_translations() -> Vec<BibleTranslation> {
    LocalStorage::get::<Vec<BibleTranslation>>(CUSTOM_TRANSLATIONS_KEY).unwrap_or_default()
}

pub fn get_builtin_translations() -> Vec<BibleTranslation> {
    vec![
        BibleTranslation {
            name: String::from("Staten vertaling"),
            short_name: String::from("nl_sv"),
            release_year: 1637,
            languages: vec![Language::Dutch],
            iagon: String::from("https://gw.iagon.com/api/v2/storage/shareable/link/Njg5MjEyOTM0NzVmZTAwZjg3Y2VjN2Iy:MjhiNDNiOTMyNDllYTAwMzRmYWM4ZTdmOTdlZDU3NGExNzQxNjA4MzBiNzU3MThmNjE5ZGEzODZiNjVlOWE2MA"),
        },
        BibleTranslation {
            name: String::from("Petrus Canicius vertaling"),
            short_name: String::from("pcv"),
            release_year: 1939,
            languages: vec![Language::Dutch],
            iagon: String::from("https://gw.iagon.com/api/v2/storage/shareable/link/NjhhNWE4M2NlZDM0YjkxMmFjZjBlZWUx:OGI2ODYxMDRmMWNlMTNmNDBhOWQ0M2U5NjAwZjA1OGY2ZWI4MGQwNDE0MThkYWQwYTc3NDc2YWI4OWJhMTViYQ"),
        },
        BibleTranslation {
            name: String::from("King james version"),
            short_name: String::from("en_kjv"),
            release_year: 1611,
            languages: vec![Language::English],
            iagon: String::from("https://gw.iagon.com/api/v2/storage/shareable/link/Njg5MjEyZGM0NzVmZTAwZjg3Y2VkNDU0:Yjc0MjAwNzMzN2RmM2UyMGVkZDgzYThiMWRjZWIxMjM0OTUwMjZhNDVhMWFkOGZmMThjOTU4NTUzMmUwY2FhYQ"),
        },
        BibleTranslation {
            name: String::from("American King james version"),
            short_name: String::from("en_akjv"),
            release_year: 1999,
            languages: vec![Language::English],
            iagon: String::from("https://gw.iagon.com/api/v2/storage/shareable/link/Njg5MjEyZGI0NzVmZTAwZjg3Y2VkNDQ2:MWRjOGI2N2Y3OGE1MWY5MmU1YmMwYjhiZjY2NjM3ZWRkMjY0OWZiMWY4ZDg3MTZmMmU1ODViOTgwNDM4ZjU3Zg"),
        },
        BibleTranslation {
            name: String::from("Americain Standard Version"),
            short_name: String::from("en_asv"),
            release_year: 1901,
            languages: vec![Language::English],
            iagon: String::from("https://gw.iagon.com/api/v2/storage/shareable/link/Njg5MjI0ZmM0NzVmZTAwZjg3Y2YzMTg4:MTEzMjZkOTVlZTFkMWNhOGM0YmFmNDkwOWFkMjdmOTI3NjY5YjQ2NzA3NjViOTJlYTE2MzNmMzFkMzRiY2MwNQ"),
        },
        BibleTranslation {
            name: String::from("Green's Modern King James Version"),
            short_name: String::from("en_mkjv"),
            release_year: 1962,
            languages: vec![Language::English],
            iagon: String::from("https://gw.iagon.com/api/v2/storage/shareable/link/Njg5MjIxMTQ0NzVmZTAwZjg3Y2VmOTEw:YzEzMGExYjU0OWI1M2I4ODk4MWJmYjgwNmM3YzE1ODJkZWJmMjhiNmYxOGMzMGY2ZTk0MTFlYjUyN2IzOGRjZQ"),
        }
    ]
}

pub fn get_translations() -> Vec<BibleTranslation> {
    let mut translations = get_builtin_translations();
    translations.extend(get_custom_translations());
    translations
}

pub fn get_current_translation() -> Option<BibleTranslation> {
    let selected_short_name = get_selected_translation().unwrap_or_else(|| "sv".to_string());

    get_translations()
        .into_iter()
        .find(|t| t.short_name == selected_short_name)
}

pub fn get_available_languages() -> Vec<Language> {
    let mut languages = Vec::new();
    for translation in get_translations() {
        for language in translation.languages {
            if !languages.contains(&language) {
                languages.push(language);
            }
        }
    }
    languages.sort_by(|a, b| match (a, b) {
        (Language::Dutch, Language::English) => std::cmp::Ordering::Less,
        (Language::English, Language::Dutch) => std::cmp::Ordering::Greater,
        _ => std::cmp::Ordering::Equal,
    });
    languages
}

pub fn get_translations_by_language(language: &Language) -> Vec<BibleTranslation> {
    get_translations()
        .into_iter()
        .filter(|translation| translation.languages.contains(language))
        .collect()
}

impl Language {
    pub fn display_name(&self) -> &str {
        match self {
            Language::Dutch => "Nederlands",
            Language::English => "English",
        }
    }
}