<script>
	import { derived, writable } from "svelte/store";

	import Discography from "@/routes/artist/Discography.svelte";
	import DistinctAlbumPages from "@/routes/artist/DistinctAlbumPages.svelte";

	import {
		tabs,
		extort_tab_state,
		extort_data_state,
		style_tab,
		style_tab_bar,
	} from "@/routes/artist/Artist.hooks.js";

	/** @type {{ id?: string }} */
	export let params = {};

	// Note(Curstantine):
	// svelte-spa-router doesn't remount when the param is changed. (e.g. cases where the user navigates to a new artist page from an artist page)
	// causing data to stay stale. We can fix that by listening to id and hoisting it as a readable.
	const id = writable(params.id);
	$: $id = params.id;

	const { artist, albums } = extort_data_state(id);
	const { active_tab, on_tab_click } = extort_tab_state();

	const tab_rel_type = derived(active_tab, ({ rel_type }) => rel_type);
</script>

<div class="flex flex-col container">
	<div class="h-64 flex items-center p-6">
		{#if $artist !== null}
			<div class="h-48 w-48 rounded-full bg-secondary">
				<img src={$artist.picture_big} alt="{$artist.name}'s Avatar" class="w-full rounded-full" />
			</div>

			<div class="flex flex-col pl-6 text-sm text-muted-foreground space-y-1">
				<h1 class="select-text text-4xl text-foreground font-bold">{$artist.name}</h1>
				<div class="flex gap-2">
					<span>{$artist.nb_fan.toLocaleString()} fans</span> ·
					<span>{$artist.nb_album.toLocaleString()} releases</span>
				</div>
			</div>
		{:else}
			<div class="h-48 w-48 animate-pulse rounded-full bg-secondary use-transition-emphasized"></div>

			<div class="flex flex-col gap-2 pl-6 transition-colors use-transition-emphasized">
				<div class="h-6 w-xs animate-pulse rounded bg-secondary" />

				<div class="flex items-center gap-2 text-muted-foreground">
					<div class="h-3 w-16 animate-pulse rounded bg-secondary" />
					<div class="h-3 w-12 animate-pulse rounded bg-secondary" />
				</div>
			</div>
		{/if}
	</div>

	<div use:style_tab_bar style="--un-bg-color-opacity: 1;" class="tab_bar">
		{#each tabs as { id, label }}
			<button {id} class="tab" class:active={$active_tab.id === id} on:click={on_tab_click}>{label}</button>
		{/each}

		<div
			use:style_tab={active_tab}
			class="absolute bottom-2 left-0 h-[2px] transform-gpu rounded-full bg-primary use-transition-standard"
		/>
	</div>

	{#if $tab_rel_type === null}
		<Discography data={albums} />
	{:else}
		<DistinctAlbumPages type={tab_rel_type} data={albums} />
	{/if}
</div>

<style>
	.tab_bar {
		--at-apply: sticky top-12 flex gap-2 bg-background/[--un-bg-color-opacity] px-6 backdrop-blur-xl;
	}

	.tab {
		--at-apply: h-12 px-2 text-sm text-muted-foreground use-transition-standard transition-colors;
	}

	.tab:hover,
	.tab.active {
		--at-apply: text-foreground;
	}
</style>
