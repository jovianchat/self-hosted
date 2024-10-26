<script lang="ts">
    import ChatTopbar from './Topbar.svelte'
    import TextArea from './TextArea.svelte'
    const { data, children } = $props();
    let { llmSavedSettings } = $derived(data)
</script>

<div class="px-6 mx-auto flex flex-col justify-between min-h-screen">
    <div class="sticky top-0 bg-base-100">
        <div class="syncSidebarTopbar_Alignment flex flex-nowrap justify-between items-center relative">
        <ChatTopbar {llmSavedSettings} />
        </div>
    </div>
        <div class="flex-1 content_chattextarea_width flex flex-col justify-between">
            <div class = "flex-1 flex flex-col">
                {@render children()}
            </div>
            <div class="sticky bottom-0 bg-base-100">
                <div class="mt-4">
                    <TextArea {llmSavedSettings} />
                </div>
            </div>
        </div>
</div>

<style lang="postcss">
    .content_chattextarea_width {
        @apply mx-auto w-7/12 max-w-screen-lg
    }
</style>