/**
 * Navigation utility service for handling URL routing and state management.
 *
 * Supports the Bible reference URL schema:
 * - Path = scroll position, a single reference point, e.g. /john1 or /john1v3
 * - Query `v` = selection (highlighted verse/range), e.g. ?v=john1v3-5
 *
 * Reference expression formats (used for both the path and the `v` query value):
 * - matt5 (single chapter)
 * - matt5v3 (single verse)
 * - matt5v1-12 (verse range in same chapter)
 * - matt5-7 (chapter range in same book)
 * - matt5-7v30 (chapter range ending at verse)
 * - matt28v10-mark1v5 (cross-book range)
 * - matt28-mark2 (cross-book chapter range)
 *
 * For backwards compatibility, older URL shapes are still accepted when
 * parsing and are silently rewritten to the canonical form:
 * - a dot between the book and chapter (e.g. "matt.5v3")
 * - the selection living in the path with the scroll position in the hash
 *   (e.g. "/matt5-7#matt5", from before the selection moved to `?v=`)
 */

import { BibleBook, toBibleBook, matchBookPrefix } from "$lib/book";
import { formatSelectionQuery, formatScrollPath } from "$lib/app";
import type { BibleReference, BibleSelection } from "$lib/app";
import { Option } from "effect";

/**
 * Parse a single reference point like "matt5" or "matt5v3"
 * (also accepts the legacy "matt.5" / "matt.5v3" form).
 * Returns { book, chapter, verse } or null if invalid
 */
const parseReferencePoint = (str: string): BibleReference | null => {
	const bookMatch = matchBookPrefix(str);
	if (!bookMatch) return null;

	// Legacy URLs separated the chapter from the book with a dot
	const rest = bookMatch.rest.startsWith('.') ? bookMatch.rest.slice(1) : bookMatch.rest;

	const match = rest.match(/^(\d+)(?:v(\d+))?$/i);
	if (!match) return null;

	const chapter = parseInt(match[1]);
	const verse = match[2] ? parseInt(match[2]) : null;

	if (isNaN(chapter) || chapter < 1) return null;
	if (verse !== null && (isNaN(verse) || verse < 1)) return null;

	return { book: bookMatch.book, chapter, verse };
};

/**
 * Parse a Bible reference URL path (without leading slash)
 * Handles all URL formats including ranges
 */
export const parseReferenceUrl = (path: string): BibleSelection | null => {
	// Remove leading slash if present
	const cleanPath = path.startsWith('/') ? path.slice(1) : path;

	// Check for a range (single reference points never contain a dash)
	const dashIndex = cleanPath.indexOf('-');
	if (dashIndex !== -1) {
		const leftPart = cleanPath.slice(0, dashIndex);
		const rightPart = cleanPath.slice(dashIndex + 1);

		const start = parseReferencePoint(leftPart);
		if (start) {
			// Cross-book range: the part after the dash has its own book abbreviation
			const crossBookEnd = parseReferencePoint(rightPart);
			if (crossBookEnd) {
				return { start, end: crossBookEnd };
			}

			// Same-book range: the part after the dash is just chapter[vVerse]
			const sameBookMatch = rightPart.match(/^(\d+)(?:v(\d+))?$/i);
			if (sameBookMatch) {
				const endNum = parseInt(sameBookMatch[1]);
				const endVerse = sameBookMatch[2] ? parseInt(sameBookMatch[2]) : null;

				// Determine if end is a chapter or a verse
				// If start has a verse and end is a small number without a v prefix, treat as verse
				if (start.verse !== null && !sameBookMatch[2] && endNum <= start.verse + 100) {
					// Same chapter verse range: matt5v1-12 means verses 1-12 of chapter 5
					return {
						start,
						end: { book: start.book, chapter: start.chapter, verse: endNum }
					};
				}

				// Chapter range (with optional verse): matt5-7 or matt5-7v30
				return {
					start,
					end: { book: start.book, chapter: endNum, verse: endVerse }
				};
			}
		}
	}

	// Single reference: book+chapter or book+chapterVverse
	const singleRef = parseReferencePoint(cleanPath);
	if (singleRef) {
		return { start: singleRef, end: null };
	}

	return null;
};

/**
 * Update the URL without triggering a page navigation/reload.
 * Uses history.replaceState for smooth updates.
 *
 * @param url - The URL to set
 */
export const navigateToUrl = (url: string): void => {
	if (typeof window === 'undefined') return;

	console.log('NavigationService: updating URL to:', url);
	// Use history.replaceState to update URL without navigation
	window.history.replaceState(window.history.state, '', url);
};

