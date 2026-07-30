<script lang="ts">
    import type { PageProps } from "./$types";

    let { data }: PageProps = $props();
</script>

<div class="grid">
    {#each data.assets as asset}
        <article>
            {asset.name} - <a href="/assets/edit/{asset.uuid}">editer</a>
        </article>
    {/each}
</div>
