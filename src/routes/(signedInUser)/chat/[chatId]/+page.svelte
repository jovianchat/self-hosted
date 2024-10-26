<script lang="ts">
  import { afterNavigate, beforeNavigate } from '$app/navigation'
  import { marked } from 'marked'
  import { chatState } from './chat.svelte'
  import { onMount, tick } from 'svelte'
  import { esClose_SaveDb } from './chatResponse'

  const { data } = $props()
  const { chat } = $derived(data)

  beforeNavigate(({ from, to, cancel }) => {
    if (from?.url.pathname === to?.url.pathname) {
      cancel()
    } else {
      const chatId = from?.url.pathname.split('/').pop()
      esClose_SaveDb(chatId)
      chatState.emptyQR()
    }
  })
  afterNavigate(() => scrollToBottom({ instant: true }))

  const scrollToBottom = ({ instant = false } = {}) => {
    tick().then(() => {
      window.scrollTo({ top: document.body.scrollHeight, behavior: instant ? 'instant' : 'smooth' })
    })
  }
  $effect.pre(() => {
    if (chatState.qr.length > 0) chatState.qr[chatState.qr.length - 1].assistant_response

    // const scrollUp = window.scrollY + window.innerHeight >= document.body.scrollHeight - 1;
    if (chatState.qr.length > 0) {
      scrollToBottom()
    }
  })
</script>

<div class="flex flex-col gap-6">
  {#each chat.messages as { user_query, assistant_response }}
    <div class="query_bg custom_border prose prose-sm w-fit ml-auto max-w-[92%] overflow-auto">
      <div class="px-4 py-2 shadow-sm prose-cyan whitespace-pre">
        {user_query}
      </div>
    </div>
    <div class="prose min-w-full">
      <div class="bg-base-300 px-4 py-2 rounded-lg shadow-md prose-cyan">
        {@html marked.parse(assistant_response)}
      </div>
    </div>
  {/each}
  {#each chatState.qr as { user_query, assistant_response }}
    <div class="query_bg custom_border prose prose-sm w-fit ml-auto max-w-[92%] overflow-auto">
      <div class="px-4 py-2 shadow-sm prose-cyan whitespace-pre">
        {user_query}
      </div>
    </div>
    <div class="prose min-w-full">
      <div class="bg-base-300 px-4 py-2 rounded-lg shadow-md prose-cyan">
        {@html marked.parse(assistant_response)}
      </div>
    </div>
  {/each}
</div>

<style lang="postcss">
  .custom_border {
    border-width: 1px;
    border-color: var(--fallback-bc, oklch(var(--bc) / 0.2));
  }
  .query_bg {
    background-color: #30302d;
    @apply rounded-badge;
  }
</style>
