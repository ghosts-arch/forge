<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import type { PageProps } from "./$types";
    import { goto } from "$app/navigation";
    import type { Asset } from "$lib/types";

    let name = $state("");
    const createAsset = async (event: SubmitEvent) => {
        event.preventDefault();
        const asset: Asset = await invoke("create_asset", {
            name,
        });
        goto(`/assets/edit/${asset.uuid}`);
    };
</script>

<a href="/">Retour</a>

<form onsubmit={createAsset}>
    <input bind:value={name} />
    <button>Ajouter</button>
</form>
