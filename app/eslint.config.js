import js from "@eslint/js";
import unocss from "@unocss/eslint-config/flat";
import svelte from "eslint-plugin-svelte";
import globals from "globals";

/** @type {import('eslint').Linter.FlatConfig[]} */
const config = [
	js.configs.recommended,
	unocss,
	...svelte.configs["flat/recommended"],
	{ languageOptions: { globals: { ...globals.browser, ...globals.node } } },
	{
		ignores: ["build/", ".svelte-kit/", "package/"],
	},
];

export default config;
