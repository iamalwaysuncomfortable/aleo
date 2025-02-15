export function logAndThrow(message: string): never {
    console.error(message);
    throw new Error(message);
}


export function parseJSON(json: string): any {
    function revive(key: string, value: any, context: any) {
        if (Number.isInteger(value)) {
            return BigInt(context.source);

        } else {
            return value;
        }
    }

    return JSON.parse(json, revive as any);
}


export async function get(url: URL | string, options?: RequestInit) {
    const response = await fetch(url, options);

    if (!response.ok) {
        throw new Error(response.status + " could not get URL " + url);
    }

    return response;
}


export async function post(url: URL | string, options: RequestInit) {
    options.method = "POST";

    const response = await fetch(url, options);

    if (!response.ok) {
        throw new Error(response.status + " could not post URL " + url);
    }

    return response;
}
