<script>
	import { fly } from "svelte/transition";
	import { cubicIn, cubicOut } from "svelte/easing";

	/** @type {number} */
	let offset_height;

	/** @type {boolean} */
	export let expand;

	/** @type {number} */
	export let index;

	/** @type {number} */
	export let length;

	/** @type {import("svelte/store").Readable<import("$lib/types/snack").SnackInstance>} */
	export let data;

	$: turnover = length - (index + 1);
	$: yCoefficient = expand ? turnover * (offset_height + 12) : turnover * 12;
</script>

<div
	bind:offsetHeight={offset_height}
	class="fixed right-4 min-h-14 min-w-xs flex transform-gpu border-(1 border solid) rounded bg-background p-2 shadow transition-bottom use-transition-emphasized"
	in:fly={{ y: 100, duration: 300, easing: cubicOut }}
	out:fly={{ y: 100, duration: 300, easing: cubicIn }}
	style="bottom: calc(1.5rem + {yCoefficient}px);"
>
	<div class="h-fit flex flex-col">
		<span class="text-foreground font-medium">{$data.label}</span>
		{#if $data.description}
			<span class="text-sm text-muted-foreground">{$data.description}</span>
		{/if}
	</div>
</div>
