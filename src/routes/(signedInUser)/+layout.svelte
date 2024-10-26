<script lang="ts">
    import Sidebar from '$/components/layout/Sidebar.svelte'
    import { sidebar } from '$/components/layout/sidebar.svelte.js'
    import type { ChatHistory } from '$lib/types/chat.js'
    import { slide } from 'svelte/transition'
	import { page } from '$app/stores'

    let { data, children } = $props();
    let { chats }: { chats: ChatHistory } = $derived(data);

	let isChatPage = $derived($page.url.pathname.startsWith('/chat'))
</script>

<main>
	<div class="flex">
		{#if sidebar.isOpen || !isChatPage}
		<div class="page_height w-[20%] max-w-72 bg-base-300 sticky top-0" transition:slide="{{ delay: 20, duration: 800, axis: 'x' }}">
			<Sidebar {chats} />
		</div>
		{/if}
		<div class="flex-1"> <!-- Flex 1 means full width left from flex -->
        	{@render children()}
		</div>
	</div>
</main>

<style lang="postcss">
	:global(.syncSidebarTopbar_Alignment) {
		@apply h-12 my-1
	}
	:global(.page_height) {
		@apply h-screen
	}
</style>