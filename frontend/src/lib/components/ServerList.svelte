<script lang="ts">
  import type { ServerDTO } from "$lib/dto";
  	// Images
	import MikuNormalImg from "$lib/assets/img/miku-normal.png";
	import MikuDeadImg from "$lib/assets/img/miku-dead.png";

  const { server }: { server: ServerDTO } = $props();
</script>

<a href={`/${server.info.name}`} class="server">
  <div class="server__status">
    {#if server.status}
      <img src={MikuNormalImg} alt="">
      <!-- <svg class="server__status__icon" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M7 7h10v4h5v2h-5v4H7v-4H2v-2h5V7zm2 2v6h6V9H9z" fill="var(--color-2)"/></svg> -->
    {:else}
      <img src={MikuDeadImg} alt="">
      <!-- <svg class="server__status__icon" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M5 5h2v2H5V5zm4 4H7V7h2v2zm2 2H9V9h2v2zm2 0h-2v2H9v2H7v2H5v2h2v-2h2v-2h2v-2h2v2h2v2h2v2h2v-2h-2v-2h-2v-2h-2v-2zm2-2v2h-2V9h2zm2-2v2h-2V7h2zm0 0V5h2v2h-2z" fill="var(--color-2)"/></svg> -->
    {/if}
  </div>

  <div class="server__info">
    <div>
      <h4>{server.info.name}</h4>
      <p>{server.info.host}:{server.info.port}</p>
    </div>

    <div>
      {#if server.status}
        <p>{server.status.players.online}/{server.status.players.max}</p>
      {:else}
        <div class="server__info__error">
          <p>Offline</p>
        </div>
      {/if}
    </div>
  </div>
</a>

<style>
  .server {
    height: 4rem;

    display: grid;
    grid-template-columns: 2.5rem 1fr;
    gap: var(--padding-s);

    background-color: var(--color-4);
    padding: var(--padding-s);
  }

  .server * {
    color: var(--color-2);
  }

  .server__status img {
    width: 100%;
    image-rendering: pixelated;
  }

  .server__info {
    display: flex;
    justify-content: space-between;
  }

  .server__info__error {
    display: flex;
    align-items: center;
    gap: var(--padding-s);
  }
</style>
