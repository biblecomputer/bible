<script lang="ts">
	import { onMount, onDestroy } from "svelte";
	import { Option } from "effect";
	import type { StopwatchState } from "$lib/app";

	let {
		stopwatchState,
		isActive = false,
		onStateChange
	}: {
		stopwatchState: StopwatchState;
		isActive?: boolean;
		onStateChange?: (newState: StopwatchState) => void;
	} = $props();

	// Simple local state
	let displayTime = $state<number>(0);
	let isRunning = $state<boolean>(false);
	let startTime = $state<Option.Option<number>>(Option.none());
	let intervalId: number | undefined;

	// Format time as MM:SS.mmm
	function formatTime(milliseconds: number): string {
		const totalSeconds = Math.floor(milliseconds / 1000);
		const minutes = Math.floor(totalSeconds / 60);
		const seconds = totalSeconds % 60;
		const ms = Math.floor((milliseconds % 1000) / 10); // Show centiseconds
		
		return `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}.${ms.toString().padStart(2, '0')}`;
	}


	function startStopwatch() {
		if (!isRunning) {
			const now = Date.now();
			startTime = Option.some(now - displayTime); // Account for existing elapsed time
			isRunning = true;

			// Simple interval that will work even when tab is hidden (since element stays in DOM)
			intervalId = setInterval(() => {
				if (Option.isSome(startTime)) {
					displayTime = Date.now() - startTime.value;
					onStateChange?.({
						elapsedTime: displayTime,
						isRunning: true
					});
				}
			}, 50) as unknown as number; // 20 times per second - smooth but not excessive
		}
	}

	function stopStopwatch() {
		if (isRunning) {
			isRunning = false;
			
			if (intervalId) {
				clearInterval(intervalId);
				intervalId = undefined;
			}
			
			onStateChange?.({
				elapsedTime: displayTime,
				isRunning: false
			});
		}
	}

	function resetStopwatch() {
		if (isRunning) {
			isRunning = false;
			if (intervalId) {
				clearInterval(intervalId);
				intervalId = undefined;
			}
		}

		displayTime = 0;
		startTime = Option.none();

		onStateChange?.({
			elapsedTime: 0,
			isRunning: false
		});
	}

	function toggleStopwatch() {
		if (isRunning) {
			stopStopwatch();
		} else {
			startStopwatch();
		}
	}

	// Handle keyboard shortcuts
	function handleKeydown(event: KeyboardEvent) {
		// Only handle if this tab is active
		if (!isActive) return;

		// Only handle if not typing in an input/textarea
		const target = event.target as HTMLElement;
		if (target && (target.tagName === 'INPUT' || target.tagName === 'TEXTAREA' || target.contentEditable === 'true')) {
			return;
		}

		if (event.key === ' ') { // Space key
			event.preventDefault();
			toggleStopwatch();
		} else if (event.key === 'r') {
			event.preventDefault();
			resetStopwatch();
		}
	}

	// Initialize state on mount
	onMount(() => {
		displayTime = stopwatchState.elapsedTime;
		isRunning = stopwatchState.isRunning;
		
		if (isRunning) {
			startStopwatch();
		}

		// Add keyboard event listener
		if (typeof window !== 'undefined') {
			window.addEventListener('keydown', handleKeydown);
			return () => window.removeEventListener('keydown', handleKeydown);
		}
	});

	// Cleanup on unmount
	onDestroy(() => {
		if (intervalId) {
			clearInterval(intervalId);
		}
	});
</script>

<div class="h-full flex items-center justify-center bg-gray-900 light:bg-gray-50">
	<div class="text-center max-w-md w-full mx-4">
		<!-- Stopwatch Display -->
		<div class="mb-8">
			<h1 class="text-2xl font-bold text-gray-100 light:text-gray-900 mb-6">Stopwatch</h1>
			<div class="bg-gray-800 light:bg-white rounded-2xl p-8 border border-gray-700 light:border-gray-300">
				<div class="text-6xl font-mono font-bold text-blue-400 mb-4">
					{formatTime(displayTime)}
				</div>
				<div class="text-sm text-gray-400 light:text-gray-500">
					{isRunning ? 'Running' : 'Stopped'}
				</div>
			</div>
		</div>

		<!-- Control Buttons -->
		<div class="flex gap-4 justify-center">
			<button
				onclick={toggleStopwatch}
				title="{isRunning ? 'Stop' : 'Start'} stopwatch (space)"
				class="px-6 py-3 rounded-lg font-semibold transition-all duration-200 {
					isRunning 
						? 'bg-red-600 hover:bg-red-700 text-white' 
						: 'bg-green-600 hover:bg-green-700 text-white'
				}"
			>
				{isRunning ? 'Stop' : 'Start'} (space)
			</button>

			<button
				onclick={resetStopwatch}
				title="Reset stopwatch (r)"
				class="px-6 py-3 bg-gray-700 light:bg-gray-200 hover:bg-gray-600 light:hover:bg-gray-300 text-gray-200 light:text-gray-800 rounded-lg font-semibold transition-all duration-200"
			>
				Reset (r)
			</button>
		</div>

		<!-- Additional Info -->
		<div class="mt-6 text-sm text-gray-500">
			<p>Precision: 10ms intervals</p>
		</div>
	</div>
</div>