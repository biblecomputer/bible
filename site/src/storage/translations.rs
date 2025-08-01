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
            iagon: String::from("https://gw.iagon.com/api/v2/storage/shareable/link/Njg4NjlmY2VlMTNjNGI2YzhhMmU3MzQx:NWFlN2IwYmIyZDQ0OWI3OTQ1ZmJhYWI4NGFjODJkYjYyMmM1MWJkZmEzYmI1NTA1NzgyZWEwNGQwOGMyMGM3MQ"),
        },
        BibleTranslation {
            name: String::from("Petrus Canisiusvertaling"),
            short_name: String::from("pcv"),
            description: String::from("De Petrus Canisiusvertaling is een Nederlandse rooms-katholieke Bijbelvertaling uit 1939, genoemd naar de heilige Petrus Canisius. Deze vertaling werd gemaakt door katholieke exegeten en kenmerkt zich door haar toegankelijke Nederlandse taal die tegelijkertijd de kerkelijke traditie respecteert."),
            wikipedia: String::from("https://nl.wikipedia.org/wiki/Petrus_Canisiusvertaling"),
            release_year: 1939,
            languages: vec![Language::Dutch],
            iagon: String::from("https://gw.iagon.com/api/v2/storage/shareable/link/Njg4NjYwZTZlMTNjNGI2YzhhMmU0Zjc0:MGYyYTlkYmRiZDhhNThjYjRmNzk4NzA2ODZkODY0M2NlMzJjZDRkMjM3YWJmNjQ5MWU4NmFkMTRmNDMwZWMzYQ"),
        },
        BibleTranslation {
            name: String::from("King james version"),
            short_name: String::from("kjv"),
            description: String::from("The King James Version is an English translation of the Bible published in 1611, commissioned by King James I of England. It was created by 47 scholars and is known for its literary excellence and enduring influence on English literature and Protestant Christianity. The KJV uses formal equivalence translation and features majestic, archaic English that has shaped religious language for centuries."),
            wikipedia: String::from("https://en.wikipedia.org/wiki/King_James_Version"),
            release_year: 1611,
            languages: vec![Language::English],
            iagon: String::from("https://gw.iagon.com/api/v2/storage/shareable/link/Njg4NjYzMDBlMTNjNGI2YzhhMmU1Yzc2:MzI3ZTY3NjBmMDAwMzBlMDVlZGM3NGQxNjU5MDIxMDdlNTE0MDA2ZWJkNTRkMjAyZGJjZWE1ZTlhMTQzNmYzNg"),
        }
    ]
}

pub fn get_current_translation() -> Option<BibleTranslation> {
    let selected_short_name = get_selected_translation().unwrap_or_else(|| "sv".to_string());

    get_translations()
        .into_iter()
        .find(|t| t.short_name == selected_short_name)
}
