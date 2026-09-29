<script lang="ts">
	import { onMount } from "svelte";
	import type { Snippet } from "svelte";

	let {
		searchPlaceholder = "Search...",
		isMobile = false,
		shouldFocusSearch = false,
		searchQuery = $bindable(""),
		isSearchFocused = $bindable(false),
		onSearchSubmit,
		children,
		searchResults,
	}: {
		searchPlaceholder?: string;
		isMobile?: boolean;
		shouldFocusSearch?: boolean;
		searchQuery?: string;
		isSearchFocused?: boolean;
		onSearchSubmit?: () => void;
		children: Snippet;
		searchResults?: Snippet;
	} = $props();

	let searchInputRef: HTMLInputElement;

	function handleSearchFocus() {
		isSearchFocused = true;
	}

	function handleSearchBlur() {
		// Small delay to allow clicks on search results
		setTimeout(() => {
			isSearchFocused = false;
			searchQuery = "";
		}, 150);
	}

	function handleSearchInput(event: Event) {
		const target = event.target as HTMLInputElement;
		searchQuery = target.value;
	}

	function handleKeydown(event: KeyboardEvent) {
		// Don't handle shortcuts if typing in an input/textarea (except for our search input)
		const target = event.target as HTMLElement;
		const isInOurInput = target === searchInputRef;
		const isTyping = target && (target.tagName === 'INPUT' || target.tagName === 'TEXTAREA' || target.contentEditable === 'true');

		// Global 'o' key to focus search (only when not typing)
		if (event.key === 'o' && !isSearchFocused && !isTyping && searchInputRef) {
			event.preventDefault();
			searchInputRef.focus();
			return;
		}

		// Handle Escape to blur search field (even when empty)
		if (isSearchFocused && event.key === 'Escape') {
			event.preventDefault();
			blurSearch();
			return;
		}

		// Handle Enter to submit and blur (when in search input)
		if (isInOurInput && event.key === 'Enter' && onSearchSubmit) {
			// Don't prevent default here - let SearchResults handle it
			// But schedule a blur after the submit
			setTimeout(() => {
				blurSearch();
			}, 0);
		}
	}

	// Public function to blur the search input
	export function blurSearch() {
		searchInputRef?.blur();
		isSearchFocused = false;
		searchQuery = "";
	}

	// Focus search input when shouldFocusSearch prop changes
	$effect(() => {
		if (shouldFocusSearch && searchInputRef) {
			setTimeout(() => {
				searchInputRef?.focus();
			}, 100);
		}
	});

	onMount(() => {
		const handleGlobalKeydown = (event: KeyboardEvent) => handleKeydown(event);
		if (typeof window !== 'undefined') {
			window.addEventListener('keydown', handleGlobalKeydown);
			return () => window.removeEventListener('keydown', handleGlobalKeydown);
		}
	});
</script>

<div class="h-full flex flex-col">
	<!-- Search Input -->
	<div class="p-4 border-b border-gray-700 light:border-gray-300">
		<div class="relative">
			<input
				bind:this={searchInputRef}
				type="text"
				placeholder={isMobile ? searchPlaceholder : `${searchPlaceholder} (o)`}
				value={searchQuery}
				oninput={handleSearchInput}
				onfocus={handleSearchFocus}
				onblur={handleSearchBlur}
				class="w-full px-3 py-2 bg-gray-800 light:bg-white border border-gray-600 light:border-gray-300 rounded-lg text-gray-200 light:text-gray-900 placeholder-gray-400 light:placeholder-gray-500 focus:outline-none focus:border-blue-500 focus:bg-gray-700 light:focus:bg-gray-100 transition-colors text-sm"
			/>
			{#if searchQuery}
				<button
					onclick={() => {
						searchQuery = "";
						searchInputRef?.focus();
					}}
					class="absolute right-2 top-1/2 transform -translate-y-1/2 text-gray-400 light:text-gray-500 hover:text-gray-200 light:hover:text-gray-700 transition-colors"
				>
					&times;
				</button>
			{/if}
		</div>
	</div>

	<!-- Content Area -->
	<div class="flex-1 overflow-hidden">
		{#if isSearchFocused && searchResults}
			{@render searchResults()}
		{:else}
			<div class="h-full overflow-y-auto">
				{@render children()}
			</div>
		{/if}
	</div>
</div>
