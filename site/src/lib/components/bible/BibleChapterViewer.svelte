<script lang="ts">
	import type { Chapter } from "$lib/translations/translation";
	import { BibleBook, getDisplayName } from "$lib/book";

	let {
		chapter,
		book,
		chapterNumber,
		showBookHeader = false,
		selectedVerse = null as number | null,
		selectionRange = null as { start: number; end: number } | null,
		onVerseClick = (_verse: number) => {},
		onWikiLinkClick = (_page: string) => {}
	}: {
		chapter: Chapter;
		book: BibleBook;
		chapterNumber: number;
		showBookHeader: boolean;
		selectedVerse?: number | null;
		selectionRange?: { start: number; end: number } | null;
		onVerseClick?: (verse: number) => void;
		onWikiLinkClick?: (page: string) => void;
	} = $props();

	// Check if a verse is in the selection range
	const isVerseInSelection = (verseNum: number): boolean => {
		if (!selectionRange) return false;
		return verseNum >= selectionRange.start && verseNum <= selectionRange.end;
	};

	// Parse markdown-style wiki links [text](/wiki/page) to HTML
	function parseWikiLinks(text: string): string {
		return text.replace(/\[([^\]]+)\]\(\/wiki\/([^)]+)\)/g, (_, linkText, wikiPage) => {
			return `<a href="/wiki/${wikiPage}" class="wiki-link underline decoration-blue-400 hover:decoration-blue-300 cursor-pointer" data-wiki-page="${wikiPage}">${linkText}</a>`;
		});
	}

	// Handle click events on the verse text
	function handleTextClick(event: MouseEvent) {
		const target = event.target as HTMLElement;
		if (target.classList.contains('wiki-link')) {
			event.preventDefault();
			event.stopPropagation();
			const wikiPage = target.getAttribute('data-wiki-page');
			if (wikiPage) {
				onWikiLinkClick(wikiPage);
			}
		}
	}
</script>

<div class="w-full">
	<div class="px-8 py-2">
		<div class="max-w-2xl mx-auto">
			<!-- Book header (when crossing book boundaries) -->
			{#if showBookHeader}
				<header class="mt-16 mb-24 text-center">
					<h1 class="text-5xl md:text-7xl font-bold text-gray-100 light:text-gray-900 uppercase tracking-wide">{getDisplayName(book)}</h1>
					<div class="h-1 w-32 bg-blue-500 rounded mx-auto mt-3"></div>
				</header>
			{/if}

			<!-- Verses as continuous paragraph with drop-cap chapter number -->
			<div class="text-gray-200 light:text-gray-800 leading-relaxed text-lg">
				{#each chapter.verses as verse, i}
					{@const isSelected = isVerseInSelection(verse.verse)}
					<span
						id="verse-{book}-{chapterNumber}-{verse.verse}"
						class="verse-marker focus:outline-none cursor-pointer rounded-sm {selectedVerse === verse.verse || isSelected ? 'bg-amber-400/30' : ''}"
						tabindex="-1"
						onclick={(e) => {
							// Only trigger verse click if not clicking a wiki link
							const target = e.target as HTMLElement;
							if (!target.classList.contains('wiki-link')) {
								onVerseClick(verse.verse);
							}
						}}
						role="button"
					>{#if i === 0}<h2 id="chapter-{book}-{chapterNumber}" tabindex="-1" aria-label="{getDisplayName(book)} {chapterNumber}" class="float-left text-5xl font-bold text-gray-400 light:text-gray-500 mr-2 leading-none mt-1 focus:outline-none">{chapterNumber}</h2>{:else}<sup class="text-blue-400 font-medium text-xs mr-1">{verse.verse}</sup>{/if}<span class="text-gray-200 light:text-gray-800" onclick={handleTextClick}>{@html parseWikiLinks(verse.text)} </span></span>
				{/each}
			</div>
		</div>
	</div>
</div>
