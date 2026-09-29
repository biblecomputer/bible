<script lang="ts">
    import { onMount } from "svelte";

    let {
        onChooseApp,
        onClose
    }: {
        onChooseApp: (appType: "bible" | "about" | "stopwatch" | "wiki" | "library") => void;
        onClose?: () => void;
    } = $props();

    type AppOption = {
        id: "bible" | "about" | "stopwatch" | "wiki" | "library";
        name: string;
        description: string;
    };

    const apps: AppOption[] = [
        { id: "bible", name: "bible", description: "Read and explore the Bible" },
        { id: "wiki", name: "wiki", description: "Browse the Bible encyclopedia" },
        { id: "library", name: "library", description: "Read public domain documents" },
        { id: "about", name: "about", description: "Learn more about Bible Computer" },
        { id: "stopwatch", name: "stopwatch", description: "Track time with a precise stopwatch" }
    ];

    let searchInputRef: HTMLInputElement;
    let searchQuery = $state("");
    let selectedIndex = $state(0);

    // Filter apps based on search query
    let filteredApps = $derived.by(() => {
        if (!searchQuery.trim()) {
            return apps;
        }
        
        const query = searchQuery.toLowerCase();
        return apps.filter(app => 
            app.name.toLowerCase().includes(query) || 
            app.description.toLowerCase().includes(query)
        );
    });

    // Reset selection when filtered apps change
    $effect(() => {
        if (selectedIndex >= filteredApps.length) {
            selectedIndex = 0;
        }
    });

    function handleSearchInput(event: Event) {
        const target = event.target as HTMLInputElement;
        searchQuery = target.value;
        selectedIndex = 0; // Reset selection when query changes
    }

    function handleKeydown(event: KeyboardEvent) {
        if (filteredApps.length === 0) return;

        if (event.key === 'ArrowDown') {
            event.preventDefault();
            selectedIndex = Math.min(selectedIndex + 1, filteredApps.length - 1);
        } else if (event.key === 'ArrowUp') {
            event.preventDefault();
            selectedIndex = Math.max(selectedIndex - 1, 0);
        } else if (event.key === 'Enter') {
            event.preventDefault();
            if (filteredApps[selectedIndex]) {
                selectApp(filteredApps[selectedIndex]);
            }
        } else if (event.key === 'Escape') {
            event.preventDefault();
            if (searchQuery.trim()) {
                // If there's a search query, clear it
                searchQuery = "";
                searchInputRef?.focus();
            } else {
                // If search is empty, close the app chooser tab
                onClose?.();
            }
        }
    }

    function selectApp(app: AppOption) {
        onChooseApp(app.id);
    }

    function handleAppClick(app: AppOption) {
        selectApp(app);
    }

    onMount(() => {
        // Auto-focus the search input
        if (searchInputRef) {
            searchInputRef.focus();
        }

        // Add global keyboard event listener
        const handleGlobalKeydown = (event: KeyboardEvent) => handleKeydown(event);
        if (typeof window !== 'undefined') {
            window.addEventListener('keydown', handleGlobalKeydown);
            return () => window.removeEventListener('keydown', handleGlobalKeydown);
        }
    });
</script>

<div class="h-full flex items-center justify-center bg-gray-900 light:bg-gray-50">
    <div class="max-w-md w-full mx-4">
        <div class="text-center mb-8">
            <h2 class="text-2xl font-bold text-gray-100 light:text-gray-900 mb-2">Choose App</h2>
            <p class="text-gray-400 light:text-gray-500">Type to search or use arrow keys</p>
        </div>

        <!-- Search Input -->
        <div class="mb-6 relative">
            <input
                bind:this={searchInputRef}
                type="text"
                placeholder="Search apps..."
                value={searchQuery}
                oninput={handleSearchInput}
                class="w-full px-4 py-3 bg-gray-800 light:bg-white border border-gray-600 light:border-gray-300 rounded-lg text-gray-200 light:text-gray-900 placeholder-gray-400 light:placeholder-gray-500 focus:outline-none focus:border-blue-500 focus:bg-gray-700 light:focus:bg-gray-100 transition-colors"
            />
            {#if searchQuery}
                <button
                    onclick={() => {
                        searchQuery = "";
                        searchInputRef?.focus();
                    }}
                    class="absolute right-3 top-1/2 transform -translate-y-1/2 text-gray-400 light:text-gray-500 hover:text-gray-200 light:hover:text-gray-700 transition-colors"
                >
                    ×
                </button>
            {/if}
        </div>

        <!-- App Results -->
        <div class="space-y-2">
            {#if filteredApps.length > 0}
                {#each filteredApps as app, index}
                    <button
                        onclick={() => handleAppClick(app)}
                        class="w-full p-4 text-left rounded-lg transition-colors {
                            index === selectedIndex 
                                ? 'bg-blue-600 text-white border border-blue-500' 
                                : 'bg-gray-800 light:bg-white hover:bg-gray-700 light:hover:bg-gray-100 text-gray-200 light:text-gray-800 border border-gray-600 light:border-gray-300 hover:border-gray-500 light:hover:border-gray-400'
                        }"
                    >
                        <div class="font-medium text-lg mb-1">
                            {app.name}
                        </div>
                        <div class="text-sm {index === selectedIndex ? 'text-blue-100' : 'text-gray-400 light:text-gray-500'}">
                            {app.description}
                        </div>
                    </button>
                {/each}
            {:else}
                <div class="text-center text-gray-500 light:text-gray-500 py-8">
                    <div class="text-sm">No apps found</div>
                    <div class="text-xs mt-1">Try "bible", "about", or "stopwatch"</div>
                </div>
            {/if}
        </div>

        <div class="mt-6 text-center text-xs text-gray-500 light:text-gray-500">
            Use ↑↓ arrow keys to navigate, Enter to select
        </div>
    </div>
</div>