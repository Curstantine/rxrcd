<script>
	import { fly } from "svelte/transition";
	import { cubicIn, cubicOut } from "svelte/easing";

	/** @type {number} */
	export let index;

	/** @type {number} */
	export let length;

	/** @type {import("svelte/store").Readable<import("@/types/snack").SnackInstance>} */
	export let data;

	let turnover = length - (index + 1) * 12;
</script>

<div
	class="fixed bottom-6 right-4 min-h-14 min-w-xs flex border-(1 border solid) rounded bg-background p-2 shadow"
	in:fly={{ y: 100 + turnover, duration: 300, easing: cubicOut }}
	out:fly={{ y: 100, duration: 300, easing: cubicIn }}
>
	<div class="h-fit flex flex-col">
		<span class="text-foreground font-medium">{$data.label}</span>
		{#if $data.description}
			<span class="text-sm text-muted-foreground">{$data.description}</span>
		{/if}
	</div>
</div>
