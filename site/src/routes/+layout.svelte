<script lang="ts">
	import "../app.css";
	import { Option } from "effect";
	import AppContainer from "$lib/components/AppContainer.svelte";
	import type { LayoutData } from "./$types";

	let { children, data }: { children: any; data: LayoutData } = $props();
</script>

<div class="h-screen overflow-hidden bg-gray-900 light:bg-gray-50 text-white light:text-gray-900">
	<div class="h-full">
		{#if Option.isSome(data.bibleData)}
			<AppContainer translation={data.bibleData.value} />
		{:else}
			<div class="flex items-center justify-center h-full">
				<div class="text-center">
					<p class="text-xl text-gray-400 light:text-gray-600">Failed to load Bible data</p>
					<p class="text-sm text-gray-500 mt-2">Please refresh the page</p>
				</div>
			</div>
		{/if}
	</div>
</div>
