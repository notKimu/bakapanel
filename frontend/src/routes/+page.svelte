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

	function updateHostData() {
		fetchHost().then(data => {
			hostData = data;
		}).catch(err => {
			hostErr = err;
		});
	}

	function updateServerListData() {
		fetchServerList().then(data => {
			serverListData = data;
		}).catch(err => {
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
					style={hostData.cpu_threads.length <= 4
						? `grid-template-columns: repeat(${hostData.cpu_threads.length}, 1fr); grid-template-rows 1fr;`
						: `grid-template-columns: repeat(${hostData.cpu_threads.length / 2}, 1fr); grid-template-rows 1fr 1fr;`}
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
		gap: var(--padding-m);
	}

	.server-list {
		width: 100%;
		display: flex;
		flex-direction: column;
		gap: var(--padding-s);
	}
</style>
