use crate::chapter_view::ChapterDetail;
use crate::sidebar::Sidebar;
use crate::types::*;
use leptos::prelude::*;
use leptos::ev;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::hooks::{use_location, use_navigate};
use leptos_router::path;
use leptos::web_sys::KeyboardEvent;

mod chapter_view;
mod sidebar;
mod types;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let bible: Bible =
        serde_json::from_str(include_str!("../src/stv.json")).expect("Failed to parse Bible JSON");

    {
        let bible_for_routes = bible.clone();
        let bible_for_sidebar = bible.clone();
        let bible_for_nav = bible.clone();
        
        view! {
            <Router>
                <KeyboardNavigationHandler bible=bible_for_nav />
                <nav class="bg-white border-b border-gray-200 px-4 py-2">
                    <p class="text-sm text-gray-600">github</p>
                </nav>
                <div class="flex h-screen">
                    <aside class="w-64 bg-gray-50 border-r border-gray-200 p-3 overflow-y-auto">
                        <Sidebar bible=&bible_for_sidebar />
                    </aside>
                    <main class="flex-1 p-6 overflow-y-auto">
                        <Routes fallback=|| "Not found.">
                            <Route path=path!("/") view=Home />
                            <Route
                                path=path!("/:book/:chapter")
                                view={
                                    move || {
                                        let chapter = Chapter::from_url(bible_for_routes.clone()).unwrap();
                                        view! {
                                            <ChapterDetail chapter=chapter />
                                        }
                                    }
                                }
                            />
                        </Routes>
                    </main>
                </div>
            </Router>
        }
    }
}

#[component]
fn KeyboardNavigationHandler(bible: Bible) -> impl IntoView {
    let navigate = use_navigate();
    let location = use_location();
    
    // Set up keyboard event handler
    let handle_keydown = move |e: KeyboardEvent| {
        let pathname = location.pathname.get();
        
        // Parse current path to get book and chapter
        let path_parts: Vec<&str> = pathname.trim_start_matches('/').split('/').collect();
        if path_parts.len() == 2 {
            let book_name = path_parts[0].replace('_', " ");
            if let Ok(chapter_num) = path_parts[1].parse::<u32>() {
                if let Ok(current_chapter) = bible.get_chapter(&book_name, chapter_num) {
                    match e.key().as_str() {
                        "ArrowRight" => {
                            if let Some(next_chapter) = bible.get_next_chapter(&current_chapter) {
                                navigate(&next_chapter.to_path(), Default::default());
                            }
                        }
                        "ArrowLeft" => {
                            if let Some(prev_chapter) = bible.get_previous_chapter(&current_chapter) {
                                navigate(&prev_chapter.to_path(), Default::default());
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    };

    // Add global keydown listener - this runs only once when the App mounts
    window_event_listener(ev::keydown, handle_keydown);

    view! {
        // This component renders nothing, it just handles keyboard events
        <></>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <h1>Bijbel</h1>
    }
}
