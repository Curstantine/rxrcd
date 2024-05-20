<script>
	import { user_data } from "@/stores/user";

	import SettingsHeading from "@/components/SettingsHeading.svelte";
	import SettingsOptionArea from "@/components/SettingsOptionArea.svelte";

	import { initialize_state } from "@/routes/settings/Account.hooks";
	import { slide } from "svelte/transition";
	import { logout } from "@/utils/auth";

	const { input_email, input_password, input_arl, on_login_submit } = initialize_state();

	$: is_login_cred_disabled = $input_arl.length > 0;
	$: is_login_arl_disabled = $input_email?.length > 0 || $input_password?.length > 0;
</script>

<article id="account">
	<SettingsHeading heading="Account" sub="Manage your authentication sessions and regions." class="mb-3" />

	{#if $user_data !== null}
		<div in:slide={{ axis: "y", duration: 300 }} class="auth-card">
			<div class="grid-area-[image] h-24 w-24 self-center justify-self-center rounded-full bg-secondary"></div>

			<div class="grid-area-[description] flex flex-col justify-center">
				<span class="leading-tight">{$user_data.name}</span>
				<div class="inline text-sm text-muted-foreground">
					<span>{$user_data.offer_name}</span> ·
					<span>{$user_data.country}</span>
				</div>
			</div>

			<div class="grid-area-[actions]">
				<button class="button-primary" on:click={logout}>Logout</button>
			</div>
		</div>
	{/if}

	<!-- TODO(Curstantine): Restore whatever auth we used and hide unrelated methods in an authenticated context. -->
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
				class="mb-2 w-full input"
				placeholder="Email"
				disabled={is_login_cred_disabled}
				bind:value={$input_email}
			/>
			<input
				id="login_password"
				type="text"
				class="w-full input"
				placeholder="Password"
				disabled={is_login_cred_disabled}
				bind:value={$input_password}
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
				disabled={is_login_arl_disabled}
				bind:value={$input_arl}
			/>
		</SettingsOptionArea>

		<div class="mt-4 max-w-lg flex justify-end">
			<button
				type="submit"
				class="w-full button-primary"
				disabled={!(is_login_cred_disabled || is_login_arl_disabled)}
			>
				Continue
			</button>
		</div>
	</form>
</article>

<style>
	.auth-card {
		--at-apply: grid border-border border-1 rounded-md p-4 gap-x-4 mb-3 max-w-lg;
		grid-template-columns: auto 1fr 1fr;
		grid-template-areas:
			"image description description"
			"image description description"
			"image actions actions";
	}
</style>
