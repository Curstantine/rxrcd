<script>
	/** @type {import("@/types/search").SearchEntry}*/
	export let data;
</script>

<div class="{$$props.class} flex flex-col p-2 hover:bg-secondary use-transition-standard transition-colors rounded">
	<span class="text-sm text-foreground">{data.title}</span>
	{#if data.subtitle}
		<span class="text-xs text-muted-foreground">{data.subtitle}</span>
	{/if}
</div>
