import { Data, Schema, Option } from "effect";
import type { BibleBook } from "$lib/book";
import { BibleBookSchema, getDisplayName as getBibleBookDisplayName, getShortName } from "$lib/book";
import type { Translation } from "$lib/translations/translation";
import { TranslationSchema } from "$lib/translations/translation";

// Bible reference - a point in the Bible (book, chapter, optional verse)
export const BibleReferenceSchema = Schema.Struct({
	book: BibleBookSchema,
	chapter: Schema.Number,
	verse: Schema.NullOr(Schema.Number)
});

// Bible selection - a range from start to optional end
export const BibleSelectionSchema = Schema.Struct({
	start: BibleReferenceSchema,
	end: Schema.NullOr(BibleReferenceSchema)
});

// Effect Schema definitions - App-specific state
export const BibleStateSchema = Schema.Struct({
	currentBook: BibleBookSchema,
	currentChapter: Schema.Number,
	currentVerse: Schema.NullOr(Schema.Number),
	// Selection state (separate from scroll state)
	selection: Schema.NullOr(BibleSelectionSchema),
	translation: TranslationSchema,
	showCanonExplorer: Schema.Boolean
});

export const StopwatchStateSchema = Schema.Struct({
	elapsedTime: Schema.Number, // in milliseconds
	isRunning: Schema.Boolean
});

export const WikiStateSchema = Schema.Struct({
	page: Schema.String, // The wiki page name (e.g., "Abraham")
	showSidebar: Schema.Boolean
});

export const LibraryStateSchema = Schema.Struct({
	document: Schema.String, // The document path (e.g., "en/sempruijs/the-goodness-of-God")
	showSidebar: Schema.Boolean
});

// App schema - represents the application running in a tab
export const AppSchema = Schema.Union(
	Schema.Struct({
		_tag: Schema.Literal("Bible"),
		bibleState: BibleStateSchema
	}),
	Schema.Struct({
		_tag: Schema.Literal("About")
	}),
	Schema.Struct({
		_tag: Schema.Literal("ChooseApp")
	}),
	Schema.Struct({
		_tag: Schema.Literal("Stopwatch"),
		stopwatchState: StopwatchStateSchema
	}),
	Schema.Struct({
		_tag: Schema.Literal("Wiki"),
		wikiState: WikiStateSchema
	}),
	Schema.Struct({
		_tag: Schema.Literal("Library"),
		libraryState: LibraryStateSchema
	})
);

// TabState schema - combines ID with app
export const TabStateSchema = Schema.Struct({
	id: Schema.String,
	app: AppSchema
});

// TabsState schema - the complete tabs system state
export const TabsStateSchema = Schema.Struct({
	tabs: Schema.Array(TabStateSchema),
	activeTabId: Schema.String,
	nextTabId: Schema.Number
});

// Helper type to make deeply readonly types writable (for Svelte $state compatibility)
type DeepWritable<T> = T extends object
	? { -readonly [P in keyof T]: DeepWritable<T[P]> }
	: T;

// Type exports - Schema.Type generates readonly types, make them writable for $state
type BibleReferenceReadonly = Schema.Schema.Type<typeof BibleReferenceSchema>;
type BibleSelectionReadonly = Schema.Schema.Type<typeof BibleSelectionSchema>;
type BibleStateReadonly = Schema.Schema.Type<typeof BibleStateSchema>;
type StopwatchStateReadonly = Schema.Schema.Type<typeof StopwatchStateSchema>;
type WikiStateReadonly = Schema.Schema.Type<typeof WikiStateSchema>;
type LibraryStateReadonly = Schema.Schema.Type<typeof LibraryStateSchema>;
type AppReadonly = Schema.Schema.Type<typeof AppSchema>;
type TabStateReadonly = Schema.Schema.Type<typeof TabStateSchema>;
type TabsStateReadonly = Schema.Schema.Type<typeof TabsStateSchema>;

export type BibleReference = DeepWritable<BibleReferenceReadonly>;
export type BibleSelection = DeepWritable<BibleSelectionReadonly>;
export type BibleState = DeepWritable<BibleStateReadonly>;
export type StopwatchState = DeepWritable<StopwatchStateReadonly>;
export type WikiState = DeepWritable<WikiStateReadonly>;
export type LibraryState = DeepWritable<LibraryStateReadonly>;
export type App = DeepWritable<AppReadonly>;
export type TabState = DeepWritable<TabStateReadonly>;
export type TabsState = DeepWritable<TabsStateReadonly>;

