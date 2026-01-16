const fs = require("node:fs");
const path = require("node:path");

const bindingPath = path.join(__dirname, "index.node");

if (!fs.existsSync(bindingPath)) {
  throw new Error(
    'Native binding not found. Build it with "npm run build:native" so index.node is available.'
  );
}

const native = require(bindingPath);

/**
 * Compile and execute a TypeScript snippet through the Rust pipeline.
 * @param {string} source TypeScript source containing the heavy computation.
 * @returns {Promise<{ stdout: string; rust: string }>}
 */
function heavy(source) {
  if (typeof source !== "string") {
    return Promise.reject(
      new TypeError("heavy() expects a string containing TypeScript code")
    );
  }

  return native.heavy(source).then((result) => {
    const parsed = extractOutput(result.stdout);
    return {
      ...result,
      stdout: parsed.stdout,
      output: parsed.output,
    };
  });
}

function prepareArgs(factory) {
  if (typeof factory !== "function") {
    throw new TypeError("heavy.prepareArgs expects a function");
  }
  return factory();
}

const OUTPUT_PREFIX = "__HEAVY_OUTPUT__:";

function extractOutput(stdout) {
  const lines = stdout.split(/\r?\n/);
  let output;
  const kept = [];

  for (const line of lines) {
    if (output === undefined && line.startsWith(OUTPUT_PREFIX)) {
      output = line.slice(OUTPUT_PREFIX.length);
      continue;
    }
    kept.push(line);
  }

  const trimmed = kept.join("\n");
  return {
    stdout: trimmed,
    output,
  };
}

heavy.prepareArgs = prepareArgs;

module.exports = { heavy, prepareArgs };
