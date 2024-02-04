<script>
	/** @type {string} */
	export let input;
</script>

<input
	{...$$props}
	class="{$$props.class} h-8 inline-flex items-center border-(1 border solid) rounded bg-background px-3 text-sm transition-colors use-transition-standard"
	bind:value={input}
/>

<style>
	input:active,
	input:focus {
		outline: 0;
	}

	input:focus {
		--at-apply: border-ring;
	}

	input::placeholder {
		--at-apply: text-sm;
	}

	input:focus::placeholder {
		opacity: 0;
	}
</style>
