<script lang="ts">
    import { page } from "$app/stores";
    let { data, children } = $props();
    const links = [
        { name: 'Change Password', href: '/profile/password' },
    ]
</script>

<div class="flex flex-col min-h-screen">
    <div class="flex-1 w-5/6 mx-auto my-32 flex flex-row">
        <ul class="w-3/12 flex flex-col gap-2 my-10">
            {#each links as link}
                <li class="text-center truncate">
                    <a href={link.href}
                        class="btn text-lg btn-ghost w-full justify-center"
                        class:active="{link.href === $page.url.pathname}">{link.name}</a>
                </li>
            {/each}
        </ul>
        <div class="flex-1">
            {@render children()}
        </div>
    </div>
</div>

<style lang="postcss">
    .active {
        @apply bg-base-300 bg-opacity-65 text-accent;
    }
</style>