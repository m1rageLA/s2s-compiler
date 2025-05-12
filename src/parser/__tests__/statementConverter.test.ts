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

vi.mock("../expressionConverter", () => ({
    convertExpression: vi.fn().mockReturnValue(exprSentinel),
}));


// Import after mocks so SUT picks them up
import {
    convertFunctionDeclaration,
    convertVariableStatement,
    convertReturnStatement,
    convertIfStatement,
    convertWhileKeyword,
    convertForStatement,
    convertBlock,
} from "../specificConverters";
import { convertExpression } from "../expressionConverter";
import { convertStatement } from "../statementConverters";

// -----------------------------------------------------------------------------
// Test suite
// -----------------------------------------------------------------------------

beforeEach(() => {
    vi.clearAllMocks();
});

describe("convertStatement – handles every statement variation", () => {
    it("delegates FunctionDeclaration", () => {
        const funcNode = ts.factory.createFunctionDeclaration(
      /* decorators */ undefined,
      /* modifiers  */ undefined,
      /* name       */ "foo",
      /* typeParams */ undefined,
      /* parameters */[],
      /* returnType */ undefined,
            ts.factory.createBlock([], true)
        );

        const result = convertStatement(funcNode);

        //It did call the helper.
        expect(convertFunctionDeclaration).toHaveBeenCalledOnce();

        //It called the helper with the right input.
        expect(convertFunctionDeclaration).toHaveBeenCalledWith(funcNode);

        //It returned the expected result.
        expect(result).toBe(fnSentinel);
    });




    



    it("delegates VariableStatement", () => {
        const varDecl = ts.factory.createVariableDeclaration(
            "x",
      /* exclamationToken */ undefined,
      /* type */ undefined,
            ts.factory.createNumericLiteral(1)
        );
        const varDeclList = ts.factory.createVariableDeclarationList(
            [varDecl],
            ts.NodeFlags.Const
        );
        const varStmt = ts.factory.createVariableStatement(undefined, varDeclList);

        const result = convertStatement(varStmt);

        expect(convertVariableStatement).toHaveBeenCalledOnce();
        expect(convertVariableStatement).toHaveBeenCalledWith(varStmt);
        expect(result).toBe(varSentinel);
    });










    it("delegates ReturnStatement", () => {
        const retStmt = ts.factory.createReturnStatement(ts.factory.createNumericLiteral(0));
        const result = convertStatement(retStmt);

        expect(convertReturnStatement).toHaveBeenCalledOnce();
        expect(convertReturnStatement).toHaveBeenCalledWith(retStmt);
        expect(result).toBe(retSentinel);
    });



    

    it("delegates IfStatement", () => {
        const condition = ts.factory.createIdentifier("cond");
        const ifStmt = ts.factory.createIfStatement(
            condition,
            ts.factory.createBlock([], true),
            undefined
        );

        const result = convertStatement(ifStmt);

        expect(convertIfStatement).toHaveBeenCalledOnce();
        expect(convertIfStatement).toHaveBeenCalledWith(ifStmt);
        expect(result).toBe(ifSentinel);
    });

    it("delegates WhileStatement", () => {
        const condition = ts.factory.createIdentifier("cond");
        const whileStmt = ts.factory.createWhileStatement(
            condition,
            ts.factory.createBlock([], true)
        );

        const result = convertStatement(whileStmt);

        expect(convertWhileKeyword).toHaveBeenCalledOnce();
        expect(convertWhileKeyword).toHaveBeenCalledWith(whileStmt);
        expect(result).toBe(whileSentinel);
    });

    it("delegates ForStatement", () => {
        const forStmt = ts.factory.createForStatement(
      /* initializer */ undefined,
      /* condition   */ undefined,
      /* incrementor */ undefined,
            ts.factory.createBlock([], true)
        );

        const result = convertStatement(forStmt);

        expect(convertForStatement).toHaveBeenCalledOnce();
        expect(convertForStatement).toHaveBeenCalledWith(forStmt);
        expect(result).toBe(forSentinel);
    });

    it("delegates Block", () => {
        const blk = ts.factory.createBlock([], true);

        const result = convertStatement(blk);

        expect(convertBlock).toHaveBeenCalledOnce();
        expect(convertBlock).toHaveBeenCalledWith(blk);
        expect(result).toBe(blockSentinel);
    });

    it("converts ExpressionStatement inline", () => {
        const expr = ts.factory.createIdentifier("x");
        const exprStmt = ts.factory.createExpressionStatement(expr);

        const result = convertStatement(exprStmt);

        expect(convertExpression).toHaveBeenCalledOnce();
        expect(convertExpression).toHaveBeenCalledWith(expr);
        expect(result).toEqual({
            kind: "ExpressionStatement",
            expression: exprSentinel,
        });
    });

    it("unwraps LabeledStatement", () => {
        const innerVarDecl = ts.factory.createVariableDeclaration(
            "y",
            undefined,
            undefined,
            ts.factory.createNumericLiteral(2)
        );
        const innerList = ts.factory.createVariableDeclarationList(
            [innerVarDecl],
            ts.NodeFlags.Const
        );
        const innerVarStmt = ts.factory.createVariableStatement(undefined, innerList);

        const labeled = ts.factory.createLabeledStatement(
            ts.factory.createIdentifier("lbl"),
            innerVarStmt
        );

        const result = convertStatement(labeled);

        expect(convertVariableStatement).toHaveBeenCalledOnce();
        expect(convertVariableStatement).toHaveBeenCalledWith(innerVarStmt);
        expect(result).toBe(varSentinel);
    });

    it("throws on unsupported node kind", () => {
        const unsupported = ts.factory.createToken(ts.SyntaxKind.EndOfFileToken);

        expect(() => convertStatement(unsupported)).toThrow(/Unsupported node kind/);
    });
});
