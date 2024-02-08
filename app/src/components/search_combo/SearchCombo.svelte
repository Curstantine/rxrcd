<script>
	import { fade } from "svelte/transition";
	import { bounceIn, bounceOut } from "svelte/easing";

	import ListItemSkeleton from "@/components/search_combo/ListItemSkeleton.svelte";
	import ListItem from "@/components/search_combo/ListItem.svelte";
	import GridItem from "@/components/search_combo/GridItem.svelte";
	import GridItemSkeleton from "@/components/search_combo/GridItemSkeleton.svelte";

	/** @type {boolean} */
	export let loading;

	/** @type {import("@/types/search").SearchEntries} */
	export let data;
</script>

<div
	in:fade={{ duration: 300, easing: bounceIn }}
	out:fade={{ duration: 300, easing: bounceOut }}
	class="absolute inset-x-0 top-12 grid grid-cols-1 max-h-md overflow-y-auto border-(1 border solid) rounded bg-background p-2 shadow"
	on:blur
>
	{#if loading}
		<div class="mx-2 mb-1 h-2 w-12 animate-pulse rounded bg-secondary use-transition-emphasized" />
		<div class="flex flex-col pb-4">
			<ListItemSkeleton />
			<ListItemSkeleton />
			<ListItemSkeleton />
		</div>

		<div class="mx-2 mb-1 h-2 w-12 animate-pulse rounded bg-secondary use-transition-emphasized" />
		<div class="grid auto-cols-max grid-flow-col pb-4">
			<GridItemSkeleton />
			<GridItemSkeleton />
			<GridItemSkeleton />
		</div>
	{:else}
		{#each data as [title, type, items]}
			<span class="px-2 pb-1 text-sm text-muted-foreground">{title}</span>

			{#if type === "list"}
				<div class="flex flex-col pb-4">
					{#each items as item}
						<ListItem data={item} />
					{/each}
				</div>
			{:else}
				<div class="grid auto-cols-max grid-flow-col pb-4">
					{#each items as item}
						<GridItem data={item} />
					{/each}
				</div>
			{/if}
		{/each}
	{/if}
</div>
