<script>
	import Input from "@/components/Input.svelte";

	/** @type {string} */
	let search;
</script>

<div class="flex overflow-y-auto px-4 py-2">
	<Input bind:input={search} placeholder="Search" />
</div>
