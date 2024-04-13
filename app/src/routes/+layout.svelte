<script>
	import { onMount } from "svelte";
	import { invoke } from "@tauri-apps/api";
	import { appWindow } from "@tauri-apps/api/window";

	import { initialize_theme } from "$lib/utils/theme";

	import Nav from "$lib/components/Nav.svelte";
	import SnackBarManager from "$lib/components/snack/SnackBarManager.svelte";

	onMount(async () => {
		await invoke("setup");
		await initialize_theme();

		await appWindow.show();
		await appWindow.setFocus();
	});
</script>

<Nav />
<slot />
<SnackBarManager />
