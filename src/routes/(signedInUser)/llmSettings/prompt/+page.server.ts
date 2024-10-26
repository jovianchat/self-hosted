import type { Actions } from "./$types";

export const actions: Actions = {
    savePromptConfig: async ({ request, fetch }) => {
        const formData = await request.formData()
        let promptConfig: PromptConfig = {
            name: formData.get('name') as string,
            max_tokens: Number(formData.get('max-tokens')),
            temperature: Number(formData.get('temperature')),
            system_prompt: formData.get('system-prompt') as string
        }

        const id = formData.get('id')
        let res_method;
        if (!id) {
            // Create new prompt
            const promptConfigsLength = formData.get('promptConfigsLength')
            promptConfig.id = Number(promptConfigsLength)
            res_method = 'POST'
        } else {
            // Update existing prompt
            promptConfig.id = Number(id)
            res_method = 'PATCH'
        }
        const res = await fetch('/http-server/llm-settings/prompt-engineering', {
            method: res_method,
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(promptConfig)
        })
        if (res.ok) {
            return { success: true }
        } else {
            throw new Error(await res.text())
        }
    },
    deletePromptConfig: async ({ request, fetch }) => {
        const formData = await request.formData()
        const id = formData.get('id') as string
        const res = await fetch('/http-server/llm-settings/prompt-engineering', {
            method: 'DELETE',
            headers: {
                'Content-Type': 'text/plain'
            },
            body: id
        })
        if (res.ok) {
            return { success: true }
        } else {
            throw new Error(await res.text())
        }
    },
    saveSelectedApiPrompt: async ({ request, fetch }) => {
        const formData = await request.formData()
        const selectedApiPrompt: SelectedApiPrompt = {
            api_id: Number(formData.get('selectedApiId')),
            model: formData.get('selectedModel') as string,
            prompt_id: Number(formData.get('selectedPromptId'))
        }

        const res = await fetch('/http-server/llm-settings/selected-api-prompt', {
            method: 'PUT',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(selectedApiPrompt)
        })
        if (res.ok) {
            return { success: true }
        } else {
            throw new Error(await res.text())
        }
    }
}