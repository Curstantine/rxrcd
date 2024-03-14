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
	<div class="h-64 flex items-center p-8 space-x-6">
		<div class="aspect-square h-full rounded-lg bg-secondary">
			<!-- <img src={$artist.picture_big} alt="{$artist.name}'s Avatar" class="w-full rounded-full" /> -->
		</div>

		<div class="flex flex-col">
			<h1 class="select-text text-4xl text-foreground font-bold">Nurture</h1>
			<h1 class="select-text text-xl text-primary">Porter Robinson</h1>
			<div class="flex gap-2 pt-1 text-sm text-muted-foreground">
				<span>14 tracks</span> ·
				<span>58 minutes</span> ·
				<span>{new Date("2021-04-24").toLocaleDateString()}</span> ·
				<span>3,766 fans</span>
			</div>
		</div>
	</div>
	<span>{$id}</span>
</div>
