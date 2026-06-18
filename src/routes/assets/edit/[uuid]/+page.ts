import type { Asset, Relation} from "$lib/types";
import { invoke } from "@tauri-apps/api/core";
import type { PageLoad } from "./$types";
import { error } from "@sveltejs/kit";

export const load: PageLoad = async ({ params }) => {
	try {
		const asset = await invoke<Asset>("get_asset", {
			uuid: params.uuid,
    });
    const relationsFor = await invoke<Relation[]>("get_relations_for", {
      source : params.uuid
		})
    return {
      asset,
      relationsFor
		};
	} catch (err) {
		console.error(err);
		error(500, `${err}`);
	}
};
