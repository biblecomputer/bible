use leptos::prelude::*;
use leptos::ev;
use leptos::web_sys::KeyboardEvent;

#[component]
pub fn ShortcutsHelp() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    
    // Listen for keyboard events on window
    window_event_listener(ev::keydown, move |evt: KeyboardEvent| {
        if evt.key() == "?" && !evt.ctrl_key() && !evt.meta_key() && !evt.alt_key() {
            evt.prevent_default();
            set_is_open.set(true);
        }
        if evt.key() == "Escape" && is_open.get() {
            evt.prevent_default();
            set_is_open.set(false);
        }
    });

    view! {
        <Show when=move || is_open.get() fallback=|| view! { <></> }>
            <div class="fixed inset-0 bg-black bg-opacity-50 z-[9999] flex items-center justify-center p-4">
                <div class="bg-white rounded-lg shadow-xl max-w-lg w-full p-6">
                    <div class="flex items-center justify-between mb-6">
                        <h2 class="text-lg font-bold text-black">Keyboard Shortcuts</h2>
                        <button
                            class="text-black hover:text-gray-600 transition-colors"
                            on:click=move |_| set_is_open.set(false)
                        >
                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                            </svg>
                        </button>
                    </div>
                    
                    <div class="space-y-4">
                        <div class="flex items-center justify-between">
                            <span class="text-black">Open Quick Switcher</span>
                            <div class="flex items-center gap-1">
                                <kbd class="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs">Cmd</kbd>
                                <span class="text-black">+</span>
                                <kbd class="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs">K</kbd>
                            </div>
                        </div>
                        
                        <div class="flex items-center justify-between">
                            <span class="text-black">Browse Current Chapter</span>
                            <div class="flex items-center gap-1">
                                <kbd class="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs">Cmd</kbd>
                                <span class="text-black">+</span>
                                <kbd class="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs">K</kbd>
                                <span class="text-gray-400 mx-1">then</span>
                                <kbd class="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs">:</kbd>
                            </div>
                        </div>
                        
                        <div class="flex items-center justify-between">
                            <span class="text-black">Go to Specific Verse</span>
                            <div class="flex items-center gap-1">
                                <kbd class="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs">Cmd</kbd>
                                <span class="text-black">+</span>
                                <kbd class="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs">K</kbd>
                                <span class="text-gray-400 mx-1">then</span>
                                <kbd class="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs">:5</kbd>
                            </div>
                        </div>
                        
                        <div class="flex items-center justify-between">
                            <span class="text-black">Toggle Books Sidebar</span>
                            <div class="flex items-center gap-1">
                                <kbd class="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs">Ctrl</kbd>
                                <span class="text-black">+</span>
                                <kbd class="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs">B</kbd>
                            </div>
                        </div>
                        
                        <div class="flex items-center justify-between">
                            <span class="text-black">Toggle Cross-References</span>
                            <div class="flex items-center gap-1">
                                <kbd class="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs">Ctrl</kbd>
                                <span class="text-black">+</span>
                                <kbd class="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs">Shift</kbd>
                                <span class="text-black">+</span>
                                <kbd class="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs">R</kbd>
                            </div>
                        </div>
                        
                        <div class="flex items-center justify-between">
                            <span class="text-black">Switch to Previous Chapter</span>
                            <kbd class="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs">S</kbd>
                        </div>
                        
                        <div class="flex items-center justify-between">
                            <span class="text-black">Copy Selected Verses</span>
                            <kbd class="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs">C</kbd>
                        </div>
                        
                        <div class="flex items-center justify-between">
                            <span class="text-black">Previous Chapter</span>
                            <div class="flex items-center gap-2">
                                <kbd class="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs">Left Arrow</kbd>
                                <span class="text-gray-400">or</span>
                                <kbd class="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs">H</kbd>
                            </div>
                        </div>
                        
                        <div class="flex items-center justify-between">
                            <span class="text-black">Next Chapter</span>
                            <div class="flex items-center gap-2">
                                <kbd class="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs">Right Arrow</kbd>
                                <span class="text-gray-400">or</span>
                                <kbd class="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs">L</kbd>
                            </div>
                        </div>
                        
                        <div class="flex items-center justify-between">
                            <span class="text-black">Previous Book</span>
                            <div class="flex items-center gap-1">
                                <kbd class="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs">Shift</kbd>
                                <span class="text-black">+</span>
                                <kbd class="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs">H</kbd>
                            </div>
                        </div>
                        
                        <div class="flex items-center justify-between">
                            <span class="text-black">Next Book</span>
                            <div class="flex items-center gap-1">
                                <kbd class="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs">Shift</kbd>
                                <span class="text-black">+</span>
                                <kbd class="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs">L</kbd>
                            </div>
                        </div>
                        
                        <div class="flex items-center justify-between">
                            <span class="text-black">Previous Verse</span>
                            <div class="flex items-center gap-2">
                                <kbd class="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs">Up Arrow</kbd>
                                <span class="text-gray-400">or</span>
                                <kbd class="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs">K</kbd>
                            </div>
                        </div>
                        
                        <div class="flex items-center justify-between">
                            <span class="text-black">Next Verse</span>
                            <div class="flex items-center gap-2">
                                <kbd class="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs">Down Arrow</kbd>
                                <span class="text-gray-400">or</span>
                                <kbd class="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs">J</kbd>
                            </div>
                        </div>
                        
                        <div class="flex items-center justify-between">
                            <span class="text-black">Go to Verse (instant)</span>
                            <div class="flex items-center gap-1">
                                <span class="text-gray-600 text-xs">Type</span>
                                <kbd class="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs">5</kbd>
                                <span class="text-gray-400">or</span>
                                <kbd class="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs">16</kbd>
                            </div>
                        </div>
                        
                        <div class="flex items-center justify-between">
                            <span class="text-black">First Verse of Chapter</span>
                            <div class="flex items-center gap-1">
                                <kbd class="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs">Shift</kbd>
                                <span class="text-black">+</span>
                                <kbd class="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs">K</kbd>
                            </div>
                        </div>
                        
                        <div class="flex items-center justify-between">
                            <span class="text-black">Last Verse of Chapter</span>
                            <div class="flex items-center gap-1">
                                <kbd class="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs">Shift</kbd>
                                <span class="text-black">+</span>
                                <kbd class="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs">J</kbd>
                            </div>
                        </div>
                        
                        <div class="flex items-center justify-between">
                            <span class="text-black">First Chapter of Bible</span>
                            <kbd class="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs">gg</kbd>
                        </div>
                        
                        <div class="flex items-center justify-between">
                            <span class="text-black">Last Chapter of Bible</span>
                            <div class="flex items-center gap-1">
                                <kbd class="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs">Shift</kbd>
                                <span class="text-black">+</span>
                                <kbd class="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs">G</kbd>
                            </div>
                        </div>
                        
                        <div class="flex items-center justify-between">
                            <span class="text-black">Show This Help</span>
                            <kbd class="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-xs">?</kbd>
                        </div>
                    </div>
                    
                    <div class="mt-6 pt-4 border-t border-gray-200">
                        <button
                            class="w-full px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors"
                            on:click=move |_| set_is_open.set(false)
                        >
                            Close
                        </button>
                    </div>
                </div>
            </div>
        </Show>
    }
}