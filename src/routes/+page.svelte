<script lang="ts">
    import type { PageProps } from "./$types";

    let { data }: PageProps = $props();
</script>

<a href="/assets/create">Ajouter un asset</a>
<a href="/assets">liste des assets</a>

<ul>
    {#each data.assets as asset}
        <li>{asset.name} - <a href="/assets/edit/{asset.uuid}">editer</a></li>
    {/each}
</ul>
