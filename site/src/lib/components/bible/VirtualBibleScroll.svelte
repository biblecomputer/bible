<script lang="ts">
	import { Option } from "effect";
	import { VList } from "virtua/svelte";
	import type { TranslationContent, Chapter } from "$lib/translations/translation";
	import { BibleBook } from "$lib/book";
	import type { BibleSelection, BibleReference } from "$lib/app";
	import { protestantBookOrder } from "$lib/translations/bookOrder";
	import BibleChapterViewer from "$lib/components/bible/BibleChapterViewer.svelte";

	let {
		translationContent,
		scrollTarget,
		selection = null as BibleSelection | null,
		onScrollStateChange = () => {},
		onVerseSelect = (_book: BibleBook, _chapter: number, _verse: number) => {},
		onWikiLinkClick = (_page: string) => {},
		isActive = true
	}: {
		translationContent: Option.Option<TranslationContent>;
		scrollTarget: { book: BibleBook; chapter: number; verse: number | null; version: number };
		selection?: BibleSelection | null;
		onScrollStateChange?: (book: BibleBook, chapter: number, verse: number | null) => void;
		onVerseSelect?: (book: BibleBook, chapter: number, verse: number) => void;
		onWikiLinkClick?: (page: string) => void;
		isActive?: boolean;
	} = $props();

	// Get the book order index for comparison
	const getBookIndex = (book: BibleBook): number => protestantBookOrder.books.indexOf(book);

	// Compare two references: returns -1 if a < b, 0 if a == b, 1 if a > b
	const compareRefs = (a: BibleReference, b: BibleReference): number => {
		const bookIndexA = getBookIndex(a.book);
		const bookIndexB = getBookIndex(b.book);
		if (bookIndexA !== bookIndexB) return bookIndexA < bookIndexB ? -1 : 1;
		if (a.chapter !== b.chapter) return a.chapter < b.chapter ? -1 : 1;
		const verseA = a.verse ?? 1;
		const verseB = b.verse ?? 1;
		if (verseA !== verseB) return verseA < verseB ? -1 : 1;
		return 0;
	};

	// Check if a verse is within the selection range
	const isVerseSelected = (book: BibleBook, chapter: number, verse: number): boolean => {
		if (!selection) return false;

		const ref: BibleReference = { book, chapter, verse };
		const start = selection.start;
		const end = selection.end ?? selection.start;

		// Handle single chapter selection (no verse specified)
		const effectiveStart = { ...start, verse: start.verse ?? 1 };
		const effectiveEnd = { ...end, verse: end.verse ?? 9999 }; // Large number to include all verses

		return compareRefs(ref, effectiveStart) >= 0 && compareRefs(ref, effectiveEnd) <= 0;
	};

	// Get selected verse range for a chapter (returns null if chapter not in selection)
	const getChapterSelection = (book: BibleBook, chapter: number): { start: number; end: number } | null => {
		if (!selection) return null;

		const start = selection.start;
		const end = selection.end ?? selection.start;

		const bookIndex = getBookIndex(book);
		const startBookIndex = getBookIndex(start.book);
		const endBookIndex = getBookIndex(end.book);

		// Check if this chapter is in the selection range
		if (bookIndex < startBookIndex || bookIndex > endBookIndex) return null;
		if (bookIndex === startBookIndex && chapter < start.chapter) return null;
		if (bookIndex === endBookIndex && chapter > end.chapter) return null;

		// Determine start verse for this chapter
		let startVerse = 1;
		if (bookIndex === startBookIndex && chapter === start.chapter) {
			startVerse = start.verse ?? 1;
		}

		// Determine end verse for this chapter (9999 means to end of chapter)
		let endVerse = 9999;
		if (bookIndex === endBookIndex && chapter === end.chapter) {
			endVerse = end.verse ?? 9999;
		}

		return { start: startVerse, end: endVerse };
	};

	interface ChapterItem {
		book: BibleBook;
		chapterNumber: number;
		chapter: Chapter;
		showBookHeader: boolean;
	}

	// Build flat list of all chapters
	let items = $state<ChapterItem[]>([]);
	let virtualListRef: any = $state(null);

	$effect(() => {
		if (Option.isSome(translationContent)) {
			const chapterItems: ChapterItem[] = [];

			protestantBookOrder.books.forEach((book) => {
				const bookData = translationContent.value.books.find(b => b.name === book);

				if (bookData) {
					bookData.chapters.forEach((chapterData, index) => {
						chapterItems.push({
							book: book,
							chapterNumber: chapterData.chapter,
							chapter: chapterData,
							showBookHeader: index === 0
						});
					});
				}
			});

			items = chapterItems;
		}
	});

	// Track the last version and position we scrolled to
	let lastScrolledVersion = $state(0);
	let lastScrolledBook = $state<BibleBook | null>(null);
	let lastScrolledChapter = $state<number | null>(null);
	// Flag to pause scroll tracking during navigation
	let isNavigating = $state(false);

	// Scroll to chapter/verse when scrollTarget.version changes (explicit navigation)
	$effect(() => {
		const currentVersion = scrollTarget.version;

		// Only scroll if version actually incremented
		if (currentVersion > lastScrolledVersion && virtualListRef && items.length > 0) {
			// Pause scroll tracking during navigation
			isNavigating = true;
			lastScrolledVersion = currentVersion;

			// Check if we need to scroll to a different chapter
			const chapterChanged = lastScrolledBook !== scrollTarget.book || lastScrolledChapter !== scrollTarget.chapter;

			if (chapterChanged) {
				const targetIndex = items.findIndex(
					item => item.book === scrollTarget.book && item.chapterNumber === scrollTarget.chapter
				);

				if (targetIndex >= 0) {
					virtualListRef.scrollToIndex(targetIndex, { align: 'start' });
					lastScrolledBook = scrollTarget.book;
					lastScrolledChapter = scrollTarget.chapter;
				}

				// After scrolling to chapter, scroll to specific verse if specified
				setTimeout(() => {
					if (scrollTarget.verse) {
						const verseId = `verse-${scrollTarget.book}-${scrollTarget.chapter}-${scrollTarget.verse}`;
						const verseElement = document.getElementById(verseId);
						if (verseElement) {
							verseElement.scrollIntoView({ block: 'start' });
							verseElement.focus();
						}
					} else {
						const headingId = `chapter-${scrollTarget.book}-${scrollTarget.chapter}`;
						const heading = document.getElementById(headingId);
						if (heading) {
							heading.focus();
						}
					}
					setTimeout(() => { isNavigating = false; }, 100);
				}, 100);
			} else {
				// Same chapter - just scroll to the verse directly
				if (scrollTarget.verse) {
					const verseId = `verse-${scrollTarget.book}-${scrollTarget.chapter}-${scrollTarget.verse}`;
					const verseElement = document.getElementById(verseId);
					if (verseElement) {
						verseElement.scrollIntoView({ block: 'start', behavior: 'smooth' });
						verseElement.focus();
					}
				}
				setTimeout(() => { isNavigating = false; }, 100);
			}
		}
	});

	// Force virtual list to recalculate when tab becomes active
	$effect(() => {
		if (isActive && virtualListRef) {
			// Small delay to ensure tab transition completes
			setTimeout(() => {
				if (virtualListRef && typeof virtualListRef.scrollTo === 'function') {
					// Trigger a tiny scroll to force re-render without changing visible content
					const currentOffset = virtualListRef.getScrollOffset();
					virtualListRef.scrollTo(currentOffset);
				}
			}, 50);
		}
	});

	// Track scroll position and update URL/canon explorer (but don't trigger scroll)
	let lastReportedBook = $state<BibleBook | null>(null);
	let lastReportedChapter = $state<number | null>(null);
	let lastReportedVerse = $state<number | null>(null);
	let scrollThrottleTimer: ReturnType<typeof setTimeout> | null = null;

	// Find the first visible verse in the viewport
	function findVisibleVerse(): { book: BibleBook; chapter: number; verse: number } | null {
		const verseElements = document.querySelectorAll('.verse-marker');
		for (const element of verseElements) {
			const rect = element.getBoundingClientRect();
			// Check if the verse is visible in the viewport (use a buffer zone to avoid flickering)
			if (rect.top >= -20 && rect.top < window.innerHeight / 3) {
				const id = element.id; // verse-{book}-{chapter}-{verse}
				const parts = id.replace('verse-', '').match(/^(.+)-(\d+)-(\d+)$/);
				if (parts) {
					return {
						book: parts[1] as BibleBook,
						chapter: parseInt(parts[2]),
						verse: parseInt(parts[3])
					};
				}
			}
		}
		return null;
	}

	function updateScrollState() {
		if (!virtualListRef || items.length === 0) {
			return;
		}

		try {
			const scrollOffset = virtualListRef.getScrollOffset();
			const visibleIndex = virtualListRef.findItemIndex(scrollOffset + 10);

			if (visibleIndex >= 0 && visibleIndex < items.length) {
				const visibleItem = items[visibleIndex];

				// Try to find the specific verse that's visible
				const visibleVerse = findVisibleVerse();
				const verse = visibleVerse?.verse ?? null;

				// Only update if something changed
				if (lastReportedBook !== visibleItem.book || lastReportedChapter !== visibleItem.chapterNumber || lastReportedVerse !== verse) {
					lastReportedBook = visibleItem.book;
					lastReportedChapter = visibleItem.chapterNumber;
					lastReportedVerse = verse;

					// Update URL and canon explorer (but NOT scrollTarget, so no scroll triggered)
					onScrollStateChange(visibleItem.book, visibleItem.chapterNumber, verse);
				}
			}
		} catch (error) {
			console.error('Error detecting scroll position:', error);
		}
	}

	function handleScroll() {
		// Skip scroll tracking during navigation to avoid state conflicts
		if (isNavigating) return;

		// Throttle scroll updates to avoid flickering
		if (scrollThrottleTimer) return;

		scrollThrottleTimer = setTimeout(() => {
			scrollThrottleTimer = null;
			if (!isNavigating) {
				updateScrollState();
			}
		}, 100);
	}
</script>

{#if items.length > 0}
	<VList bind:this={virtualListRef} data={items} style="height: 100%;" onscroll={handleScroll}>
		{#snippet children(item)}
			{@const isCurrentChapter = item.book === scrollTarget.book && item.chapterNumber === scrollTarget.chapter}
			{@const chapterSelection = getChapterSelection(item.book, item.chapterNumber)}
			<BibleChapterViewer
				chapter={item.chapter}
				book={item.book}
				chapterNumber={item.chapterNumber}
				showBookHeader={item.showBookHeader}
				selectedVerse={isCurrentChapter ? scrollTarget.verse : null}
				selectionRange={chapterSelection}
				onVerseClick={(verse) => onVerseSelect(item.book, item.chapterNumber, verse)}
				{onWikiLinkClick}
			/>
		{/snippet}
	</VList>
{:else}
	<div class="flex items-center justify-center h-full">
		<div class="text-center text-gray-400 light:text-gray-500">
			<p>Loading Bible...</p>
		</div>
	</div>
{/if}
