use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <h1>"Hello World from Leptos!"</h1>
        <p>"This is a minimal Leptos application."</p>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
