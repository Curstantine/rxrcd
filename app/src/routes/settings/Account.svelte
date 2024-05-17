<script>
	import { user_data } from "@/stores/user";

	import SettingsHeading from "@/components/SettingsHeading.svelte";
	import SettingsOptionArea from "@/components/SettingsOptionArea.svelte";

	import { initialize_state } from "@/routes/settings/Account.hooks";

	const { on_login_submit } = initialize_state();

	/** @type {string} */
	let loginUsername;

	/** @type {string} */
	let loginPassword;

	/** @type {string} */
	let loginARL;

	$: disabledLoginCred = loginARL?.length > 0;
	$: disabledLoginARL = loginUsername?.length > 0 || loginPassword?.length > 0;
</script>

<article id="account">
	<SettingsHeading heading="Account" sub="Manage your authentication sessions and regions." />

	<div class="auth-card" class:disabled={$user_data === null}>
		<div class="grid-area-[image] h-24 w-24 self-center justify-self-center rounded-full bg-secondary"></div>

		<div class="grid-area-[description] flex flex-col justify-center">
			<span class="o">{$user_data?.name ?? "Not logged in"}</span>
			<span class="text-sm text-muted-foreground">{$user_data?.is_premium ? "Hi-Fi" : "Free"}</span>
		</div>

		<div class="grid-area-[actions]">
			<button class="button-primary">Logout</button>
		</div>
	</div>

	<form name="login" class="max-w-lg" on:submit|preventDefault={on_login_submit}>
		<SettingsOptionArea
			layout="col"
			label="Login with credentials"
			subtitle="Login with your email and password"
			form_id="login_email"
			option_class="flex flex-col"
		>
			<input
				id="login_email"
				type="email"
				name="email"
				class="mb-2 w-full input"
				placeholder="Email"
				disabled={disabledLoginCred}
				bind:value={loginUsername}
			/>
			<input
				id="login_password"
				type="text"
				name="password"
				class="w-full input"
				placeholder="Password"
				disabled={disabledLoginCred}
				bind:value={loginPassword}
			/>
		</SettingsOptionArea>

		<div class="h-6 flex items-center gap-2 -mb-2">
			<div class="h-[1px] flex-1 bg-border"></div>
			<span class="text-sm">or</span>
			<div class="h-[1px] flex-1 bg-border"></div>
		</div>

		<SettingsOptionArea
			layout="col"
			label="Login with ARL"
			subtitle="You can also login from the ARL token available in your session cookies"
			form_id="login_arl"
		>
			<input
				id="login_arl"
				type="text"
				name="arl"
				class="w-full input"
				placeholder="ARL"
				disabled={disabledLoginARL}
				bind:value={loginARL}
			/>
		</SettingsOptionArea>

		<div class="mt-4 max-w-lg flex justify-end">
			<button type="submit" class="w-full button-primary" disabled={!(disabledLoginCred || disabledLoginARL)}>
				Continue
			</button>
		</div>
	</form>
</article>

<style>
	.auth-card {
		--at-apply: grid border-border border-1 rounded-md p-4 gap-x-4 my-3 max-w-lg;
		grid-template-columns: auto 1fr 1fr;
		grid-template-areas:
			"image description description"
			"image description description"
			"image actions actions";
	}

	.auth-card.disabled {
		--at-apply: pointer-events-none opacity-50;
	}
</style>
