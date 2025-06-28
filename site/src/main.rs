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

    view! {
        <Router>
            <nav>
                <p>github</p>
            </nav>
            <Sidebar bible=&bible />
            <main>
                <Routes fallback=|| "Not found.">
                    <Route path=path!("/") view=Home />
                    <Route
                        path=path!("/:book/:chapter")
                        view=move || {
                            let chapter = Chapter::from_url(bible.clone()).unwrap();
                            view! {
                                <ChapterDetail chapter=chapter />
                            }
                        }
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
