const { defineConfig } = require("eslint-define-config");

module.exports = defineConfig({
	extends: [
		"../.eslintrc.cjs",
		"plugin:svelte/recommended",
		"@unocss",
	],
	rules: {
		"no-unused-vars": ["error", { argsIgnorePattern: "^_" }],
	},
});
