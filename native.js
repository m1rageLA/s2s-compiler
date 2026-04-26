import fs from "node:fs";
import path from "node:path";
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";

const require = createRequire(import.meta.url);
const COMPILER_ROOT = path.dirname(fileURLToPath(import.meta.url));
const ARTIFACT_PATH = path.join(COMPILER_ROOT, "artifacts", "ts2rust-native.node");

let nativeModule = null;

export function loadNative() {
  if (nativeModule) {
    return nativeModule;
  }

  if (!fs.existsSync(ARTIFACT_PATH)) {
    throw new Error(
      `Native artifact is missing: ${ARTIFACT_PATH}. Run "npm --prefix compiler run build:native" first.`,
    );
  }

  nativeModule = require(ARTIFACT_PATH);
  return nativeModule;
}

export function nativeArtifactPath() {
  return ARTIFACT_PATH;
}
