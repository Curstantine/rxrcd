<script>
	import { Link } from "svelte-routing";

	import Input from "@/components/Input.svelte";

	/** @type {string} */
	let search;
</script>

<div class="sticky inset-x-0 top-0 flex bg-background/95 px-4 py-2 backdrop-blur-xl space-x-4">
	<div class="inline-flex space-x-2">
		<Link to="/" let:active class="h-8 w-8 icon-button-layout ghost-button">
			<div class="i-symbols-home-rounded h-5 w-5" class:text-foreground={active} />
		</Link>
		<button class="h-8 w-8 icon-button-layout ghost-button">
			<div class="i-symbols-chevron-left h-5 w-5" />
		</button>
		<button class="h-8 w-8 icon-button-layout ghost-button">
			<div class="i-symbols-chevron-right h-5 w-5" />
		</button>
	</div>

	<Input bind:input={search} placeholder="Search" class="flex-1" />

	<Link to="/settings" let:active class="h-8 w-8 icon-button-layout ghost-button">
		<div class="i-symbols-settings-rounded h-5 w-5" class:text-foreground={active} />
	</Link>
</div>
