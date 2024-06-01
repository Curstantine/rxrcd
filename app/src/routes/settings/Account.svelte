<script>
	import { slide } from "svelte/transition";

	import { user_data } from "@/stores/user";

	import SettingsHeading from "@/components/SettingsHeading.svelte";
	import SettingsOptionArea from "@/components/SettingsOptionArea.svelte";
	import CopyWrapper from "@/components/InputCopyable.svelte";

	import { initialize_state } from "@/routes/settings/Account.hooks";

	const { auth_state, input_email, input_password, input_arl, on_login_submit, on_logout } = initialize_state();

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
				<button class="button-primary" on:click={on_logout}>Logout</button>
			</div>
		</div>
	{/if}

	<form name="login" class="max-w-lg" on:submit|preventDefault={on_login_submit}>
		{#if $auth_state === null || $auth_state.type === "NotLoggedIn" || $auth_state.data?.type === "Credentials"}
			<SettingsOptionArea
				layout="col"
				label="Login with credentials"
				subtitle="Login with your email and password"
				form_id="login_email"
				option_class="flex flex-col"
			>
				{#if $auth_state !== null && $auth_state.type === "LoggedIn" && $auth_state.data?.type === "Credentials"}
					<CopyWrapper grammar="email" value={$auth_state.data.email}>
						<input
							id="login_email"
							type="email"
							readonly
							class="w-full input pr-10"
							bind:value={$auth_state.data.email}
						/>
					</CopyWrapper>
					<CopyWrapper grammar="password" value={$auth_state.data.password}>
						<input
							id="login_password"
							type="password"
							readonly
							class="w-full input pr-10"
							bind:value={$auth_state.data.password}
						/>
					</CopyWrapper>
				{:else}
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
						type="password"
						class="w-full input"
						placeholder="Password"
						disabled={is_login_cred_disabled}
						bind:value={$input_password}
					/>
				{/if}
			</SettingsOptionArea>
		{/if}

		{#if $auth_state === null || $auth_state.type === "NotLoggedIn"}
			<div class="h-6 flex items-center gap-2 -mb-2">
				<div class="h-[1px] flex-1 bg-border"></div>
				<span class="text-sm">or</span>
				<div class="h-[1px] flex-1 bg-border"></div>
			</div>
		{/if}

		{#if $auth_state === null || $auth_state.type === "NotLoggedIn" || $auth_state.data?.type === "Arl"}
			<SettingsOptionArea
				layout="col"
				label="Login with ARL"
				subtitle="You can also login from the ARL token available in your session cookies"
				form_id="login_arl"
			>
				{#if $auth_state !== null && $auth_state.type === "LoggedIn" && $auth_state.data?.type === "Arl"}
					<CopyWrapper grammar="ARL" value={$auth_state.data.arl}>
						<input
							id="login_arl"
							type="text"
							readonly
							class="w-full input pr-10"
							bind:value={$auth_state.data.arl}
						/>
					</CopyWrapper>
				{:else}
					<input
						id="login_arl"
						type="text"
						class="w-full input"
						placeholder="ARL"
						disabled={is_login_arl_disabled}
						bind:value={$input_arl}
					/>
				{/if}
			</SettingsOptionArea>
		{/if}

		{#if $auth_state === null || $auth_state.type === "NotLoggedIn"}
			<div class="mt-4 max-w-lg flex justify-end">
				<button
					type="submit"
					class="w-full button-primary"
					disabled={!(is_login_cred_disabled || is_login_arl_disabled)}
				>
					Continue
				</button>
			</div>
		{/if}
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
