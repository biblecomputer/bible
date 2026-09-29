<script lang="ts" generics="T">
	import { onMount } from "svelte";
	import type { Snippet } from "svelte";

	let {
		items,
		searchQuery = "",
		filterFn,
		onSelect,
		emptyMessage = "No matches found",
		emptyHint = "Try a different search term",
		placeholderMessage = "Type to search",
		placeholderHint = "",
		resultSnippet,
	}: {
		items: T[];
		searchQuery: string;
		filterFn: (item: T, query: string) => boolean;
		onSelect: (item: T) => void;
		emptyMessage?: string;
		emptyHint?: string;
		placeholderMessage?: string;
		placeholderHint?: string;
		resultSnippet: Snippet<[{ item: T; isSelected: boolean }]>;
	} = $props();

	let selectedIndex = $state(0);

	let filteredResults = $derived.by(() => {
		if (!searchQuery.trim()) return [];
		const query = searchQuery.toLowerCase().trim();
		return items.filter(item => filterFn(item, query)).slice(0, 20);
	});

	function handleResultClick(item: T) {
		onSelect(item);
	}

	function handleKeyNavigation(event: KeyboardEvent) {
		if (filteredResults.length === 0) return;

		if (event.key === 'ArrowDown') {
			event.preventDefault();
			selectedIndex = Math.min(selectedIndex + 1, filteredResults.length - 1);
		} else if (event.key === 'ArrowUp') {
			event.preventDefault();
			selectedIndex = Math.max(selectedIndex - 1, 0);
		} else if (event.key === 'Enter') {
			event.preventDefault();
			if (filteredResults[selectedIndex]) {
				handleResultClick(filteredResults[selectedIndex]);
			}
		}
	}

	// Reset selection when search results change
	$effect(() => {
		filteredResults;
		selectedIndex = 0;
	});

	// Key navigation handler
	onMount(() => {
		const handler = (event: KeyboardEvent) => handleKeyNavigation(event);
		if (typeof window !== 'undefined') {
			window.addEventListener('keydown', handler);
			return () => window.removeEventListener('keydown', handler);
		}
	});
</script>

<div class="p-4 h-full overflow-y-auto">
	{#if searchQuery.trim()}
		{#if filteredResults.length > 0}
			<div class="space-y-1">
				<div class="text-xs text-gray-400 light:text-gray-500 mb-2">
					{filteredResults.length} result{filteredResults.length === 1 ? '' : 's'}
				</div>
				{#each filteredResults as item, index}
					<button
						onclick={() => handleResultClick(item)}
						class="w-full text-left px-3 py-2 rounded text-gray-200 light:text-gray-800 hover:bg-gray-700 light:hover:bg-gray-200 transition-colors
							{index === selectedIndex ? 'bg-gray-600 light:bg-gray-300' : 'bg-gray-800 light:bg-gray-100'}"
					>
						{@render resultSnippet({ item, isSelected: index === selectedIndex })}
					</button>
				{/each}
			</div>
		{:else}
			<div class="text-center text-gray-500 light:text-gray-500 py-8">
				<div class="text-sm">{emptyMessage}</div>
				{#if emptyHint}
					<div class="text-xs mt-1">{emptyHint}</div>
				{/if}
			</div>
		{/if}
	{:else}
		<div class="text-center text-gray-500 py-8">
			<div class="text-sm">{placeholderMessage}</div>
			{#if placeholderHint}
				<div class="text-xs mt-1">{placeholderHint}</div>
			{/if}
		</div>
	{/if}
</div>
