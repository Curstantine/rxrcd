<script>
	/** @type {string | undefined} */
	export let id;

	/** @type {boolean} */
	export let value;

	/** @type {boolean | undefined} */
	export let disabled;

	/** @type {(val: boolean) => void | Promise<void>} */
	export let on_click;
</script>

<button
	{id}
	{disabled}
	role="switch"
	aria-checked={value}
	class:bg-primary={value}
	class="h-5 w-9 inline-flex items-center rounded-full bg-input transition-background-color"
	on:click={() => on_click(!value)}
>
	<div
		class="ml-0.5 size-4 translate-x-0 transform-gpu rounded-full bg-background transition-transform"
		class:translate-x-full={value}
	></div>
</button>
