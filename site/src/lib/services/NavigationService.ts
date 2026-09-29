/**
 * Navigation utility service for handling URL routing and state management.
 * Supports the Bible reference URL schema:
 * - matt5 (single chapter)
 * - matt5v3 (single verse)
 * - matt5v1-12 (verse range in same chapter)
 * - matt5-7 (chapter range in same book)
 * - matt5-7v30 (chapter range ending at verse)
 * - matt28v10-mark1v5 (cross-book range)
 * - matt28-mark2 (cross-book chapter range)
 *
 * For backwards compatibility, a dot between the book and chapter is still
 * accepted when parsing (e.g. "matt.5v3"), but URLs are always generated
 * without it.
 */

import { BibleBook, toBibleBook, matchBookPrefix } from "$lib/book";
import { selectionToUrl, formatScrollHash } from "$lib/app";
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
 * Parse a URL pathname to extract Bible selection.
 * Supports both old format (/book/chapter) and new format (/book.chapter)
 *
 * @param pathname - The URL pathname to parse
 * @returns Option containing BibleSelection if valid, Option.none() otherwise
 */
export const parseURL = (pathname: string): Option.Option<BibleSelection> => {
	// Try new format first
	const selection = parseReferenceUrl(pathname);
	if (selection) {
		return Option.some(selection);
	}

	// Try legacy format: /book/chapter or /book/chapterVverse
	const urlParts = pathname.split('/').filter(Boolean);
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
 * Parse the URL hash to extract scroll position (e.g., "#john.1v1")
 */
export const parseScrollHash = (hash: string): BibleReference | null => {
	if (!hash || hash === '#') return null;
	// Remove the # prefix
	const cleanHash = hash.startsWith('#') ? hash.slice(1) : hash;
	return parseReferencePoint(cleanHash);
};

/**
 * If the current URL uses the legacy dot-separated Bible reference format
 * (e.g. "/matt.5v3#matt.5v3"), rewrite it in place to the canonical
 * dot-less form (e.g. "/matt5v3#matt5v3"), without adding a history entry
 * or reloading the page.
 */
export const normalizeLegacyUrl = (): void => {
	if (typeof window === 'undefined') return;

	const { pathname, search, hash } = window.location;
	if (!pathname.includes('.') && !hash.includes('.')) return;

	const selection = parseReferenceUrl(pathname);
	const newPathname = selection ? selectionToUrl(selection) : pathname;

	const scrollPosition = parseScrollHash(hash);
	const newHash = scrollPosition
		? formatScrollHash(scrollPosition.book, scrollPosition.chapter, scrollPosition.verse)
		: hash;

	if (newPathname === pathname && newHash === hash) return;

	window.history.replaceState(window.history.state, '', newPathname + search + newHash);
};

/**
 * Get the initial application state based on the current URL.
 * Path = selection, Hash = scroll position
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

	// Rewrite any legacy dot-separated reference in the URL to the canonical form
	normalizeLegacyUrl();

	const pathname = window.location.pathname;
	const hash = window.location.hash;

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

	// Parse selection from path
	const selectionOption = parseURL(pathname);
	const selection = Option.isSome(selectionOption) ? selectionOption.value : null;

	// Parse scroll position from hash
	const scrollPosition = parseScrollHash(hash);

	// If we have a scroll position in hash, use it
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

	// If we have a selection but no scroll hash, scroll to selection start
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
	parseScrollHash,
	normalizeLegacyUrl,
	getInitialState
};
