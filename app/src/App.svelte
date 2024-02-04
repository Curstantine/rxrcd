<script>
	import { invoke } from "@tauri-apps/api";
	import { appWindow } from "@tauri-apps/api/window";
	import { onMount } from "svelte";

	import { initialize_theme } from "@/utils/theme";

	onMount(async () => {
		await invoke("setup");
		await initialize_theme();

		appWindow.show();
		appWindow.setFocus();
	});
</script>

<span> hai :3 </span>
