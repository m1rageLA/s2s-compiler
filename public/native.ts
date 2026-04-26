import fs from "node:fs";
import path from "node:path";
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";

const require = createRequire(import.meta.url);
const COMPILER_ROOT = path.dirname(path.dirname(fileURLToPath(import.meta.url)));
const ARTIFACT_PATH = path.join(COMPILER_ROOT, "artifacts", "ts2rust-native.node");

interface NativeModule {
  callArtifact(artifactPath: string, argsJson: string): string;
  compileAndExecute(source: string, argsJson: string): string;
  compileFunction(source: string, signatureJson: string): string;
  compileToRust(source: string): string;
}

let nativeModule: NativeModule | null = null;


// ========================
// load native .node module
// ========================
export function loadNative(): NativeModule {
  if (nativeModule) {
    return nativeModule;
  }

  if (!fs.existsSync(ARTIFACT_PATH)) {
    throw new Error(
      `Native artifact is missing: ${ARTIFACT_PATH}. Run "npm --prefix compiler run build:native" first.`,
    );
  }

  nativeModule = require(ARTIFACT_PATH) as NativeModule;
  return nativeModule;
}

export function nativeArtifactPath(): string {
  return ARTIFACT_PATH;
}
