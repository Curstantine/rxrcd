<script>
	/** @type {string} */
	export let heading;

	/** @type {string} */
	export let sub;
</script>

<div class="{$$props.class} flex flex-col">
	<h1 class="text-lg font-medium leading-tight">{heading}</h1>
	<span class="text-sm text-muted-foreground">{sub}</span>
</div>
