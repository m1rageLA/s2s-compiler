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
        case ts.SyntaxKind.WhileStatement:

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
        case ts.SyntaxKind.LabeledStatement: {
            const lbl = node as ts.LabeledStatement;
            // ignore the label, just transpile the wrapped statement
            return convertStatement(lbl.statement);
        }
        case ts.SyntaxKind.SwitchStatement: {
            const stmt = node as ts.SwitchStatement;
            const cases = stmt.caseBlock.clauses.map(clause => {
                if (ts.isCaseClause(clause)) {
                    return {
                        test: convertExpression(clause.expression),
                        consequent: clause.statements.map(s => convertStatement(s))
                    };
                } else {
                    return {
                        test: "default" as const,
                        consequent: clause.statements.map(s => convertStatement(s))
                    };
                }
            });

            return {
                kind: "Switch",
                expression: convertExpression(stmt.expression),
                cases
            };
        }
        case ts.SyntaxKind.BreakStatement: {
            const br = node as ts.BreakStatement;
            return br.label
                ? { kind: "BreakStatement", label: br.label.text }
                : { kind: "BreakStatement" };
        }

        default:
            throw new Error("Unsupported node kind: " + ts.SyntaxKind[node.kind]);
    }
}
