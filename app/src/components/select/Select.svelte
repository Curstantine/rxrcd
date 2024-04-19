<script>
	/** @type {string} */
	export let label;

	let expanded = false;
</script>

<button
	role="combobox"
	aria-expanded="false"
	aria-controls="test"
	on:click={() => (expanded = !expanded)}
	class="{$$props.class} flex justify-between items-center min-w-32 rounded-md h-9 px-3 border-(1 solid border)"
>
	<span class="text-sm">{label}</span>
	<div class="i-symbols-expand-all-rounded"></div>
</button>
