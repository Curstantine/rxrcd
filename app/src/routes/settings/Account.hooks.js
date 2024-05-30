import { onMount } from "svelte";
import { get, readonly, writable } from "svelte/store";

import { get_auth_state, login } from "@/bindings/user";
import { set_user_data } from "@/stores/user";

import { create_snack } from "@/components/snack/snack";

export function initialize_state() {
	/** @type {import("svelte/store").Writable<import("@/types/user").UserAuthState | null>} */
	const auth_state = writable(null);

	const input_email = writable("");
	const input_password = writable("");
	const input_arl = writable("");

	const on_login_submit = async () => {
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
			} catch (e) {
				snack.update({ label: "Failed to authenticate with the ARL", description: e?.toString() });
			}
		}

		return snack.close();
	};

	onMount(async () => {
		try {
			const data = await get_auth_state();
			auth_state.set(data);
		} catch (e) {
			create_snack({
				label: "Failed to retrieve authentication state",
				description: e?.toString(),
			});
		}
	});

	return {
		input_email,
		input_password,
		input_arl,
		auth_state: readonly(auth_state),
		on_login_submit,
	};
}
