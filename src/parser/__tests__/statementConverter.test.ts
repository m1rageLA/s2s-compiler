import { describe, it, expect, vi, beforeEach } from "vitest";
import * as ts from "typescript";
import {
    fnSentinel,
    varSentinel,
    retSentinel,
    ifSentinel,
    whileSentinel,
    forSentinel,
    blockSentinel,
    exprSentinel,
} from "./sentinels";

import { convertFunctionDeclaration, convertVariableStatement } from "../specificConverters";
import { convertStatement } from "../statementConverters";

vi.mock("../specificConverters", () => ({
    convertFunctionDeclaration: vi.fn().mockReturnValue(fnSentinel),
    convertVariableStatement: vi.fn().mockReturnValue(varSentinel),
}));

describe('convertStatement - it should handle all statements variation', () => {
    it("delegates FunctionDeclaration", () => {
        const functNode = ts.factory.createFunctionDeclaration(
            /* decorators */ undefined,
            /* modifiers  */ undefined,
            /* name       */ "foo",
            /* typeParams */ undefined,
            /* params     */[],
            /* returnType */ undefined,
            ts.factory.createBlock([]),
        );

        const result = convertStatement(functNode);

        expect(convertFunctionDeclaration).toHaveBeenCalledOnce();
        expect(convertFunctionDeclaration).toHaveBeenCalledWith(functNode);
        expect(result).toEqual(fnSentinel);
    });

    it("delegates VaribaleStatement", () => {
        const varNode = ts.factory.createVariableDeclaration(
            /* name */ "x",
            /* exclamationToken */ undefined,
            /* type */ undefined,
            ts.factory.createNumericLiteral(1),
        );
        const varDeclList = ts.factory.createVariableDeclarationList(
            [varNode],
            ts.NodeFlags.Const
        );

        const varStmt = ts.factory.createVariableStatement(
            /* modifiers */ undefined,
            varDeclList
        );

        const result = convertStatement(varStmt);

        expect(convertVariableStatement).toHaveBeenCalledOnce();
        expect(convertVariableStatement).toHaveBeenCalledWith(varStmt);
        expect(result).toEqual(varSentinel);
    });
})
