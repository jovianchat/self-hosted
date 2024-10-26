<script lang="ts">
  import { scale } from 'svelte/transition'
  import { page } from '$app/stores'
  import MdiArrowTopThick from '~icons/mdi/arrow-top-thick'
  import FontistoPaperclip from '~icons/fontisto/paperclip'
  import IcBaselineStopCircle from '~icons/ic/baseline-stop-circle'
  import FluentSettingsChat16Filled from '~icons/fluent/settings-chat-16-filled'

  let { llmSavedSettings }: { llmSavedSettings: LLMSavedSettings } = $props()

  import { submitQuery, textArea } from './textArea.svelte'
  import { esClose_SaveDb } from './[chatId]/chatResponse'
  import { chatState } from './[chatId]/chat.svelte'

  let chatId = $derived($page.url.pathname.split('/').pop())
  let isSendButtonDisabled = $derived(textArea.value.trim() === '')

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Enter' && !event.shiftKey) {
      event.preventDefault()
      if (textArea.value.trim() === '') {
        return // If the textarea is empty, don't submit
      }
      submitQuery(chatId)
    }
  }
  function handleSubmit(event: any) {
    event.preventDefault()
    submitQuery(chatId)
  }

  function handleStopGeneratingResponse(event: any) {
    event.preventDefault()
    esClose_SaveDb(chatId)
  }

  let selectedApiPrompt = $derived({
    api: llmSavedSettings.llmApiModels.find(
      (api) => api.id === llmSavedSettings.selectedApiPrompt.api_id,
    ),
    model: llmSavedSettings.selectedApiPrompt.model,
    prompt: llmSavedSettings.promptConfigs.find(
      (prompt) => prompt.id === llmSavedSettings.selectedApiPrompt.prompt_id,
    ),
  })
</script>

<div class="flex flex-col">
  <div class="textarea textarea-bordered textarea-md rounded-badge bg-base-200">
    <div class="flex flex-col">
      <div class="text-base w-full flex gap-2 items-end">
        <label for="upload_file_icon" class="btn btn-sm btn-circle btn-ghost text-accent">
          <FontistoPaperclip class="w-6 h-6" />
        </label>
        <input type="file" id="upload_file_icon" class="hidden" />
        <textarea
          name="query"
          onkeydown="{handleKeydown}"
          bind:value="{textArea.value}"
          style="min-height: 1lh; max-height: 8lh; field-sizing:content"
          class="w-full resize-none overflow-auto outline-none bg-transparent mb-[3px]"
          placeholder="Enter your query!">
        </textarea>
        {#if !isSendButtonDisabled}
          <button
            onclick="{handleSubmit}"
            class="btn btn-sm btn-circle btn-accent"
            transition:scale="{{ delay: 10, duration: 400 }}"
            ><MdiArrowTopThick class="w-7 h-7" /></button>
        {:else if chatState.isResponseGenerating}
          <button
            onclick="{handleStopGeneratingResponse}"
            class="btn btn-sm btn-circle"
            transition:scale="{{ delay: 10, duration: 400 }}"
            ><IcBaselineStopCircle class="w-7 h-7" /></button>
        {/if}
      </div>
    </div>
  </div>
  <div
    class="pointer-events-none w-5/6 mx-auto rounded-md flex items-center justify-between px-2 bg-base-300 text-gray-500 text-xs">
    <!-- Show Api Name and Model by getting from Id -->
    <div class="flex gap-2 items-center max-w-[47%] truncate">
      <span>API: {selectedApiPrompt.api?.name}</span>
      <span>Model: {selectedApiPrompt.model}</span>
    </div>

    <div class="mx-1">
      <FluentSettingsChat16Filled class="w-[14px] h-[14px] text-accent text-opacity-70" />
    </div>
    <div class="flex gap-2 items-center max-w-[47%] truncate">
      <span>Max Tokens: {selectedApiPrompt.prompt?.max_tokens}</span>
      <span>Temp: {selectedApiPrompt.prompt?.temperature}</span>
    </div>
  </div>
</div>
