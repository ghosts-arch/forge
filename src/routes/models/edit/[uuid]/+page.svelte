<script lang="ts">
    import { addField, getModelQuery } from "$lib/api/models.remote";
    import { FieldKind } from "$lib/types";
    const { params } = $props();
    const model = $derived(await getModelQuery(params.uuid));
</script>

<a href="/">retour</a>

<span>{model.name}</span>

<form {...addField}>
    <input {...addField.fields.uuid.as("hidden", model.id)} />
    <input {...addField.fields.name.as("text")} />
    <select {...addField.fields.type.as("select")}>
        {#each FieldKind as type}
            <option>{type}</option>
        {/each}
    </select>
    <button>Ajouter</button>
</form>
