<script>
	import SettingsNavigationItem from "$lib/components/items/SettingsNavigationItem.svelte";
</script>

<main class="grid grid-cols-[16rem_1fr] w-full 2xl:container">
	<div class="flex flex-col border-r-(1 border solid) px-2 pt-2">
		<SettingsNavigationItem icon="i-symbols-person-outline-rounded" label="Account" />
		<SettingsNavigationItem icon="i-symbols-palette-outline" label="Appearance" />
		<SettingsNavigationItem icon="i-symbols-download-rounded" label="Downloads" />
	</div>

	<slot />
</main>
