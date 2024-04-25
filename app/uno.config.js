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
				"Inter, \"Segoe UI\", Roboto, Helvetica, Arial, system-ui, sans-serif, \"Apple Color Emoji\", \"Segoe UI Emoji\", \"Segoe UI Symbol\"",
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

		"button-layout": [
			"inline-flex justify-center items-center gap-2 rounded-md px-4 py-1 font-medium text-sm",
			"use-transition-standard transition-colors disabled:(opacity-50 pointer-events-none)",
		].join(" "),
		"icon-button-layout": [
			"inline-flex justify-center items-center aspect-square rounded-md font-medium text-sm",
			"use-transition-standard transition-colors disabled:(opacity-50 pointer-events-none)",
		].join(" "),

		"button-variant-ghost": "text-muted-foreground hover:(bg-secondary text-foreground)",

		"button-primary": "button-layout shadow bg-primary text-primary-foreground hover:bg-primary/90",
		"button-ghost": "button-layout  button-variant-ghost",

		"icon-button": "icon-button-layout border-(1 solid border) hover:(bg-secondary)",
		"icon-button-ghost": "icon-button-layout button-variant-ghost",

		input: [
			"bg-transparent h-9 inline-flex items-center border-(1 border solid) rounded px-3 text-sm",
			"transition-colors use-transition-standard",
			"active:outline-0 focus:(outline-0 border-ring) placeholder:text-sm focus:placeholder:opacity-0",
		].join(" "),

		skeletal: "animate-pulse rounded bg-secondary use-transition-emphasized",
	},
});
