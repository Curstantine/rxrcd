<script>
	import { writable } from "svelte/store";

	/** @type {{ id?: string }} */
	export let params = {};

	// Note(Curstantine):
	// svelte-spa-router doesn't remount when the param is changed. (e.g. cases where the user navigates to a new artist page from an artist page)
	// causing data to stay stale. We can fix that by listening to id and hoisting it as a readable.
	const id = writable(params.id);
	$: $id = params.id;
</script>

<div class="flex flex-col">
	<div class="info_bar">
		<div class="grid-area-[artwork] mr-8 aspect-square h-52 w-52 rounded-lg bg-secondary">
			<!-- <img src={$artist.picture_big} alt="{$artist.name}'s Avatar" class="w-full rounded-full" /> -->
		</div>

		<div class="grid-area-[headings] max-w-sm flex flex-col self-center">
			<h1 class="select-text text-4xl text-foreground font-bold">Nurture</h1>
			<h1 class="select-text text-xl text-primary">Porter Robinson</h1>
			<div class="flex gap-2 pt-1 text-sm text-muted-foreground">
				<span>14 tracks</span> ·
				<span>58 minutes</span> ·
				<span>{new Date("2021-04-24").toLocaleDateString()}</span> ·
				<span>3,766 fans</span>
			</div>
		</div>

		<div class="grid-area-[primary-actions] h-8 flex">
			<button class="button-primary">
				<div class="i-symbols-download-rounded h-4 w-4" />
				Download
			</button>
		</div>

		<div class="grid-area-[secondary-actions] h-8 flex">
			<button class="icon-button">
				<div class="i-symbols-more-vert h-4 w-4" />
			</button>
		</div>
	</div>

	<div class="flex flex-col px-8 py-4">
		{#each new Array(12) as _, i}
			<div class="song-entry">
				<span class="w-10">{i + 1}.</span>
				<span class="flex-1 text-foreground">do-re-mi-fa-so-la-ti-do</span>
				<span>3:34</span>
				<button class="ml-2 w-8 icon-button-layout">
					<div class="i-symbols-more-horiz h-4 w-4" />
				</button>
			</div>
		{/each}
	</div>
</div>

<style>
	.info_bar {
		--at-apply: grid p-8 grid-rows-[1fr_auto_auto] grid-cols-[auto_1fr_auto];
		grid-template-areas:
			"artwork headings headings"
			"artwork headings headings"
			"artwork primary-actions secondary-actions";
	}

	.song-entry {
		--at-apply: h-10 inline-flex items-center rounded-md pl-4 pr-2 text-sm text-muted-foreground transition-colors
			use-transition-standard;
	}

	.song-entry:hover {
		--at-apply: bg-muted/50;
	}
</style>
