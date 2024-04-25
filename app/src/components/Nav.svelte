<script>
	import { link } from "svelte-spa-router";
	import active from "svelte-spa-router/active";

	import SearchCombo from "@/components/search_combo/SearchCombo.svelte";

	import { extort_nav_state, extort_search_state } from "./Nav.hooks.js";

	const search_input_id = "search_input";

	const [[show, search], entries, { close }] = extort_search_state();
	const [[back_disabled, forward_disabled], { back, forward }] = extort_nav_state();
</script>

<div class="sticky inset-x-0 top-0 z-20 h-12 flex items-center bg-background/95 px-4 backdrop-blur-xl space-x-4">
	<div class="inline-flex space-x-2">
		<a use:link use:active href="/" class="action_button">
			<div class="i-symbols-home-rounded h-5 w-5" />
		</a>

		<button on:click={back} class="h-8 w-8 icon-button-ghost" disabled={$back_disabled}>
			<div class="i-symbols-chevron-left h-5 w-5" />
		</button>
		<button on:click={forward} class="h-8 w-8 icon-button-ghost" disabled={$forward_disabled}>
			<div class="i-symbols-chevron-right h-5 w-5" />
		</button>
	</div>

	<div class="relative flex-1">
		<input
			id={search_input_id}
			autocomplete="off"
			placeholder="Search"
			bind:value={$search}
			class="h-8 w-full input"
		/>

		{#if $show}
			<SearchCombo data={$entries} coupling_ids={[search_input_id]} {close} />
		{/if}
	</div>

	<a use:link use:active href="/settings" class="action_button">
		<div class="i-symbols-settings-rounded h-5 w-5" />
	</a>
</div>

<style>
	.action_button {
		--at-apply: h-8 w-8 icon-button-ghost;
	}

	.action_button:global(.active) {
		--at-apply: text-foreground;
	}
</style>
