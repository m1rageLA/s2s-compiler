const fs = require("node:fs");
const path = require("node:path");

const repoRoot = path.join(__dirname, "..");
const profile = process.env.PROFILE === "debug" ? "debug" : "release";
const targetDir = path.join(repoRoot, "target", profile);

const candidates =
  process.platform === "win32"
    ? ["ts2rust_node.dll", "ts2rust_node.node"]
    : process.platform === "darwin"
      ? ["libts2rust_node.dylib", "ts2rust_node.node"]
      : ["libts2rust_node.so", "ts2rust_node.node"];

const source = candidates
  .map((name) => path.join(targetDir, name))
  .find((candidate) => fs.existsSync(candidate) && fs.statSync(candidate).isFile());

if (!source) {
  const flag = profile === "release" ? " --release" : "";
  throw new Error(
    `Native artifact not found in ${targetDir}. Run "cargo build -p ts2rust-node${flag}" first.`
  );
}

const destination = path.join(repoRoot, "index.node");
fs.copyFileSync(source, destination);

console.log(`Copied ${path.basename(source)} -> ${path.relative(repoRoot, destination)}`);
