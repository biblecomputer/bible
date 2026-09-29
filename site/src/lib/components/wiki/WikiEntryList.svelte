<script lang="ts">
	let {
		entries,
		currentPage,
		onEntrySelect,
	}: {
		entries: string[];
		currentPage: string;
		onEntrySelect: (page: string) => void;
	} = $props();

	// Group entries alphabetically
	let groupedEntries = $derived.by(() => {
		const groups: Record<string, string[]> = {};

		for (const entry of entries) {
			const firstLetter = entry.charAt(0).toUpperCase();
			if (!groups[firstLetter]) {
				groups[firstLetter] = [];
			}
			groups[firstLetter].push(entry);
		}

		// Sort entries within each group
		for (const letter in groups) {
			groups[letter].sort((a, b) => a.localeCompare(b));
		}

		return groups;
	});

	let sortedLetters = $derived(Object.keys(groupedEntries).sort());
</script>

<div class="p-4">
	<div class="space-y-4">
		{#each sortedLetters as letter}
			<div>
				<h3 class="text-sm font-bold text-gray-400 light:text-gray-500 mb-2 px-2">{letter}</h3>
				<div class="space-y-1">
					{#each groupedEntries[letter] as entry}
						{@const normalizedEntry = entry.toLowerCase().replace(/_/g, ' ')}
						{@const normalizedCurrent = currentPage.toLowerCase().replace(/_/g, ' ')}
						{@const isActive = normalizedEntry === normalizedCurrent}
						<button
							onclick={() => onEntrySelect(entry)}
							class="w-full text-left px-3 py-2 rounded text-sm transition-colors {
								isActive
									? 'bg-blue-600 text-white'
									: 'text-gray-200 light:text-gray-800 hover:bg-gray-700 light:hover:bg-gray-200'
							}"
						>
							{entry}
						</button>
					{/each}
				</div>
			</div>
		{/each}
	</div>
</div>
