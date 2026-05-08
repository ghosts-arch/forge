<script>
    import { createModel } from "$lib/api/models.remote";
</script>

<a href="/">Retour</a>

<form {...createModel}>
    <input {...createModel.fields.name.as("text")} />
    <button>Ajouter</button>
</form>
