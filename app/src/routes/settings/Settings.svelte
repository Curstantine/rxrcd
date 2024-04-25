<script>
	import { onMount } from "svelte";
	import Router, { replace } from "svelte-spa-router";

	import SettingsNavigationItem from "@/components/items/SettingsNavigationItem.svelte";

	import Appearance from "@/routes/settings/Appearance.svelte";
	import Account from "@/routes/settings/Account.svelte";
	import Downloads from "@/routes/settings/Downloads.svelte";

	/** @type { { wild : string }} */
	export let params;

	const routes = {
		"/account": Account,
		"/appearance": Appearance,
		"/downloads": Downloads,
	};

	/** @type {{ icon: string, label: string, sub_route: string }[]} */
	const tabs = [
		{ icon: "i-symbols-person-outline-rounded", label: "Account", sub_route: "account" },
		{ icon: "i-symbols-palette-outline", label: "Appearance", sub_route: "appearance" },
		{ icon: "i-symbols-download-rounded", label: "Downloads", sub_route: "downloads" },
	];

	onMount(() => {
		if (!params?.wild) replace("/settings/account");
	});
</script>

<main class="grid grid-cols-[16rem_1fr] w-full 2xl:container">
	<div class="side_nav">
		{#each tabs as { icon, label, sub_route }}
			<SettingsNavigationItem active={params?.wild === sub_route} href="/settings/{sub_route}" {icon} {label} />
		{/each}
	</div>

	<div class="col-start-2 px-6 pt-4">
		<Router prefix="/settings" {routes} />
	</div>
</main>

<style>
	.side_nav {
		height: calc(100vh - var(--nav-height));
		--at-apply: sticky top-12 w-64 flex flex-col border-r-(1 border solid) px-2 pt-2;
	}
</style>
