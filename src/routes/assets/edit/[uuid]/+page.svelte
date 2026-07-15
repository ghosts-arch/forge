<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import type { PageProps } from "./$types";
    import Select from "$lib/components/select.svelte";
    import { goto } from "$app/navigation";
    import type { Asset, AssetInformations, Relation } from "$lib/types";
    import { FieldKind } from "$lib/types";
    import { validInput } from "$lib/validation";

    let { data }: PageProps = $props();
    let asset = $derived(data.asset);
    let relationsFor = $derived(data.relationsFor);
    let editedAsset: Asset | undefined = $state(undefined);
    $inspect(editedAsset);
    let editedRelationsFor: Relation[] | undefined = $state(undefined);
    $effect(() => {
        editedAsset = asset;
        editedRelationsFor = relationsFor;
    });
    let targetAssetuuid = $state("");
    let relationDescription = $state("");
    let message = $state("");
    const removeField = (index: number) => {
        if (!editedAsset?.fields) return;
        editedAsset.fields = editedAsset.fields.filter((_, i) => i != index);
    };
    const addField = () => {
        editedAsset?.fields.push({
            uuid: undefined,
            name: "",
            kind: FieldKind.TEXT,
            value: "",
            asset_id: asset.uuid,
        });
    };

    const saveAsset = async (event: SubmitEvent) => {
        event.preventDefault();
        try {
            editedAsset?.fields.forEach((field) =>
                validInput(field.kind, field.value),
            );
            editedAsset = await invoke("update_asset", {
                uuid: editedAsset?.uuid,
                name: editedAsset?.name,
                fields: editedAsset?.fields,
            });
            message = "L'asset à bien été sauvegardé !";
        } catch (err) {
            console.error(err);
        }
    };
    const deleteAsset = async () => {
        try {
            let userChoice = confirm("Voulez vous supprimer cet asset ?");
            if (!userChoice) return;
            let result = await invoke("delete_asset", {
                uuid: editedAsset?.uuid,
            });
            if (!result) return;
            goto("/");
        } catch (err) {
            console.error(err);
        }
    };
    const addRelation = async (event: SubmitEvent) => {
        event.preventDefault();
        try {
            await invoke("create_relation", {
                description: relationDescription,
                sourceAssetUuid: editedAsset?.uuid,
                targetAssetUuid: targetAssetuuid,
            });
            editedRelationsFor = await invoke("get_relations_for", {
                source: editedAsset?.uuid,
            });
        } catch (err) {
            console.error(err);
        }
    };
</script>

<a href="/">Retour</a>

{#if editedAsset}
    <form onsubmit={addRelation}>
        <label for="">Ajouter une relation</label>
        <input type="text" bind:value={relationDescription} />
        <select bind:value={targetAssetuuid}>
            {#await invoke<AssetInformations[]>("get_relations") then relations}
                {#each relations as relation}
                    <option value={relation.uuid}>{relation.name}</option>
                {/each}
            {/await}
        </select>
        <button>Ajouter</button>
    </form>
    <div>
        {#each editedRelationsFor as relation}
            ▶️ {relation.description} :
            <a href="/assets/edit/{relation.target_asset_uuid}"
                >{relation.name}</a
            >
        {/each}
    </div>
    <div>
        {#await invoke<Relation[]>( "get_relations_from", { source: editedAsset.uuid } ) then relations}
            {#each relations as relation}
                ↩️ est référencé par :
                <a href="/assets/edit/{relation.source_asset_uuid}"
                    >{relation.name}</a
                >
            {/each}
        {/await}
    </div>
    <form onsubmit={saveAsset}>
        <button type="button" onclick={deleteAsset}>Supprimer</button>
        <label for="asset_name">Nom :</label>
        <input type="text" id="asset_name" bind:value={editedAsset.name} />
        <ul>
            {#each editedAsset.fields as field, index}
                <li>
                    <button type="button" onclick={() => removeField(index)}
                        >Supprimer</button
                    >
                    <label for={`field_name_${index}`}>Nom :</label>
                    <input
                        type="text"
                        id={`field_name_${index}`}
                        bind:value={field.name}
                    />
                    <label for={`field_kind_${index}`}>Type : </label>
                    <select bind:value={field.kind} id={`field_kind_${index}`}>
                        {#each Object.entries(FieldKind) as [key, value]}
                            <option {value}>{key.toLowerCase()}</option>
                        {/each}
                    </select>
                    <span>{field.value}</span>
                    <Select kind={field.kind} bind:value={field.value} />
                </li>
            {/each}
            <button type="button" onclick={addField}>Ajouter un champ</button>
        </ul>
        <button>Sauvegarder</button>
    </form>
    {#if message}
        <span style="color: green;">{message}</span>
    {/if}
{/if}
