use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <article class="max-w-2xl mx-auto px-4 py-12">
            <h1 class="text-2xl font-bold mb-8">"About"</h1>
            
            <div class="space-y-8 text-sm leading-relaxed">
                <section>
                    <p class="mb-4">
                        "A keyboard-driven Bible reading website. Built with Rust and open source."
                    </p>
                    <a 
                        href="https://github.com/sempruijs/bible" 
                        target="_blank" 
                        rel="noopener noreferrer"
                        class="text-blue-600 hover:underline"
                    >
                        "View source on GitHub"
                    </a>
                </section>

                <section>
                    <h2 class="font-medium mb-3">"Basic Navigation"</h2>
                    <div class="space-y-1 font-mono text-xs">
                        <div class="flex justify-between">
                            <span>"Next verse"</span>
                            <span>"j"</span>
                        </div>
                        <div class="flex justify-between">
                            <span>"Previous verse"</span>
                            <span>"k"</span>
                        </div>
                        <div class="flex justify-between">
                            <span>"Open command palette"</span>
                            <span>"Ctrl+O"</span>
                        </div>
                        <div class="flex justify-between">
                            <span>"Navigate palette results"</span>
                            <span>"Ctrl+J/K"</span>
                        </div>
                    </div>
                </section>

                <section>
                    <h2 class="font-medium mb-3">"Command Palette"</h2>
                    <p class="mb-2">
                        "Press " <code class="px-1 bg-gray-100 rounded text-xs">"Ctrl+O"</code> " to search:"
                    </p>
                    <ul class="space-y-1 text-xs">
                        <li>"• Bible text: " <code class="px-1 bg-gray-100 rounded">"love"</code></li>
                        <li>"• Verses: " <code class="px-1 bg-gray-100 rounded">"John 3:16"</code></li>
                        <li>"• Chapters: " <code class="px-1 bg-gray-100 rounded">"Genesis 1"</code></li>
                        <li>"• Commands: " <code class="px-1 bg-gray-100 rounded">">copy"</code></li>
                    </ul>
                </section>
            </div>
        </article>
    }
}