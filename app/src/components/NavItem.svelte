<script>
	import { Link } from "svelte-routing";

	/** @type {string} */
	export let href;

	/** @type {string} */
	export let icon;
</script>

<Link
	to={href}
	let:active
	class="h-10 w-10 inline-flex items-center justify-center text-muted-foreground transition-color use-transition-standard hover:text-foreground"
>
	<div class="{icon} w-5 h-5" class:text-foreground={active} />
</Link>
