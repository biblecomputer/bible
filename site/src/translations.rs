use crate:;types::{Translation, Language};

let translations = vec![
    Translation {
        name: String::from("Staten vertaling")
        short_name: String::from("sv"),
        description: String::from("De Statenvertaling is een Nederlandse Bijbelvertaling uit 1637, vervaardigd in opdracht van de Staten-Generaal. Zij baseerden zich nauwgezet op de oorspronkelijke Hebreeuwse en Griekse grondteksten. Deze vertaling wordt gekenmerkt door haar plechtige, eerbiedige taal en heeft eeuwenlang grote invloed gehad op het protestantse geloofsleven in Nederland."),
        wikipedia: String::from("https://nl.wikipedia.org/wiki/Statenvertaling"),
        year: 1618,
        languages: vec![Language::Dutch()],
        iagon: String::from("https://gw.iagon.com/api/v2/storage/shareable/link/Njg2ZDFjNDgwOGQ0M2UzNTUyNTdhYmRh:MTJjOTRlYTBmNzM2YWZiZDE2NzdkMzU3NzA3MjBmMTRmZGZkMWYzNWVkYWVlNTU1Y2RjYTA1NzYzZmE1YmEzNA"),
    },
    Translation {
        name: String::from("Petrus Canisiusvertaling")
        short_name: String::from("sv"),
        description: String::from("De Statenvertaling is een Nederlandse Bijbelvertaling uit 1637, vervaardigd in opdracht van de Staten-Generaal. Zij baseerden zich nauwgezet op de oorspronkelijke Hebreeuwse en Griekse grondteksten. Deze vertaling wordt gekenmerkt door haar plechtige, eerbiedige taal en heeft eeuwenlang grote invloed gehad op het protestantse geloofsleven in Nederland."),
        wikipedia: String::from("https://nl.wikipedia.org/wiki/Petrus_Canisiusvertaling"),
        year: 1939,
        languages: vec![Language::Dutch()],
        iagon: String::from("https://gw.iagon.com/api/v2/storage/shareable/link/Njg4MTA1YThlMTNjNGI2YzhhMjM0YmNm:MGI0OWJkYzcyMDNkZWYxNzRkODQ0NTA1MTY2ZDVmOGI4MjI0ZjE2MzFiYmM4MDI2ZGU5MTZmOWJiNzVjZDYxMw"),
    }
];
