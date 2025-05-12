import * as ts from "typescript";
import { IRNode } from "./coreSchema";

export function convertExpression(expr: ts.Expression): IRNode {
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
            const operatorToken = binExpr.operatorToken.kind;
            const operator = ts.tokenToString(operatorToken) || "unknown_operator";

            if (operatorToken === ts.SyntaxKind.EqualsToken) {
                return {
                    kind: "Assignment",
                    left: convertExpression(binExpr.left),
                    right: convertExpression(binExpr.right),
                };
            };
            return {
                kind: "Binary",
                operator,
                left: convertExpression(binExpr.left),
                right: convertExpression(binExpr.right),
            };
        };
        case ts.SyntaxKind.CallExpression: {
            const call = expr as ts.CallExpression;
            return {
                kind: "CallExpression",
                callee: convertExpression(call.expression),
                args: call.arguments.map(convertExpression),
            };
        };
        case ts.SyntaxKind.ObjectLiteralExpression: {
            const objExp = expr as ts.ObjectLiteralExpression;

            const plainProps = objExp.properties.filter(p =>
                ts.isPropertyAssignment(p) || ts.isShorthandPropertyAssignment(p)
            );

            const props = plainProps.map((prop): { key: string; value: IRNode } => {
                // ---- name ----
                const nameNode = (prop as ts.NamedDeclaration).name!;
                let key: string | undefined;
                if (ts.isIdentifier(nameNode)) key = nameNode.escapedText.toString();
                else if (ts.isStringLiteral(nameNode)
                    || ts.isNumericLiteral(nameNode)) key = nameNode.text;
                if (!key) {
                    throw new Error(
                        "Unsupported or missing object‑literal key: " +
                        ts.SyntaxKind[nameNode.kind]
                    );
                }

                // ---- value ----
                const value = ts.isShorthandPropertyAssignment(prop)
                    ? convertExpression(nameNode as ts.Identifier)          // `{ x }`
                    : convertExpression((prop as ts.PropertyAssignment).initializer); // `{ x: expr }`

                return { key, value };
            });

            return { kind: "ObjectLiteral", properties: props };
        }

        case ts.SyntaxKind.ArrayLiteralExpression: {
            return {
                kind: "ArrayLiteral",
                elements: (expr as ts.ArrayLiteralExpression).elements.map(convertExpression),
            }
        }

        case ts.SyntaxKind.ParenthesizedExpression:
            return convertExpression((expr as ts.ParenthesizedExpression).expression);

        // ---------- Property Access  obj.prop -------------------------------
        case ts.SyntaxKind.PropertyAccessExpression: {
            const node = expr as ts.PropertyAccessExpression;
            return {
                kind: "PropertyAccess",
                object: convertExpression(node.expression),
                property: convertExpression(node.name)   // Identifier → IdentifierNode
            };
        }

        // ---------- Element Access  obj[idx] -------------------------------
        case ts.SyntaxKind.ElementAccessExpression: {
            const node = expr as ts.ElementAccessExpression;
            return {
                kind: "ElementAccess",
                object: convertExpression(node.expression),
                index: convertExpression(node.argumentExpression!)
            };
        }

        default:
            throw new Error("Unsupported expression kind: " + ts.SyntaxKind[expr.kind]);
    }
}
