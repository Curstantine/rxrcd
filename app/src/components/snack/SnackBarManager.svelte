<script>
	import SnackBar from "@/components/snack/SnackBar.svelte";
	import { snacks } from "@/components/snack/snack";
</script>

{#each $snacks as data, index}
	<SnackBar {data} {index} length={$snacks.length} />
{/each}
