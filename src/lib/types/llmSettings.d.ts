type PromptConfig = {
    id?: number
    name: string
    max_tokens: number
    temperature: number
    system_prompt: string
}

type LlmSdk = "OpenAI" | "Anthropic";
type LLMApiConfig = {
    id?: number
    name: string
    sdk: LlmSdk
    api_key?: string
    base_url: string
    models: string[]
}

type SelectedApiPrompt = {
    api_id: number
    model: string
    prompt_id: number
}

type LLMSavedSettings = {
    llmApiModels: LLMApiConfig[]
    promptConfigs: PromptConfig[]
    selectedApiPrompt: SelectedApiPrompt
}