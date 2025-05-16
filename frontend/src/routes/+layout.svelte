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

<main>
    {@render children()}
</main>

<div class="miku01">
    <img src={MikuLogoImg} alt="">
</div>

<style>
    header {
        position: fixed;
        left: 0;
        top: 0;

        height: 3rem;
        width: 100vw;

        display: flex;
        align-items: center;

        padding: 0 var(--padding-horizontal);
    }

    .header__filler {
        height: 3rem;
    }

    .miku01 {
        position: fixed;
        bottom: var(--padding-vertical);
        right: var(--padding-horizontal);
        height: 5rem;
    }
    .miku01 img {
        height: 100%;
        width: auto;
        image-rendering: pixelated;
    }
</style>