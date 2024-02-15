<script>
	import GridItem from "@/components/items/GridItem.svelte";

	const subs = [
		{ id: "albums", title: "Albums", items: new Array(8) },
		{ id: "eps", title: "EPs", items: new Array(4) },
		{ id: "singles", title: "Singles", items: new Array(2) },
		{ id: "compilations", title: "Compilations", items: new Array(1) },
	];
</script>

<div class="flex flex-col px-4 pt-4">
	{#each subs as sub}
		<h2 id={sub.id} class="pl-2 font-medium">{sub.title}</h2>
		<div class="grid-list">
			{#each sub.items as _}
				<GridItem title="Test subject 2.0" subtitle="By testing" href="/" image={null} />
			{/each}
		</div>
	{/each}
</div>

<style>
	.grid-list {
		--at-apply: grid gap-y-2 pt-1 pb-6;
		grid-template-columns: repeat(auto-fill, minmax(9rem, 1fr));
	}
</style>
