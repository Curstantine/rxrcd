<script>
	import { tabs, extort_tab_state, extort_data_state, style_tab } from "@/routes/artist/Artist.hooks.js";
	import { writable } from "svelte/store";

	/** @type {{ id?: string }} */
	export let params = {};

	// Note(Curstantine):
	// svelte-spa-router doesn't remount when the param is changed. (e.g. cases where the user navigates to a new artist page from an artist page)
	// causing data to stay stale. We can fix that by listening to id and hoisting it as a readable.
	const id = writable(params.id);
	$: $id = params.id;

	const { artist } = extort_data_state(id);
	const { active_tab, on_tab_click } = extort_tab_state();
</script>

<div class="flex flex-col">
	<div class="h-48 flex items-center p-6">
		{#if $artist !== null}
			<div class="h-36 w-36 rounded-full bg-secondary">
				<img src={$artist.picture_big} alt="{$artist.name}'s Avatar" class="w-full rounded-full" />
			</div>

			<div class="grid grid-cols-4 flex-1 pl-6 text-sm text-muted-foreground space-y-1">
				<h1 class="col-span-full text-3xl text-foreground font-bold">{$artist.name}</h1>
				<span>Fans: {$artist.nb_fan}</span>
				<span>Albums: {$artist.nb_album}</span>
				<span>Tracks: 1024</span>
			</div>
		{:else}
			<div class="h-36 w-36 animate-pulse rounded-full bg-secondary use-transition-emphasized"></div>

			<div class="grid grid-cols-4 pl-6 transition-colors use-transition-emphasized space-y-1">
				<div class="col-span-full h-6 w-xs animate-pulse rounded bg-secondary" />
				<div class="h-3 w-24 animate-pulse rounded bg-secondary" />
				<div class="h-3 w-24 animate-pulse rounded bg-secondary" />
			</div>
		{/if}
	</div>

	<div class="sticky top-12 flex gap-2 bg-background/95 px-6 backdrop-blur-xl">
		{#each tabs as { id, label }}
			<button {id} class="tab" class:active={$active_tab.id === id} on:click={on_tab_click}>{label}</button>
		{/each}

		<div
			use:style_tab={active_tab}
			class="absolute bottom-2 left-0 h-[2px] transform-gpu rounded-full bg-primary use-transition-standard"
		/>
	</div>

	<svelte:component this={$active_tab.component} />
</div>

<style>
	.tab {
		--at-apply: h-12 px-2 text-sm text-muted-foreground use-transition-standard transition-colors;
	}

	.tab:hover,
	.tab.active {
		--at-apply: text-foreground;
	}
</style>
