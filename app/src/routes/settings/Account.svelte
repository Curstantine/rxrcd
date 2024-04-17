<script>
	import SettingsHeading from "@/components/SettingsHeading.svelte";
</script>

<article id="account">
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
