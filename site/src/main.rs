use leptos::prelude::*;
use leptos_router::hooks::{use_params_map};
use leptos_router::params::Params;
use leptos_router::path;
use leptos_router::components::{Router, Route, Routes};
use crate::types::*;
use crate::sidebar::Sidebar;
use crate::chapter_view::ChapterView;

mod types;
mod chapter_view;
mod sidebar;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App)
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
            <Sidebar bible=&bible />
            <main>
                <Routes fallback=|| "Not found.">
                    <Route path=path!("/") view=Home />
                    <Route
                        path=path!("/:book/:chapter")
                        view=move || view! {
                            <ChapterView bible=bible.clone() />
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
