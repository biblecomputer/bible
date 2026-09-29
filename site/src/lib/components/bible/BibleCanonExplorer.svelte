<script lang="ts">
	import { BibleBook, getDisplayName, getShortName } from "$lib/book";
	import { protestantBookOrder } from "$lib/translations/bookOrder";
	import type { TranslationContent } from "$lib/translations/translation";
	import { Option } from "effect";

	let {
		content,
		currentBook,
		currentChapter,
		onChapterSelect
	}: {
		content: TranslationContent | null;
		currentBook: BibleBook;
		currentChapter: number;
		onChapterSelect: (book: string, chapter: number) => void;
	} = $props();

	// Use Greek order as default (keeping the type system for potential future use)
	const bookOrder = protestantBookOrder;
	let selectedBook = $state<Option.Option<BibleBook>>(Option.none());
	let chapterButtonRefs = $state<Map<number, HTMLButtonElement>>(new Map());

	// Auto-expand the book that contains the current chapter
	$effect(() => {
		if (currentBook) {
			const book = orderedBooks.find(book => book.name === currentBook);
			if (book) {
				selectedBook = Option.some(book.name);
			}
		}
	});

	// Auto-scroll to current chapter when it changes
	$effect(() => {
		if (currentChapter &&
			Option.isSome(selectedBook) &&
			selectedBook.value === currentBook) {

			// Small delay to ensure book expansion completes before scrolling
			setTimeout(() => {
				const buttonRef = chapterButtonRefs.get(currentChapter);
				if (buttonRef) {
					buttonRef.scrollIntoView({
						behavior: 'auto',
						block: 'nearest',
						inline: 'nearest'
					});
				}
			}, 150);
		}
	});

	function setSelected(book: BibleBook) {
		const result =
			Option.isSome(selectedBook) && selectedBook.value === book
				? Option.none()
				: Option.some(book);
		selectedBook = result;
	}

	function trackChapterRef(node: HTMLButtonElement, chapterNum: number) {
		chapterButtonRefs.set(chapterNum, node);
		return {
			destroy() {
				chapterButtonRefs.delete(chapterNum);
			}
		};
	}

	let orderedBooks = $derived(
		content ? bookOrder.books
			.flatMap((bookName) => {
				const book = content.books.find((book) => book.name === bookName);
				return book ? [book] : [];
			}) : []
	);
</script>

<div class="p-4">
	<div class="space-y-4">
		{#each orderedBooks as book}
			<div class="border border-gray-600 light:border-gray-300 rounded-lg overflow-hidden bg-gray-800 light:bg-gray-100 transition-all duration-200">
				<button
					onclick={() => setSelected(book.name)}
					class="w-full px-4 py-3 text-left font-medium text-gray-200 light:text-gray-800 bg-gray-800 light:bg-gray-100 hover:bg-gray-700 light:hover:bg-gray-200 transition-colors flex justify-between items-center"
					aria-label="{getDisplayName(book.name)}, {Option.isSome(selectedBook) && selectedBook.value === book.name ? 'expanded' : 'collapsed'}"
					aria-expanded={Option.isSome(selectedBook) && selectedBook.value === book.name}
				>
					<span>{getDisplayName(book.name)}</span>
					<span class="text-sm text-gray-400 light:text-gray-500" aria-hidden="true">
						{Option.isSome(selectedBook) && selectedBook.value === book.name ? '↓' : '→'}
					</span>
				</button>
				
				{#if Option.isSome(selectedBook) && selectedBook.value === book.name}
					<div class="p-4 bg-gray-900 light:bg-white border-t border-gray-700 light:border-gray-300">
						<div class="flex flex-col gap-3 max-w-fit">
							{#each Array.from({ length: Math.ceil(book.chapters.length / 5) }, (_, rowIndex) => book.chapters.slice(rowIndex * 5, (rowIndex + 1) * 5)) as chapterRow}
								<div class="flex gap-3">
									{#each chapterRow as chapter}
										{@const isCurrentChapter = currentBook && 
											book.name === currentBook && 
											chapter.chapter === currentChapter}
										<button
											use:trackChapterRef={chapter.chapter}
											onclick={() => onChapterSelect(getShortName(book.name), chapter.chapter)}
											class="flex items-center justify-center w-12 h-12 font-medium rounded border transition-colors {
												isCurrentChapter
													? 'bg-blue-600 text-white border-blue-500 hover:bg-blue-700'
													: 'bg-gray-800 light:bg-gray-100 hover:bg-gray-700 light:hover:bg-gray-200 text-gray-200 light:text-gray-800 border-gray-600 light:border-gray-300 hover:border-gray-500 light:hover:border-gray-400'
											}"
										>
											{chapter.chapter}
										</button>
									{/each}
								</div>
							{/each}
						</div>
					</div>
				{/if}
			</div>
		{/each}
	</div>
</div>