export type HostDTO = {
    os_name?: string;
    os_kernel_version?: string;
    cpu_usage: number;
    cpu_threads: Threads[];
    ram_max: number;
    ram_used: number;
}

type Threads = {
    name: string;
    usage: number;
    frecuency: number;
}
