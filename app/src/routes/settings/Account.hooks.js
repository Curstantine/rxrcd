import { login_with_arl } from "@/bindings/user";
import { create_snack } from "@/components/snack/snack";
import { set_user_data } from "@/stores/user";

export function initialize_state() {
	/**
	 * @param {SubmitEvent & { currentTarget: EventTarget & HTMLFormElement; }} e
	 */
	const on_login_submit = async (e) => {
		const data = new FormData(e.currentTarget);
		// const _email = data.get("email");
		// const _password = data.get("password");
		const arl = data.get("arl");

		// if (email !== null && password !== null) {
		// }

		if (arl !== null) {
			try {
				const user = await login_with_arl(arl.toString());
				set_user_data(user);
			} catch (e) {
				create_snack({ label: "Failed to authenticate with the ARL", description: e?.toString() });
			}
		}
	};

	return {
		on_login_submit,
	};
}
