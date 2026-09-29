<script lang="ts">
	import type { Translation } from "$lib/translations/translation";
	import { BibleBook } from "$lib/book";
	import {
		type TabsState,
		type TabState,
		App,
		TabsState as TabsStateNS,
		createTab,
		ChooseApp
	} from "$lib/app";
	import * as NavigationService from "$lib/services/NavigationService";
	import * as ResponsiveService from "$lib/services/ResponsiveService";
	import TabBar from "$lib/components/tabs/TabBar.svelte";
	import TabContent from "$lib/components/tabs/TabContent.svelte";
	import { page } from "$app/stores";
	import { onMount } from "svelte";
	import { Option } from "effect";
	import { useKeyboardShortcuts } from "$lib/utils/keyboard";

	let { translation }: { translation: Translation } = $props();

	// State management - single source of truth
	let tabsState = $state<TabsState>({
		tabs: [],
		activeTabId: "tab1",
		nextTabId: 2
	});

	// Flag to prevent syncFromURL during programmatic navigation
	let isProgrammaticNavigation = $state(false);

	// Mobile responsiveness state
	let isMobile = $state<boolean>(false);

	// Initialize the first tab
	function initializeTab() {
		const initialState = NavigationService.getInitialState();

		let initialTab: TabState;
		if (initialState.isAbout) {
			// Create About tab if URL is /about
			initialTab = createTab({ app: "About", id: "tab1" });
		} else if (initialState.isStopwatch) {
			// Create Stopwatch tab if URL is /stopwatch
			initialTab = createTab({ app: "Stopwatch", id: "tab1" });
		} else if (initialState.isWiki) {
			// Create Wiki tab if URL is /wiki or /wiki/*
			// On mobile, show content immediately (homepage or entry)
			const showSidebar = ResponsiveService.getInitialWikiSidebarState();
			initialTab = createTab({ app: "Wiki", id: "tab1", page: initialState.wikiPage || '', showSidebar });
		} else if (initialState.isLibrary) {
			// Create Library tab if URL is /library or /library/*
			// On mobile, show content immediately (homepage or document)
			const showSidebar = ResponsiveService.getInitialWikiSidebarState();
			initialTab = createTab({ app: "Library", id: "tab1", document: initialState.libraryDocument || '', showSidebar });
		} else {
			// Create Bible tab with parsed book/chapter/verse and selection
			const canonState = ResponsiveService.getInitialCanonState();
			initialTab = createTab({
				app: "Bible",
				id: "tab1",
				book: initialState.book,
				chapter: initialState.chapter,
				verse: initialState.verse,
				selection: initialState.selection,
				translation,
				showCanonExplorer: canonState
			});
		}

		tabsState.tabs = [initialTab];
	}

	// Get active tab reference - derived from tabsState
	let activeTabOption = $derived(TabsStateNS.getActiveTab(tabsState));


	// Update tab state (for scroll/view position - updates hash in URL)
	function updateTabState(updatedTab: TabState) {
		console.log('📝 updateTabState called with:', updatedTab);
		tabsState = TabsStateNS.updateTab(tabsState, updatedTab);

		// Update URL hash for scroll position (Bible) or full URL (Wiki/Library)
		if (updatedTab.id === tabsState.activeTabId) {
			if (updatedTab.app._tag === "Bible" || updatedTab.app._tag === "Wiki" || updatedTab.app._tag === "Library") {
				const url = App.getUrl(updatedTab.app);
				NavigationService.navigateToUrl(url);
			}
		}
	}

	// Update tab state with selection change (updates full URL)
	function updateTabStateWithSelection(updatedTab: TabState) {
		console.log('📝 updateTabStateWithSelection called with:', updatedTab);
		tabsState = TabsStateNS.updateTab(tabsState, updatedTab);

		// Update full URL for selection change
		if (updatedTab.id === tabsState.activeTabId && updatedTab.app._tag === "Bible") {
			const url = App.getUrl(updatedTab.app);
			console.log('🔗 Updating URL to:', url);
			NavigationService.navigateToUrl(url);
		}
	}

	// Add new tab
	function addTab() {
		tabsState = TabsStateNS.addTab(tabsState, ChooseApp());
	}

	// Remove tab
	async function removeTab(tabId: string) {
		const wasActiveTab = tabId === tabsState.activeTabId;
		tabsState = TabsStateNS.removeTab(tabsState, tabId);

		// Update URL if we removed the active tab (we switched to a different one)
		if (wasActiveTab) {
			const activeTabOption = TabsStateNS.getActiveTab(tabsState);
			if (Option.isSome(activeTabOption)) {
				const url = App.getUrl(activeTabOption.value.app);
				isProgrammaticNavigation = true;
				await NavigationService.navigateToUrl(url);
				isProgrammaticNavigation = false;
			}
		}
	}

	// Set active tab
	async function setActiveTab(tabId: string) {
		tabsState = TabsStateNS.setActiveTab(tabsState, tabId);

		// Update URL for the active tab
		const activeTabOption = TabsStateNS.getActiveTab(tabsState);
		if (Option.isSome(activeTabOption)) {
			const url = App.getUrl(activeTabOption.value.app);
			isProgrammaticNavigation = true;
			await NavigationService.navigateToUrl(url);
			isProgrammaticNavigation = false;
		}
	}

	// Tab navigation functions
	async function goToNextTab() {
		tabsState = TabsStateNS.nextTab(tabsState);
		const activeTabOption = TabsStateNS.getActiveTab(tabsState);
		if (Option.isSome(activeTabOption)) {
			const url = App.getUrl(activeTabOption.value.app);
			isProgrammaticNavigation = true;
			await NavigationService.navigateToUrl(url);
			isProgrammaticNavigation = false;
		}
	}

	async function goToPreviousTab() {
		tabsState = TabsStateNS.previousTab(tabsState);
		const activeTabOption = TabsStateNS.getActiveTab(tabsState);
		if (Option.isSome(activeTabOption)) {
			const url = App.getUrl(activeTabOption.value.app);
			isProgrammaticNavigation = true;
			await NavigationService.navigateToUrl(url);
			isProgrammaticNavigation = false;
		}
	}

	// Handle app choice in ChooseApp tabs
	async function handleAppChoice(appType: "bible" | "about" | "stopwatch" | "wiki" | "library") {
		const tabIndex = tabsState.tabs.findIndex(tab => tab.id === tabsState.activeTabId);
		if (tabIndex === -1) return;

		let newTab: TabState;
		if (appType === "bible") {
			const canonState = ResponsiveService.getInitialCanonState();
			newTab = createTab({
				app: "Bible",
				id: tabsState.activeTabId,
				book: BibleBook.John,
				chapter: 1,
				verse: null,
				translation,
				showCanonExplorer: canonState
			});
		} else if (appType === "about") {
			newTab = createTab({ app: "About", id: tabsState.activeTabId });
		} else if (appType === "stopwatch") {
			newTab = createTab({ app: "Stopwatch", id: tabsState.activeTabId });
		} else if (appType === "wiki") {
			// On mobile, show content immediately
			const showSidebar = ResponsiveService.getInitialWikiSidebarState();
			newTab = createTab({ app: "Wiki", id: tabsState.activeTabId, page: "Abraham", showSidebar });
		} else if (appType === "library") {
			// On mobile, show content immediately
			const showSidebar = ResponsiveService.getInitialWikiSidebarState();
			newTab = createTab({ app: "Library", id: tabsState.activeTabId, document: "", showSidebar });
		} else {
			console.error(`Unknown app type: ${appType}`);
			return;
		}

		tabsState.tabs = tabsState.tabs.map((tab, index) => index === tabIndex ? newTab : tab);

		// Update URL for the new app
		const url = App.getUrl(newTab.app);
		isProgrammaticNavigation = true;
		await NavigationService.navigateToUrl(url);
		isProgrammaticNavigation = false;
	}

	// Handle wiki link click from Bible - create new wiki tab
	async function handleWikiLinkClick(page: string) {
		// On mobile, show content immediately
		const showSidebar = ResponsiveService.getInitialWikiSidebarState();
		const newTab = createTab({
			app: "Wiki",
			id: `tab${tabsState.nextTabId}`,
			page,
			showSidebar
		});

		tabsState = {
			tabs: [...tabsState.tabs, newTab],
			activeTabId: newTab.id,
			nextTabId: tabsState.nextTabId + 1
		};

		// Update URL to wiki page
		const url = App.getUrl(newTab.app);
		isProgrammaticNavigation = true;
		await NavigationService.navigateToUrl(url);
		isProgrammaticNavigation = false;
	}

	// Handle internal link click - create new tab based on path type
	async function handleOpenInNewTab(path: string) {
		const showSidebar = ResponsiveService.getInitialWikiSidebarState();
		const canonState = ResponsiveService.getInitialCanonState();
		let newTab: TabState;

		// Check for wiki path
		const wikiMatch = path.match(/^\/wiki\/(.+)$/);
		if (wikiMatch) {
			newTab = createTab({
				app: "Wiki",
				id: `tab${tabsState.nextTabId}`,
				page: wikiMatch[1],
				showSidebar
			});
		}
		// Check for library path
		else if (path.match(/^\/library\/(.+)$/)) {
			const libraryMatch = path.match(/^\/library\/(.+)$/);
			newTab = createTab({
				app: "Library",
				id: `tab${tabsState.nextTabId}`,
				document: libraryMatch![1],
				showSidebar
			});
		}
		// Check for about page
		else if (path === '/about') {
			newTab = createTab({
				app: "About",
				id: `tab${tabsState.nextTabId}`
			});
		}
		// Check for stopwatch page
		else if (path === '/stopwatch') {
			newTab = createTab({
				app: "Stopwatch",
				id: `tab${tabsState.nextTabId}`
			});
		}
		// Try to parse as Bible reference
		else {
			const pathWithoutSlash = path.startsWith('/') ? path.slice(1) : path;
			const selection = NavigationService.parseReferenceUrl(pathWithoutSlash);
			if (selection) {
				newTab = createTab({
					app: "Bible",
					id: `tab${tabsState.nextTabId}`,
					book: selection.start.book,
					chapter: selection.start.chapter,
					verse: selection.start.verse,
					selection: selection,
					translation,
					showCanonExplorer: canonState
				});
			} else {
				// Unknown path, don't open a tab
				console.warn('Unknown internal link path:', path);
				return;
			}
		}

		tabsState = {
			tabs: [...tabsState.tabs, newTab],
			activeTabId: newTab.id,
			nextTabId: tabsState.nextTabId + 1
		};

		// Update URL to the new tab
		const url = App.getUrl(newTab.app);
		isProgrammaticNavigation = true;
		await NavigationService.navigateToUrl(url);
		isProgrammaticNavigation = false;
	}

	// Handle browser navigation - ONLY updates selection, NOT scroll position
	function syncFromURL() {
		// Skip sync if we're in the middle of programmatic navigation
		if (isProgrammaticNavigation) {
			console.log('⏭️ Skipping syncFromURL - programmatic navigation in progress');
			return;
		}

		const currentPage = $page;
		const urlStateOption = NavigationService.parseURL(currentPage.url.pathname);

		if (Option.isNone(urlStateOption)) return;
		if (Option.isNone(activeTabOption)) return;

		const activeTab = activeTabOption.value;
		if (activeTab.app._tag !== "Bible") return;

		const selection = urlStateOption.value;
		const currentSelection = activeTab.app.bibleState.selection;

		// Only check if selection changed - scroll position is independent
		const selectionChanged = JSON.stringify(selection) !== JSON.stringify(currentSelection);
		if (!selectionChanged) return;

		console.log('🔄 syncFromURL: Selection changed via browser navigation');
		// Only update selection, keep current scroll position
		const updatedTab = createTab({
			app: "Bible",
			id: activeTab.id,
			book: activeTab.app.bibleState.currentBook,
			chapter: activeTab.app.bibleState.currentChapter,
			verse: activeTab.app.bibleState.currentVerse,
			selection: selection,
			translation: activeTab.app.bibleState.translation,
			showCanonExplorer: activeTab.app.bibleState.showCanonExplorer
		});

		updateTabState(updatedTab);
	}

	// Initialize on mount
	onMount(() => {
		// Initialize the first tab
		initializeTab();
		syncFromURL();

		// Initialize mobile state
		isMobile = ResponsiveService.isMobile();

		// Listen for window resize to update mobile state
		const cleanupResize = ResponsiveService.createResizeObserver((mobile) => {
			isMobile = mobile;
		});

		// Setup global keyboard shortcuts
		const cleanupKeyboard = useKeyboardShortcuts({
			't': () => addTab(),
			'n': () => goToNextTab(),
			'p': () => goToPreviousTab(),
			'w': () => removeTab(tabsState.activeTabId)
		}, { skipInInputs: true });

		return () => {
			cleanupResize();
			cleanupKeyboard();
		};
	});

	// Watch for URL changes
	$effect(() => {
		$page;
		syncFromURL();
	});
</script>

<div class="h-full flex flex-col bg-gray-900 light:bg-gray-50">
	<TabBar
		tabs={tabsState.tabs}
		activeTabId={tabsState.activeTabId}
		onTabSelect={setActiveTab}
		onTabRemove={removeTab}
		onAddTab={addTab}
		{isMobile}
	/>

	<TabContent
		tabs={tabsState.tabs}
		activeTabId={tabsState.activeTabId}
		onScrollStateChange={updateTabState}
		onSelectionChange={updateTabStateWithSelection}
		onAppChoice={handleAppChoice}
		onTabRemove={removeTab}
		onWikiLinkClick={handleWikiLinkClick}
		onOpenInNewTab={handleOpenInNewTab}
		{translation}
	/>
</div>