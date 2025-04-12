import { bin } from "npm";
import * as ts from "typescript";
import { IRNode } from "./coreSchema";

/**
 * Главная точка входа.
 */
export function transpileFileToIR(filePath: string): IRNode {
    const program = ts.createProgram([filePath], {
        target: ts.ScriptTarget.ESNext,
        module: ts.ModuleKind.CommonJS,
    });
    const sourceFile = program.getSourceFile(filePath);
    if (!sourceFile) {
        throw new Error(`Source file not found: ${filePath}`);
    }
    const body: IRNode[] = [];

    for (const stmt of sourceFile.statements) {
        body.push(convertStatement(stmt));
    }
    return { kind: "Program", body };
}

// =======================
// 🔁 STATEMENT CONVERTERS
// =======================
function convertStatement(node: ts.Node): IRNode {
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

// =======================
// 🧠 EXPRESSION CONVERTER
// =======================
function convertExpression(expr: ts.Expression): IRNode {
    switch (expr.kind) {
        case ts.SyntaxKind.NumericLiteral:
            return { kind: "Literal", value: parseFloat((expr as ts.NumericLiteral).text) };

        case ts.SyntaxKind.StringLiteral:
            return { kind: "Literal", value: (expr as ts.StringLiteral).text };

        case ts.SyntaxKind.TrueKeyword:
            return { kind: "Literal", value: true };

        case ts.SyntaxKind.FalseKeyword:
            return { kind: "Literal", value: false };

        case ts.SyntaxKind.NullKeyword:
            return { kind: "Literal", value: null };

        case ts.SyntaxKind.Identifier:
            return { kind: "Identifier", name: (expr as ts.Identifier).text };

        case ts.SyntaxKind.BinaryExpression: {
            const binExpr = expr as ts.BinaryExpression;
            const operator = ts.tokenToString(binExpr.operatorToken.kind) || "unknown_operator";
            return {
                kind: "Binary",
                operator,
                left: convertExpression(binExpr.left),
                right: convertExpression(binExpr.right),
            };
        }

        case ts.SyntaxKind.CallExpression: {
            const call = expr as ts.CallExpression;
            return {
                kind: "CallExrpression",
                calle: convertExpression(call.expression),
                args: call.arguments.map(arg => convertExpression(arg)),
            };
        }

        default:
            throw new Error("Unsupported expression kind: " + ts.SyntaxKind[expr.kind]);
    }
}

// =======================
// 🔍 SPECIFIC CONVERTERS
// =======================
function convertFunctionDeclaration(node: ts.FunctionDeclaration): IRNode {
    const name = node.name?.text || "anonymous";
    const params = node.parameters.map(p => {
        if (ts.isIdentifier(p.name)) {
            return p.name.text;
        } else {
            throw new Error(`Unsupported parameter pattern: ${p.name.getText?.() || "unknown"}`);
        }
    });
    const body = node.body?.statements.map(convertStatement) || [];

    return {
        kind: "Function",
        name,
        params,
        body,
    };
}

function convertVariableStatement(node: ts.VariableStatement): IRNode {
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
    return {
        kind: "VariableDeclaration",
        name,
        value: initializer,
    };
}

function convertReturnStatement(node: ts.ReturnStatement): IRNode {
    if (!node.expression) {
        throw new Error("Return without expression is unsupported.");
    }
    return {
        kind: "Return",
        value: convertExpression(node.expression),
    };
}

function convertIfStatement(node: ts.IfStatement): IRNode {
    return {
        kind: "If",
        condition: convertExpression(node.expression),
        theBlock: convertBlockLike(node.thenStatement),
        elseBlock: node.elseStatement ? convertBlockLike(node.elseStatement) : undefined,
    };
}

function convertWhileKeyword(node: ts.WhileStatement): IRNode {
    return {
        kind: "While",
        condition: convertExpression(node.expression),
        body: node.statement ? [convertStatement(node.statement)] : [],
    };
}

function convertForStatement(node: ts.ForStatement): IRNode {
    let init: IRNode | undefined;
    if (node.initializer) {
        if (ts.isVariableDeclarationList(node.initializer)) {
            init = convertVariableStatement(node.initializer.parent as ts.VariableStatement);
        } else {
            init = convertExpression(node.initializer as ts.Expression);
        }
    }
    const condition = node.condition ? convertExpression(node.condition) : undefined;
    const increment = node.incrementor ? convertExpression(node.incrementor) : undefined;
    const body = convertBlockLike(node.statement);
    return { kind: "For", init, condition, increment, body };
}

function convertBlock(node: ts.Block): IRNode {
    return {
        kind: "Block",
        statements: node.statements.map(convertStatement),
    };
}

function convertBlockLike(stmt: ts.Statement): IRNode {
    if (ts.isBlock(stmt)) {
        return convertBlock(stmt);
    }
    return { kind: "Block", statements: [convertStatement(stmt)] };
}
