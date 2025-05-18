export type ServerDTO = {
    info: ServerInfo;
    status?: ServerStatus;
    error?: { [key: string]: string; };
}

/**
 * Information needed to reach and identify the server
 * It's always present
 */
type ServerInfo = {
    name: string;
    host: string;
    port: number;
    rcon: boolean;
}

/**
 * The information obtained by querying the server
 * If an error occurs when reaching the server it won't be present
 */
type ServerStatus = {
    version: Version;
    players: Players;
    description: ChatObject;
    favicon?: string;
    previews_chat?: boolean;
    enforces_secure_chat?: boolean;

}

type Version = {
    name: string;
    protocol: number;
}

type Players = {
    max: number,
    online: number,
}

export type ChatObject = ChatComponentObject | ChatObject[]; // Any

export type ChatComponentObject = {
    text?: string;
    translate?: string;
    keybind?: string;
    bold?: boolean;
    italic?: boolean;
    underlined?: boolean;
    strikethrough?: boolean;
    obfuscated?: boolean;
    font?: string;
    color?: string;
    insertion?: string;
    clickEvent?: ChatClickEvent;
    hoverEvent?: ChatHoverEvent;
    extra?: ChatObject[];
}

type ChatClickEvent = {
    open_url?: string,
    run_command?: string,
    suggest_command?: string,
    copy_to_clipboard?: string,
}

type ChatHoverEvent = {
    show_text?: ChatObject,
    value?: ChatObject,
    show_item?: string,
    show_entity?: string,
}