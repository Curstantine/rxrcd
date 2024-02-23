const { defineConfig } = require("eslint-define-config");

module.exports = defineConfig({
	root: true,
	env: {
		browser: true,
		es2021: true,
		node: true,
	},
	extends: [
		"eslint:recommended",
	],
	ignorePatterns: ["**/dist", "**/node_modules", "**/*.d.ts"],
	overrides: [
		{
			env: {
				node: true,
			},
			files: ["**/*.eslintrc.cjs"],
			parserOptions: {
				sourceType: "script",
			},
		},
	],
	parserOptions: {
		ecmaVersion: "latest",
		sourceType: "module",
	},
});
