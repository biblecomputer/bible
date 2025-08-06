use leptos::prelude::*;
use crate::themes::{get_themes, Theme};
use crate::storage::save_selected_theme;

#[component]
pub fn ThemeSwitcher(
    current_theme: ReadSignal<Theme>,
    set_current_theme: WriteSignal<Theme>,
) -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    let themes = StoredValue::new(get_themes());

    view! {
        <div class="relative">
            // Theme switcher button
            <button
                class="p-2 text-gray-600 hover:text-gray-900 hover:bg-gray-100 rounded transition-colors"
                style="color: var(--theme-text-secondary)"
                on:click=move |_| {
                    set_is_open.update(|open| *open = !*open);
                }
                aria-label="Switch theme"
                title="Switch theme"
            >
                <svg
                    width="20"
                    height="20"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    aria-hidden="true"
                >
                    // Palette/theme icon
                    <circle cx="13.5" cy="6.5" r=".5"/>
                    <circle cx="17.5" cy="10.5" r=".5"/>
                    <circle cx="8.5" cy="7.5" r=".5"/>
                    <circle cx="6.5" cy="11.5" r=".5"/>
                    <circle cx="12.5" cy="13.5" r=".5"/>
                    <circle cx="16.5" cy="17.5" r=".5"/>
                    <circle cx="6.5" cy="17.5" r=".5"/>
                    <circle cx="12.5" cy="20.5" r=".5"/>
                    <path d="M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10c.926 0 1.648-.746 1.648-1.688 0-.437-.18-.835-.437-1.125-.29-.289-.438-.652-.438-1.125a1.64 1.64 0 0 1 1.668-1.668h1.996c3.051 0 5.555-2.503 5.555-5.554C21.965 6.012 17.461 2 12 2z"/>
                </svg>
            </button>

            // Dropdown menu
            <Show
                when=move || is_open.get()
                fallback=|| view! { <></> }
            >
                <div class="absolute right-0 mt-2 w-48 bg-white border border-gray-200 rounded-lg shadow-lg z-50"
                     style="background-color: var(--theme-background); border-color: var(--theme-sidebar-border)">
                    <div class="py-2">
                        <div class="px-3 py-1 text-xs font-semibold text-gray-500 uppercase tracking-wide"
                             style="color: var(--theme-text-muted)">
                            "Themes"
                        </div>
                        {themes.get_value().into_iter().map(|theme| {
                            let theme_clone = theme.clone();
                            let theme_id_for_click = theme.id.clone();
                            let theme_id_for_show = theme.id.clone();
                            let theme_name = theme.name.clone();
                            let current_theme_id = move || current_theme.get().id.clone();
                            
                            view! {
                                <button
                                    class="w-full px-3 py-2 text-left hover:bg-gray-50 transition-colors flex items-center justify-between"
                                    style="color: var(--theme-text-primary)"
                                    on:click=move |_| {
                                        #[cfg(target_arch = "wasm32")]
                                        web_sys::console::log_1(&format!("Switching to theme: {}", theme_id_for_click).into());
                                        
                                        set_current_theme.set(theme_clone.clone());
                                        save_selected_theme(&theme_id_for_click);
                                        set_is_open.set(false);
                                        
                                        #[cfg(target_arch = "wasm32")]
                                        web_sys::console::log_1(&"Theme switch complete".into());
                                    }
                                >
                                    <span>{theme_name.clone()}</span>
                                    <Show
                                        when=move || current_theme_id() == theme_id_for_show
                                        fallback=|| view! { <></> }
                                    >
                                        <svg
                                            width="16"
                                            height="16"
                                            viewBox="0 0 24 24"
                                            fill="none"
                                            stroke="currentColor"
                                            stroke-width="2"
                                            class="text-blue-600"
                                            style="color: var(--theme-button-primary-background)"
                                        >
                                            <path d="M20 6 9 17l-5-5"/>
                                        </svg>
                                    </Show>
                                </button>
                            }
                        }).collect_view()}
                    </div>
                </div>
            </Show>

            // Backdrop to close dropdown when clicking outside
            <Show
                when=move || is_open.get()
                fallback=|| view! { <></> }
            >
                <div
                    class="fixed inset-0 z-40"
                    on:click=move |_| set_is_open.set(false)
                />
            </Show>
        </div>
    }
}