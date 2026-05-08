<script lang="ts">
    import { deleteModel, getModels } from "$lib/api/models.remote";
</script>

<a href="/">Retour</a>

{#each await getModels() as model}
    <span>{model.name}</span>
    {#each model.fields as field}
        <span>{field.name} - {field.type}</span>
    {/each}
    <button
        onclick={async () => {
            try {
                if (confirm("Supprimer ce modele ?")) {
                    await deleteModel(model.id).updates(getModels);
                }
            } catch (err) {
                console.error(err);
            }
        }}>Supprimer</button
    >
{/each}
