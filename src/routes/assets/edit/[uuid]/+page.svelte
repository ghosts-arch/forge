<script lang="ts">
    import { addAssetField, getAssetQuery } from "$lib/api/assets.remote";
    let { params } = $props();
    const asset = $derived(await getAssetQuery(params.uuid));
</script>

<a href="/">Retour</a>
<span>{asset.name}</span>
{#each asset?.model?.fields as { name, type }}
    <form {...addAssetField}>
        <input {...addAssetField.fields.uuid.as("hidden", asset?.id)} />
        <input {...addAssetField.fields.name.as("hidden", name)} />
        <label>{name} <input {...addAssetField.fields.value.as(type)} /></label>
        <button>Modifier</button>
    </form>
{/each}
