import { loadNative } from "../native.js";

export function compileToRust(source: string): string {
    return loadNative().compileToRust(source);
}
