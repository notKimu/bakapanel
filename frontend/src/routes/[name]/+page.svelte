<script lang="ts">
	import { fetchServer } from "$lib/api/server";
	import type { ChatObject } from "$lib/dto";
    import type { PageProps } from "../$types";

    let { data }: PageProps = $props();

    function renderMotd(motd: ChatObject): string {
        let text = "";

        if (Array.isArray(motd)) {
            motd.forEach(obj => {
                text += renderMotd(obj);
            });
        } else if (typeof motd === "string") {
            text += motd as string;
        } else {
            let motdText = motd.text || "";
            
            if (motd.strikethrough)
                motdText = `<span style='text-decoration: line-through;'>${motdText}</span>`;
            if (motd.underlined)
                motdText = `<span style='text-decoration: underline;'>${motdText}</span>`;
            if (motd.italic) motdText = `<span style="font-style: italic;">${motdText}</span>`;
            if (motd.bold) motdText = `<span style="font-weight: bold;">${motdText}</span>`;
    
            motdText = `<span ${motd.color ? ("style='color: " + motd.color + ";'") : ""}>${motdText}</span>`;

            text += motdText;

            if (motd.extra) {
                motd.extra.forEach(obj => {
                    text += renderMotd(obj);
                });
            }
        }

        return text.replace("\n", "<br>");
    }

    // ERROR DVD BOUNCY
    let rect: DOMRect = $state(new DOMRect());
    let x = $state(0);
    let y = $state(0);
</script>

<div class="server__name">
    <h1>{data.slug}</h1>
</div>
    
{#await fetchServer(data.slug)}
    <p>Fetching server...</p>
{:then server}
    {#if server.status}
        <div class="server__status">
            <div class="server__status__motd">
                <img src={server.status?.favicon} alt="">
                <div class="server__status__motd__text">
                    <span class="motd" style="overflow-x: auto; white-space: pre; display:inline-block">{@html renderMotd(server.status.description)}</span>
                </div>
            </div>

            <div class="server__status__info">
                <p><b>Host:</b> {server.info.host}:{server.info.port}</p>
                <p><b>Version:</b> {server.status.version.name} : {server.status.version.protocol}</p>
                <p><b>Players:</b> {server.status.players.online} / {server.status.players.max}</p>
            </div>
        </div>
    {:else if server.error}
        {#each Object.entries(server.error) as [key, val]}
            <p bind:contentRect={rect} style={`position: absolute; left: ${x}px; top: ${y}px;`} class="error">{key} : {val}</p>
        {/each}
    {/if}
{:catch err}
    <p>Error: {err}</p>
{/await}


<style>
    .server__name {
        display: flex;
        justify-content: space-between;
    }

    .server__status {
        display: flex;
        flex-direction: column;
        gap: var(--padding-x);
    }

    .server__status__motd {
        display: flex;
        gap: var(--padding-m);
    }
    .server__status__motd img {
        width: 3rem;
    }
    .motd {
        color: var(--color-2);
    }

    .server__status__motd__text {
        display: flex;
        align-items: center;
        justify-content: space-between;

        background-color: var(--color-4);
        padding: 0 var(--padding-x);
    }

    .server__status__info {
        display: flex;
        flex-direction: column;
        gap: var(--padding-x);
    }

    .error {
        color: var(--color-3);
    }

    /* RESPONSIVE */
    @media screen and (max-width: 728px) {
        .server__status__motd {
            flex-direction: column;
        }
    }
</style>