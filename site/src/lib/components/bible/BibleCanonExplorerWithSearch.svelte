<script lang="ts">
	import { BibleBook } from "$lib/book";
	import type { TranslationContent } from "$lib/translations/translation";
	import Sidebar from "$lib/components/ui/Sidebar.svelte";
	import BibleCanonExplorer from "$lib/components/bible/BibleCanonExplorer.svelte";
	import BibleSearchResults from "$lib/components/bible/BibleSearchResults.svelte";

	let {
		content,
		currentBook,
		currentChapter,
		onChapterSelect,
		shouldFocusSearch = false,
		isMobile = false
	}: {
		content: TranslationContent | null;
		currentBook: BibleBook;
		currentChapter: number;
		onChapterSelect: (book: string, chapter: number) => void;
		shouldFocusSearch?: boolean;
		isMobile?: boolean;
	} = $props();

	let searchQuery = $state("");
	let isSearchFocused = $state(false);
	let sidebarRef: Sidebar;

	function handleChapterSelect(bookShort: string, chapter: number) {
		onChapterSelect(bookShort, chapter);
		// Clear search and blur after selection
		sidebarRef?.blurSearch();
	}
</script>

<Sidebar
	bind:this={sidebarRef}
	searchPlaceholder="Search chapters..."
	{isMobile}
	{shouldFocusSearch}
	bind:searchQuery
	bind:isSearchFocused
>
	{#snippet children()}
		{#if content}
			<BibleCanonExplorer
				{content}
				{currentBook}
				{currentChapter}
				onChapterSelect={handleChapterSelect}
			/>
		{:else}
			<div class="flex items-center justify-center h-full text-gray-400 light:text-gray-500">
				<span>Loading...</span>
			</div>
		{/if}
	{/snippet}

	{#snippet searchResults()}
		{#if content}
			<BibleSearchResults
				{content}
				{searchQuery}
				onChapterSelect={handleChapterSelect}
			/>
		{/if}
	{/snippet}
</Sidebar>
