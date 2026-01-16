export interface HeavyResult {
  stdout: string;
  rust: string;
  /**
   * Value returned by the heavy snippet (captured from the generated program output).
   * Undefined if the snippet did not return anything.
   */
  output?: string;
}

/**
 * Compile and execute a TypeScript snippet through the Rust pipeline.
 * The returned stdout is the output of the generated Rust binary.
 */
export function heavy(source: string): Promise<HeavyResult>;
export function heavy(fn: (...args: any[]) => any): Promise<HeavyResult>;
export function heavy(
  fn: (...args: any[]) => any,
  args: Record<string, unknown>
): Promise<HeavyResult>;
export function prepareArgs<T extends Record<string, unknown>>(
  factory: () => T
): T;
export namespace heavy {
  function prepareArgs<T extends Record<string, unknown>>(factory: () => T): T;
}
