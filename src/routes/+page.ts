import { invoke } from "@tauri-apps/api/core";
import type { PageLoad } from "./$types";
import type { Asset } from "$lib/types";
import { error } from "@sveltejs/kit";

export const load: PageLoad = async () => {
	try {
		const assets = await invoke<Asset[]>("get_assets");
		return {
			assets,
		};
	} catch (err) {
		console.error(err);
		error(500, `${err}`);
	}
};
