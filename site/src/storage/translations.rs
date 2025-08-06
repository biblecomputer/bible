use crate::storage::translation_storage::{get_selected_translation, BibleTranslation, Language};

pub fn get_translations() -> Vec<BibleTranslation> {
    vec![
        BibleTranslation {
            name: String::from("Staten vertaling"),
            short_name: String::from("svv"),
            description: String::from("De Statenvertaling is een Nederlandse Bijbelvertaling uit 1637, vervaardigd in opdracht van de Staten-Generaal. Zij baseerden zich nauwgezet op de oorspronkelijke Hebreeuwse en Griekse grondteksten. Deze vertaling wordt gekenmerkt door haar plechtige, eerbiedige taal en heeft eeuwenlang grote invloed gehad op het protestantse geloofsleven in Nederland."),
            wikipedia: String::from("https://nl.wikipedia.org/wiki/Statenvertaling"),
            release_year: 1637,
            languages: vec![Language::Dutch],
            iagon: String::from("https://gw.iagon.com/api/v2/storage/shareable/link/Njg5MjEyOTM0NzVmZTAwZjg3Y2VjN2Iy:MjhiNDNiOTMyNDllYTAwMzRmYWM4ZTdmOTdlZDU3NGExNzQxNjA4MzBiNzU3MThmNjE5ZGEzODZiNjVlOWE2MA"),
        },
        BibleTranslation {
            name: String::from("Petrus Canisiusvertaling"),
            short_name: String::from("pcv"),
            description: String::from("De Petrus Canisiusvertaling is een Nederlandse rooms-katholieke Bijbelvertaling uit 1939, genoemd naar de heilige Petrus Canisius. Deze vertaling werd gemaakt door katholieke exegeten en kenmerkt zich door haar toegankelijke Nederlandse taal die tegelijkertijd de kerkelijke traditie respecteert."),
            wikipedia: String::from("https://nl.wikipedia.org/wiki/Petrus_Canisiusvertaling"),
            release_year: 1939,
            languages: vec![Language::Dutch],
            iagon: String::from("https://gw.iagon.com/api/v2/storage/shareable/link/Njg5MjEyOGY0NzVmZTAwZjg3Y2VjN2Iw:N2U3ZTg1MTkxMmFiZGY5NTU0ZjM5MTdhZThkMDIzYWYxZDg4ZDQ1N2EyMDA1NzhiZWRiYmY4NmUzZTA3ZWIyOA"),
        },
        BibleTranslation {
            name: String::from("King james version"),
            short_name: String::from("kjv"),
            description: String::from("The King James Version is an English translation of the Bible published in 1611, commissioned by King James I of England. It was created by 47 scholars and is known for its literary excellence and enduring influence on English literature and Protestant Christianity. The KJV uses formal equivalence translation and features majestic, archaic English that has shaped religious language for centuries."),
            wikipedia: String::from("https://en.wikipedia.org/wiki/King_James_Version"),
            release_year: 1611,
            languages: vec![Language::English],
            iagon: String::from("https://gw.iagon.com/api/v2/storage/shareable/link/Njg5MjEyZGM0NzVmZTAwZjg3Y2VkNDU0:Yjc0MjAwNzMzN2RmM2UyMGVkZDgzYThiMWRjZWIxMjM0OTUwMjZhNDVhMWFkOGZmMThjOTU4NTUzMmUwY2FhYQ"),
        },
        BibleTranslation {
            name: String::from("American King james version"),
            short_name: String::from("akjv"),
            description: String::from("The American King James Version (AKJV) is a modernized update of the original 1769 Oxford edition of the King James Bible. Created by Michael Peter Engelbrite, this version retains the literary beauty and structure of the original KJV while updating some archaic English words and spellings to make the text more accessible to contemporary readers. Importantly, the AKJV makes no changes to the underlying meaning of the scriptures, preserving the integrity and style of the King James tradition. It is in the public domain and widely used by those who appreciate the KJV but prefer a slightly more readable form."),
            wikipedia: String::from("https://studybible.info/version/AKJV"),
            release_year: 1999,
            languages: vec![Language::English],
            iagon: String::from("https://gw.iagon.com/api/v2/storage/shareable/link/Njg5MjEyZGI0NzVmZTAwZjg3Y2VkNDQ2:MWRjOGI2N2Y3OGE1MWY5MmU1YmMwYjhiZjY2NjM3ZWRkMjY0OWZiMWY4ZDg3MTZmMmU1ODViOTgwNDM4ZjU3Zg"),
        },
        BibleTranslation {
            name: String::from("Americain Standard Version"),
            short_name: String::from("asv"),
            description: String::from("American Standard Version (ASV) – Published in 1901, the ASV is a highly literal English translation of the Bible, rooted in the tradition of the King James Version but updated to reflect more accurate renderings from the Hebrew and Greek manuscripts. It is known for its formal equivalence (word-for-word translation), the use of “Jehovah” for God’s name, and its influence on later versions such as the NASB. Though somewhat archaic in style today, it remains a respected choice for serious Bible study."),
            wikipedia: String::from("https://en.wikipedia.org/wiki/American_Standard_Version"),
            release_year: 1901,
            languages: vec![Language::English],
            iagon: String::from("https://gw.iagon.com/api/v2/storage/shareable/link/Njg5MjI0ZmM0NzVmZTAwZjg3Y2YzMTg4:MTEzMjZkOTVlZTFkMWNhOGM0YmFmNDkwOWFkMjdmOTI3NjY5YjQ2NzA3NjViOTJlYTE2MzNmMzFkMzRiY2MwNQ"),
        },
        BibleTranslation {
            name: String::from("Green's Modern King James Version"),
            short_name: String::from("mkjv"),
            description: String::from("Green’s Modern King James Version (MKJV) is a conservative update of the King James Bible, produced by Jay P. Green Sr. It retains the traditional style and majesty of the original KJV while updating archaic words and grammar for clarity and readability. First published in the 1960s, the MKJV seeks to preserve the accuracy and literary beauty of the Authorized Version, using the Textus Receptus as its Greek source. It is often appreciated by readers who desire a modernized yet faithful rendering of the Scriptures."),
            wikipedia: String::from("https://www.gotquestions.org/Modern-King-James-Version-MKJV.html"),
            release_year: 1962,
            languages: vec![Language::English],
            iagon: String::from("https://gw.iagon.com/api/v2/storage/shareable/link/Njg5MjIxMTQ0NzVmZTAwZjg3Y2VmOTEw:YzEzMGExYjU0OWI1M2I4ODk4MWJmYjgwNmM3YzE1ODJkZWJmMjhiNmYxOGMzMGY2ZTk0MTFlYjUyN2IzOGRjZQ"),
        }
    ]
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
    languages.sort_by(|a, b| {
        match (a, b) {
            (Language::Dutch, Language::English) => std::cmp::Ordering::Less,
            (Language::English, Language::Dutch) => std::cmp::Ordering::Greater,
            _ => std::cmp::Ordering::Equal,
        }
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
