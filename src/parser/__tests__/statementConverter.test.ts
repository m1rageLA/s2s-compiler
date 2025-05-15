import { describe, it, expect, beforeEach } from "vitest";
import ts from "typescript";

import {
  convertStatement
} from "../statementConverters";
import {
  convertFunctionDeclaration,
  convertVariableStatement,
  convertReturnStatement,
  convertIfStatement,
  convertWhileKeyword,
  convertForStatement,
  convertBlock
} from "../specificConverters";
import { convertExpression } from "../expressionConverter";

import type { IRNode } from "../coreSchema";

describe("convertStatement – real integration", () => {
  // ------------------------------------------------------------------
  // 1. Проверяем, что результат == тому, что вернул helper
  // ------------------------------------------------------------------
  it("delegates FunctionDeclaration and returns real IR", () => {
    const funcNode = ts.factory.createFunctionDeclaration(
      undefined, undefined, "foo", undefined, [],
      undefined,
      ts.factory.createBlock([], true)
    );

    const direct = convertFunctionDeclaration(funcNode);
    const viaStmt = convertStatement(funcNode);

    expect(viaStmt).toStrictEqual(direct);
  });

  it("delegates VariableStatement and returns real IR", () => {
    const varDecl = ts.factory.createVariableDeclaration(
      "x", undefined, undefined, ts.factory.createNumericLiteral(1)
    );
    const varStmt = ts.factory.createVariableStatement(
      undefined,
      ts.factory.createVariableDeclarationList([varDecl], ts.NodeFlags.Const)
    );

    const direct = convertVariableStatement(varStmt);
    const viaStmt = convertStatement(varStmt);

    expect(viaStmt).toStrictEqual(direct);
  });

  it("delegates ReturnStatement", () => {
    const node = ts.factory.createReturnStatement(
      ts.factory.createStringLiteral("ok")
    );
    expect(convertStatement(node)).toStrictEqual(
      convertReturnStatement(node)
    );
  });

  it("delegates IfStatement", () => {
    const node = ts.factory.createIfStatement(
      ts.factory.createTrue(),
      ts.factory.createBlock([], true),
      undefined
    );
    expect(convertStatement(node)).toStrictEqual(
      convertIfStatement(node)
    );
  });

  it("delegates WhileStatement", () => {
    const node = ts.factory.createWhileStatement(
      ts.factory.createFalse(),
      ts.factory.createBlock([], true)
    );
    expect(convertStatement(node)).toStrictEqual(
      convertWhileKeyword(node)
    );
  });

  it("delegates ForStatement", () => {
    const node = ts.factory.createForStatement(
      undefined, undefined, undefined,
      ts.factory.createBlock([], true)
    );
    expect(convertStatement(node)).toStrictEqual(
      convertForStatement(node)
    );
  });

  it("delegates Block", () => {
    const blk = ts.factory.createBlock([], true);
    expect(convertStatement(blk)).toStrictEqual(convertBlock(blk));
  });

  it("SwitchStatement with break and continue", () => {
  const sw = ts.factory.createSwitchStatement(
    ts.factory.createIdentifier("x"),
    ts.factory.createCaseBlock([
      ts.factory.createCaseClause(ts.factory.createNumericLiteral(1), [
        ts.factory.createBreakStatement()
      ]),
      ts.factory.createCaseClause(ts.factory.createNumericLiteral(2), [
        ts.factory.createContinueStatement()    // ← new continue
      ]),
      ts.factory.createDefaultClause([
        ts.factory.createBreakStatement()
      ])
    ])
  );

  const ir = convertStatement(sw);

  expect(ir).toEqual<IRNode>({
    kind: "Switch",
    expression: convertExpression(ts.factory.createIdentifier("x")),
    cases: [
      {
        test: convertExpression(ts.factory.createNumericLiteral(1)),
        consequent: [{ kind: "BreakStatement" }]
      },
      {
        test: convertExpression(ts.factory.createNumericLiteral(2)),
        consequent: [{ kind: "ContinueStatement" }]   // ← asserted
      },
      {
        test: "default",
        consequent: [{ kind: "BreakStatement" }]
      }
    ]
  });
});
  it("SwitchStatement with break and continue (with label)", () => {
    const sw = ts.factory.createSwitchStatement(
      ts.factory.createIdentifier("x"),
      ts.factory.createCaseBlock([
        ts.factory.createCaseClause(ts.factory.createNumericLiteral(1), [
          ts.factory.createBreakStatement(ts.factory.createIdentifier("label"))
        ]),
        ts.factory.createCaseClause(ts.factory.createNumericLiteral(2), [
          ts.factory.createContinueStatement(ts.factory.createIdentifier("label"))    // ← new continue
        ]),
        ts.factory.createDefaultClause([
          ts.factory.createBreakStatement()
        ])
      ])
    );

    const ir = convertStatement(sw);

    expect(ir).toEqual<IRNode>({
      kind: "Switch",
      expression: convertExpression(ts.factory.createIdentifier("x")),
      cases: [
        {
          test: convertExpression(ts.factory.createNumericLiteral(1)),
          consequent: [{ kind: "BreakStatement", label: "label" }]
        },
        {
          test: convertExpression(ts.factory.createNumericLiteral(2)),
          consequent: [{ kind: "ContinueStatement", label: "label" }]   // ← asserted
        },
        {
          test: "default",
          consequent: [{ kind: "BreakStatement" }]
        }
      ]
    });
  });
});
