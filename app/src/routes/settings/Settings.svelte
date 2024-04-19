<script>
	import { onMount } from "svelte";
	import Router, { replace } from "svelte-spa-router";

	import SettingsNavigationItem from "@/components/items/SettingsNavigationItem.svelte";

	import Appearance from "@/routes/settings/Appearance.svelte";
	import Account from "@/routes/settings/Account.svelte";

	/** @type { { wild : string }} */
	export let params;

	const routes = {
		"/account": Account,
		"/appearance": Appearance,
	};

	onMount(() => {
		if (!params?.wild) replace("/settings/account");
	});
</script>

<main class="grid grid-cols-[16rem_1fr] w-full 2xl:container">
	<div class="flex flex-col border-r-(1 border solid) px-2 pt-2">
		<SettingsNavigationItem href="/settings/account" label="Account" icon="i-symbols-person-outline-rounded" />
		<SettingsNavigationItem href="/settings/appearance" label="Appearance" icon="i-symbols-palette-outline" />
		<SettingsNavigationItem href="/settings/downloads" label="Downloads" icon="i-symbols-download-rounded" />
	</div>

	<div class="px-6 pt-4">
		<Router prefix="/settings" {routes} />
	</div>
</main>
