<script lang="ts">
    import type { PageProps } from "./$types";

    let { data }: PageProps = $props();
</script>

<a href="/assets/create">Ajouter un asset</a>
<a href="/assets">liste des assets</a>
<a href="/models/create">Ajouter un model</a>
<a href="/models">liste des models</a>

<ul>
    {#each data.models as model}
        <li>{model.name} - <a href="/models/edit/{model.id}">editer</a></li>
    {/each}
</ul>
