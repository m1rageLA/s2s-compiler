import { extractFunctionWithTypes, normalizeSourceFunction } from "../extract/index.js";
import { loadNative } from "../native.js";
import { getOrBuildArtifact, parseArtifactResult } from "./helpers.js";
import type { CompiledFunction, CompileOptions } from "./types.js";

export function compile<Args extends unknown[] = unknown[], Result = unknown>(
    fn: (...args: Args) => Result,
    options?: CompileOptions,
): CompiledFunction<Args, Result>;

export function compile<Args extends unknown[] = unknown[], Result = unknown>(
    source: string,
    options?: CompileOptions,
): CompiledFunction<Args, Result>;

export function compile<Args extends unknown[] = unknown[], Result = unknown>(
    fnOrSource: string | ((...args: Args) => Result),
    options: CompileOptions = {},
): CompiledFunction<Args, Result> {
    const extracted =
        typeof fnOrSource === "string"
            ? normalizeSourceFunction(fnOrSource, options)
            : extractFunctionWithTypes(fnOrSource as Function, options);

    const artifactPath = getOrBuildArtifact(extracted);

    function compiled(...args: Args): Result {
        const stdout = loadNative().callArtifact(artifactPath, JSON.stringify(args));
        return parseArtifactResult(stdout) as Result;
    }

    Object.defineProperties(compiled, {
        artifactPath: { value: artifactPath, enumerable: true },
        source: { value: extracted.source, enumerable: true },
        signature: { value: extracted.signature, enumerable: true },
    });

    return compiled as CompiledFunction<Args, Result>;
}
