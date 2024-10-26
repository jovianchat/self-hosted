import { redirect } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";
import { sidebar } from "$/components/layout/sidebar.svelte";
import type { ChatHistory } from "$lib/types/chat";

export const load: LayoutServerLoad = async ({ cookies, fetch }) => {
	// Log out if token expired
	const refresh_token = cookies.get("refresh_token");
	const expiration = cookies.get("refresh_token_expiration");
	const currentTime = Math.ceil(Date.now() / 1000);
	if (!refresh_token || !expiration || Number(expiration) < currentTime) {
		return redirect(302, "/auth");
	}

	let chats: ChatHistory;
	let llmSavedSettings: LLMSavedSettings = {
		llmApiModels: [],
		promptConfigs: [],
		selectedApiPrompt: { api_id: 0, model: "", prompt_id: 0 }
	};
	// if (sidebar.chatHistory.length === 0) {
	const res = await fetch(`/http-server/chat/history`)
	if (res.ok) {
		chats = await res.json()
	} else {
		throw new Error(await res.text())
	}
	// }
	/* Load LLM Api & Models and Prompt Configs for selector and to be sent with each user_query as to which model to use and what prompt settings */
	const res_llmApiModels = await fetch('/http-server/llm-settings/llm-api-config', {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json'
		}
	})
	const res_promptConfigs = await fetch('/http-server/llm-settings/prompt-engineering', {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json'
		}
	})
	const res_selectedApiPrompt = await fetch('/http-server/llm-settings/selected-api-prompt', {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json'
		}
	})
	if (res_llmApiModels.ok && res_promptConfigs.ok && res_selectedApiPrompt.ok) {
		llmSavedSettings.llmApiModels = await res_llmApiModels.json()
		llmSavedSettings.promptConfigs = await res_promptConfigs.json()
		llmSavedSettings.selectedApiPrompt = await res_selectedApiPrompt.json()
	} else {
		throw new Error(await res.text())
	}

	return { chats, llmSavedSettings };
};