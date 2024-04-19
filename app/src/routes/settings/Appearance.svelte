<script>
	import SettingsHeading from "@/components/SettingsHeading.svelte";
	import Select from "@/components/select/Select.svelte";
</script>

<article id="appearance">
	<SettingsHeading heading="Appearance" sub="Tweak rxrcd's appearance to your liking" />

	<div class="ambatukam mt-4 items-center">
		<span class="grid-area-[header]">Color Scheme</span>
		<span class="grid-area-[description] text-sm text-muted-foreground">Change the general colors</span>

		<Select label="Dark" class="grid-area-[option]" />
	</div>
</article>

<style>
	.ambatukam {
		--at-apply: grid grid-cols-[1fr_1fr_auto] py-2;
		grid-template-areas:
			"header header option"
			"description description option";
	}
</style>
