<script>
	import { onMount } from "svelte";
	import { invoke } from "@tauri-apps/api";
	import { appWindow } from "@tauri-apps/api/window";

	import { initialize_theme } from "$lib/utils/theme";

	import Nav from "$lib/components/nav/Nav.svelte";
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

<!-- Note(Curstantine): For some reason the global attribute does nothing -->
<style global>
	:global(:root) {
		font-synthesis: none;
		text-rendering: optimizeLegibility;
		-webkit-font-smoothing: antialiased;
		-moz-osx-font-smoothing: grayscale;

		--radius: 0.5rem;
		--at-apply: font-system;
	}

	:global(html) {
		--at-apply: w-screen h-screen;
	}

	:global(body) {
		--at-apply: m-0 h-screen grid grid-rows-[auto_1fr] min-h-screen bg-background text-foreground overflow-y-auto;
	}

	:global(::-webkit-scrollbar) {
		background-color: transparent;
		--at-apply: w-3;
	}

	:global(::-webkit-scrollbar-thumb) {
		--at-apply: bg-muted;
	}

	:global(.light) {
		--background: 0 0% 100%;
		--foreground: 240 10% 3.9%;
		--card: 0 0% 100%;
		--card-foreground: 240 10% 3.9%;
		--popover: 0 0% 100%;
		--popover-foreground: 240 10% 3.9%;
		--primary: 240 5.9% 10%;
		--primary-foreground: 0 0% 98%;
		--secondary: 240 4.8% 95.9%;
		--secondary-foreground: 240 5.9% 10%;
		--muted: 240 4.8% 95.9%;
		--muted-foreground: 240 3.8% 46.1%;
		--accent: 240 4.8% 95.9%;
		--accent-foreground: 240 5.9% 10%;
		--destructive: 0 84.2% 60.2%;
		--destructive-foreground: 0 0% 98%;
		--border: 240 5.9% 90%;
		--input: 240 5.9% 90%;
		--ring: 240 5.9% 10%;
		color-scheme: light;
	}

	:global(.dark) {
		--background: 240 10% 3.9%;
		--foreground: 0 0% 98%;
		--card: 240 10% 3.9%;
		--card-foreground: 0 0% 98%;
		--popover: 240 10% 3.9%;
		--popover-foreground: 0 0% 98%;
		--primary: 0 0% 98%;
		--primary-foreground: 240 5.9% 10%;
		--secondary: 240 3.7% 15.9%;
		--secondary-foreground: 0 0% 98%;
		--muted: 240 3.7% 15.9%;
		--muted-foreground: 240 5% 64.9%;
		--accent: 240 3.7% 15.9%;
		--accent-foreground: 0 0% 98%;
		--destructive: 0 62.8% 30.6%;
		--destructive-foreground: 0 0% 98%;
		--border: 240 3.7% 15.9%;
		--input: 240 3.7% 15.9%;
		--ring: 240 4.9% 83.9%;
		color-scheme: dark;
	}
</style>
