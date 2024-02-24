<script>
	/** @type {string} */
	export let label;

	/** @type {string} */
	export let href;
</script>

<a
	{href}
	class="h-8 flex items-center justify-between px-2 text-muted-foreground transition-colors use-transition-standard hover:text-primary"
>
	<span class="text-sm">{label}</span>
	<div class="i-symbols-chevron-right h-5 w-5" />
</a>
