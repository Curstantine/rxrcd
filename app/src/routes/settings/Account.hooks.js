export function initialize_state() {
	/**
	 * @param {SubmitEvent & { currentTarget: EventTarget & HTMLFormElement; }} e
	 */
	const on_login_submit = (e) => {
		const data = new FormData(e.currentTarget);
		console.log(data.entries().next());
	};

	return {
		on_login_submit,
	};
}
