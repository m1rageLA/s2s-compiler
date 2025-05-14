import { describe, it, expect } from 'vitest';
import { convertExpression } from '../expressionConverter';
import { IRNode } from '../coreSchema';
import ts from 'typescript';

// export type IRNode =
//   | ProgramNode
//   | FunctionNode
//   | VariableDeclarationNode
//   | ReturnNode
//   | LiteralNode
//   | IdentifierNode
//   | BinaryNode
//   | IfNode
//   | WhileNode
//   | ForNode
//   | BlockNode
//   | ExpressionStatementNode
//   | CallExpressionNode
//   | AssignmentNode
//   | ObjectLiteralNode
//   | ArrayLiteralNode;

function parseExpr(code: string | number): ts.Expression {
  // Wrap the code so the first statement in the virtual file *is* our expression.
  const sf = ts.createSourceFile('tmp.ts', `${code};`, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS)
  const stmt = sf.statements[0]

  if (ts.isExpressionStatement(stmt)) {
    return stmt.expression;
  }
  throw new Error('Expected an ExpressionStatement');
}

describe("statement converters", () => {
  // it("TEST", () => {
  //   const testData =
  //     `j
  //   function x(x) {

  //   }
  //   `;

  //   const ir = convertExpression(parseExpr(testData));
  //   console.log("===============================");
  //   console.log(testData);
  //   console.log("===============================");
  //   console.log(ir);
  //   console.log("===============================");

  // })

  it("Shuld define NumericLiteral", () => {
    const ir = convertExpression(parseExpr(10));
    expect(ir).toEqual<IRNode>({ kind: 'Literal', value: 10 });
  }
  );

  it("Should define StringLiteral", () => {
    const ir = convertExpression(parseExpr('"hello"'));
    expect(ir).toEqual<IRNode>({ kind: 'Literal', value: "hello" });
  }
  );

  it("Should define TrueKeyword", () => {
    const ir = convertExpression(parseExpr("true"));
    expect(ir).toEqual<IRNode>({ kind: 'Literal', value: true });
  }
  );

  it("Should define FalseKeyword", () => {
    const ir = convertExpression(parseExpr("false"));
    expect(ir).toEqual<IRNode>({ kind: 'Literal', value: false });
  }
  );

  it("Should define NullKeyword", () => {
    const ir = convertExpression(parseExpr("null"));
    expect(ir).toEqual<IRNode>({ kind: 'Literal', value: null });
  }
  );

  it("Should define Identifier", () => {
    const ir = convertExpression(parseExpr("myVar"));
    expect(ir).toEqual<IRNode>({ kind: 'Identifier', name: "myVar" });
  }
  );

  it("Should define binaryExpressions", () => {
    const ir = convertExpression(parseExpr("a + b"));
    expect(ir).toEqual<IRNode>({
      kind: 'Binary',
      operator: '+',
      left: { kind: 'Identifier', name: 'a' },
      right: { kind: 'Identifier', name: 'b' }
    });

    const ir2 = convertExpression(parseExpr("a - b"));
    expect(ir2).toEqual<IRNode>({
      kind: 'Binary',
      operator: '-',
      left: { kind: 'Identifier', name: 'a' },
      right: { kind: 'Identifier', name: 'b' }
    });
    const ir3 = convertExpression(parseExpr("a * b"));
    expect(ir3).toEqual<IRNode>({
      kind: 'Binary',
      operator: '*',
      left: { kind: 'Identifier', name: 'a' },
      right: { kind: 'Identifier', name: 'b' }
    });
    const ir4 = convertExpression(parseExpr("a / b"));
    expect(ir4).toEqual<IRNode>({
      kind: 'Binary',
      operator: '/',
      left: { kind: 'Identifier', name: 'a' },
      right: { kind: 'Identifier', name: 'b' }
    });
    const ir5 = convertExpression(parseExpr("a % b"));
    expect(ir5).toEqual<IRNode>({
      kind: 'Binary',
      operator: '%',
      left: { kind: 'Identifier', name: 'a' },
      right: { kind: 'Identifier', name: 'b' }
    });
    const ir6 = convertExpression(parseExpr("a ** b"));
    expect(ir6).toEqual<IRNode>({
      kind: 'Binary',
      operator: '**',
      left: { kind: 'Identifier', name: 'a' },
      right: { kind: 'Identifier', name: 'b' }
    });
    const ir7 = convertExpression(parseExpr("a == b"));
    expect(ir7).toEqual<IRNode>({
      kind: 'Binary',
      operator: '==',
      left: { kind: 'Identifier', name: 'a' },
      right: { kind: 'Identifier', name: 'b' }
    });
    const ir8 = convertExpression(parseExpr("a === b"));
    expect(ir8).toEqual<IRNode>({
      kind: 'Binary',
      operator: '===',
      left: { kind: 'Identifier', name: 'a' },
      right: { kind: 'Identifier', name: 'b' }
    });
    const ir9 = convertExpression(parseExpr("a != b"));
    expect(ir9).toEqual<IRNode>({
      kind: 'Binary',
      operator: '!=',
      left: { kind: 'Identifier', name: 'a' },
      right: { kind: 'Identifier', name: 'b' }
    });
    const ir10 = convertExpression(parseExpr("a !== b"));
    expect(ir10).toEqual<IRNode>({
      kind: 'Binary',
      operator: '!==',
      left: { kind: 'Identifier', name: 'a' },
      right: { kind: 'Identifier', name: 'b' }
    });
    const ir11 = convertExpression(parseExpr("a < b"));
    expect(ir11).toEqual<IRNode>({
      kind: 'Binary',
      operator: '<',
      left: { kind: 'Identifier', name: 'a' },
      right: { kind: 'Identifier', name: 'b' }
    });
    const ir12 = convertExpression(parseExpr("a > b"));
    expect(ir12).toEqual<IRNode>({
      kind: 'Binary',
      operator: '>',
      left: { kind: 'Identifier', name: 'a' },
      right: { kind: 'Identifier', name: 'b' }
    });
    const ir13 = convertExpression(parseExpr("a <= b"));
    expect(ir13).toEqual<IRNode>({
      kind: 'Binary',
      operator: '<=',
      left: { kind: 'Identifier', name: 'a' },
      right: { kind: 'Identifier', name: 'b' }
    });
    const ir14 = convertExpression(parseExpr("a >= b"));
    expect(ir14).toEqual<IRNode>({
      kind: 'Binary',
      operator: '>=',
      left: { kind: 'Identifier', name: 'a' },
      right: { kind: 'Identifier', name: 'b' }
    });
    const ir15 = convertExpression(parseExpr("a && b"));
    expect(ir15).toEqual<IRNode>({
      kind: 'Binary',
      operator: '&&',
      left: { kind: 'Identifier', name: 'a' },
      right: { kind: 'Identifier', name: 'b' }
    });
    const ir16 = convertExpression(parseExpr("a || b"));
    expect(ir16).toEqual<IRNode>({
      kind: 'Binary',
      operator: '||',
      left: { kind: 'Identifier', name: 'a' },
      right: { kind: 'Identifier', name: 'b' }
    });
    const ir17 = convertExpression(parseExpr("a ?? b"));
    expect(ir17).toEqual<IRNode>({
      kind: 'Binary',
      operator: '??',
      left: { kind: 'Identifier', name: 'a' },
      right: { kind: 'Identifier', name: 'b' }
    });
    const ir18 = convertExpression(parseExpr("a & b"));
    expect(ir18).toEqual<IRNode>({
      kind: 'Binary',
      operator: '&',
      left: { kind: 'Identifier', name: 'a' },
      right: { kind: 'Identifier', name: 'b' }
    });
    const ir19 = convertExpression(parseExpr("a | b"));
    expect(ir19).toEqual<IRNode>({
      kind: 'Binary',
      operator: '|',
      left: { kind: 'Identifier', name: 'a' },
      right: { kind: 'Identifier', name: 'b' }
    });
  })
  it("Should define CallExpression", () => {
    const ir = convertExpression(parseExpr("myFunc(1, 2, 3)"));
    expect(ir).toEqual<IRNode>({
      kind: 'CallExpression',
      callee: { kind: 'Identifier', name: 'myFunc' },
      args: [
        { kind: 'Literal', value: 1 },
        { kind: 'Literal', value: 2 },
        { kind: 'Literal', value: 3 }
      ]
    });
    const ir2 = convertExpression(parseExpr("myFunc('a', myVar, false)"));
    expect(ir2).toEqual<IRNode>({
      kind: 'CallExpression',
      callee: { kind: 'Identifier', name: 'myFunc' },
      args: [
        { kind: 'Literal', value: 'a' },
        { kind: 'Identifier', name: 'myVar' },
        { kind: 'Literal', value: false }
      ]
    });
  })

  it("Should define ObjectLiteralExpression", () => {
    const ir = convertExpression(parseExpr("({ x: 1, y: 2 })"));
    expect(ir).toEqual<IRNode>({
      kind: 'ObjectLiteral',
      properties: [
        { key: 'x', value: { kind: 'Literal', value: 1 } },
        { key: 'y', value: { kind: 'Literal', value: 2 } }
      ]
    });
  });
  it("Should define ObjectLiteralExpression with shorthand", () => {
    const ir = convertExpression(parseExpr("({ x, y })"));
    expect(ir).toEqual<IRNode>({
      kind: 'ObjectLiteral',
      properties: [
        { key: 'x', value: { kind: 'Identifier', name: 'x' } },
        { key: 'y', value: { kind: 'Identifier', name: 'y' } }
      ]
    });
  }
  );
  it("Should define ObjectLiteralExpression with shorthand and normal", () => {
    const ir = convertExpression(parseExpr("({ x, y: 2 })"));
    expect(ir).toEqual<IRNode>({
      kind: 'ObjectLiteral',
      properties: [
        { key: 'x', value: { kind: 'Identifier', name: 'x' } },
        { key: 'y', value: { kind: 'Literal', value: 2 } }
      ]
    });
  }
  );
  it("Should define ArrayLiteralExpression", () => {
    const ir = convertExpression(parseExpr("[1, 2, 3]"));
    expect(ir).toEqual<IRNode>({
      kind: 'ArrayLiteral',
      elements: [
        { kind: 'Literal', value: 1 },
        { kind: 'Literal', value: 2 },
        { kind: 'Literal', value: 3 }
      ]
    });
  })
  it("PropertyAccess – obj.prop", () => {
    const expr = ts.factory.createPropertyAccessExpression(
      ts.factory.createIdentifier("obj"),
      ts.factory.createIdentifier("prop")
    );

    const ir = convertExpression(expr) as IRNode;
    expect(ir.kind).toBe("PropertyAccess");
    expect((ir as any).object).toEqual({ kind: "Identifier", name: "obj" });
    expect((ir as any).property).toEqual({ kind: "Identifier", name: "prop" });
  });

  it("ElementAccess – arr[i]", () => {
    const expr = ts.factory.createElementAccessExpression(
      ts.factory.createIdentifier("arr"),
      ts.factory.createIdentifier("i")
    );

    const ir = convertExpression(expr) as IRNode;
    expect(ir.kind).toBe("ElementAccess");
    expect((ir as any).object).toEqual({ kind: "Identifier", name: "arr" });
    expect((ir as any).index).toEqual({ kind: "Identifier", name: "i" });
  });
});
