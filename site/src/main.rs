use leptos::prelude::*;
use leptos_router::hooks::{use_params_map};
use leptos_router::params::Params;
use leptos_router::path;
use leptos_router::components::{Router, Route, Routes};
use crate::types::*;

mod types;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App)
}

#[derive(Params, PartialEq)]
struct ChapterParams {
    book: Option<String>,
    chapter: Option<u32>,
}

fn ChapterView(bible: Bible) -> impl IntoView {
    let params = use_params_map();
    let book = move || params.read().get("book").unwrap();
    let chapter = move || {
        params
            .read()
            .get("chapter")
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(1) // fallback chapter number if parsing fails
    };

    let chapter: Chapter = bible.get_chapter(&book(), chapter()).unwrap();

    view! {
        <ChapterDetail chapter=chapter />
    }
}

#[component]
fn App() -> impl IntoView {
    let bible: Bible = serde_json::from_str(include_str!("../src/stv.json"))
        .expect("Failed to parse Bible JSON");
   
    view! {
        <Router>
            <nav>
                <p>github</p>
            </nav>
            <main>
                <Routes fallback=|| "Not found.">
                    <Route path=path!("/") view=Home />
                    <Route
                        path=path!("/:book/:chapter")
                        view=move || ChapterView(bible.clone())
                    />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <h1>Bijbel</h1>
    }
}

#[component]
fn Sidebar<'a>(bible: &'a Bible) -> impl IntoView + 'a {
    view! {
        <h1>Hello world</h1>
    }
}

#[component]
pub fn BibleViewer<'a>(bible: &'a Bible) -> impl IntoView + 'a {
    view! {
        <div class="bible">
            {bible.books.iter().map(|book| {
                view! {
                    <div class="book">
                        <h2>{book.name.as_str()}</h2>
                        {book.chapters.iter().map(|chapter| {
                            view! {
                                <ChapterDetail
                                    chapter=chapter.clone()
                                />
                            }
                        }).collect_view()}
                    </div>
                }
            }).collect_view()}
        </div>
    }
}

#[component]
fn ChapterDetail(chapter: Chapter) -> impl IntoView  {
    view! {
        <h1>{chapter.name.as_str()}</h1>
        {chapter.verses.iter().map(|verse| {
            view! {
                <p>{verse.text.as_str()}</p>
            }
        }).collect_view()}
    }
}

#[derive(Debug)]
enum ParamParseError {
    ChapterNotFound,
    BookNotFound,
}

impl Bible {
    pub fn get_chapter(&self, book: &str, chapter: u32) -> Result<Chapter, ParamParseError> {
        let book = self.books
            .iter()
            .find(|b| b.name.to_lowercase() == book.to_lowercase())
            .ok_or(ParamParseError::BookNotFound)?;

        let chapter = book.chapters
            .iter()
            .find(|c| c.chapter == chapter)
            .ok_or(ParamParseError::ChapterNotFound)?;

        Ok(chapter.clone())
    }
}
