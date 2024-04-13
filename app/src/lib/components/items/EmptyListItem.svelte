<script>
	const titles = ["^_____^", "（；´д｀）ゞ", "(；′⌒`)"];
	const subs = ["It's dark in here...", "Nothing in here...", "Emptiness..."];

	const selected_title = titles[Math.floor(Math.random() * titles.length)];
	const selected_sub = subs[Math.floor(Math.random() * titles.length)];
</script>

<div class="flex flex-col items-center py-4 space-y-2">
	<span class="text-xl text-foreground font-medium">{selected_title}</span>
	<span class="text-sm text-muted-foreground">{selected_sub}</span>
</div>
