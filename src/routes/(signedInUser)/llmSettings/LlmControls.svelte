<!-- LLM Controls Component -->
<script lang="ts">
  import { enhance } from '$app/forms'
  import { onMount } from 'svelte'
  import MdiClose from '~icons/mdi/close'
  let {
    llmSavedSettings,
    llmControlsModal = $bindable(),
  }: { llmSavedSettings: LLMSavedSettings; llmControlsModal: HTMLDialogElement } = $props()
  let selectedApi: LLMApiConfig | undefined = $state()
  let selectedModel: string | undefined = $state()
  let selectedPromptConfig: PromptConfig | undefined = $state()
  onMount(() => {
    selectedApi = llmSavedSettings.llmApiModels.find(
      (api) => api.id === llmSavedSettings.selectedApiPrompt.api_id,
    )
    selectedPromptConfig = llmSavedSettings.promptConfigs.find(
      (prompt) => prompt.id === llmSavedSettings.selectedApiPrompt.prompt_id,
    )
  })
  $effect(() => {
    selectedModel = selectedApi?.models[0]
  })
</script>

<dialog bind:this="{llmControlsModal}" class="modal">
  <div class="modal-box">
    <div class="flex justify-between items-center">
      <h3 class="font-bold text-lg">LLM Controls</h3>
      <form method="dialog">
        <button class="btn btn-square btn-sm btn-ghost">
          <MdiClose class="w-6 h-6" />
        </button>
      </form>
    </div>
    <div class="divider my-0"></div>
    <form
      method="POST"
      action="/llmSettings/prompt?/saveSelectedApiPrompt"
      use:enhance
      onsubmit="{() => llmControlsModal.close()}">
      <div class="flex flex-col gap-4">
        <!-- API Selector -->
        <label class="form-control w-full max-w-xs">
          <div class="label">
            <span class="label-text">Select API Model:</span>
          </div>
          <select bind:value="{selectedApi}" class="select select-bordered">
            {#each llmSavedSettings.llmApiModels as api}
              <option value="{api}">{api.name}</option>
            {/each}
          </select>
          <input type="hidden" name="selectedApiId" value="{selectedApi?.id}" />
        </label>
        <!-- Model Selector for corresponding API -->
        <label class="form-control w-full max-w-xs">
          <div class="label">
            <span class="label-text">Select Model to use available for Selected API:</span>
          </div>
          <select bind:value="{selectedModel}" class="select select-bordered">
            {#each selectedApi?.models ?? [] as model}
              <option value="{model}">{model}</option>
            {/each}
          </select>
          <input type="hidden" name="selectedModel" value="{selectedModel}" />
        </label>

        <!-- Prompt Selector -->
        <label class="form-control w-full max-w-xs">
          <div class="label">
            <span class="label-text">Select Prompt Engineered Config:</span>
          </div>
          <select bind:value="{selectedPromptConfig}" class="select select-bordered">
            {#each llmSavedSettings.promptConfigs as config}
              <option value="{config}">{config.name}</option>
            {/each}
          </select>
          <input type="hidden" name="selectedPromptId" value="{selectedPromptConfig?.id}" />
          <div class="label">
            <div class="label-text">
              {#if selectedPromptConfig}
                <p class="text-sm text-gray-500">
                  Max Tokens: {selectedPromptConfig.max_tokens}
                </p>
                <p class="text-sm text-gray-500">
                  Temperature: {selectedPromptConfig.temperature}
                </p>
                <div class="dropdown dropdown-hover dropdown-right dropdown-end">
                  <div tabindex="0" role="button" class="link link-hover text-gray-400">
                    Show System Prompt
                  </div>
                  <div
                    class="ml-1 card compact dropdown-content bg-base-200 rounded-box z-[1] w-64 shadow">
                    <div class="card-body">
                      <p>{selectedPromptConfig.system_prompt}</p>
                    </div>
                  </div>
                </div>
              {/if}
            </div>
          </div>
        </label>
      </div>
      <div class="modal-action items-center gap-4">
        <p>
          Go to <a href="/llmSettings/prompt" class="text-accent link-hover"
            >API & Prompt Settings</a>
          page to edit these options or add new!
        </p>
        <button class="btn_emerald">Save</button>
      </div>
    </form>
  </div>
</dialog>

<style lang="postcss">
  .select {
    @apply bg-base-300;
  }
</style>
