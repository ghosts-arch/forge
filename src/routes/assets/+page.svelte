<script lang="ts">
    import type { PageProps } from "./$types";

    let { data }: PageProps = $props();
</script>

<a href="/">Retour</a>

<ul>
    {#each data.assets as asset}
        <li>{asset.name} - <a href="/assets/edit/{asset.uuid}">editer</a></li>
    {/each}
</ul>
