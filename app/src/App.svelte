<script>
	import { invoke } from "@tauri-apps/api";
	import { appWindow } from "@tauri-apps/api/window";
	import { onMount } from "svelte";
	import { Route, Router } from "svelte-routing";

	import { initialize_theme } from "@/utils/theme";

	import Nav from "@/components/Nav.svelte";
	import Home from "@/routes/Home.svelte";
	import Settings from "@/routes/Settings.svelte";

	onMount(async () => {
		await invoke("setup");
		await initialize_theme();

		appWindow.show();
		appWindow.setFocus();
	});

	export let url = "";
</script>

<Router {url}>
	<Nav />

	<Route path="/" component={Home} />
	<Route path="/settings" component={Settings} />
</Router>
