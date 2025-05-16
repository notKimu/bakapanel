import type { ServerDTO } from "$lib/dto";

export async function fetchServerList(): Promise<ServerDTO[]> {
    const serversReq = await fetch('http://localhost:3000/api/servers');
    const serversData: ServerDTO[] = await serversReq.json();

    return serversData;
}

export async function fetchServer(name: string): Promise<ServerDTO> {
    const serverReq = await fetch(`http://localhost:3000/api/server/${name}`);
    console.log(serverReq.status);
    
    const serverData: ServerDTO = await serverReq.json();

    return serverData;
}