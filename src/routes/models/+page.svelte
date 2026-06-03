<script lang="ts">
    import type { PageProps } from "./$types";
    let { data }: PageProps = $props();
</script>

<a href="/">Retour</a>

{#each data.models as model}
    <span>{model.name} - <a href="/models/edit/{model.uuid}">Editer</a></span>
{/each}
