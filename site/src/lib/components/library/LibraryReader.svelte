<script lang="ts">
	import { onMount } from "svelte";
	import LibrarySidebar from "$lib/components/library/LibrarySidebar.svelte";
	import BibleVersePreview from "$lib/components/common/BibleVersePreview.svelte";
	import * as ResponsiveService from "$lib/services/ResponsiveService";
	import * as NavigationService from "$lib/services/NavigationService";
	import type { Translation } from "$lib/translations/translation";

	let {
		document,
		showSidebar = true,
		isActive = true,
		onNavigate,
		onToggleSidebar,
		onOpenInNewTab,
		translation,
	}: {
		document: string;
		showSidebar?: boolean;
		isActive?: boolean;
		onNavigate?: (document: string) => void;
		onToggleSidebar?: () => void;
		onOpenInNewTab?: (path: string) => void;
		translation?: Translation;
	} = $props();

	let content = $state<string>("");
	let loading = $state<boolean>(true);
	let error = $state<string | null>(null);
	let entries = $state<LibraryEntry[]>([]);
	let entriesLoading = $state<boolean>(true);
	let isMobile = $state<boolean>(false);
	let shouldFocusSearch = $state(false);

	// Hover preview state
	let hoverPreview = $state<{ path: string; x: number; y: number } | null>(null);
	let hoverTimeout = $state<ReturnType<typeof setTimeout> | null>(null);

	type LibraryEntry = {
		path: string;
		name: string;
		language: string;
		author: string;
	};

	// Convert path to display name (e.g., "en/sempruijs/the-goodness-of-God" -> "The Goodness of God")
	function toDisplayName(path: string): string {
		const parts = path.split('/');
		const filename = parts[parts.length - 1];
		return filename
			.replace(/-/g, ' ')
			.replace(/\b\w/g, (c) => c.toUpperCase());
	}

	// Fetch library entries list from GitHub
	async function fetchLibraryEntries() {
		entriesLoading = true;
		try {
			const response = await fetch('https://api.github.com/repos/biblecomputer/public-domain/git/trees/main?recursive=1');
			if (response.ok) {
				const data = await response.json();
				entries = data.tree
					.filter((item: any) => item.path.endsWith('.md') && item.path !== 'README.md')
					.map((item: any) => {
						const pathWithoutExt = item.path.replace('.md', '');
						const parts = pathWithoutExt.split('/');
						return {
							path: pathWithoutExt,
							name: parts[parts.length - 1],
							language: parts[0] || 'unknown',
							author: parts[1] || 'unknown'
						};
					})
					.sort((a: LibraryEntry, b: LibraryEntry) => a.name.localeCompare(b.name));
			}
		} catch (err) {
			console.error('Failed to fetch library entries:', err);
			entries = [];
		} finally {
			entriesLoading = false;
		}
	}

	// Fetch document content from GitHub
	async function fetchDocumentContent(docPath: string) {
		loading = true;
		error = null;

		const url = `https://raw.githubusercontent.com/biblecomputer/public-domain/main/${docPath}.md`;

		try {
			const response = await fetch(url);
			if (!response.ok) {
				if (response.status === 404) {
					error = `Document "${toDisplayName(docPath)}" not found`;
				} else {
					error = `Failed to fetch document: ${response.statusText}`;
				}
				content = "";
			} else {
				content = await response.text();
			}
		} catch (err) {
			error = `Failed to load document: ${err}`;
			content = "";
		} finally {
			loading = false;
		}
	}

	// Check if a URL is a Bible reference link
	function isBibleLink(url: string): boolean {
		if (!url.startsWith('/')) return false;
		// Not wiki, library, about, or stopwatch
		if (url.startsWith('/wiki/') || url.startsWith('/library/') || url === '/about' || url === '/stopwatch') {
			return false;
		}
		const pathWithoutSlash = url.slice(1);
		const bibleRef = NavigationService.parseReferenceUrl(pathWithoutSlash);
		return bibleRef !== null;
	}

	// Check if a URL is an internal bible.computer link
	function isInternalLink(url: string): boolean {
		// Must start with /
		if (!url.startsWith('/')) return false;

		// Check for wiki, library, or about paths
		if (url.startsWith('/wiki/') || url.startsWith('/library/') || url === '/about' || url === '/stopwatch') {
			return true;
		}

		// Check for Bible reference (e.g., /ps100v5, /matt5, /john3v16)
		return isBibleLink(url);
	}

	// Handle click events on internal links
	function handleContentClick(event: MouseEvent) {
		// Clear any hover preview on click
		clearHoverPreview();

		const target = event.target as HTMLElement;
		if (target.classList.contains('internal-link')) {
			event.preventDefault();
			const path = target.getAttribute('data-internal-path');
			if (path && onOpenInNewTab) {
				onOpenInNewTab(path);
			}
		}
	}

	// Handle mouseover for Bible link preview
	function handleContentMouseOver(event: MouseEvent) {
		const target = event.target as HTMLElement;
		if (target.classList.contains('bible-link') && translation) {
			const path = target.getAttribute('data-internal-path');
			if (path) {
				// Clear any existing timeout
				if (hoverTimeout) {
					clearTimeout(hoverTimeout);
				}
				// Delay showing preview slightly to avoid flickering
				hoverTimeout = setTimeout(() => {
					hoverPreview = { path, x: event.clientX, y: event.clientY };
				}, 300);
			}
		}
	}

	// Handle mouseout to clear preview
	function handleContentMouseOut(event: MouseEvent) {
		const target = event.target as HTMLElement;
		if (target.classList.contains('bible-link')) {
			if (hoverTimeout) {
				clearTimeout(hoverTimeout);
				hoverTimeout = null;
			}
			// Small delay before hiding to allow moving to the preview
			setTimeout(() => {
				if (hoverPreview) {
					hoverPreview = null;
				}
			}, 100);
		}
	}

	function clearHoverPreview() {
		if (hoverTimeout) {
			clearTimeout(hoverTimeout);
			hoverTimeout = null;
		}
		hoverPreview = null;
	}

	// Simple markdown to HTML conversion
	function renderMarkdown(text: string): string {
		let html = text;

		// Headers (order matters: most specific first)
		html = html.replace(/^###### (.+)$/gm, '<h6 class="text-sm font-bold text-gray-100 light:text-gray-900 mt-4 mb-2">$1</h6>');
		html = html.replace(/^##### (.+)$/gm, '<h5 class="text-base font-bold text-gray-100 light:text-gray-900 mt-4 mb-2">$1</h5>');
		html = html.replace(/^#### (.+)$/gm, '<h4 class="text-lg font-bold text-gray-100 light:text-gray-900 mt-5 mb-2">$1</h4>');
		html = html.replace(/^### (.+)$/gm, '<h3 class="text-xl font-bold text-gray-100 light:text-gray-900 mt-6 mb-3">$1</h3>');
		html = html.replace(/^## (.+)$/gm, '<h2 class="text-2xl font-bold text-gray-100 light:text-gray-900 mt-8 mb-4">$1</h2>');
		html = html.replace(/^# (.+)$/gm, '<h1 class="text-3xl font-bold text-gray-100 light:text-gray-900 mt-8 mb-4">$1</h1>');

		// Bold and italic
		html = html.replace(/\*\*\*(.+?)\*\*\*/g, '<strong><em>$1</em></strong>');
		html = html.replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>');
		html = html.replace(/\*(.+?)\*/g, '<em>$1</em>');

		// Links (standard markdown) - check for internal links
		html = html.replace(/\[([^\]]+)\]\(([^)]+)\)/g, (_, linkText, url) => {
			if (isBibleLink(url)) {
				// Bible links get special class for hover preview
				return `<a href="${url}" class="internal-link bible-link text-blue-400 hover:text-blue-300 underline cursor-pointer" data-internal-path="${url}">${linkText}</a>`;
			}
			if (isInternalLink(url)) {
				return `<a href="${url}" class="internal-link text-blue-400 hover:text-blue-300 underline cursor-pointer" data-internal-path="${url}">${linkText}</a>`;
			}
			return `<a href="${url}" class="text-blue-400 hover:text-blue-300 underline" target="_blank" rel="noopener noreferrer">${linkText}</a>`;
		});

		// Convert each line to a paragraph for better screen reader navigation
		html = html.split(/\n+/).map(para => {
			para = para.trim();
			if (!para) return '';
			if (para.startsWith('<h') || para.startsWith('<ul') || para.startsWith('<ol')) {
				return para;
			}
			return `<p class="text-gray-200 light:text-gray-800 leading-relaxed inline">${para} </p>`;
		}).filter(Boolean).join('\n');

		return html;
	}

	function handleEntrySelect(entryPath: string) {
		if (onNavigate) {
			onNavigate(entryPath);
		}
		// On mobile, close sidebar to show the selected entry
		if (isMobile && showSidebar && onToggleSidebar) {
			onToggleSidebar();
		}
	}

	// Check if we're on the overview page
	let isOverview = $derived(document === '' || document === 'index');

	// Fetch content when document changes (skip for overview)
	$effect(() => {
		if (!isOverview) {
			fetchDocumentContent(document);
		} else {
			loading = false;
			content = '';
			error = null;
		}
	});

	// Reset focus flag when sidebar is hidden
	$effect(() => {
		if (!showSidebar) {
			shouldFocusSearch = false;
		}
	});

	onMount(() => {
		// Fetch entries list
		fetchLibraryEntries();

		// Initialize mobile state
		isMobile = ResponsiveService.isMobile();

		// Listen for window resize
		const cleanupResize = ResponsiveService.createResizeObserver((mobile) => {
			isMobile = mobile;
		});

		// Setup keyboard shortcuts
		const handleKeydown = (event: KeyboardEvent) => {
			if (!isActive) return;

			const target = event.target as HTMLElement;
			if (target && (target.tagName === 'INPUT' || target.tagName === 'TEXTAREA' || target.contentEditable === 'true')) {
				return;
			}

			if (event.key === 'b') {
				event.preventDefault();
				if (!showSidebar) {
					shouldFocusSearch = false;
				}
				onToggleSidebar?.();
			} else if (event.key === 'o' && !showSidebar) {
				event.preventDefault();
				shouldFocusSearch = true;
				onToggleSidebar?.();
			}
		};

		if (typeof window !== 'undefined') {
			window.addEventListener('keydown', handleKeydown);
		}

		return () => {
			cleanupResize();
			if (typeof window !== 'undefined') {
				window.removeEventListener('keydown', handleKeydown);
			}
		};
	});
</script>

<div class="h-full flex flex-col">
	<!-- Toolbar -->
	<div class="bg-gray-800 light:bg-gray-100 border-b border-gray-700 light:border-gray-300 px-4 py-2 flex items-center gap-2 justify-between flex-shrink-0">
		<div class="flex items-center gap-2">
			<button
				onclick={() => onToggleSidebar?.()}
				class="px-3 py-1 bg-gray-700 light:bg-gray-200 hover:bg-gray-600 light:hover:bg-gray-300 text-gray-300 light:text-gray-700 text-sm rounded transition-colors flex items-center gap-1"
				title={showSidebar
					? (isMobile ? "Show Content" : "Hide Sidebar (b)")
					: (isMobile ? "Show Documents" : "Show Sidebar (b)")}
			>
				{#if isMobile}
					{#if showSidebar}
						Content
					{:else}
						Documents
					{/if}
				{:else}
					{#if showSidebar}
						Hide Sidebar (b)
					{:else}
						Show Sidebar (b)
					{/if}
				{/if}
			</button>
		</div>
	</div>

	<!-- Main Content -->
	<div class="flex flex-1 relative overflow-hidden">
		<!-- Sidebar -->
		{#if showSidebar}
			<div
				class="bg-gray-800 light:bg-gray-100 border-gray-700 light:border-gray-300 h-full overflow-y-auto flex-shrink-0
					   {isMobile
						 ? 'absolute inset-0 z-10 w-full'
						 : 'w-80 border-r relative'}"
			>
				{#if entriesLoading}
					<div class="flex items-center justify-center h-full text-gray-400 light:text-gray-500">
						<span>Loading documents...</span>
					</div>
				{:else}
					<LibrarySidebar
						{entries}
						currentDocument={document}
						onEntrySelect={handleEntrySelect}
						{shouldFocusSearch}
						{isMobile}
					/>
				{/if}
			</div>
		{/if}

		<!-- Content Viewer -->
		<div class="flex-1 {isMobile && showSidebar ? 'hidden' : 'block'} overflow-y-auto bg-gray-900 light:bg-gray-50">
			<div class="max-w-3xl mx-auto px-8 py-8">
				{#if isOverview}
					<!-- Overview Page -->
					<header class="mb-8 border-b border-gray-700 light:border-gray-300 pb-4">
						<h1 class="text-4xl font-bold text-gray-100 light:text-gray-900">Library</h1>
						<p class="text-sm text-gray-500 mt-2">Public Domain Documents</p>
					</header>

					<div class="space-y-6">
						<p class="text-gray-200 light:text-gray-800 leading-relaxed">
							Welcome to the Bible Computer Library. Browse public domain documents using the sidebar or search for specific texts.
						</p>

						{#if entries.length > 0}
							<div class="bg-gray-800 light:bg-gray-100 rounded-lg p-4">
								<p class="text-gray-300 light:text-gray-700">
									<span class="text-2xl font-bold text-gray-100 light:text-gray-900">{entries.length}</span> documents available
								</p>
							</div>
						{/if}

						<div class="border-t border-gray-700 light:border-gray-300 pt-6">
							<h2 class="text-xl font-bold text-gray-100 light:text-gray-900 mb-3">Contribute</h2>
							<p class="text-gray-300 light:text-gray-700 mb-4">
								This library is open source. You can contribute by adding public domain documents on GitHub.
							</p>
							<a
								href="https://github.com/biblecomputer/public-domain"
								target="_blank"
								rel="noopener noreferrer"
								class="inline-flex items-center gap-2 px-4 py-2 bg-gray-700 light:bg-gray-200 hover:bg-gray-600 light:hover:bg-gray-300 text-gray-200 light:text-gray-800 rounded-lg transition-colors"
							>
								<svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
									<path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
								</svg>
								View on GitHub
							</a>
						</div>
					</div>
				{:else}
					<!-- Document Title -->
					<header class="mb-8 border-b border-gray-700 light:border-gray-300 pb-4 flex justify-between items-start">
						<div>
							<h1 class="text-4xl font-bold text-gray-100 light:text-gray-900">{toDisplayName(document)}</h1>
							<p class="text-sm text-gray-500 mt-2">Library</p>
						</div>
						<a
							href="https://github.com/biblecomputer/public-domain/edit/main/{document}.md"
							target="_blank"
							rel="noopener noreferrer"
							class="p-2 text-gray-400 light:text-gray-500 hover:text-gray-200 light:hover:text-gray-800 hover:bg-gray-700 light:hover:bg-gray-200 rounded-lg transition-colors"
							title="Edit this document on GitHub"
						>
							<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
							</svg>
						</a>
					</header>

					<!-- Content -->
					{#if loading}
						<div class="flex items-center justify-center py-12">
							<div class="text-gray-400 light:text-gray-500">Loading...</div>
						</div>
					{:else if error}
						<div class="bg-red-900/20 light:bg-red-50 border border-red-700 light:border-red-300 rounded-lg p-6 text-center">
							<p class="text-red-400 light:text-red-600">{error}</p>
							<p class="text-gray-500 text-sm mt-2">
								Check that the document exists in the <a href="https://github.com/biblecomputer/public-domain" target="_blank" rel="noopener noreferrer" class="text-blue-400 hover:text-blue-300 underline">library repository</a>.
							</p>
							<button
								onclick={() => onToggleSidebar?.()}
								class="mt-4 px-4 py-2 bg-gray-700 light:bg-gray-200 hover:bg-gray-600 light:hover:bg-gray-300 text-gray-200 light:text-gray-800 rounded-lg transition-colors"
							>
								Browse documents
							</button>
						</div>
					{:else if content}
						<!-- eslint-disable-next-line svelte/no-at-html-tags -->
						<div
							class="library-content"
							onclick={handleContentClick}
							onmouseover={handleContentMouseOver}
							onmouseout={handleContentMouseOut}
						>
							{@html renderMarkdown(content)}
						</div>
					{:else}
						<div class="text-gray-500 text-center py-12">
							<p>This document is empty.</p>
						</div>
					{/if}
				{/if}
			</div>
		</div>
	</div>
</div>

<!-- Bible verse hover preview -->
{#if hoverPreview && translation}
	<BibleVersePreview
		path={hoverPreview.path}
		x={hoverPreview.x}
		y={hoverPreview.y}
		{translation}
		onClose={clearHoverPreview}
	/>
{/if}

<style>
	.library-content :global(p) {
		margin-bottom: 1rem;
	}

	.library-content :global(h1),
	.library-content :global(h2),
	.library-content :global(h3),
	.library-content :global(h4),
	.library-content :global(h5),
	.library-content :global(h6) {
		color: #f3f4f6;
	}

	@media (prefers-color-scheme: light) {
		.library-content :global(h1),
		.library-content :global(h2),
		.library-content :global(h3),
		.library-content :global(h4),
		.library-content :global(h5),
		.library-content :global(h6) {
			color: #111827;
		}
	}
</style>
