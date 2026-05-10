import { invoke } from "@tauri-apps/api/core";

import type { Asset } from "$lib/types";
import type { PageLoad } from "../$types";

export const load: PageLoad = async () => {
	const assets = await invoke<Asset[]>("get_assets");
	return {
		assets,
	};
};
