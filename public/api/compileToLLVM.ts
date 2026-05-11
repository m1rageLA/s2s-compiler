import { loadNative } from "../native.js";

export function compileToLLVM(source: string): string {
    return loadNative().compileToLLVM(source);
}