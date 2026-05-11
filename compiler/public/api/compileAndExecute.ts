import { loadNative } from "../native.js";

export function compileAndExecute(source: string, args: unknown[] = []): string {
    return loadNative().compileAndExecute(source, JSON.stringify(args));
}
