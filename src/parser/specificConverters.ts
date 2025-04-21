import * as ts from "typescript";
import { IRNode } from "./coreSchema";
import { convertExpression } from "./expressionConverter";
import { convertStatement } from "./statementConverters";

export function convertFunctionDeclaration(node: ts.FunctionDeclaration): IRNode {
    const name = node.name?.text || "anonymous";
    const params = node.parameters.map((p) => {
        if (ts.isIdentifier(p.name)) return p.name.text;
        throw new Error(`Unsupported parameter pattern: ${p.name.getText?.() || "unknown"}`);
    });
    const body = node.body?.statements.map(convertStatement) || [];
    return { kind: "Function", name, params, body };
}

export function convertVariableStatement(node: ts.VariableStatement): IRNode {
    const decls = node.declarationList.declarations;
    if (decls.length === 1) {
        return singleVarDeclToIR(decls[0]);
    }
    return {
        kind: "Block",
        statements: decls.map(singleVarDeclToIR),
    };
}

function singleVarDeclToIR(decl: ts.VariableDeclaration): IRNode {
    if (!ts.isIdentifier(decl.name)) {
        throw new Error(`Unsupported variable declaration pattern: ${decl.name.getText?.() || "unknown"}`);
    }
    const name = decl.name.text;
    const initializer = decl.initializer ? convertExpression(decl.initializer) : undefined;
    return { kind: "VariableDeclaration", name, value: initializer };
}

export function convertReturnStatement(node: ts.ReturnStatement): IRNode {
    if (!node.expression) throw new Error("Return without expression is unsupported.");
    return { kind: "Return", value: convertExpression(node.expression) };
}

export function convertIfStatement(node: ts.IfStatement): IRNode {
    return {
        kind: "If",
        condition: convertExpression(node.expression),
        thenBlock: convertBlockLike(node.thenStatement),
        elseBlock: node.elseStatement ? convertBlockLike(node.elseStatement) : undefined,
    };
}

export function convertWhileKeyword(node: ts.WhileStatement): IRNode {
    return {
        kind: "While",
        condition: convertExpression(node.expression),
        body: node.statement ? [convertStatement(node.statement)] : [],
    };
}

export function convertForStatement(node: ts.ForStatement): IRNode {
    let init: IRNode | undefined = undefined;
    if (node.initializer) {
        if (ts.isVariableDeclarationList(node.initializer)) {
            init = convertVariableStatement(node.initializer.parent as ts.VariableStatement);
        } else if (ts.isExpression(node.initializer)) {
            init = convertExpression(node.initializer);
        }
    }
    const condition = node.condition ? convertExpression(node.condition) : undefined;
    const increment = node.incrementor ? convertExpression(node.incrementor) : undefined;
    const body = convertBlockLike(node.statement);

    return { kind: "For", init, condition, increment, body };
}

export function convertBlock(node: ts.Block): IRNode {
    return {
        kind: "Block",
        statements: node.statements.map(convertStatement),
    };
}

export function convertBlockLike(stmt: ts.Statement): IRNode {
    if (ts.isBlock(stmt)) return convertBlock(stmt);
    return { kind: "Block", statements: [convertStatement(stmt)] };
}
