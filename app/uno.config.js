import extractorSvelte from "@unocss/extractor-svelte";
import {
	defineConfig,
	presetIcons,
	presetTypography,
	presetUno,
	presetWebFonts,
	transformerDirectives,
	transformerVariantGroup,
} from "unocss";

/**
 * @param {string} name
 * @returns {() => Promise<IconifyJSON>}
 */
const importIconCollection = (name) => {
	return () => import(`@iconify-json/${name}/icons.json`).then((i) => i.default);
};

export default defineConfig({
	presets: [
		presetUno({ dark: false }),
		presetTypography(),
		presetIcons({
			collections: {
				symbols: importIconCollection("material-symbols"),
			},
		}),
		presetWebFonts({
			fonts: {
				sans: { name: "Inter", weights: [400, 500] },
			},
		}),
	],
	transformers: [
		transformerDirectives(),
		transformerVariantGroup(),
	],
	extractors: [extractorSvelte()],
	theme: {
		easing: {
			emphasized: "cubic-bezier(0.4, 0.0, 0.2, 1.0)",
			standard: "cubic-bezier(0.2, 0.0, 0, 1.0)",
		},
		duration: {
			standard: "300ms",
			emphasized: "500ms",
		},
	},
	shortcuts: {
		"use-transition-standard": "duration-standard ease-standard",
		"use-transition-emphasized": "duration-emphasized ease-emphasized",
	},
});
