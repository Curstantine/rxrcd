<script>
	import { onMount } from "svelte";

	import { external_hit } from "@/utils/actions";

	/** @type {string} */
	export let label;

	/** @type {string} */
	export let aria_controls;

	/** @type {{ label: string; value: string }[]} */
	export let actions;

	/** @type {(value: string) => (void | Promise<void>)} */
	export let on_change;

	const combobox_id = `${aria_controls}-hit-box`;

	/** @type {HTMLButtonElement} */
	let button;

	let expanded = false;
	let [listbox_top, listbox_left, listbox_width] = [0, 0, 0];

	/** @param {string} action_value */
	const on_action_click = (action_value) => {
		expanded = false;
		on_change(action_value);
	};

	onMount(() => {
		const rect = button.getBoundingClientRect();
		listbox_top = rect.top + window.scrollY;
		listbox_left = rect.left + window.scrollX;
		listbox_width = rect.right - listbox_left;
	});
</script>

<button
	bind:this={button}
	id={combobox_id}
	role="combobox"
	aria-expanded={expanded}
	aria-controls={aria_controls}
	on:click={() => (expanded = !expanded)}
	class="{$$props.class} flex justify-between items-center min-w-32 rounded-md h-9 px-3 border-(1 solid border)"
>
	<span class="pointer-events-none text-sm">{label}</span>
	<div class="i-symbols-expand-all-rounded text-muted-foreground"></div>
</button>

{#if expanded}
	<div
		role="listbox"
		class="absolute z-10 flex flex-col border-(1 border solid) rounded-md bg-background py-1 shadow-md"
		style:top="calc({listbox_top}px + 2.75rem)"
		style:left="{listbox_left}px"
		style:min-width="{listbox_width}px"
		use:external_hit={{ coupling_ids: [combobox_id], close: () => (expanded = false) }}
	>
		{#each actions as { label, value }}
			<button class="action" on:click|once={() => on_action_click(value)}>{label}</button>
		{/each}
	</div>
{/if}

<style>
	.action {
		--at-apply: h-8 w-full px-3 text-start text-sm transition-colors use-transition-standard;
	}

	.action:hover {
		--at-apply: bg-secondary/75;
	}
</style>
