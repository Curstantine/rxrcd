<script>
	import { location } from "svelte-spa-router";

	import Input from "@/components/Input.svelte";
	import { extort_nav_state } from "./Nav.hooks.js";

	/** @type {string} */
	let search;

	const [[back_disabled, forward_disabled], { back, forward }] = extort_nav_state();
</script>

<div class="sticky inset-x-0 top-0 flex bg-background/95 px-4 py-2 backdrop-blur-xl space-x-4">
	<div class="inline-flex space-x-2">
		<a href="#/" class="h-8 w-8 icon-button-layout ghost-button">
			<div class="i-symbols-home-rounded h-5 w-5" class:text-foreground={$location === "/"} />
		</a>

		<button on:click={back} class="h-8 w-8 icon-button-layout ghost-button" disabled={$back_disabled}>
			<div class="i-symbols-chevron-left h-5 w-5" />
		</button>
		<button on:click={forward} class="h-8 w-8 icon-button-layout ghost-button" disabled={$forward_disabled}>
			<div class="i-symbols-chevron-right h-5 w-5" />
		</button>
	</div>

	<Input bind:input={search} placeholder="Search" class="flex-1" />

	<a href="#/settings" class="h-8 w-8 icon-button-layout ghost-button">
		<div class="i-symbols-settings-rounded h-5 w-5" class:text-foreground={$location === "/settings"} />
	</a>
</div>
