import { loadNative } from "../native.js";
import type { ExtractedFunction } from "./types.js";

const artifactCache = new Map<string, string>();

export function getOrBuildArtifact(extracted: ExtractedFunction): string {
    const key = `${extracted.source}\n${JSON.stringify(extracted.signature)}`;
    const cached = artifactCache.get(key);
    if (cached) {
        return cached;
    }

    const artifactPath = loadNative().compileFunction(
        extracted.source,
        JSON.stringify(extracted.signature),
    );
    artifactCache.set(key, artifactPath);
    return artifactPath;
}

export function parseArtifactResult(stdout: string): unknown {
    const lines = stdout.split(/\r?\n/).filter((line) => line.length > 0);
    const lastLine = lines.at(-1);
    if (lastLine === undefined) {
        return undefined;
    }

    try {
        return JSON.parse(lastLine);
    } catch (error) {
        throw new Error(`Generated artifact returned non-JSON output: ${lastLine}`, {
            cause: error,
        });
    }
}
