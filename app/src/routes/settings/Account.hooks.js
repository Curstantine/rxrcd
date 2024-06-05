import { onMount } from "svelte";
import { get, readonly, writable } from "svelte/store";

import { config_get_account, config_set_account } from "@/bindings/config";
import { change_data_language as invoke_change_data_language, get_auth_state, login } from "@/bindings/user";
import { set_user_data } from "@/stores/user";
import { logout } from "@/utils/auth";

import { create_snack, DEFAULT_SNACK_TIMEOUT } from "@/components/snack/snack";

/**
 * @type {Record<import("@/types/config").DataLanguage, string>}
 * @constant
 */
export const language_labels = {
	Default: "Default",
	English: "English",
	Japanese: "Japanese",
	BrazilianPortuguese: "Brazilian Portuguese",
};

/** @type {import("@/types/select").Action<import("@/types/config").DataLanguage>[]} */
export const data_language_actions = [
	{ value: "Default", label: language_labels["Default"], sub: "(Deezer Default)" },
	{ value: "English", label: language_labels["English"] },
	{ value: "Japanese", label: language_labels["Japanese"] },
	{ value: "BrazilianPortuguese", label: language_labels["BrazilianPortuguese"] },
];

export function initialize_state() {
	/** @type {import("svelte/store").Writable<import("@/types/config").ConfigurationAccount | null>} */
	const settings = writable(null);

	/** @type {import("svelte/store").Writable<import("@/types/user").UserAuthState | null>} */
	const auth_state = writable(null);

	const input_email = writable("");
	const input_password = writable("");
	const input_arl = writable("");

	onMount(async () => {
		const [conf] = await Promise.all([config_get_account(), initialize_auth_state()]);
		settings.set(conf);
	});

	async function initialize_auth_state() {
		try {
			const data = await get_auth_state();
			auth_state.set(data);
		} catch (e) {
			create_snack({
				label: "Failed to retrieve authentication state",
				description: e?.toString(),
			});
		}
	}

	async function on_login_submit() {
		const email = get(input_email);
		const password = get(input_password);
		const arl = get(input_arl);

		const snack = create_snack({
			label: "Authenticating...",
			description: "Signing you into deezer",
			persistent: true,
		});

		if (email.length > 0 && password.length > 0) {
			snack.update({
				label: "Authentication failed",
				description: "Credential authentication is not supported yet!",
			});
		}

		if (arl.length > 0) {
			try {
				const user = await login({ type: "Arl", arl: arl.toString() });
				set_user_data(user);
				await initialize_auth_state();
			} catch (e) {
				snack.update({ label: "Failed to authenticate with the ARL", description: e?.toString() });
			}
		}

		return setTimeout(() => snack.close(), get(auth_state)?.type === "NotLoggedIn" ? DEFAULT_SNACK_TIMEOUT : 0);
	}

	async function on_logout() {
		await logout();
		await initialize_auth_state();
	}

	/**
	 * @param {import("@/types/config").DataLanguage} language
	 */
	async function change_data_language(language) {
		const conf = get(settings);

		if (conf === null) {
			return create_snack({
				label: "Configuration state is missing!",
				description: "Please wait while configuration is being loaded",
			});
		}

		conf.data_language = language;
		await config_set_account(conf);
		await invoke_change_data_language(language);

		settings.set(conf);
	}

	return {
		auth_state: readonly(auth_state),
		settings: readonly(settings),
		input_email,
		input_password,
		input_arl,
		on_login_submit,
		on_logout,
		change_data_language,
	};
}
