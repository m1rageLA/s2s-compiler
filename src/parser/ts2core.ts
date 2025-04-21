import * as ts from "typescript";
import { IRNode } from "./coreSchema";
import { convertStatement } from "./statementConverters";

export function transpileFileToIR(filePath: string): IRNode {
    const program = ts.createProgram([filePath], {
        target: ts.ScriptTarget.ESNext,
        module: ts.ModuleKind.CommonJS,
    });
    const sourceFile = program.getSourceFile(filePath);
    if (!sourceFile) {
        throw new Error(`Source file not found: ${filePath}`);
    }

    const body: IRNode[] = sourceFile.statements.map(convertStatement);
    return { kind: "Program", body };
}
