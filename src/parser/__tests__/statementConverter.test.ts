import { describe, it, expect, vi } from "vitest";
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

import { convertFunctionDeclaration, convertReturnStatement, convertVariableStatement } from "../specificConverters";
import { convertStatement } from "../statementConverters";

vi.mock("../specificConverters", () => ({
    convertFunctionDeclaration: vi.fn().mockReturnValue(fnSentinel),
    convertVariableStatement: vi.fn().mockReturnValue(varSentinel),
    convertReturnStatement: vi.fn().mockReturnValue(retSentinel),
    convertIfStatement: vi.fn().mockReturnValue(ifSentinel),
    convertWhileKeyword: vi.fn().mockReturnValue(whileSentinel),
    convertForStatement: vi.fn().mockReturnValue(forSentinel),
    convertBlock: vi.fn().mockReturnValue(blockSentinel),
    convertBlockLike: vi.fn().mockReturnValue(blockSentinel),
}));

vi.mock("../expressionConverter", () => ({
    convertExpression: vi.fn().mockReturnValue(exprSentinel),
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

    it("delegates ReturnStatement", () => {
        const retStmt = ts.factory.createReturnStatement(ts.factory.createNumericLiteral(1));
        const result = convertStatement(retStmt);
        expect(convertReturnStatement).toHaveBeenCalledOnce();
        expect(convertReturnStatement).toHaveBeenCalledWith(retStmt);
        expect(result).toEqual(retSentinel);
    });
})
