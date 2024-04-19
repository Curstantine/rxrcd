<script>
	import SettingsHeading from "@/components/SettingsHeading.svelte";
	import Select from "@/components/select/Select.svelte";

	let selected_mode = "dark";
	const scheme_modes = [
		{ label: "Dark", value: "dark" },
		{ label: "Light", value: "light" },
	];
</script>

<article id="appearance">
	<SettingsHeading heading="Appearance" sub="Tweak rxrcd's appearance to your liking" />

	<div class="ambatukam mt-4 items-center">
		<span class="grid-area-[header]">Color Scheme</span>
		<span class="grid-area-[description] text-sm text-muted-foreground">Change the general colors</span>

		<Select
			label={scheme_modes.find(({ value }) => value === selected_mode).label}
			class="grid-area-[option] w-42"
			aria_controls="color-scheme"
			actions={scheme_modes}
			on_change={(mode) => (selected_mode = mode)}
		/>
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
