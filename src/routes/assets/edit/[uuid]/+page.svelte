<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import type { PageProps } from "./$types";
    import { goto } from "$app/navigation";
    let { data }: PageProps = $props();
    const asset = $state(data.asset);

    const addField = () => {
        asset.fields.push({
            name: "",
            kind: "text",
            value: "",
            asset_id: asset.uuid,
        });
    };

    const saveAsset = async (event: SubmitEvent) => {
        event.preventDefault();
        await invoke("update_asset", {
            uuid: asset.uuid,
            name: asset.name,
            fields: asset.fields,
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

<form onsubmit={saveAsset}>
    <button type="button" onclick={deleteAsset}>Supprimer</button>
    <label for="asset_name">Nom :</label>
    <input type="text" id="asset_name" bind:value={asset.name} />
    <li>
        {#each asset.fields as field, index}
            <ul>
                <label for={`field_name_${index}`}>Nom :</label>
                <input
                    type="text"
                    id={`field_name_${index}`}
                    bind:value={field.name}
                />
                <label for={`field_kind_${index}`}>Type : </label>
                <select bind:value={field.kind} id={`field_kind_${index}`}>
                    <option value="text">Texte</option>
                    <option value="number">Nombre</option>
                    <option value="date">Date</option>
                </select>
                <input type={field.kind} bind:value={field.value} />
            </ul>
        {/each}
        <button type="button" onclick={addField}>Ajouter un champ</button>
    </li>
    <button>Sauvegarder</button>
</form>
