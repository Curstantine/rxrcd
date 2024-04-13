import js from "@eslint/js";
import globals from "globals";

/** @type {import('eslint').Linter.FlatConfig[]} */
export default [
	js.configs.recommended,
	{ languageOptions: { globals: { ...globals.browser, ...globals.node } } },
	{ ignores: ["**/dist", "**/node_modules", "**/*.d.ts"] },
];
