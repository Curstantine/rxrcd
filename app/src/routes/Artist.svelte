<script>
	import { onMount } from "svelte";

	/** @type {{ id: string }} */
	export let params = {};

	/** @type {HTMLDivElement}*/
	let active_rod;
	let active_tab = "latest";

	const tabs = [
		["latest", "Latest & Greatest"],
		["albums", "Albums"],
		["eps", "EPs"],
		["singles", "Singles"],
		["compilations", "Compilations"],
	];

	/**
	 * @param e {MouseEvent & { currentTarget: HTMLButtonElement }}
	 */
	function on_tab_click(e) {
		active_tab = e.currentTarget.id;
		style_active_rod(e.currentTarget);
	}

	/**
	 * @param e {HTMLElement}
	 */
	function style_active_rod(e) {
		active_rod.style.width = `${e.offsetWidth}px`;
		active_rod.style.transform = `translateX(${e.offsetLeft}px)`;
	}

	onMount(() => {
		const x = document.getElementById(active_tab);
		style_active_rod(x);
	});
</script>

<div class="flex flex-col">
	<div class="h-48 flex items-center p-6">
		<div class="h-36 w-36 rounded-full bg-secondary"></div>

		<div class="grid grid-cols-4 flex-1 pl-6 text-sm text-muted-foreground space-y-1">
			<h1 class="col-span-full text-3xl text-foreground font-medium">Daft Punk</h1>
			<span>Fans: 120123123</span>
			<span>Albums: 45</span>
			<span>Tracks: 1024</span>
		</div>
	</div>

	<div class="sticky top-12 flex gap-2 bg-background/95 px-6 backdrop-blur-xl">
		{#each tabs as [id, label]}
			<button {id} class="tab" class:active={active_tab === id} on:click={on_tab_click}>
				{label}
			</button>
		{/each}

		<div
			bind:this={active_rod}
			class="absolute bottom-1 left-0 h-[2px] transform-gpu rounded-full bg-primary use-transition-standard"
		/>
	</div>

	<div class="flex flex-col gap-4 px-6">
		<div class="h-32 w-full bg-white"></div>
		<div class="h-32 w-full bg-white"></div>
		<div class="h-32 w-full bg-white"></div>
		<div class="h-32 w-full bg-white"></div>
		<div class="h-32 w-full bg-white"></div>
		<div class="h-32 w-full bg-white"></div>
		<div class="h-32 w-full bg-white"></div>
		<div class="h-32 w-full bg-white"></div>
		<div class="h-32 w-full bg-white"></div>
		<div class="h-32 w-full bg-white"></div>
	</div>
</div>

<style>
	.tab {
		--at-apply: h-12 px-2 text-muted-foreground use-transition-standard transition-colors;
	}

	.tab.active {
		--at-apply: text-foreground;
	}
</style>
