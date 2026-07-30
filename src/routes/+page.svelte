<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    import type { Asset } from "$lib/types";

    let userInput: string = $state("");
    let assets: Asset[] = $state([]);
    let timer: ReturnType<typeof setTimeout>;
    const searchAssets = async () => {
        assets = await invoke("search_assets", { query: userInput });
    };
    const debounce = () => {
        clearTimeout(timer);
        timer = setTimeout(searchAssets, 300);
    };

    $effect(() => {
        if (!userInput.trim()) {
            assets = [];
        } else {
            debounce();
        }
        return () => {
            clearTimeout(timer);
        };
    });
</script>

<section>
    <form role="search">
        <input type="text" bind:value={userInput} />
    </form>

    <div class="grid">
        {#each assets as asset}
            <article>
                <span>{asset.name}</span>
                <a href="/assets/edit/{asset.uuid}">Editer</a>
            </article>
        {/each}
    </div>
</section>
