<script>
	import { onMount } from "svelte";

	import Discography from "@/routes/artist/Discography.svelte";

	/** @type {{ id: string }} */
	export let params = {};

	/** @type {{ id: string, label: string, component: import("svelte").ComponentType | null }[]} */
	const tabs = [
		{ id: "tab_discography", label: "Discography", component: Discography },
		{ id: "tab_albums", label: "Albums", component: null },
		{ id: "tab_eps", label: "EPs", component: null },
		{ id: "tab_singles", label: "Singles", component: null },
		{ id: "tab_compilations", label: "Compilations", component: null },
	];

	/** @type {HTMLDivElement}*/
	let active_rod;
	let active_tab = tabs[0];

	/**
	 * @param e {MouseEvent & { currentTarget: HTMLButtonElement }}
	 */
	function on_tab_click(e) {
		active_tab = tabs.find(({ id }) => e.currentTarget.id === id);
		style_active_rod(e.currentTarget);
	}

	/**
	 * @param e {HTMLElement}
	 */
	function style_active_rod(e) {
		active_rod.style.width = `${e.offsetWidth}px`;
		active_rod.style.transform = `translateX(${e.offsetLeft}px)`;
	}

	onMount(() => style_active_rod(document.getElementById(active_tab.id)));
</script>

<div class="flex flex-col">
	<div class="h-48 flex items-center p-6">
		<div class="h-36 w-36 rounded-full bg-secondary"></div>

		<div class="grid grid-cols-4 flex-1 pl-6 text-sm text-muted-foreground space-y-1">
			<h1 class="col-span-full text-3xl text-foreground font-bold">Daft Punk</h1>
			<span>Fans: 120123123</span>
			<span>Albums: 45</span>
			<span>Tracks: 1024</span>
		</div>
	</div>

	<div class="sticky top-12 flex gap-2 bg-background/95 px-6 backdrop-blur-xl">
		{#each tabs as { id, label }}
			<button {id} class="tab" class:active={active_tab.id === id} on:click={on_tab_click}>{label}</button>
		{/each}

		<div
			bind:this={active_rod}
			class="absolute bottom-2 left-0 h-[2px] transform-gpu rounded-full bg-primary use-transition-standard"
		/>
	</div>

	<svelte:component this={active_tab.component} />
</div>

<style>
	.tab {
		--at-apply: h-12 px-2 text-sm text-muted-foreground use-transition-standard transition-colors;
	}

	.tab:hover,
	.tab.active {
		--at-apply: text-foreground;
	}
</style>
