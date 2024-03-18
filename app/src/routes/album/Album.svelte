<script>
	import { writable } from "svelte/store";

	import { extort_data_state } from "@/routes/album/Album.hook";

	/** @type {{ id?: string }} */
	export let params = {};

	// Note(Curstantine):
	// svelte-spa-router doesn't remount when the param is changed. (e.g. cases where the user navigates to a new artist page from an artist page)
	// causing data to stay stale. We can fix that by listening to id and hoisting it as a readable.
	const id = writable(params.id);
	$: $id = params.id;

	const { album } = extort_data_state(id);
</script>

<div class="flex flex-col">
	<div class="info_bar">
		{#if $album !== null}
			<div class="artwork">
				<img src={$album.cover_big ?? ""} alt="{$album.title} cover" class="w-full rounded-md" />
			</div>

			<div class="headings">
				<h1 class="select-text text-4xl text-foreground font-bold">{$album.title}</h1>
				<h1 class="select-text text-xl text-primary">{$album.artist.name}</h1>
				<div class="flex items-center gap-2 pt-1 text-sm text-muted-foreground">
					<span>{$album.tracks?.data.length ?? "N/A"} tracks</span> ·
					<span>58 minutes</span> ·
					<span>{new Date($album.release_date).toLocaleDateString()}</span> ·
					<span>3,766 fans</span>
				</div>
			</div>
		{:else}
			<div class="artwork animate-pulse use-transition-standard" />
			<div class="headings gap-2">
				<div class="h-6 w-48 animate-pulse rounded bg-secondary" />
				<div class="h-4 w-24 animate-pulse rounded bg-secondary" />
				<div class="flex items-center gap-1 pt-2">
					<div class="h-3 w-16 animate-pulse rounded bg-secondary" />
					·
					<div class="h-3 w-16 animate-pulse rounded bg-secondary" />
					·
					<div class="h-3 w-16 animate-pulse rounded bg-secondary" />
				</div>
			</div>
		{/if}

		<div class="grid-area-[primary-actions] h-8 flex">
			<button class="button-primary" disabled={$album === null}>
				<div class="i-symbols-download-rounded h-4 w-4" />
				Download
			</button>
		</div>

		<div class="grid-area-[secondary-actions] h-8 flex">
			<button class="icon-button" disabled={$album === null}>
				<div class="i-symbols-more-vert h-4 w-4" />
			</button>
		</div>
	</div>

	<div class="flex flex-col px-8 py-4">
		<div class="h-10 inline-flex items-center px-4 text-sm text-muted-foreground font-medium">
			<span class="w-10">#</span>
			<span class="max-w-lg flex-1">Title</span>
			<span class="flex-1">Featuring Artists</span>
			<span class="mr-8">Length</span>
		</div>
		{#each new Array(12) as _, i}
			<div class="song-entry">
				<span class="w-10">{i + 1}.</span>
				<span class="max-w-lg flex-1 text-foreground">do-re-mi-fa-so-la-ti-do</span>
				<span class="flex-1">{i === 4 ? "Totally Enormous Extinct Dinosaurs" : "-"}</span>
				<span>3:34</span>
				<button class="ml-6 icon-button-layout w-8">
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

	.artwork {
		grid-area: artwork;
		--at-apply: mr-8 aspect-square h-64 w-64 rounded-lg bg-secondary;
	}

	.headings {
		grid-area: headings;
		--at-apply: max-w-sm flex flex-col self-center;
	}

	.song-entry {
		--at-apply: h-10 inline-flex items-center rounded-md pl-4 pr-2 text-sm text-muted-foreground transition-colors
			use-transition-standard;
	}

	.song-entry:hover {
		--at-apply: bg-muted/50;
	}
</style>
