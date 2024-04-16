<script>
	import { page } from "$app/stores";

	/** @type {import('./$types').PageData} */
	export let data;

	/**
	 * @typedef {{ id: string, label: string, rel_type: import("$lib/types/deezer").AlbumRecordType | "discography" }} TabItem
	 * @type {TabItem[]}
	 */
	const tabs = [
		{ id: "tab_discography", rel_type: "discography", label: "Discography" },
		{ id: "tab_albums", rel_type: "album", label: "Albums" },
		{ id: "tab_eps", rel_type: "ep", label: "EPs" },
		{ id: "tab_singles", rel_type: "single", label: "Singles" },
		{ id: "tab_compilations", rel_type: "compilation", label: "Compilations" },
	];

	/**
	 *  @param {HTMLDivElement} node
	 */
	function style_tab(node) {
		/**
		 * @param {TabItem} param0
		 */
		const update_styles = ({ id }) => {
			const element = document.getElementById(id);
			if (element === null) throw new Error(`Tab by #${id} doesn't exist`);

			node.style.width = `${element.offsetWidth}px`;
			node.style.transform = `translateX(${element.offsetLeft}px)`;
		};

		const un_sub = page.subscribe((o) => {
			o.url;
			update_styles();
		});

		return {
			destroy: () => un_sub(),
		};
	}

	/**
	 *  @param {HTMLDivElement} node
	 */
	function style_tab_bar(node) {
		const body = document.getElementsByTagName("body")[0];
		let marked = false;

		const styler = () => {
			if (!marked && body.scrollTop >= node.offsetTop) {
				node.style.setProperty("--un-bg-color-opacity", "0.95");
				marked = true;
				return;
			}

			if (marked && node.offsetTop > body.scrollTop) {
				node.style.setProperty("--un-bg-color-opacity", "1");
				marked = false;
				return;
			}
		};

		body.addEventListener("scroll", styler);

		return {
			destroy: () => body.removeEventListener("scroll", styler),
		};
	}
</script>

<div class="flex flex-col container">
	<div class="h-64 flex items-center p-6">
		{#await data.artist}
			<div class="h-48 w-48 animate-pulse rounded-full bg-secondary use-transition-emphasized"></div>

			<div class="flex flex-col gap-2 pl-6 transition-colors use-transition-emphasized">
				<div class="h-6 w-xs animate-pulse rounded bg-secondary" />

				<div class="flex items-center gap-2 text-muted-foreground">
					<div class="h-3 w-16 animate-pulse rounded bg-secondary" />
					<div class="h-3 w-12 animate-pulse rounded bg-secondary" />
				</div>
			</div>
		{:then artist}
			<div class="h-48 w-48 rounded-full bg-secondary">
				<img src={artist.picture_big} alt="{artist.name}'s Avatar" class="w-full rounded-full" />
			</div>

			<div class="flex flex-col pl-6 text-sm text-muted-foreground space-y-1">
				<h1 class="select-text text-4xl text-foreground font-bold">{artist.name}</h1>
				<div class="flex gap-2">
					<span>{artist.nb_fan.toLocaleString()} fans</span> ·
					<span>{artist.nb_album.toLocaleString()} releases</span>
				</div>
			</div>
		{/await}
	</div>

	<div use:style_tab_bar style="--un-bg-color-opacity: 1;" class="tab_bar">
		{#each tabs as { id, label, rel_type }}
			<a {id} class="tab" class:active={$page.url.pathname.endsWith(rel_type)} href="./{rel_type}">{label}</a>
		{/each}

		<div use:style_tab class="tab_bar_decoration" />
	</div>

	<slot />
</div>

<style>
	.tab_bar {
		--at-apply: sticky top-12 flex gap-2 bg-background/[--un-bg-color-opacity] px-6 backdrop-blur-xl;
	}

	.tab {
		--at-apply: h-12 px-2 text-sm text-muted-foreground use-transition-standard transition-colors;
	}

	.tab:hover,
	.tab.active {
		--at-apply: text-foreground;
	}

	.tab_bar_decoration {
		--at-apply: absolute bottom-2 left-0 h-[2px] transform-gpu rounded-full bg-primary use-transition-standard;
	}
</style>
