<script>
	import { onMount } from "svelte";

	import { external_hit } from "@/utils/actions";

	/** @type {boolean} */
	export let disabled = false;

	/** @type {string} */
	export let label;

	/** @type {string} */
	export let id;

	/** @type {import("@/types/select").Action[]} */
	export let actions;

	/** @type {(value: string) => (void | Promise<void>)} */
	export let on_change;

	/** @type {HTMLButtonElement} */
	let button;

	/** @type {number} */
	let resize_timeout;

	let expanded = false;
	let [listbox_top, listbox_left, listbox_width] = [0, 0, 0];

	/** @param {string} action_value */
	const on_action_click = (action_value) => {
		expanded = false;
		on_change(action_value);
	};

	const calculate_listbox_position = () => {
		const rect = button.getBoundingClientRect();
		listbox_top = rect.top + window.scrollY;
		listbox_left = rect.left + window.scrollX;
		listbox_width = rect.right - listbox_left;
	};

	onMount(calculate_listbox_position);
</script>

<svelte:window
	on:resize={() => {
		if (expanded) expanded = false;
		clearTimeout(resize_timeout);
		resize_timeout = window.setTimeout(calculate_listbox_position, 1000);
	}}
/>

<button
	{id}
	{disabled}
	bind:this={button}
	role="combobox"
	aria-expanded={expanded}
	aria-controls={id}
	on:click={() => (expanded = !expanded)}
	class="{$$props.class} combobox"
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
		use:external_hit={{ coupling_ids: [id], close: () => (expanded = false) }}
	>
		{#each actions as { label, value, sub }}
			<button class="action" on:click|once={() => on_action_click(value)}>
				{label}
				{#if sub}
					<span class="text-xs text-muted-foreground">{sub}</span>
				{/if}
			</button>
		{/each}
	</div>
{/if}

<style>
	.combobox {
		--at-apply: flex justify-between items-center min-w-32 rounded-md h-9 px-3 border-(1 solid border);
	}

	.combobox:disabled {
		--at-apply: opacity-50 pointer-events-none;
	}

	.action {
		--at-apply: h-9 w-full px-3 text-start text-sm transition-colors use-transition-standard;
	}

	.action:hover {
		--at-apply: bg-secondary/75;
	}
</style>