<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import type { PageProps } from "./$types";
    import { goto } from "$app/navigation";
    import { validInput } from "$lib/validation";
    let { data }: PageProps = $props();
    const asset = $state(data.asset);

    const saveAsset = async (event: SubmitEvent) => {
        event.preventDefault();
        for (const modelField of asset.model_fields) {
            for (const field of asset.fields) {
                validInput(modelField.kind, field.value);
            }
        }
        await invoke("update_asset_fields", {
            uuid: asset.uuid,
            new_fields: asset.fields,
        });
    };
    const deleteAsset = async () => {
        try {
            let userChoice = confirm("Voulez vous supprimer cet asset ?");
            if (!userChoice) return;
            let result = await invoke("delete_asset", { uuid: asset?.uuid });
            if (!result) return;
            goto("/");
        } catch (err) {
            console.error(err);
        }
    };
</script>

<a href="/">Retour</a>

<button onclick={deleteAsset}>Supprimer</button>

<span>{asset?.name}</span>

<form onsubmit={saveAsset}>
    {#each asset.model_fields as field, index}
        <label
            >{field.name}
            <input type={field.kind} value={asset.fields[index].value} /></label
        >
    {/each}
    <button>sauvegarder</button>
</form>
