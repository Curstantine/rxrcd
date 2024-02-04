import extractorSvelte from "@unocss/extractor-svelte";
import {
	defineConfig,
	presetIcons,
	presetTypography,
	presetUno,
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
		presetUno({ dark: "class" }),
		presetTypography(),
		presetIcons({
			collections: {
				symbols: importIconCollection("material-symbols"),
			},
		}),
	],
	transformers: [
		transformerDirectives(),
		transformerVariantGroup(),
	],
	extractors: [extractorSvelte()],
	theme: {
		fontFamily: {
			system:
				"system-ui, \"Segoe UI\", Roboto, Helvetica, Arial, sans-serif, \"Apple Color Emoji\", \"Segoe UI Emoji\", \"Segoe UI Symbol\"",
		},
		easing: {
			emphasized: "cubic-bezier(0.4, 0.0, 0.2, 1.0)",
			standard: "cubic-bezier(0.2, 0.0, 0, 1.0)",
		},
		duration: {
			standard: "300ms",
			emphasized: "500ms",
		},

		container: {
			center: true,
			padding: "2rem",
			screens: {
				"2xl": "1400px",
			},
		},

		colors: {
			border: "hsl(var(--border) / <alpha-value>)",
			input: "hsl(var(--input) / <alpha-value>)",
			ring: "hsl(var(--ring) / <alpha-value>)",
			background: "hsl(var(--background) / <alpha-value>)",
			foreground: "hsl(var(--foreground) / <alpha-value>)",
			primary: {
				DEFAULT: "hsl(var(--primary) / <alpha-value>)",
				foreground: "hsl(var(--primary-foreground) / <alpha-value>)",
			},
			secondary: {
				DEFAULT: "hsl(var(--secondary) / <alpha-value>)",
				foreground: "hsl(var(--secondary-foreground) / <alpha-value>)",
			},
			destructive: {
				DEFAULT: "hsl(var(--destructive) / <alpha-value>)",
				foreground: "hsl(var(--destructive-foreground) / <alpha-value>)",
			},
			muted: {
				DEFAULT: "hsl(var(--muted) / <alpha-value>)",
				foreground: "hsl(var(--muted-foreground) / <alpha-value>)",
			},
			accent: {
				DEFAULT: "hsl(var(--accent) / <alpha-value>)",
				foreground: "hsl(var(--accent-foreground) / <alpha-value>)",
			},
			popover: {
				DEFAULT: "hsl(var(--popover) / <alpha-value>)",
				foreground: "hsl(var(--popover-foreground) / <alpha-value>)",
			},
			card: {
				DEFAULT: "hsl(var(--card) / <alpha-value>)",
				foreground: "hsl(var(--card-foreground) / <alpha-value>)",
			},
		},

		borderRadius: {
			lg: "var(--radius)",
			md: "calc(var(--radius) - 2px)",
			sm: "calc(var(--radius) - 4px)",
		},
	},
	shortcuts: {
		"use-transition-standard": "duration-standard ease-standard",
		"use-transition-emphasized": "duration-emphasized ease-emphasized",

		"icon-button-layout":
			"inline-flex items-center justify-center rounded transition-colors use-transition-standard",
		"ghost-button": "text-muted-foreground hover:(bg-secondary text-foreground)",
	},
});