// Maintain backward compatibility with Data constructors
export const BibleState = Data.case<BibleState>();
export const StopwatchState = Data.case<StopwatchState>();
export const WikiState = Data.case<WikiState>();
export const LibraryState = Data.case<LibraryState>();

// App constructors
export const { Bible, About, ChooseApp, Stopwatch, Wiki, Library, $match } = Data.taggedEnum<App>()

// Format time as MM:SS for tab title
const formatTimeForTitle = (milliseconds: number): string => {
	const totalSeconds = Math.floor(milliseconds / 1000);
	const minutes = Math.floor(totalSeconds / 60);
	const seconds = totalSeconds % 60;
	return `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
};

// Helper to format a single reference point as URL segment (e.g., "matt5v3" or "matt5")
const formatReferencePoint = (ref: BibleReference): string => {
	const bookShort = getShortName(ref.book);
	return ref.verse !== null
		? `${bookShort}${ref.chapter}v${ref.verse}`
		: `${bookShort}${ref.chapter}`;
};

// Format a selection as a reference-range expression for the `v` query param
// Formats:
// - matt5 (single chapter)
// - matt5v3 (single verse)
// - matt5v1-12 (verse range in same chapter)
// - matt5-7 (chapter range in same book)
// - matt5-7v30 (chapter range ending at verse)
// - matt28v10-mark1v5 (cross-book range)
// - matt28-mark2 (cross-book chapter range)
export const formatSelectionQuery = (selection: BibleSelection): string => {
	const { start, end } = selection;

	if (!end) {
		// Single point
		return formatReferencePoint(start);
	}

	const sameBook = start.book === end.book;
	const sameChapter = sameBook && start.chapter === end.chapter;

	if (sameChapter) {
		// Same chapter: matt5v1-12
		const bookShort = getShortName(start.book);
		if (start.verse !== null && end.verse !== null) {
			return `${bookShort}${start.chapter}v${start.verse}-${end.verse}`;
		}
		// Just chapter (no verse range)
		return `${bookShort}${start.chapter}`;
	}

	if (sameBook) {
		// Same book, different chapters: matt5-7 or matt5-7v30
		const bookShort = getShortName(start.book);
		const startPart = start.verse !== null
			? `${start.chapter}v${start.verse}`
			: `${start.chapter}`;
		const endPart = end.verse !== null
			? `${end.chapter}v${end.verse}`
			: `${end.chapter}`;
		return `${bookShort}${startPart}-${endPart}`;
	}

	// Different books: matt28v10-mark1v5
	return `${formatReferencePoint(start)}-${formatReferencePoint(end)}`;
};

// Helper to format scroll position as the URL path (e.g., "/john1v1")
export const formatScrollPath = (book: BibleBook, chapter: number, verse: number | null): string => {
	const bookShort = getShortName(book);
	return verse !== null
		? `/${bookShort}${chapter}v${verse}`
		: `/${bookShort}${chapter}`;
};

// App namespace - operations on App type
export namespace App {
	export const getUrl = (app: App): string => {
		return $match(app, {
			Bible: ({ bibleState }) => {
				// Path is the scroll position; selection (if any) is the `v` query param
				const scrollPath = formatScrollPath(
					bibleState.currentBook,
					bibleState.currentChapter,
					bibleState.currentVerse
				);

				if (bibleState.selection) {
					return `${scrollPath}?v=${formatSelectionQuery(bibleState.selection)}`;
				}
				return scrollPath;
			},
			About: () => "/about",
			ChooseApp: () => "/",
			Stopwatch: () => "/stopwatch",
			Wiki: ({ wikiState }) => wikiState.page ? `/wiki/${wikiState.page.replace(/ /g, '_')}` : '/wiki',
			Library: ({ libraryState }) => libraryState.document ? `/library/${libraryState.document}` : '/library'
		});
	};

	export const getTitle = (app: App): string => {
		return $match(app, {
			Bible: ({ bibleState }) => {
				const reference = bibleState.currentVerse
					? `${getBibleBookDisplayName(bibleState.currentBook)} ${bibleState.currentChapter}:${bibleState.currentVerse}`
					: `${getBibleBookDisplayName(bibleState.currentBook)} ${bibleState.currentChapter}`;
				return `${reference} (${bibleState.translation.metadata.shortName})`;
			},
			About: () => "About",
			ChooseApp: () => "Choose App",
			Stopwatch: ({ stopwatchState }) => {
				if (stopwatchState.elapsedTime > 0 || stopwatchState.isRunning) {
					return formatTimeForTitle(stopwatchState.elapsedTime);
				}
				return "Stopwatch";
			},
			Wiki: ({ wikiState }) => wikiState.page ? wikiState.page.replace(/_/g, ' ') : 'Wiki',
			Library: ({ libraryState }) => {
				if (!libraryState.document) return 'Library';
				// Extract filename from path (e.g., "en/sempruijs/the-goodness-of-God" -> "the-goodness-of-God")
				const parts = libraryState.document.split('/');
				const filename = parts[parts.length - 1];
				return filename.replace(/-/g, ' ');
			}
		});
	};
}

// TabsState namespace - operations on TabsState type
// These are pure functions for state transformations
export namespace TabsState {
	export const addTab = (state: TabsState, app: App): TabsState => {
		const tabId = `tab${state.nextTabId}`;
		const newTab: TabState = { id: tabId, app };

		return {
			tabs: [...state.tabs, newTab],
			activeTabId: tabId,
			nextTabId: state.nextTabId + 1
		};
	};

	export const removeTab = (state: TabsState, tabId: string): TabsState => {
		// Don't allow removing last tab
		if (state.tabs.length === 1) {
			return state;
		}

		const newTabs = state.tabs.filter(tab => tab.id !== tabId);

		// If removing active tab, switch to first tab
		const newActiveTabId = state.activeTabId === tabId
			? newTabs[0]?.id ?? state.activeTabId
			: state.activeTabId;

		return {
			tabs: newTabs,
			activeTabId: newActiveTabId,
			nextTabId: state.nextTabId
		};
	};

	export const setActiveTab = (state: TabsState, tabId: string): TabsState => {
		// Verify tab exists
		const tabExists = state.tabs.some(tab => tab.id === tabId);
		if (!tabExists) {
			console.error(`Tab ${tabId} not found`);
			return state;
		}

		return {
			...state,
			activeTabId: tabId
		};
	};

	export const nextTab = (state: TabsState): TabsState => {
		const currentIndex = state.tabs.findIndex(tab => tab.id === state.activeTabId);
		if (currentIndex === -1) return state;

		const nextIndex = (currentIndex + 1) % state.tabs.length;
		return {
			...state,
			activeTabId: state.tabs[nextIndex].id
		};
	};

	export const previousTab = (state: TabsState): TabsState => {
		const currentIndex = state.tabs.findIndex(tab => tab.id === state.activeTabId);
		if (currentIndex === -1) return state;

		const prevIndex = currentIndex === 0 ? state.tabs.length - 1 : currentIndex - 1;
		return {
			...state,
			activeTabId: state.tabs[prevIndex].id
		};
	};

	export const updateTab = (state: TabsState, updatedTab: TabState): TabsState => {
		return {
			...state,
			tabs: state.tabs.map(tab => tab.id === updatedTab.id ? updatedTab : tab)
		};
	};

	export const getActiveTab = (state: TabsState): Option.Option<TabState> => {
		const tab = state.tabs.find(t => t.id === state.activeTabId);
		return tab ? Option.some(tab) : Option.none();
	};
}


// Create tab configuration types
export type CreateTabConfig =
	| { app: "Bible", id: string, book: BibleBook, chapter: number, verse?: number | null, selection?: BibleSelection | null, translation: Translation, showCanonExplorer: boolean }
	| { app: "Stopwatch", id: string }
	| { app: "About", id: string }
	| { app: "ChooseApp", id: string }
	| { app: "Wiki", id: string, page: string, showSidebar?: boolean }
	| { app: "Library", id: string, document: string, showSidebar?: boolean };

// Unified tab creation function - pure data construction
export const createTab = (config: CreateTabConfig): TabState => {
	switch (config.app) {
		case "Bible":
			return {
				id: config.id,
				app: Bible({
					bibleState: BibleState({
						currentBook: config.book,
						currentChapter: config.chapter,
						currentVerse: config.verse ?? null,
						selection: config.selection ?? null,
						translation: config.translation as any, // Translation types are readonly but compatible
						showCanonExplorer: config.showCanonExplorer
					})
				})
			};
		case "Stopwatch":
			return {
				id: config.id,
				app: Stopwatch({
					stopwatchState: StopwatchState({
						elapsedTime: 0,
						isRunning: false
					})
				})
			};
		case "About":
			return {
				id: config.id,
				app: About()
			};
		case "ChooseApp":
			return {
				id: config.id,
				app: ChooseApp()
			};
		case "Wiki":
			return {
				id: config.id,
				app: Wiki({
					wikiState: WikiState({
						page: config.page,
						showSidebar: config.showSidebar ?? true
					})
				})
			};
		case "Library":
			return {
				id: config.id,
				app: Library({
					libraryState: LibraryState({
						document: config.document,
						showSidebar: config.showSidebar ?? true
					})
				})
			};
	}
};

