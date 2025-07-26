use crate::translation_storage::{BibleTranslation, Language};

pub fn get_translations() -> Vec<BibleTranslation> {
    vec![
        BibleTranslation {
            name: String::from("Staten vertaling"),
            short_name: String::from("sv"),
            description: String::from("De Statenvertaling is een Nederlandse Bijbelvertaling uit 1637, vervaardigd in opdracht van de Staten-Generaal. Zij baseerden zich nauwgezet op de oorspronkelijke Hebreeuwse en Griekse grondteksten. Deze vertaling wordt gekenmerkt door haar plechtige, eerbiedige taal en heeft eeuwenlang grote invloed gehad op het protestantse geloofsleven in Nederland."),
            wikipedia: String::from("https://nl.wikipedia.org/wiki/Statenvertaling"),
            release_year: 1637,
            languages: vec![Language::Dutch],
            iagon: String::from("https://gw.iagon.com/api/v2/storage/shareable/link/Njg2ZDFjNDgwOGQ0M2UzNTUyNTdhYmRh:MTJjOTRlYTBmNzM2YWZiZDE2NzdkMzU3NzA3MjBmMTRmZGZkMWYzNWVkYWVlNTU1Y2RjYTA1NzYzZmE1YmEzNA"),
        },
        BibleTranslation {
            name: String::from("Petrus Canisiusvertaling"),
            short_name: String::from("pcv"),
            description: String::from("De Petrus Canisiusvertaling is een Nederlandse rooms-katholieke Bijbelvertaling uit 1939, genoemd naar de heilige Petrus Canisius. Deze vertaling werd gemaakt door katholieke exegeten en kenmerkt zich door haar toegankelijke Nederlandse taal die tegelijkertijd de kerkelijke traditie respecteert."),
            wikipedia: String::from("https://nl.wikipedia.org/wiki/Petrus_Canisiusvertaling"),
            release_year: 1939,
            languages: vec![Language::Dutch],
            iagon: String::from("https://gw.iagon.com/api/v2/storage/shareable/link/Njg4MTA1YThlMTNjNGI2YzhhMjM0YmNm:MGI0OWJkYzcyMDNkZWYxNzRkODQ0NTA1MTY2ZDVmOGI4MjI0ZjE2MzFiYmM4MDI2ZGU5MTZmOWJiNzVjZDYxMw"),
        },
        BibleTranslation {
            name: String::from("King james version"),
            short_name: String::from("kjv"),
            description: String::from("The King James Version is an English translation of the Bible published in 1611, commissioned by King James I of England. It was created by 47 scholars and is known for its literary excellence and enduring influence on English literature and Protestant Christianity. The KJV uses formal equivalence translation and features majestic, archaic English that has shaped religious language for centuries."),
            wikipedia: String::from("https://nl.wikipedia.org/wiki/King_James_Version"),
            release_year: 1611,
            languages: vec![Language::English],
            iagon: String::from("https://gw.iagon.com/api/v2/storage/shareable/link/Njg4M2Y3NmRlMTNjNGI2YzhhMmQyM2Ji:ZmIwZjczMTNkZGZkNjFhYjkwMjYzNWE0NGUwNzEzMGU4YThjYTZjNWZmOTdiZWJkNTg0ZDFhODVlNjBjMTZhNw"),
        }
    ]
}
