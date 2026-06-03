import type { Model } from "$lib/types";
import { invoke } from "@tauri-apps/api/core";
import type { PageLoad } from "./$types";
import { error } from "@sveltejs/kit";

export const load: PageLoad = async ({ params }) => {
	try {
		const model = await invoke<Model>("get_model", { uuid: params.uuid });
		return {
			model,
		};
	} catch (err) {
		console.error(err);
		error(500, `${err}`);
	}
};
