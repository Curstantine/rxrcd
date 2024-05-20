import { login_with_arl } from "@/bindings/user";
import { create_snack } from "@/components/snack/snack";
import { set_user_data } from "@/stores/user";
import { get, writable } from "svelte/store";

export function initialize_state() {
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
				const user = await login_with_arl(arl.toString());
				console.log(user);
				set_user_data(user);
			} catch (e) {
				snack.update({ label: "Failed to authenticate with the ARL", description: e?.toString() });
			}
		}

		return snack.close();
	};

	return {
		input_email,
		input_password,
		input_arl,
		on_login_submit,
	};
}
