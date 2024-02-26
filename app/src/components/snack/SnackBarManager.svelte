<script>
	import SnackBar from "@/components/snack/SnackBar.svelte";
	import { pauseSnackTimeouts, resumeSnackTimeouts, snacks } from "@/components/snack/snack";

	let expand = false;

	const onEnter = () => {
		if (expand) return;

		expand = true;
		pauseSnackTimeouts();
		console.log("enter");
	};

	const onLeave = () => {
		if (!expand) return;

		expand = false;
		resumeSnackTimeouts();
		console.log("leave");
	};
</script>

<ul class="contents" role="group" on:mouseenter={onEnter} on:mouseleave={onLeave}>
	{#each $snacks as data, index}
		<SnackBar {data} {index} length={$snacks.length} {expand} />
	{/each}
</ul>
