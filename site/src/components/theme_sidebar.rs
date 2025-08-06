use leptos::prelude::*;
use crate::themes::{get_themes, Theme};
use crate::storage::{save_selected_theme, save_references_sidebar_open};
use crate::utils::is_mobile_screen;

#[component]
pub fn ThemeSidebar(
    current_theme: ReadSignal<Theme>,
    set_current_theme: WriteSignal<Theme>,
    set_sidebar_open: WriteSignal<bool>,
) -> impl IntoView {
    let themes = get_themes();
    
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
            
            <div class="flex-1 space-y-3">
                {themes.into_iter().map(|theme| {
                    let theme_clone = theme.clone();
                    let theme_id_for_style = theme.id.clone();
                    let theme_id_for_click = theme.id.clone();
                    let theme_id_for_show = theme.id.clone();
                    let theme_name = theme.name.clone();
                    let current_theme_id = move || current_theme.get().id.clone();
                    
                    view! {
                        <button
                            class="w-full p-4 rounded-lg border-2 transition-all duration-200 text-left group hover:shadow-md"
                            style=move || {
                                if current_theme_id() == theme_id_for_style {
                                    "border-color: var(--theme-button-primary-background); background-color: var(--theme-button-primary-background); color: var(--theme-button-primary-text); transform: scale(1.02)"
                                } else {
                                    "border-color: var(--theme-sidebar-border); background-color: var(--theme-sidebar-background); color: var(--theme-text-primary)"
                                }
                            }
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
                <p class="text-xs opacity-75" style="color: var(--theme-text-muted)">
                    "Press " 
                    <kbd class="px-1.5 py-0.5 rounded text-xs font-mono border" style="background-color: var(--theme-sidebar-background); border-color: var(--theme-sidebar-border)">
                        "Shift+T"
                    </kbd>
                    " to toggle this panel"
                </p>
            </div>
        </div>
    }
}