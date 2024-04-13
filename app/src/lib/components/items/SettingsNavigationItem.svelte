<script>
	/** @type {string} */
	export let label;

	/** @type {string} */
	export let icon;
</script>

<div class="h-10 flex items-center gap-x-2 rounded-md px-2 text-foreground use-transition-standard hover:bg-secondary">
	<div class="{icon} h-5 w-5" />
	<span class="text-sm">{label}</span>
</div>
