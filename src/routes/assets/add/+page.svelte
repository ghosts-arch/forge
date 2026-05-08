<script lang="ts">
    import { createAsset } from "$lib/api/assets.remote";
    import { getModels } from "$lib/api/models.remote";

    const models = await getModels();
    let selectedModel = $state("");
    let d = $derived(models.find((model) => model.id === selectedModel));
</script>

<a href="/">Retour</a>

<form {...createAsset}>
    <input {...createAsset.fields.name.as("text")} />
    <select
        {...createAsset.fields.modelId.as("select")}
        bind:value={selectedModel}
    >
        {#each models as model}
            <option value={model.id}>{model.name}</option>
        {/each}
    </select>

    {#if d}
        {#each d.fields as { name, type }, index}
            <input
                {...createAsset.fields.fields[index].name.as("hidden", name)}
            />
            <label
                >{name}
                <input
                    {...createAsset.fields.fields[index].value.as("text")}
                    {type}
                /></label
            >
        {/each}
    {/if}
    <button>Ajouter</button>
</form>

{#each createAsset.fields.allIssues() as issue}
    <span>{issue.message} {issue.path}</span>{/each}
