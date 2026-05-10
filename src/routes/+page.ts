import { invoke } from "@tauri-apps/api/core";
import type { PageLoad } from "./$types";
import type { Model } from "$lib/types";

export const load: PageLoad = async () => {
	const models = await invoke<Model[]>("get_models");
	return {
		models,
	};
};
