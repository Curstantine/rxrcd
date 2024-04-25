<script>
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

	/** @type {number} */
	let resize_timeout;

	/** @type {HTMLDivElement} */
	let listbox;

	let expanded = false;
	let listbox_width = 0;

	/** @type {"top" | "bottom" }*/
	let flyout_position = "bottom";

	/** @param {string} action_value */
	const on_action_click = (action_value) => {
		expanded = false;
		on_change(action_value);
	};

	/** @type {import("svelte/action").Action<HTMLButtonElement>} */
	function combobox_extensions(node) {
		let node_top = 0;

		const calculate_listbox_position = () => {
			const rect = node.getBoundingClientRect();
			listbox_width = rect.width;
			node_top = rect.top;
		};

		const on_click = () => {
			const probable_listbox_height = node.offsetHeight * actions.length;
			if (document.documentElement.offsetHeight > node_top + probable_listbox_height) {
				flyout_position = "bottom";
			} else {
				flyout_position = "top";
			}

			expanded = !expanded;
		};

		const on_window_resize = () => {
			if (expanded) expanded = false;
			clearTimeout(resize_timeout);
			resize_timeout = window.setTimeout(calculate_listbox_position, 1000);
		};

		calculate_listbox_position();
		node.addEventListener("click", on_click);
		window.addEventListener("resize", on_window_resize);

		return {
			destroy: () => {
				node.removeEventListener("click", on_click);
				window.removeEventListener("resize", on_window_resize);
			},
		};
	}
</script>

<div class="relative">
	<button
		{id}
		{disabled}
		role="combobox"
		aria-expanded={expanded}
		aria-controls={id}
		class={$$props.class}
		use:combobox_extensions
	>
		<span class="pointer-events-none text-sm">{label}</span>
		<div class="i-symbols-expand-all-rounded text-muted-foreground"></div>
	</button>

	{#if expanded}
		<div
			bind:this={listbox}
			role="listbox"
			class:bottom-11={flyout_position === "top"}
			class:top-11={flyout_position === "bottom"}
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
</div>

<style>
	button[role="combobox"] {
		--at-apply: flex justify-between items-center min-w-32 rounded-md h-9 px-3 border-(1 solid border);
	}

	button[role="combobox"]:disabled {
		--at-apply: opacity-50 pointer-events-none;
	}

	div[role="listbox"] {
		--at-apply: absolute inset-x-0 z-10 w-fit flex flex-col border-(1 border solid) rounded-md bg-background py-1
			shadow-md;
	}

	.action {
		--at-apply: h-9 w-full px-3 text-start text-sm transition-colors use-transition-standard leading-tight
			text-nowrap;
	}

	.action:hover {
		--at-apply: bg-secondary/75;
	}
</style>
