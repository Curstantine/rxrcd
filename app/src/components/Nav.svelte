<script>
	import { location } from "svelte-spa-router";

	import Input from "@/components/Input.svelte";
	import SearchCombo from "@/components/search_combo/SearchCombo.svelte";

	import { extort_nav_state, extort_search_state } from "./Nav.hooks.js";

	const search_input_id = "search_input";

	const [[show, search], entries, { close }] = extort_search_state();
	const [[back_disabled, forward_disabled], { back, forward }] = extort_nav_state();
</script>

<div class="sticky inset-x-0 top-0 z-20 flex bg-background/95 px-4 py-2 backdrop-blur-xl space-x-4">
	<div class="inline-flex space-x-2">
		<a href="#/" class="h-8 w-8 icon-button-ghost">
			<div class="i-symbols-home-rounded h-5 w-5" class:text-foreground={$location === "/"} />
		</a>

		<button on:click={back} class="h-8 w-8 icon-button-ghost" disabled={$back_disabled}>
			<div class="i-symbols-chevron-left h-5 w-5" />
		</button>
		<button on:click={forward} class="h-8 w-8 icon-button-ghost" disabled={$forward_disabled}>
			<div class="i-symbols-chevron-right h-5 w-5" />
		</button>
	</div>

	<div class="relative flex-1">
		<Input id={search_input_id} bind:input={$search} placeholder="Search" class="w-full" />

		{#if $show}
			<SearchCombo data={$entries} coupling_id={search_input_id} {close} />
		{/if}
	</div>

	<a href="#/settings" class="h-8 w-8 icon-button-ghost">
		<div class="i-symbols-settings-rounded h-5 w-5" class:text-foreground={$location === "/settings"} />
	</a>
</div>
