<script>
	import { pushToSnackStack } from "@/components/snack/snack";
</script>

<div class="flex flex-1 flex-col gap-2 overflow-y-auto px-2 px-4 py-2">
	<span>hai home :3</span>

	<div class="inline-flex gap-4">
		<button
			class="w-fit button-primary"
			on:click={() => {
				pushToSnackStack({ label: "Hai :3", description: "Hello from home snack button summoner! :D" });
			}}
		>
			Snack message
		</button>

		<button
			class="w-fit button-primary"
			on:click={() => {
				const instance = pushToSnackStack({
					persistent: true,
					label: "Progress",
					description: "This is progression snack instance message!",
				});

				for (let i = 0; i <= 20; i++) {
					setTimeout(() => instance.update({ label: `Progress (${i}/20)` }), i * 2000);
				}

				setTimeout(() => instance.close(), 40000);
			}}
		>
			Snack progression message
		</button>
	</div>
</div>
