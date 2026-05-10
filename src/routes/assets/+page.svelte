<script lang="ts">
    import type { PageProps } from "./$types";

    let { data }: PageProps = $props();
</script>

<a href="/">Retour</a>

<ul>
    {#each data.assets as asset}
        <li>{asset.name} - <a href="/models/edit/{asset.id}">editer</a></li>
    {/each}
</ul>
