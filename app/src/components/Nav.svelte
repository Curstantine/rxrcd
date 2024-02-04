<script>
	import NavItem from "@/components/NavItem.svelte";
</script>

<nav class="max-h-full w-10 flex flex-col border-r-(1 border solid)">
	<NavItem href="/" icon="i-symbols-home-rounded" />
	<NavItem href="/advance_search" icon="i-symbols-search-rounded" />

	<div class="flex-1" />

	<NavItem href="/settings" icon="i-symbols-settings-rounded" />
</nav>
