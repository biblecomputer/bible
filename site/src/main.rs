use crate::chapter_view::ChapterDetail;
use crate::sidebar::Sidebar;
use crate::types::*;
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

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
        
        view! {
            <Router>
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
fn Home() -> impl IntoView {
    view! {
        <h1>Bijbel</h1>
    }
}
