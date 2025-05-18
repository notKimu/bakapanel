<script lang="ts">
    import "$lib/assets/css/globals.css";
    // Images
    import MikuLogoImg from "$lib/assets/img/miku-01.png";

    const { children } = $props();
</script>

<header>
    <p>BakaPanel</p>
</header>
<div class="header__filler"></div>

<img class="miku01" src={MikuLogoImg} alt="">

<main>
    {@render children()}
</main>

<style>
    header {
        position: fixed;
        height: 3rem;
        width: 100vw;
        left: 0;
        top: 0;

        display: flex;
        align-items: center;

        padding: 0 var(--padding-horizontal);
        z-index: 1000;
    }

    .header__filler {
        height: 3rem;
    }

    .miku01 {
        position: fixed;
        height: 15rem;
        bottom: 3rem;
        right: var(--padding-vertical);

        image-rendering: pixelated;
        opacity: .25;
        z-index: 0;
    }

    main {
        position: relative;
        z-index: 10;
    }
</style>