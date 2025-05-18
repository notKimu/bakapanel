<script lang="ts">
	import CpuThread from '$lib/components/CpuThread.svelte';
	import RamUsage from '$lib/components/RamUsage.svelte';
	import ServerList from '$lib/components/ServerList.svelte';
	import { fetchHost } from '$lib/api/host';
	import { fetchServerList } from '$lib/api/server';
	import { onDestroy, onMount } from 'svelte';
	import type { HostDTO, ServerDTO } from '$lib/dto';

	let hostData: HostDTO | null = $state(null);
	let serverListData: ServerDTO[] | null = $state(null);
	let hostIntervalId: number;
	let serversIntervalId: number;
	let hostErr: any = $state(null);
	let serverListErr: any = $state(null);

	let serversUp: number = $state(0);

	function updateHostData() {
		fetchHost().then(data => {
			hostErr = null;
			hostData = data;
		}).catch(err => {
			hostData = null;
			hostErr = err;
		});
	}

	function updateServerListData() {
		fetchServerList().then(data => {
			serverListErr = null;
			serverListData = data;
			serverListData.forEach(sv => {
				if (sv.status) serversUp += 1;
			});
		}).catch(err => {
			serverListData = null;
			serverListErr = err;
		});
	}

	// Fetch data initially when the component mounts
	onMount(() => {
		updateHostData();
		updateServerListData()

		hostIntervalId = setInterval(updateHostData, 1000);
		serversIntervalId = setInterval(updateHostData, 10000);
	});

	// Clear the intervals when the component is destroyed
	onDestroy(() => {
		clearInterval(hostIntervalId);
		clearInterval(serversIntervalId);
	});
</script>

<div class="dashboard">
	{#if hostData}
		<div class="host">
			<div>
				<div
					class="host__thread--list"
				>
					{#each hostData.cpu_threads as cpu, idx}
						<CpuThread {idx} usage={cpu.usage} />
					{/each}
				</div>
			</div>

			<div>
				<RamUsage used={hostData.ram_used} max={hostData.ram_max} />
			</div>
		</div>
	{/if}

	{#if serverListData}
		<div class="server-list">
			{#each serverListData as sv}
				<ServerList server={sv} />
			{/each}
		</div>

	{/if}
</div>

<footer>
	<div>
		{#if hostData}
			<p>Host is up</p>
		{:else if hostErr}
			<p>Error getting host data</p>
		{:else}
			<p>Retrieving host status...</p>
		{/if}
	</div>

	<div>
		{#if serverListData}
			{#if serversUp === serverListData.length}
				<p>All servers up</p>
			{:else if serversUp === 0}
				<p>All servers down</p>
			{:else}
				<p>Some servers are down [{serversUp}/{serverListData.length}]</p>
			{/if}
		{:else if serverListErr}
			<p>Error retrieving servers</p>
		{:else}
			<p>Fetching servers...</p>
		{/if}
	</div>
</footer>

<p>{hostErr}<br>{serverListErr}</p>

<style>
	.dashboard {
		display: flex;
		flex-direction: column;
		gap: var(--padding-xl);
	}

	.host {
		display: flex;
		flex-direction: column;
		gap: var(--padding-x);
	}

	.host__thread--list {
		width: 100%;
		display: grid;
		grid-template-rows: repeat(2, 1fr);
		grid-auto-flow: column;
		gap: var(--padding-m);
	}

	.server-list {
		width: 100%;
		display: flex;
		flex-direction: column;
		gap: var(--padding-s);
	}

	/* COOL MACOS STYLE FOOTER*/
	footer {
		position: fixed;
		height: 2rem;
		width: 100vw;
		left: 0;
		bottom: 0;

		display: flex;

		border-top: .2rem solid var(--color-4);
		padding: 0 var(--padding-x);
	}

	footer div {
		height: 100%;

		display: flex;
		align-items: center;
	}
	footer div:not(:first-child) {
		padding-left: var(--padding-x);
	}
	footer div:not(:last-child) {
		padding-right: var(--padding-x);
		border-right: .2rem solid var(--color-4);
	}

	/* RESPONSIVE */
	@media screen and (max-width: 728px) {
		.host__thread--list {
			grid-template-rows: repeat(4, 1fr);
		}
	}
</style>
