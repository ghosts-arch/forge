import { invoke } from "@tauri-apps/api/core";

import type { Model } from "$lib/types";
import type { PageLoad } from "./$types";
import { error } from "@sveltejs/kit";

export const load: PageLoad = async () => {
	try {
		const models = await invoke<Model[]>("get_models");
		return {
			models,
		};
	} catch (err) {
		console.error(err);
		error(500, `${err}`);
	}
};
