import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

export interface Callsite {
  file: string;
  line: number;
  column: number;
}

const COMPILER_ROOT = path.dirname(path.dirname(fileURLToPath(import.meta.url)));

export function findUserCallsite(): Callsite {
  const stack = new Error().stack?.split("\n").slice(1) ?? [];

  for (const line of stack) {
    const frame = parseStackLine(line);
    if (!frame) {
      continue;
    }
    if (frame.file.startsWith("node:") || frame.file.includes(`${path.sep}node_modules${path.sep}`)) {
      continue;
    }
    if (!frame.file.startsWith(COMPILER_ROOT)) {
      return frame;
    }
  }

  throw new Error("Cannot determine compile(...) callsite");
}

export function resolveTypeScriptSource(runtimeFile: string): string {
  if (runtimeFile.endsWith(".ts") || runtimeFile.endsWith(".tsx")) {
    return runtimeFile;
  }

  const parsed = path.parse(runtimeFile);
  const candidates = [
    path.join(parsed.dir, `${parsed.name}.ts`),
    path.join(parsed.dir, `${parsed.name}.tsx`),
  ];

  const source = candidates.find((candidate) => fs.existsSync(candidate));
  if (!source) {
    throw new Error(
      `Cannot find TypeScript source for ${runtimeFile}. Compile from a .ts file or keep the .ts next to emitted JS.`,
    );
  }

  return source;
}

function parseStackLine(line: string): Callsite | null {
  const match = line.match(/\(?((?:file:\/\/)?\/.*?):(\d+):(\d+)\)?$/);
  if (!match) {
    return null;
  }

  const file = match[1].startsWith("file://") ? fileURLToPath(match[1]) : match[1];
  return {
    file,
    line: Number(match[2]),
    column: Number(match[3]),
  };
}
