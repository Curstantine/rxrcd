<script>
	import { set_theme, selected_theme, themes } from "@/utils/theme";

	import SettingsHeading from "@/components/SettingsHeading.svelte";
	import Select from "@/components/select/Select.svelte";
	import SettingsOptionArea from "@/components/SettingsOptionArea.svelte";

	const actioned_themes = themes.map(({ label, id }) => ({ label, value: id }));

	/** @param {string} changed */
	const change_theme = async (changed) => {
		// @ts-expect-error changed is of type Theme as we feed values in themes
		if (changed !== $selected_theme?.id) await set_theme(changed);
	};
</script>

<article id="appearance">
	<SettingsHeading heading="Appearance" sub="Tweak rxrcd's look and feel" />

	<SettingsOptionArea class="mt-4" label="Theme" subtitle="Change application-wide color scheme">
		<Select
			label={$selected_theme?.label ?? "N/A"}
			class="w-42"
			aria_controls="select-color-scheme"
			actions={actioned_themes}
			on_change={change_theme}
		/>
	</SettingsOptionArea>
</article>
