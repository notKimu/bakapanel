<script lang="ts">
	import { fetchServer } from "$lib/api/server";
	import type { ChatObject } from "$lib/dto";
    import type { PageProps } from "../$types";
	import RconConsole from "$lib/components/server/RconConsole.svelte";
    // Images
    import ServerIconImg from "$lib/assets/img/server-icon.png";

    let { data }: PageProps = $props();

    const serverName = data.slug;

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
</script>

<div class="server__name">
    <h1>{serverName}</h1>
</div>
    
{#await fetchServer(serverName)}
    <p>Fetching server...</p>
{:then server}
        <div class="server__status">
            <div class="server__status__motd">
                <img src={server.status!.favicon || ServerIconImg} alt="">
                <div class="server__status__motd__text">
                    <span class="motd" style="overflow-x: auto; white-space: pre; display:inline-block">{@html renderMotd(server.status!.description)}</span>
                </div>
            </div>

            <div class="server__status__info">
                <p><b>Host:</b> {server.info.host}:{server.info.port}</p>
                <p><b>Version:</b> {server.status!.version.name} : {server.status!.version.protocol}</p>
                <p><b>Players:</b> {server.status!.players.online} / {server.status!.players.max}</p>
            </div>

            {#if server.info.rcon}
                <RconConsole serverName={serverName} />
            {/if}
        </div>
{:catch err}
    <div class="error">
        <p>Unable to reach this server</p>
    </div>
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

    /* ERROR DISPLAY */
    .error {
        background-color: var(--color-4);
        padding: var(--padding-m);
    }
    .error * {
        color: var(--color-2);
    }

    /* RESPONSIVE */
    @media screen and (max-width: 728px) {
        .server__status__motd {
            flex-direction: column;
        }
    }
</style>