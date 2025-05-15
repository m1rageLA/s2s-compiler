// В старой реализации при конвертации ForStatement мы предполагали, что initializer всегда
// имеет тип VariableStatement и пытались читать declarationList у node.initializer,
// что приводило к ошибке при узлах вида for(let i=0;;) или при любых других выражениях.
// В этой версии:
// - Добавлена проверка node.initializer на VariableDeclarationList и на Expression.
// - convertVariableStatement теперь принимает и ts.VariableDeclarationList и ts.VariableStatement.
// - При отсутствии initializer возвращается undefined.

import * as ts from "typescript";
import { IRNode } from "./coreSchema";
import { convertExpression } from "./expressionConverter";
import { convertStatement } from "./statementConverters";

// Конвертация FunctionDeclaration
export function convertFunctionDeclaration(node: ts.FunctionDeclaration): IRNode {
    const name = node.name?.text ?? "anonymous";
    const params = node.parameters.map(p => {
        if (ts.isIdentifier(p.name)) {
            return p.name.text;
        }
        throw new Error(`Unsupported parameter pattern: ${p.name.getText()}`);
    });
    const bodyStmts = node.body?.statements.map(convertStatement) ?? [];
    return { kind: "Function", name, params, body: bodyStmts };
}

// Поддержка и ts.VariableStatement, и ts.VariableDeclarationList
export function convertVariableStatement(
    node: ts.VariableStatement | ts.VariableDeclarationList
): IRNode {
    const declList = ts.isVariableStatement(node)
        ? node.declarationList
        : node;
    const decls = declList.declarations;
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
        throw new Error(`Unsupported variable declaration pattern: ${decl.name.getText()}`);
    }
    const name = decl.name.text;
    const value = decl.initializer ? convertExpression(decl.initializer) : undefined;
    return { kind: "VariableDeclaration", name, value };
}

export function convertReturnStatement(node: ts.ReturnStatement): IRNode {
    if (!node.expression) {
        throw new Error("Return without expression is unsupported.");
    }
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

export function convertWhileStatement(node: ts.WhileStatement): IRNode {
    return {
        kind: "While",
        condition: convertExpression(node.expression),
        body: node.statement ? [convertStatement(node.statement)] : [],
    };
}

// Исправленная обработка ForStatement
export function convertForStatement(node: ts.ForStatement): IRNode {
    let init: IRNode | undefined;
    if (node.initializer) {
        if (ts.isVariableDeclarationList(node.initializer)) {
            init = convertVariableStatement(node.initializer);
        } else if (ts.isExpression(node.initializer)) {
            init = { kind: "ExpressionStatement", expression: convertExpression(node.initializer) };
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
    if (ts.isBlock(stmt)) {
        return convertBlock(stmt);
    }
    return { kind: "Block", statements: [convertStatement(stmt)] };
}
