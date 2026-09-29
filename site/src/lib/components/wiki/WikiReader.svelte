<script lang="ts">
	import { onMount } from "svelte";
	import WikiSidebar from "$lib/components/wiki/WikiSidebar.svelte";
	import BibleVersePreview from "$lib/components/common/BibleVersePreview.svelte";
	import * as ResponsiveService from "$lib/services/ResponsiveService";
	import * as NavigationService from "$lib/services/NavigationService";
	import type { Translation } from "$lib/translations/translation";

	let {
		page,
		showSidebar = true,
		isActive = true,
		onNavigate,
		onToggleSidebar,
		onOpenInNewTab,
		translation,
	}: {
		page: string;
		showSidebar?: boolean;
		isActive?: boolean;
		onNavigate?: (page: string) => void;
		onToggleSidebar?: () => void;
		onOpenInNewTab?: (path: string) => void;
		translation?: Translation;
	} = $props();

	let content = $state<string>("");
	let loading = $state<boolean>(true);
	let error = $state<string | null>(null);
	let entries = $state<string[]>([]);
	let entriesLoading = $state<boolean>(true);
	let isMobile = $state<boolean>(false);
	let shouldFocusSearch = $state(false);

	// Hover preview state
	let hoverPreview = $state<{ path: string; x: number; y: number } | null>(null);
	let hoverTimeout = $state<ReturnType<typeof setTimeout> | null>(null);

	// Normalize page name for comparison (lowercase, underscores to spaces)
	function normalizePageName(str: string): string {
		return str.toLowerCase().replace(/_/g, ' ');
	}

	// Convert URL format to display format (underscores to spaces)
	function toDisplayName(str: string): string {
		return str.replace(/_/g, ' ');
	}

	// Find the correct case-sensitive entry name from the entries list
	function findCorrectCase(pageName: string): string | null {
		const normalized = normalizePageName(pageName);
		return entries.find(entry => normalizePageName(entry) === normalized) || null;
	}

	// Fetch wiki entries list from GitHub
	async function fetchWikiEntries() {
		entriesLoading = true;
		try {
			const response = await fetch('https://api.github.com/repos/biblecomputer/wiki/contents/source/en');
			if (response.ok) {
				const data = await response.json();
				entries = data
					.filter((item: any) => item.name.endsWith('.md') && item.name !== 'README.md')
					.map((item: any) => item.name.replace('.md', ''))
					.sort((a: string, b: string) => a.localeCompare(b));
			}
		} catch (err) {
			console.error('Failed to fetch wiki entries:', err);
			entries = [];
		} finally {
			entriesLoading = false;
		}
	}

	// Fetch wiki content from GitHub
	async function fetchWikiContent(pageName: string) {
		loading = true;
		error = null;

		// Try to find the correct case from entries list
		const correctName = findCorrectCase(pageName);
		const displayName = toDisplayName(pageName);

		// Use the correct case if found, otherwise use the page name as-is
		const fileName = correctName || pageName.replace(/_/g, ' ');
		const url = `https://raw.githubusercontent.com/biblecomputer/wiki/main/source/en/${encodeURIComponent(fileName)}.md`;

		try {
			const response = await fetch(url);
			if (!response.ok) {
				if (response.status === 404) {
					error = `Page "${displayName}" not found`;
				} else {
					error = `Failed to fetch page: ${response.statusText}`;
				}
				content = "";
			} else {
				content = await response.text();
				// If we found the correct case, update the URL to reflect it
				if (correctName && correctName !== pageName && onNavigate) {
					// Navigate to the correct case version (this updates the URL)
					onNavigate(correctName);
				}
			}
		} catch (err) {
			error = `Failed to load wiki page: ${err}`;
			content = "";
		} finally {
			loading = false;
		}
	}

	// Parse Obsidian-style links [[PageName]] and convert to clickable links
	function parseWikiLinks(text: string): string {
		return text.replace(/\[\[([^\]]+)\]\]/g, (_, pageName) => {
			// Convert spaces to underscores for URL, preserve original case
			const urlName = pageName.replace(/ /g, '_');
			return `<a href="/wiki/${urlName}" class="wiki-link text-blue-400 hover:text-blue-300 underline cursor-pointer" data-wiki-page="${pageName}">${pageName}</a>`;
		});
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

	// Simple markdown to HTML conversion
	function renderMarkdown(text: string): string {
		let html = text;

		// Parse wiki links first
		html = parseWikiLinks(html);

		// Headers
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

		// Line breaks - convert double newlines to paragraphs
		html = html.split(/\n\n+/).map(para => {
			para = para.trim();
			if (para.startsWith('<h') || para.startsWith('<ul') || para.startsWith('<ol')) {
				return para;
			}
			return `<p class="text-gray-200 light:text-gray-800 leading-relaxed mb-4">${para.replace(/\n/g, '<br>')}</p>`;
		}).join('\n');

		return html;
	}

	// Handle click events on wiki links and internal links
	function handleContentClick(event: MouseEvent) {
		// Clear any hover preview on click
		clearHoverPreview();

		const target = event.target as HTMLElement;
		if (target.classList.contains('wiki-link')) {
			event.preventDefault();
			const pageName = target.getAttribute('data-wiki-page');
			if (pageName && onNavigate) {
				onNavigate(pageName);
			}
		} else if (target.classList.contains('internal-link')) {
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
				// Only clear if we're not hovering over the preview itself
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

	function handleEntrySelect(entryPage: string) {
		if (onNavigate) {
			onNavigate(entryPage);
		}
		// On mobile, close sidebar to show the selected entry
		if (isMobile && showSidebar && onToggleSidebar) {
			onToggleSidebar();
		}
	}

	// Check if we're on the overview page
	let isOverview = $derived(page === '' || page === 'index');

	// Fetch content when page changes (skip for overview)
	$effect(() => {
		if (!isOverview) {
			fetchWikiContent(page);
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
		fetchWikiEntries();

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
					: (isMobile ? "Show Entries" : "Show Sidebar (b)")}
			>
				{#if isMobile}
					{#if showSidebar}
						📖 Content
					{:else}
						📚 Entries
					{/if}
				{:else}
					{#if showSidebar}
						← Hide Sidebar (b)
					{:else}
						→ Show Sidebar (b)
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
						<span>Loading entries...</span>
					</div>
				{:else}
					<WikiSidebar
						{entries}
						currentPage={page}
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
						<h1 class="text-4xl font-bold text-gray-100 light:text-gray-900">Wiki</h1>
						<p class="text-sm text-gray-500 mt-2">Bible Encyclopedia</p>
					</header>

					<div class="space-y-6">
						<p class="text-gray-200 light:text-gray-800 leading-relaxed">
							Welcome to the Bible Computer Wiki. Browse entries using the sidebar or search for specific topics.
						</p>

						{#if entries.length > 0}
							<div class="bg-gray-800 light:bg-gray-100 rounded-lg p-4">
								<p class="text-gray-300 light:text-gray-700">
									<span class="text-2xl font-bold text-gray-100 light:text-gray-900">{entries.length}</span> entries available
								</p>
							</div>
						{/if}

						<div class="border-t border-gray-700 light:border-gray-300 pt-6">
							<h2 class="text-xl font-bold text-gray-100 light:text-gray-900 mb-3">Contribute</h2>
							<p class="text-gray-300 light:text-gray-700 mb-4">
								This wiki is open source. You can contribute by editing or adding entries on GitHub.
							</p>
							<a
								href="https://github.com/biblecomputer/wiki"
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
					<!-- Page Title -->
					<header class="mb-8 border-b border-gray-700 light:border-gray-300 pb-4 flex justify-between items-start">
						<div>
							<h1 class="text-4xl font-bold text-gray-100 light:text-gray-900">{findCorrectCase(page) || toDisplayName(page)}</h1>
							<p class="text-sm text-gray-500 mt-2">Wiki</p>
						</div>
						<a
							href="https://github.com/biblecomputer/wiki/edit/main/source/en/{encodeURIComponent(findCorrectCase(page) || toDisplayName(page))}.md"
							target="_blank"
							rel="noopener noreferrer"
							class="p-2 text-gray-400 light:text-gray-500 hover:text-gray-200 light:hover:text-gray-800 hover:bg-gray-700 light:hover:bg-gray-200 rounded-lg transition-colors"
							title="Edit this page on GitHub"
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
								Check that the page exists in the <a href="https://github.com/biblecomputer/wiki" target="_blank" rel="noopener noreferrer" class="text-blue-400 hover:text-blue-300 underline">wiki repository</a>.
							</p>
							<button
								onclick={() => onToggleSidebar?.()}
								class="mt-4 px-4 py-2 bg-gray-700 light:bg-gray-200 hover:bg-gray-600 light:hover:bg-gray-300 text-gray-200 light:text-gray-800 rounded-lg transition-colors"
							>
								Browse entries
							</button>
						</div>
					{:else if content}
						<!-- eslint-disable-next-line svelte/no-at-html-tags -->
						<div
							class="wiki-content"
							onclick={handleContentClick}
							onmouseover={handleContentMouseOver}
							onmouseout={handleContentMouseOut}
						>
							{@html renderMarkdown(content)}
						</div>
					{:else}
						<div class="text-gray-500 text-center py-12">
							<p>This page is empty.</p>
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
	.wiki-content :global(p) {
		margin-bottom: 1rem;
	}

	.wiki-content :global(h1),
	.wiki-content :global(h2),
	.wiki-content :global(h3) {
		color: #f3f4f6;
	}

	@media (prefers-color-scheme: light) {
		.wiki-content :global(h1),
		.wiki-content :global(h2),
		.wiki-content :global(h3) {
			color: #111827;
		}
	}
</style>
