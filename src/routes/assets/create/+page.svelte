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

<form onsubmit={createAsset}>
    <label
        >Nom de l'asset<input
            bind:value={name}
            placeholder="Nom de l'asset..."
        /></label
    >
    <button type="submit">Ajouter</button>
</form>
{#if error}
    <span style="color: red;">{error}</span>
{/if}
