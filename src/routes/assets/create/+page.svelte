<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import type { PageProps } from "./$types";

    let { data }: PageProps = $props();
    let name = $state("");
    let selectedModelId = $state("");
    const createAsset = async (event: SubmitEvent) => {
        event.preventDefault();
        const asset = await invoke("create_asset", {
            name,
            modelId: selectedModelId,
            fields: [],
        });
    };
</script>

<a href="/">Retour</a>

<form onsubmit={createAsset}>
    <input bind:value={name} />
    <select bind:value={selectedModelId}>
        {#each data.models as model}
            <option value={model.uuid}>{model.name} </option>
        {/each}
    </select>
    <button>Ajouter</button>
</form>
