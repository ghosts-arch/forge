<script lang="ts">
    let { kind, value = $bindable() }: { kind: string; value: string } =
        $props();
    $inspect(value);
</script>

{#if kind === "text"}
    <input type="text" bind:value />
{:else if kind === "number"}
    <input type="number" bind:value />
{:else if kind === "date"}
    <input type="date" bind:value />
{/if}
