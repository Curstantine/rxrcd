<script>
	export let small = false;
</script>

<div class="{small ? 'w-36' : 'w-42'} flex flex-col animate-pulse transition-colors use-transition-standard">
	<div class="aspect-square inline-flex items-center justify-center border-(1 border solid) rounded" />
	<div class="mt-2 h-3 w-32 rounded bg-secondary" />
	<div class="mt-1 h-2 w-22 rounded bg-secondary" />
</div>
