<script>
	import { appWindow } from "@tauri-apps/api/window";
	import { onMount } from "svelte";
	import Router from "svelte-spa-router";

	import { routes } from "@/routes";
	import { initialize_theme } from "@/utils/theme";
	import { setup } from "@/bindings";

	import Nav from "@/components/Nav.svelte";
	import SnackBarManager from "@/components/snack/SnackBarManager.svelte";

	onMount(async () => {
		await setup();
		await initialize_theme();

		await appWindow.show();
		await appWindow.setFocus();
	});
</script>

<Nav />
<Router {routes} />
<SnackBarManager />
