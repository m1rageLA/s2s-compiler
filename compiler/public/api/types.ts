import ts from "typescript";

export type CompiledFunction<Args extends unknown[] = unknown[], Result = unknown> = ((
    ...args: Args
) => Result) & {
    artifactPath: string;
    source: string;
    signature: CompilerSignature;
};

export interface CompilerSignature {
    params: Array<{
        name: string;
        type: CompilerParamType;
    }>;
}

export type CompilerParamType = "number" | "string" | "boolean";

export interface CompileOptions {
    callsite?: {
        file: string;
        line: number;
        column: number;
    };
    fileName?: string;
}

export interface ExtractedFunction {
    source: string;
    signature: CompilerSignature;
}

export type FunctionLike =
    | ts.ArrowFunction
    | ts.FunctionExpression
    | ts.FunctionDeclaration;

export interface NormalizedParam {
    name: string;
    type: CompilerParamType;
    source: string;
}
