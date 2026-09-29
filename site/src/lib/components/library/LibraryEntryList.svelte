<script lang="ts">
	type LibraryEntry = {
		path: string;
		name: string;
		language: string;
		author: string;
	};

	let {
		entries,
		currentDocument,
		onEntrySelect,
	}: {
		entries: LibraryEntry[];
		currentDocument: string;
		onEntrySelect: (path: string) => void;
	} = $props();

	// Convert name to display format
	function toDisplayName(name: string): string {
		return name
			.replace(/-/g, ' ')
			.replace(/\b\w/g, (c) => c.toUpperCase());
	}

	// Group entries by author
	let groupedEntries = $derived.by(() => {
		const groups: Record<string, LibraryEntry[]> = {};

		for (const entry of entries) {
			const groupKey = entry.author;
			if (!groups[groupKey]) {
				groups[groupKey] = [];
			}
			groups[groupKey].push(entry);
		}

		// Sort entries within each group
		for (const key in groups) {
			groups[key].sort((a, b) => a.name.localeCompare(b.name));
		}

		return groups;
	});

	let sortedAuthors = $derived(Object.keys(groupedEntries).sort());
</script>

<div class="p-4">
	<div class="space-y-4">
		{#each sortedAuthors as author}
			<div>
				<h3 class="text-sm font-bold text-gray-400 light:text-gray-500 mb-2 px-2">{author}</h3>
				<div class="space-y-1">
					{#each groupedEntries[author] as entry}
						{@const isActive = entry.path === currentDocument}
						<button
							onclick={() => onEntrySelect(entry.path)}
							class="w-full text-left px-3 py-2 rounded text-sm transition-colors {
								isActive
									? 'bg-blue-600 text-white'
									: 'text-gray-200 light:text-gray-800 hover:bg-gray-700 light:hover:bg-gray-200'
							}"
						>
							{toDisplayName(entry.name)}
						</button>
					{/each}
				</div>
			</div>
		{/each}
	</div>
</div>
