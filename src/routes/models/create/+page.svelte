<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    let name = $state("");
    const createModel = async (event: SubmitEvent) => {
        event.preventDefault();
        const model = await invoke("create_model", {
            name,
            fields: [],
        });
    };
</script>

<a href="/">Retour</a>

<form onsubmit={createModel}>
    <input bind:value={name} />
    <button>Ajouter</button>
</form>
