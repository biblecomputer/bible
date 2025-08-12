use leptos::prelude::*;
use leptos::ev;
use leptos::web_sys::KeyboardEvent;
use leptos::wasm_bindgen::JsCast;
use crate::themes::{get_themes, Theme};
use crate::storage::{save_selected_theme, save_references_sidebar_open};
use crate::utils::is_mobile_screen;

#[component]
pub fn ThemeSidebar(
    current_theme: ReadSignal<Theme>,
    set_current_theme: WriteSignal<Theme>,
    set_sidebar_open: WriteSignal<bool>,
    palette_open: ReadSignal<bool>,
) -> impl IntoView {
    let themes = get_themes();
    let themes_len = themes.len();
    
    // Track selected theme index for keyboard navigation
    let (selected_theme_index, set_selected_theme_index) = signal(0usize);
    
    // Update selected index when current theme changes
    let themes_for_effect = themes.clone();
    Effect::new(move |_| {
        let current_theme_id = current_theme.get().id;
        if let Some(index) = themes_for_effect.iter().position(|t| t.id == current_theme_id) {
            set_selected_theme_index.set(index);
        }
    });
    
    // Keyboard navigation handler
    let themes_for_keydown = themes.clone();
    let handle_keydown = move |e: KeyboardEvent| {
        if themes_len == 0 {
            return;
        }
        
        // Don't handle navigation when command palette is open (let palette handle it)
        if palette_open.get() {
            return;
        }
        
        match (e.key().as_str(), e.ctrl_key()) {
            ("j", true) => {
                // Ctrl+J: Next theme and apply it instantly
                e.prevent_default();
                let current_index = selected_theme_index.get();
                let next_index = if current_index + 1 < themes_len { 
                    current_index + 1 
                } else { 
                    0 
                };
                set_selected_theme_index.set(next_index);
                
                // Apply the theme instantly
                if let Some(selected_theme) = themes_for_keydown.get(next_index) {
                    set_current_theme.set(selected_theme.clone());
                    save_selected_theme(&selected_theme.id);
                }
                
                // Focus the selected theme button
                if let Some(window) = leptos::web_sys::window() {
                    if let Some(document) = window.document() {
                        if let Some(element) = document.get_element_by_id(&format!("theme-{}", next_index)) {
                            if let Some(html_element) = element.dyn_ref::<leptos::web_sys::HtmlElement>() {
                                let _ = html_element.focus();
                            }
                        }
                    }
                }
            }
            ("k", true) => {
                // Ctrl+K: Previous theme and apply it instantly
                e.prevent_default();
                let current_index = selected_theme_index.get();
                let prev_index = if current_index > 0 { 
                    current_index - 1 
                } else { 
                    themes_len - 1 
                };
                set_selected_theme_index.set(prev_index);
                
                // Apply the theme instantly
                if let Some(selected_theme) = themes_for_keydown.get(prev_index) {
                    set_current_theme.set(selected_theme.clone());
                    save_selected_theme(&selected_theme.id);
                }
                
                // Focus the selected theme button
                if let Some(window) = leptos::web_sys::window() {
                    if let Some(document) = window.document() {
                        if let Some(element) = document.get_element_by_id(&format!("theme-{}", prev_index)) {
                            if let Some(html_element) = element.dyn_ref::<leptos::web_sys::HtmlElement>() {
                                let _ = html_element.focus();
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    };
    
    // Add keyboard event listener
    let _cleanup = window_event_listener(ev::keydown, handle_keydown);
    
    view! {
        <div class="theme-sidebar h-full flex flex-col">
            <div class="flex items-center justify-between mb-4 pb-4 border-b" style="border-color: var(--theme-sidebar-border)">
                <h2 class="text-lg font-bold" style="color: var(--theme-sidebar-text)">Themes</h2>
                <button
                    class="p-2 hover:bg-gray-100 rounded transition-colors"
                    style="color: var(--theme-text-secondary)"
                    on:click=move |_| {
                        set_sidebar_open.set(false);
                        if is_mobile_screen() {
                            save_references_sidebar_open(false);
                        }
                    }
                    aria-label="Close themes"
                    title="Close themes"
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
                        <line x1="18" y1="6" x2="6" y2="18"/>
                        <line x1="6" y1="6" x2="18" y2="18"/>
                    </svg>
                </button>
            </div>
            
            <div class="flex-1 space-y-3" role="listbox" aria-label="Available themes">
                {themes.into_iter().enumerate().map(|(index, theme)| {
                    let theme_clone = theme.clone();
                    let theme_id_for_style = theme.id.clone();
                    let theme_id_for_click = theme.id.clone();
                    let theme_id_for_show = theme.id.clone();
                    let theme_name = theme.name.clone();
                    let current_theme_id = move || current_theme.get().id.clone();
                    let is_selected = move || selected_theme_index.get() == index;
                    
                    view! {
                        <button
                            id=format!("theme-{}", index)
                            class=move || format!(
                                "w-full p-4 rounded-lg border-2 transition-all duration-200 text-left group hover:shadow-md {}",
                                if is_selected() {
                                    "ring-2 ring-blue-500 ring-opacity-50"
                                } else {
                                    ""
                                }
                            )
                            style=move || {
                                let is_current = current_theme_id() == theme_id_for_style;
                                let base_style = if is_current {
                                    "border-color: var(--theme-button-primary-background); background-color: var(--theme-button-primary-background); color: var(--theme-button-primary-text); transform: scale(1.02)"
                                } else {
                                    "border-color: var(--theme-sidebar-border); background-color: var(--theme-sidebar-background); color: var(--theme-text-primary)"
                                };
                                base_style
                            }
                            role="option"
                            aria-selected=move || (selected_theme_index.get() == index).to_string()
                            aria-label=format!("{} theme", theme_name)
                            tabindex=move || if is_selected() { "0" } else { "-1" }
                            on:click=move |_| {
                                set_current_theme.set(theme_clone.clone());
                                save_selected_theme(&theme_id_for_click);
                                
                                // Close sidebar on mobile after selection
                                if is_mobile_screen() {
                                    set_sidebar_open.set(false);
                                    save_references_sidebar_open(false);
                                }
                            }
                        >
                            <div class="flex items-center justify-between">
                                <div class="flex-1">
                                    <h3 class="font-semibold text-base">{theme_name.clone()}</h3>
                                    <div class="mt-2 flex space-x-2">
                                        // Color preview circles
                                        <div 
                                            class="w-4 h-4 rounded-full border"
                                            style=format!("background-color: {}; border-color: var(--theme-sidebar-border)", theme.colors.background)
                                            title="Background color"
                                        />
                                        <div 
                                            class="w-4 h-4 rounded-full border"
                                            style=format!("background-color: {}; border-color: var(--theme-sidebar-border)", theme.colors.text.primary)
                                            title="Text color"
                                        />
                                        <div 
                                            class="w-4 h-4 rounded-full border"
                                            style=format!("background-color: {}; border-color: var(--theme-sidebar-border)", theme.colors.sidebar.background)
                                            title="Sidebar color"
                                        />
                                        <div 
                                            class="w-4 h-4 rounded-full border"
                                            style=format!("background-color: {}; border-color: var(--theme-sidebar-border)", theme.colors.buttons.primary.background)
                                            title="Accent color"
                                        />
                                    </div>
                                </div>
                                <Show
                                    when=move || current_theme_id() == theme_id_for_show
                                    fallback=|| view! { <></> }
                                >
                                    <svg
                                        width="20"
                                        height="20"
                                        viewBox="0 0 24 24"
                                        fill="none"
                                        stroke="currentColor"
                                        stroke-width="2"
                                        class="flex-shrink-0"
                                        style="color: var(--theme-button-primary-text)"
                                    >
                                        <path d="M20 6 9 17l-5-5"/>
                                    </svg>
                                </Show>
                            </div>
                        </button>
                    }
                }).collect_view()}
            </div>
            
            <div class="mt-4 pt-4 border-t" style="border-color: var(--theme-sidebar-border)">
                <div class="space-y-1">
                    <p class="text-xs opacity-75" style="color: var(--theme-text-muted)">
                        "Navigate & Apply: " 
                        <kbd class="px-1.5 py-0.5 rounded text-xs font-mono border" style="background-color: var(--theme-sidebar-background); border-color: var(--theme-sidebar-border)">
                            "Ctrl+J"
                        </kbd>
                        " / "
                        <kbd class="px-1.5 py-0.5 rounded text-xs font-mono border" style="background-color: var(--theme-sidebar-background); border-color: var(--theme-sidebar-border)">
                            "Ctrl+K"
                        </kbd>
                    </p>
                    <p class="text-xs opacity-75" style="color: var(--theme-text-muted)">
                        "Toggle: "
                        <kbd class="px-1.5 py-0.5 rounded text-xs font-mono border" style="background-color: var(--theme-sidebar-background); border-color: var(--theme-sidebar-border)">
                            "T"
                        </kbd>
                    </p>
                </div>
            </div>
        </div>
    }
}