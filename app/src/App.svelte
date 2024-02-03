<script>
	import { invoke } from "@tauri-apps/api";
	import { onMount } from "svelte";

	onMount(async () => {
		await invoke("setup");
		// const config_appearance = await invoke("config_get_appearance");
	});
</script>

<span> hai :3 </span>
