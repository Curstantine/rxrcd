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
		<div
			class="h-8 flex items-center justify-between px-2 text-muted-foreground transition-colors use-transition-emphasized"
		>
			<div class="h-2 w-12 animate-pulse rounded bg-secondary" />
			<div class="h-3 w-3 rounded-full bg-secondary" />
		</div>
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
		{#each data as [{ title, href, list_type }, items]}
			<a
				{href}
				class="h-8 flex items-center justify-between px-2 text-muted-foreground transition-colors use-transition-standard hover:text-primary"
			>
				<span class="text-sm">{title}</span>
				<div class="i-symbols-chevron-right h-5 w-5" />
			</a>

			{#if list_type === "list"}
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
