<script lang="ts">
    import "@picocss/pico";
    let { children } = $props();
</script>

<header class="container">
    <nav>
        <ul>
            <li><a href="/assets/create">Ajouter un asset</a></li>
            <li><a href="/assets">liste des assets</a></li>
        </ul>
    </nav>
</header>
<main class="container">
    {@render children()}
</main>
