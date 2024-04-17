<script>
	import SettingsNavigationItem from "$lib/components/items/SettingsNavigationItem.svelte";
</script>

<main class="grid grid-cols-[16rem_1fr] w-full 2xl:container">
	<div class="flex flex-col border-r-(1 border solid) px-2 pt-2">
		<SettingsNavigationItem label="Account" href="/settings/account" class="i-symbols-person-outline-rounded" />
		<SettingsNavigationItem label="Appearance" href="/settings/appearance" class="i-symbols-palette-outline" />
		<SettingsNavigationItem label="Downloads" href="/settings/downloads" class="i-symbols-download-rounded" />
	</div>

	<slot />
</main>
