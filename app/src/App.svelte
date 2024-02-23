<script>
	import { invoke } from "@tauri-apps/api";
	import { appWindow } from "@tauri-apps/api/window";
	import { onMount } from "svelte";
	import Router from "svelte-spa-router";

	import { routes } from "@/routes";
	import { initialize_theme } from "@/utils/theme";

	import Nav from "@/components/Nav.svelte";

	onMount(async () => {
		await invoke("setup");
		await initialize_theme();

		await appWindow.show();
		await appWindow.setFocus();
	});
</script>

<Nav />
<Router {routes} />
