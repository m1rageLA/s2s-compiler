export type CompilerParamType = "number" | "string" | "boolean";

export interface CompilerSignature {
  params: Array<{
    name: string;
    type: CompilerParamType;
  }>;
}

export type CompiledFunction<Args extends unknown[] = unknown[], Result = unknown> = ((
  ...args: Args
) => Result) & {
  artifactPath: string;
  source: string;
  signature: CompilerSignature;
};

export function compile<Args extends unknown[], Result>(
  fn: (...args: Args) => Result,
): CompiledFunction<Args, Result>;

export function compileSource<Args extends unknown[] = unknown[], Result = unknown>(
  source: string,
): CompiledFunction<Args, Result>;

export function compileToRust(source: string): string;

export function compileAndExecute(source: string, args?: unknown[]): string;
