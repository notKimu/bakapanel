<script lang="ts">
	import { sendRconCommand } from "$lib/api/server";
	import { onMount } from "svelte";

	const { serverName }: { serverName: string } = $props();

	let currentCmd = $state("");
	const commands: { cmd: string; out: string }[] = $state([]);
	let terminalElement: HTMLElement | null = null;

	onMount(() => {
		terminalElement = document.getElementById("terminal");
	});

	function sendCommand(cmd: string) {
		if (cmd.length === 0) return;

		sendRconCommand(serverName, cmd)
			.then((out) => {
				commands.push({ cmd, out });
			})
			.catch(() => {
				commands.push({ cmd, out: "An unexpected error occurred" });
			})
			.finally(() => {
				currentCmd = "";
				if (terminalElement) terminalElement.scrollTop = terminalElement.scrollHeight;
			});
	}
</script>

<div class="console">
	<div class="console__title">
		<p><i>RCON Console</i></p>
	</div>

	<div id="terminal" class="console__output">
		{#each commands as command}
			<p>$ {command.cmd}<br />{command.out}<br /></p>
		{/each}
	</div>

	<div class="console__input">
		<p>$</p>
		<input
			onkeydown={(key) => {
				if (key.code === "Enter") sendCommand(currentCmd);
			}}
			bind:value={currentCmd}
			type="text"
		/>
		<button
			aria-label="enter command"
			onclick={() => {
				sendCommand(currentCmd);
			}}
		>
			<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
				<path
					d="M18 16H8v2H6v-2H4v-2h2v-2h2v2h10V4h2v12h-2zM8 12v-2h2v2H8zm0 6v2h2v-2H8z"
					fill="var(--color-2)"
				/>
			</svg>
		</button>
	</div>
</div>

<style>
	.console {
		height: 25rem;
		width: 100%;

		display: grid;
		grid-template-rows: min-content 1fr 2rem;
		padding: var(--padding-s);

		background-color: var(--color-4);
	}

	.console__title {
		height: min-content;

		background-color: var(--color-2);
		text-align: center;
	}

	.console__output {
		display: flex;
		flex-direction: column;
		gap: var(--padding-m);

		overflow-y: scroll;
		padding: var(--padding-s) 0;
	}
	.console__output * {
		color: var(--color-2);
	}

	.console__input {
		display: grid;
		align-items: center;
		gap: var(--padding-m);
		grid-template-columns: min-content 1fr 2rem;

		border-top: 0.2rem solid var(--color-3);
	}
	.console__input * {
		color: var(--color-2);
	}

	.console__input input {
		height: 100%;
		width: 100%;
		background-color: transparent;
		border: none;
	}

	.console__input button {
		height: 100%;
		background-color: transparent;
		border: none;
		border-left: 0.2rem solid var(--color-3);
		padding: 0.1rem;
	}
	.console__input button:hover {
		background-color: var(--color-3);
	}
</style>
