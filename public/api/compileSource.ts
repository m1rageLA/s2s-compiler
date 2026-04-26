import { compile } from "./compile.js";
import type { CompiledFunction, CompileOptions } from "./types.js";

export function compileSource<Args extends unknown[] = unknown[], Result = unknown>(
    source: string,
    options: CompileOptions = {},
): CompiledFunction<Args, Result> {
    return compile<Args, Result>(source, options);
}
