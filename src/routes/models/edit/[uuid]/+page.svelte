<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import type { PageProps } from "./$types";
    import { goto } from "$app/navigation";
    let { data }: PageProps = $props();
    let model = $state(data.model);
    let fieldName = $state("");
    let fieldType = $state("");
    const deleteModel = async () => {
        try {
            let userChoice = confirm("Voulez vous supprimer ce modele ?");
            if (!userChoice) return;
            let result = await invoke("delete_model", { uuid: model.uuid });
            if (!result) return;
            goto("/");
        } catch (err) {
            console.error(err);
        }
    };
    const addModelField = async (event: SubmitEvent) => {
        event.preventDefault();
        if (!model) return;
        try {
            model = await invoke("add_model_field", {
                field: {
                    name: fieldName,
                    kind: fieldType,
                },
                modelUuid: model.uuid,
            });
        } catch (err) {
            console.error(err);
        }
    };
</script>

<a href="/">retour</a>

<span>{model.name}</span>

<button onclick={deleteModel}>Supprimer</button>

{#each model.fields as field}
    <span>{field.name} - {field.kind}</span>
{/each}

<form onsubmit={addModelField}>
    <input bind:value={fieldName} />
    <select bind:value={fieldType}>
        <option value="text">Texte</option>
        <option value="number">Nombre</option>
        <option value="date">Date</option>
    </select>
    <button>Ajouter</button>
</form>
