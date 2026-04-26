import fs from "node:fs";
import path from "node:path";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const compilerRoot = path.resolve(__dirname, "..");
const artifactsDir = path.join(compilerRoot, "artifacts");
const artifactPath = path.join(artifactsDir, "ts2rust-native.node");

const status = spawnSync("cargo", ["build", "--release", "-p", "ts2rust-native"], {
  cwd: compilerRoot,
  stdio: "inherit",
});

if (status.error) {
  throw status.error;
}

if (status.status !== 0) {
  process.exit(status.status ?? 1);
}

fs.mkdirSync(artifactsDir, { recursive: true });
fs.copyFileSync(nativeLibraryPath(compilerRoot), artifactPath);
console.log(`native artifact: ${artifactPath}`);

function nativeLibraryPath(root) {
  const releaseDir = path.join(root, "target", "release");

  if (process.platform === "darwin") {
    return path.join(releaseDir, "libts2rust_native.dylib");
  }

  if (process.platform === "win32") {
    return path.join(releaseDir, "ts2rust_native.dll");
  }

  return path.join(releaseDir, "libts2rust_native.so");
}
