<script>
	import { set_theme, selected_theme, themes } from "@/utils/theme";

	import SettingsHeading from "@/components/SettingsHeading.svelte";
	import Select from "@/components/select/Select.svelte";

	const actioned_themes = themes.map(({ label, id }) => ({ label, value: id }));

	/** @param {import("@/types/utils").Theme} changed */
	const change_theme = async (changed) => {
		if (changed !== $selected_theme.id) await set_theme(changed);
	};
</script>

<article id="appearance">
	<SettingsHeading heading="Appearance" sub="Tweak rxrcd's appearance to your liking" />

	<div class="ambatukam mt-4 items-center">
		<span class="grid-area-[header]">Color Scheme</span>
		<span class="grid-area-[description] text-sm text-muted-foreground">Change the general colors</span>

		<Select
			label={$selected_theme?.label ?? "N/A"}
			class="grid-area-[option] w-42"
			aria_controls="select-color-scheme"
			actions={actioned_themes}
			on_change={change_theme}
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
