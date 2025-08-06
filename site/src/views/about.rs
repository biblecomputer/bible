use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <article class="max-w-2xl mx-auto px-4 py-12">
            <h1 class="text-2xl font-bold mb-8" style="color: var(--theme-text-primary)">"About"</h1>
            
            <div class="space-y-8 text-sm leading-relaxed" style="color: var(--theme-text-primary)">
                <section>
                    <p class="mb-4">
                        "A keyboard-driven Bible reading website. Built with Rust and open source."
                    </p>
                    <a 
                        href="https://github.com/sempruijs/bible" 
                        target="_blank" 
                        rel="noopener noreferrer"
                        class="hover:underline translation-link"
                    >
                        "View source on GitHub"
                    </a>
                </section>

                <section>
                    <h2 class="font-medium mb-3" style="color: var(--theme-text-primary)">"Basic Navigation"</h2>
                    <div class="space-y-1 font-mono text-xs">
                        <div class="flex justify-between">
                            <span style="color: var(--theme-text-secondary)">"Next verse"</span>
                            <span style="color: var(--theme-text-muted)">"j"</span>
                        </div>
                        <div class="flex justify-between">
                            <span style="color: var(--theme-text-secondary)">"Previous verse"</span>
                            <span style="color: var(--theme-text-muted)">"k"</span>
                        </div>
                        <div class="flex justify-between">
                            <span style="color: var(--theme-text-secondary)">"Open command palette"</span>
                            <span style="color: var(--theme-text-muted)">"Ctrl+O"</span>
                        </div>
                        <div class="flex justify-between">
                            <span style="color: var(--theme-text-secondary)">"Navigate palette results"</span>
                            <span style="color: var(--theme-text-muted)">"Ctrl+J/K"</span>
                        </div>
                    </div>
                </section>

                <section>
                    <h2 class="font-medium mb-3" style="color: var(--theme-text-primary)">"Command Palette"</h2>
                    <p class="mb-2">
                        "Press " <code class="px-1 rounded text-xs about-code">"Ctrl+O"</code> " to search:"
                    </p>
                    <ul class="space-y-1 text-xs">
                        <li style="color: var(--theme-text-secondary)">"• Bible text: " <code class="px-1 rounded about-code">"love"</code></li>
                        <li style="color: var(--theme-text-secondary)">"• Verses: " <code class="px-1 rounded about-code">"John 3:16"</code></li>
                        <li style="color: var(--theme-text-secondary)">"• Chapters: " <code class="px-1 rounded about-code">"Genesis 1"</code></li>
                        <li style="color: var(--theme-text-secondary)">"• Commands: " <code class="px-1 rounded about-code">">copy"</code></li>
                    </ul>
                </section>
            </div>
        </article>
    }
}