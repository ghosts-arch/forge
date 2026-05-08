<script lang="ts">
    import { searchAssets } from "$lib/api/assets.remote";
    import { getModels } from "$lib/api/models.remote";

    let assets = $state([]);
    let searchAsset: string = $state("");

    const updateSearch = async () => {
        if (searchAsset.length < 3) return;
        assets = await searchAssets(searchAsset);
        return assets;
    };
</script>

<a href="/assets/add">Ajouter un asset</a>
<a href="/assets">liste des assets</a>
<a href="/models/create">Ajouter un model</a>
<a href="/models">liste des models</a>

<ul>
    {#each await getModels() as model}
        <li>{model.name} - <a href="/models/edit/{model.id}">editer</a></li>
    {/each}
</ul>
<input type="text" bind:value={searchAsset} oninput={updateSearch} />
<div>
    <ul>
        {#each assets as { id, name, fields }}
            <li>
                {name} - <a href="/assets/edit/{id}">editer</a><br />
                {#each fields as field}
                    <span>{field.name} - {field.value}</span>
                {/each}
            </li>
        {/each}
    </ul>
</div>
