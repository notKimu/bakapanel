import type { HostDTO } from "$lib/dto";

export async function fetchHost(): Promise<HostDTO> {
	const hostReq = await fetch('http://localhost:3000/api/status');
	const hostData: HostDTO = await hostReq.json();

    return hostData;
}