/**
 * Parse the URL pathname to extract the scroll position (a single reference
 * point, e.g. "/john1v1"). Also accepts the legacy dotted form ("/john.1v1").
 */
export const parseScrollPath = (pathname: string): BibleReference | null => {
	const cleanPath = pathname.startsWith('/') ? pathname.slice(1) : pathname;
	if (!cleanPath) return null;
	return parseReferencePoint(cleanPath);
};

/**
 * Extract the current Bible selection (highlighted verse/range) from a URL.
 *
 * - New scheme: the `v` query parameter, e.g. "?v=john1v3-5"
 * - Legacy fallback: a selection used to live directly in the path, e.g.
 *   "/matt5-7#matt5" (range in the path) or "/book/chapter" (oldest format)
 *
 * @param url - The URL to parse
 * @returns Option containing BibleSelection if present and valid, Option.none() otherwise
 */
export const parseURL = (url: URL): Option.Option<BibleSelection> => {
	const v = url.searchParams.get('v');
	if (v) {
		const selection = parseReferenceUrl(v);
		if (selection) return Option.some(selection);
	}

	// Legacy: the selection lived in the path, distinguishable from a plain
	// scroll-position path by a range dash or a leftover scroll hash
	if (url.pathname.includes('-') || url.hash) {
		const legacySelection = parseReferenceUrl(url.pathname);
		if (legacySelection) return Option.some(legacySelection);
	}

	// Oldest legacy format: /book/chapter or /book/chapterVverse
	const urlParts = url.pathname.split('/').filter(Boolean);
	if (urlParts.length >= 2) {
		const bookOption = toBibleBook(urlParts[0]);
		const chapterMatch = urlParts[1].match(/^(\d+)(?:v(\d+))?$/);
		if (Option.isSome(bookOption) && chapterMatch) {
			const chapter = parseInt(chapterMatch[1]);
			const verse = chapterMatch[2] ? parseInt(chapterMatch[2]) : null;
			return Option.some({
				start: { book: bookOption.value, chapter, verse },
				end: null
			});
		}
	}

	return Option.none();
};

/**
 * Parse the URL hash to extract scroll position (e.g., "#john.1v1").
 * Only used for migrating the legacy hash-based scroll position - the
 * canonical scheme keeps the scroll position in the path instead.
 */
export const parseScrollHash = (hash: string): BibleReference | null => {
	if (!hash || hash === '#') return null;
	// Remove the # prefix
	const cleanHash = hash.startsWith('#') ? hash.slice(1) : hash;
	return parseReferencePoint(cleanHash);
};

/**
 * If the current URL uses an older Bible reference URL shape, rewrite it in
 * place to the canonical form, without adding a history entry or reloading
 * the page. Handles, in combination:
 * - a dot between the book and chapter (e.g. "matt.5v3" -> "matt5v3")
 * - the selection living in the path with scroll position in the hash
 *   (e.g. "/matt5-7#matt5" -> "/matt5?v=matt5-7")
 */
export const normalizeLegacyUrl = (): void => {
	if (typeof window === 'undefined') return;

	const { pathname, hash, search } = window.location;

	// Only the Bible reader uses this reference scheme
	if (pathname === '/about' || pathname === '/stopwatch' ||
		pathname.startsWith('/wiki') || pathname.startsWith('/library')) {
		return;
	}

	const looksLegacy = hash !== '' || pathname.includes('-') || pathname.includes('.') || search.includes('.');
	if (!looksLegacy) return;

	// Old scheme: the path held the selection (possibly a range), the hash held scroll position
	const isOldScheme = pathname.includes('-') || hash !== '';

	let scrollRef: BibleReference | null;
	let selection: BibleSelection | null;

	if (isOldScheme) {
		selection = parseReferenceUrl(pathname);
		scrollRef = parseScrollHash(hash) ?? selection?.start ?? null;
	} else {
		// Already a scroll-only path, may just have a leftover dot
		scrollRef = parseScrollPath(pathname);
		const v = new URLSearchParams(search).get('v');
		selection = v ? parseReferenceUrl(v) : null;
	}

	if (!scrollRef) return; // not a Bible reference path we understand

	const newPathname = formatScrollPath(scrollRef.book, scrollRef.chapter, scrollRef.verse);
	const newSearch = selection ? `?v=${formatSelectionQuery(selection)}` : '';

	if (newPathname === pathname && newSearch === search && hash === '') return;

	window.history.replaceState(window.history.state, '', newPathname + newSearch);
};

/**
 * Get the initial application state based on the current URL.
 * Path = scroll position, `v` query param = selection
 *
 * @returns Object containing the initial book, chapter, verse, selection, and app type flags
 */
