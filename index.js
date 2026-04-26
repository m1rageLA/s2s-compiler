import { extractFunctionWithTypes, normalizeSourceFunction } from "./extractFn.js";
import { loadNative } from "./native.js";

const artifactCache = new Map();

export function compile(fnOrSource, options = {}) {
  const extracted =
    typeof fnOrSource === "string"
      ? normalizeSourceFunction(fnOrSource, options)
      : extractFunctionWithTypes(fnOrSource, options);

  const artifactPath = getOrBuildArtifact(extracted);

  function compiled(...args) {
    const stdout = loadNative().callArtifact(artifactPath, JSON.stringify(args));
    return parseArtifactResult(stdout);
  }

  Object.defineProperties(compiled, {
    artifactPath: { value: artifactPath, enumerable: true },
    source: { value: extracted.source, enumerable: true },
    signature: { value: extracted.signature, enumerable: true },
  });

  return compiled;
}

export function compileSource(source, options = {}) {
  return compile(source, options);
}

export function compileToRust(source) {
  return loadNative().compileToRust(source);
}

export function compileAndExecute(source, args = []) {
  return loadNative().compileAndExecute(source, JSON.stringify(args));
}

function getOrBuildArtifact(extracted) {
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

function parseArtifactResult(stdout) {
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
