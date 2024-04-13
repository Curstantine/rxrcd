<script>
	import SettingsHeading from "$lib/components/SettingsHeading.svelte";
	import SettingsNavigationItem from "$lib/components/items/SettingsNavigationItem.svelte";

	/** @type {import('./$types').PageData} */
	export let data;
</script>

<main class="grid grid-cols-[16rem_1fr] w-full 2xl:container">
	<div class="flex flex-col border-r-(1 border solid) px-2 pt-2">
		<SettingsNavigationItem icon="i-symbols-person-outline-rounded" label="Account" />
		<SettingsNavigationItem icon="i-symbols-palette-outline" label="Appearance" />
		<SettingsNavigationItem icon="i-symbols-download-rounded" label="Downloads" />
	</div>

	<article class="px-6 pt-4">
		<SettingsHeading heading="Account" sub="Manage your authentication sessions and regions." />

		<div class="auth-card">
			<div class="grid-area-[image] h-24 w-24 self-center justify-self-center rounded-full bg-secondary"></div>

			<div class="grid-area-[description] flex flex-col justify-center">
				<span class="o">Not logged in</span>
				<span class="text-sm text-muted-foreground">N/A</span>
			</div>

			<div class="grid-area-[actions]">
				<button class="button-primary">Logout</button>
			</div>
		</div>
	</article>
</main>

<style>
	.auth-card {
		--at-apply: grid border-border border-1 rounded-md p-4 gap-x-4 mt-3 max-w-lg;
		grid-template-columns: auto 1fr 1fr;
		grid-template-areas:
			"image description description"
			"image description description"
			"image actions actions";
	}
</style>
