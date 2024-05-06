<script>
	import SettingsHeading from "@/components/SettingsHeading.svelte";
	import SettingsOptionArea from "@/components/SettingsOptionArea.svelte";

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

	<div class="auth-card pointer-events-none opacity-50">
		<div class="grid-area-[image] h-24 w-24 self-center justify-self-center rounded-full bg-secondary"></div>

		<div class="grid-area-[description] flex flex-col justify-center">
			<span class="o">Not logged in</span>
			<span class="text-sm text-muted-foreground">N/A</span>
		</div>

		<div class="grid-area-[actions]">
			<button class="button-primary">Logout</button>
		</div>
	</div>

	<form name="login" class="max-w-lg">
		<SettingsOptionArea
			layout="col"
			label="Login with credentials"
			subtitle="Login with your username and password"
			form_id="login_username"
			option_class="flex flex-col"
		>
			<input
				id="login_username"
				type="text"
				class="mb-2 w-full input"
				placeholder="Username"
				disabled={disabledLoginCred}
				bind:value={loginUsername}
			/>
			<input
				id="login_password"
				type="text"
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
				class="w-full input"
				placeholder="ARL"
				disabled={disabledLoginARL}
				bind:value={loginARL}
			/>
		</SettingsOptionArea>

		<div class="mt-4 max-w-lg flex justify-end">
			<button class="w-full button-primary" disabled={!(disabledLoginCred || disabledLoginARL)}>Continue</button>
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
</style>
