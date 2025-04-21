import * as ts from "typescript";
import { IRNode } from "./coreSchema";
import { convertExpression } from "./expressionConverter";
import {
    convertFunctionDeclaration,
    convertVariableStatement,
    convertReturnStatement,
    convertIfStatement,
    convertWhileKeyword,
    convertForStatement,
    convertBlock,
    convertBlockLike
} from "./specificConverters";

export function convertStatement(node: ts.Node): IRNode {
    switch (node.kind) {
        case ts.SyntaxKind.FunctionDeclaration:
            return convertFunctionDeclaration(node as ts.FunctionDeclaration);
        case ts.SyntaxKind.VariableStatement:
            return convertVariableStatement(node as ts.VariableStatement);
        case ts.SyntaxKind.ReturnStatement:
            return convertReturnStatement(node as ts.ReturnStatement);
        case ts.SyntaxKind.IfStatement:
            return convertIfStatement(node as ts.IfStatement);
        case ts.SyntaxKind.WhileKeyword:
            return convertWhileKeyword(node as ts.WhileStatement);
        case ts.SyntaxKind.ForStatement:
            return convertForStatement(node as ts.ForStatement);
        case ts.SyntaxKind.Block:
            return convertBlock(node as ts.Block);
        case ts.SyntaxKind.ExpressionStatement:
            return {
                kind: "ExpressionStatement",
                expression: convertExpression((node as ts.ExpressionStatement).expression),
            };
        default:
            throw new Error("Unsupported node kind: " + ts.SyntaxKind[node.kind]);
    }
}
