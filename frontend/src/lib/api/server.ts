import type { ServerDTO } from "$lib/dto";

export async function fetchServerList(): Promise<ServerDTO[]> {
    const serversReq = await fetch('http://localhost:3000/api/servers');
    const serversData: ServerDTO[] = await serversReq.json();

    return serversData;
}

export async function fetchServer(name: string): Promise<ServerDTO> {
    const serverReq = await fetch(`http://localhost:3000/api/server/${name}`);
    if (!serverReq.ok) throw new Error(`Fetch error with status ${serverReq.status}`);

    const serverData: ServerDTO = await serverReq.json();

    return serverData;
}

export async function sendRconCommand(name: string, cmd: string): Promise<string> {
    const cmdReq = await fetch(`http://localhost:3000/api/server/${name}/rcon`, {
        method: "POST",
        body: JSON.stringify({
            cmd
        }),
        headers: {
            "Content-type": "application/json; charset=ASCII"
        }
    });
    return await cmdReq.text();
}

