<script lang="ts">
    import { getAssetsQuery } from "$lib/api/assets.remote";
    const assets = await getAssetsQuery();
</script>

<a href="/">Retour</a>

{#each assets as asset}
    <span>{asset.name}</span>
    {#each asset.fields as field}
        <span>{field.name} : {field.value}</span>
    {/each}
{/each}
