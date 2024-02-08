<script>
	/** @type {import("@/types/search").SearchEntry}*/
	export let data;
</script>

<div class="{$$props.class} flex flex-col px-2 py-1 w-36 use-transition-standard transition-colors">
	<div class="inilne-flex h-32 w-32 items-center justify-center border-(1 border solid) rounded hover:bg-secondary">
		{#if data.image}
			<img src={data.image} alt="{data.title} Cover Preview" class="rounded" />
		{/if}
	</div>
	<span class="pt-2 text-sm text-foreground leading-tight">{data.title}</span>
	{#if data.subtitle}
		<span class="pt-1 text-xs text-muted-foreground">{data.subtitle}</span>
	{/if}
</div>
