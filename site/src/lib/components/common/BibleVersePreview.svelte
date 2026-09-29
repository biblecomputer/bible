<script lang="ts">
	import { onMount } from "svelte";
	import type { Translation } from "$lib/translations/translation";
	import type { TranslationContent } from "$lib/translations/translation";
	import { loadTranslationContent } from "$lib/translations/loadTranslationContent";
	import * as NavigationService from "$lib/services/NavigationService";
	import { getDisplayName } from "$lib/book";
	import { Effect } from "effect";

	let {
		path,
		x,
		y,
		translation,
		onClose,
	}: {
		path: string;
		x: number;
		y: number;
		translation: Translation;
		onClose: () => void;
	} = $props();

	let content = $state<TranslationContent | null>(null);
	let loading = $state(true);
	let error = $state<string | null>(null);
	let verseText = $state<string>("");
	let reference = $state<string>("");

	// Parse the path and extract verse info
	function parseAndLoadVerse() {
		const pathWithoutSlash = path.startsWith('/') ? path.slice(1) : path;
		const selection = NavigationService.parseReferenceUrl(pathWithoutSlash);

		if (!selection) {
			error = "Invalid reference";
			loading = false;
			return;
		}

		// Build reference string
		const bookName = getDisplayName(selection.start.book);
		if (selection.start.verse) {
			reference = `${bookName} ${selection.start.chapter}:${selection.start.verse}`;
		} else {
			reference = `${bookName} ${selection.start.chapter}`;
		}

		// Load translation content and extract verse
		Effect.runPromise(loadTranslationContent(translation))
			.then((translationContent) => {
				content = translationContent;

				// Find the book
				const book = translationContent.books.find(b => b.name === selection.start.book);
				if (!book) {
					error = "Book not found";
					loading = false;
					return;
				}

				// Find the chapter
				const chapter = book.chapters.find(c => c.chapter === selection.start.chapter);
				if (!chapter) {
					error = "Chapter not found";
					loading = false;
					return;
				}

				// Get verse(s)
				if (selection.start.verse) {
					// Single verse or verse range
					const startVerse = selection.start.verse;
					const endVerse = selection.end?.verse ?? startVerse;

					const verses = chapter.verses.filter(v =>
						v.verse >= startVerse && v.verse <= endVerse
					);

					if (verses.length === 0) {
						error = "Verse not found";
					} else {
						verseText = verses.map(v => `${v.verse} ${v.text}`).join(" ");
						if (startVerse !== endVerse) {
							reference = `${bookName} ${selection.start.chapter}:${startVerse}-${endVerse}`;
						}
					}
				} else {
					// Whole chapter - show first few verses
					const preview = chapter.verses.slice(0, 3);
					verseText = preview.map(v => `${v.verse} ${v.text}`).join(" ");
					if (chapter.verses.length > 3) {
						verseText += "...";
					}
				}

				loading = false;
			})
			.catch((err) => {
				error = "Failed to load";
				loading = false;
				console.error("Failed to load translation:", err);
			});
	}

	onMount(() => {
		parseAndLoadVerse();
	});

	// Position the tooltip, keeping it on screen
	let tooltipStyle = $derived(() => {
		const maxWidth = 320;
		const padding = 10;

		// Adjust x to keep tooltip on screen
		let adjustedX = x;
		if (typeof window !== 'undefined') {
			if (x + maxWidth + padding > window.innerWidth) {
				adjustedX = window.innerWidth - maxWidth - padding;
			}
		}

		// Position above or below based on available space
		let adjustedY = y + 20; // Below cursor by default
		if (typeof window !== 'undefined' && y + 200 > window.innerHeight) {
			adjustedY = y - 120; // Above cursor if near bottom
		}

		return `left: ${adjustedX}px; top: ${adjustedY}px;`;
	});
</script>

<div
	class="fixed z-50 max-w-xs bg-gray-800 light:bg-white border border-gray-600 light:border-gray-300 rounded-lg shadow-xl p-3"
	style={tooltipStyle()}
	onmouseleave={onClose}
>
	{#if loading}
		<div class="text-gray-400 light:text-gray-500 text-sm">Loading...</div>
	{:else if error}
		<div class="text-red-400 text-sm">{error}</div>
	{:else}
		<div class="text-blue-400 text-xs font-semibold mb-1">{reference}</div>
		<div class="text-gray-200 light:text-gray-800 text-sm leading-relaxed">{verseText}</div>
	{/if}
</div>
