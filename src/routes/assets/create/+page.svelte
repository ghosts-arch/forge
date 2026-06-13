<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { goto } from "$app/navigation";
    import type { Asset } from "$lib/types";

    let name = $state("");
    let error = $state("");
    const createAsset = async (event: SubmitEvent) => {
        event.preventDefault();

        try {
            const asset: Asset = await invoke("create_asset", {
                name,
            });
            goto(`/assets/edit/${asset.uuid}`);
        } catch (err) {
            error = `${err}`;
        }
    };
</script>

<a href="/">Retour</a>

<form onsubmit={createAsset}>
    <input bind:value={name} />
    <button>Ajouter</button>
</form>
{#if error}
    <span style="color: red;">{error}</span>
{/if}
