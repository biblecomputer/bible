use leptos::prelude::*;
use web_sys;

#[component]
pub fn PdfLoadingProgress(
    /// Current progress value between 0.0 and 1.0
    progress: ReadSignal<f32>,
    /// Current status message
    status_message: ReadSignal<String>,
    /// Whether the loading dialog is visible
    is_visible: ReadSignal<bool>,
) -> impl IntoView {
    // Debug: Log when visibility changes
    Effect::new(move |_| {
        let visible = is_visible.get();
        web_sys::console::log_1(
            &format!("📊 PDF Progress Component visibility: {}", visible).into(),
        );
    });

    view! {
        <Show when=move || is_visible.get()>
            <div
                class="fixed inset-0 flex items-center justify-center"
                style="background: rgba(0,0,0,0.8); z-index: 9999; top: 0; left: 0; width: 100vw; height: 100vh;"
            >
                <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl p-6 w-96 max-w-full mx-4" style="background: white; border: 2px solid red;">
                    <div class="text-center">
                        // Title
                        <h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-4">
                            "Exporting PDF"
                        </h3>

                        // Progress Bar
                        <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-3 mb-4">
                            <div
                                class="bg-blue-600 h-3 rounded-full transition-all duration-300 ease-out"
                                style:width=move || format!("{}%", (progress.get() * 100.0) as u32)
                            ></div>
                        </div>

                        // Progress Percentage
                        <div class="text-sm font-medium text-blue-600 dark:text-blue-400 mb-2">
                            {move || format!("{}%", (progress.get() * 100.0) as u32)}
                        </div>

                        // Status Message
                        <div class="text-sm text-gray-600 dark:text-gray-400 mb-4">
                            {move || status_message.get()}
                        </div>

                        // Animated Spinner
                        <div class="flex items-center justify-center">
                            <svg
                                class="animate-spin h-5 w-5 text-blue-600 dark:text-blue-400"
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none"
                                viewBox="0 0 24 24"
                            >
                                <circle
                                    class="opacity-25"
                                    cx="12"
                                    cy="12"
                                    r="10"
                                    stroke="currentColor"
                                    stroke-width="4"
                                ></circle>
                                <path
                                    class="opacity-75"
                                    fill="currentColor"
                                    d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                                ></path>
                            </svg>
                            <span class="ml-2 text-sm text-gray-500 dark:text-gray-400">
                                "Processing..."
                            </span>
                        </div>
                    </div>
                </div>
            </div>
        </Show>
    }
}
