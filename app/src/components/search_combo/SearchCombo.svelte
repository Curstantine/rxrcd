<script>
	import { fade } from "svelte/transition";
	import { bounceIn, bounceOut } from "svelte/easing";

	import SearchComboItemSkeleton from "./SearchComboItemSkeleton.svelte";
	import SearchComboItem from "@/components/search_combo/SearchComboItem.svelte";

	/** @type {boolean} */
	export let loading;

	/** @type {import("@/types/search").SearchEntries} */
	export let data;
</script>

<div
	in:fade={{ duration: 300, easing: bounceIn }}
	out:fade={{ duration: 300, easing: bounceOut }}
	class="absolute inset-x-0 top-12 grid grid-cols-1 max-h-md gap-4 overflow-y-auto border-(1 border solid) rounded bg-background p-2 shadow"
	on:blur
>
	{#if loading}
		<div class="flex flex-col">
			<div class="mx-2 mb-1 h-2 w-12 animate-pulse rounded bg-secondary use-transition-emphasized" />
			<SearchComboItemSkeleton />
			<SearchComboItemSkeleton />
			<SearchComboItemSkeleton />
		</div>
	{:else}
		{#each data as [title, items]}
			<div class="flex flex-col">
				<span class="px-2 pb-1 text-sm text-muted-foreground">{title}</span>
				{#each items as item}
					<SearchComboItem data={item} />
				{/each}
			</div>
		{/each}
	{/if}
</div>