export const getInitialState = (): {
	book: BibleBook;
	chapter: number;
	verse: number | null;
	selection: BibleSelection | null;
	isAbout: boolean;
	isStopwatch: boolean;
	isWiki: boolean;
	wikiPage: string | null;
	isLibrary: boolean;
	libraryDocument: string | null;
} => {
	if (typeof window === 'undefined') {
		return {
			book: BibleBook.John,
			chapter: 1,
			verse: null,
			selection: null,
			isAbout: false,
			isStopwatch: false,
			isWiki: false,
			wikiPage: null,
			isLibrary: false,
			libraryDocument: null
		};
	}

	// Rewrite any legacy URL shape (dot-separated, selection-in-path) to the canonical form
	normalizeLegacyUrl();

	const pathname = window.location.pathname;

	// Check if it's the about page
	if (pathname === '/about') {
		return {
			book: BibleBook.John,
			chapter: 1,
			verse: null,
			selection: null,
			isAbout: true,
			isStopwatch: false,
			isWiki: false,
			wikiPage: null,
			isLibrary: false,
			libraryDocument: null
		};
	}

	// Check if it's the stopwatch page
	if (pathname === '/stopwatch') {
		return {
			book: BibleBook.John,
			chapter: 1,
			verse: null,
			selection: null,
			isAbout: false,
			isStopwatch: true,
			isWiki: false,
			wikiPage: null,
			isLibrary: false,
			libraryDocument: null
		};
	}

	// Check if it's a wiki page (with or without specific page)
	if (pathname === '/wiki' || pathname === '/wiki/') {
		return {
			book: BibleBook.John,
			chapter: 1,
			verse: null,
			selection: null,
			isAbout: false,
			isStopwatch: false,
			isWiki: true,
			wikiPage: '',
			isLibrary: false,
			libraryDocument: null
		};
	}

	const wikiMatch = pathname.match(/^\/wiki\/([^/]+)$/);
	if (wikiMatch) {
		return {
			book: BibleBook.John,
			chapter: 1,
			verse: null,
			selection: null,
			isAbout: false,
			isStopwatch: false,
			isWiki: true,
			wikiPage: wikiMatch[1],
			isLibrary: false,
			libraryDocument: null
		};
	}

	// Check if it's a library page (with or without specific document)
	if (pathname === '/library' || pathname === '/library/') {
		return {
			book: BibleBook.John,
			chapter: 1,
			verse: null,
			selection: null,
			isAbout: false,
			isStopwatch: false,
			isWiki: false,
			wikiPage: null,
			isLibrary: true,
			libraryDocument: ''
		};
	}

	const libraryMatch = pathname.match(/^\/library\/(.+)$/);
	if (libraryMatch) {
		return {
			book: BibleBook.John,
			chapter: 1,
			verse: null,
			selection: null,
			isAbout: false,
			isStopwatch: false,
			isWiki: false,
			wikiPage: null,
			isLibrary: true,
			libraryDocument: libraryMatch[1]
		};
	}

	// Parse selection from the `v` query param (with legacy fallbacks)
	const selectionOption = parseURL(new URL(window.location.href));
	const selection = Option.isSome(selectionOption) ? selectionOption.value : null;

	// Parse scroll position from the path
	const scrollPosition = parseScrollPath(pathname);

	// If we have a scroll position in the path, use it
	if (scrollPosition) {
		return {
			book: scrollPosition.book,
			chapter: scrollPosition.chapter,
			verse: scrollPosition.verse,
			selection,
			isAbout: false,
			isStopwatch: false,
			isWiki: false,
			wikiPage: null,
			isLibrary: false,
			libraryDocument: null
		};
	}

	// If we have a selection but no scroll position in the path, scroll to selection start
	if (selection) {
		return {
			book: selection.start.book,
			chapter: selection.start.chapter,
			verse: selection.start.verse,
			selection,
			isAbout: false,
			isStopwatch: false,
			isWiki: false,
			wikiPage: null,
			isLibrary: false,
			libraryDocument: null
		};
	}

	// Default to John 1
	return {
		book: BibleBook.John,
		chapter: 1,
		verse: null,
		selection: null,
		isAbout: false,
		isStopwatch: false,
		isWiki: false,
		wikiPage: null,
		isLibrary: false,
		libraryDocument: null
	};
};

export const NavigationService = {
	navigateToUrl,
	parseURL,
	parseReferenceUrl,
	parseScrollPath,
	parseScrollHash,
	normalizeLegacyUrl,
	getInitialState
};
