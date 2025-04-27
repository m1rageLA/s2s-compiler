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
  const sf = ts.createSourceFile('tmp.ts', `${code};`, ts.ScriptTarget.Latest, true)
  const stmt = sf.statements[0]

  if (!ts.isExpressionStatement(stmt)) {
    throw new Error('Expected an ExpressionStatement')
  }

  return stmt.expression
}

describe("statement converters", () => {
  // it("TEST", () => {
  //   const testData =
  //     `
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

  it("Shuld define convertNumericLiteral", () => {
    const ir = convertExpression(parseExpr(10));
    expect(ir).toEqual<IRNode>({ kind: 'Literal', value: 10 });
  }
  );

  it("Should define convertStringLiteral", () => {
    const ir = convertExpression(parseExpr('"hello"'));
    expect(ir).toEqual<IRNode>({ kind: 'Literal', value: "hello" });
  }
  );

  it("Should define convertTrueKeyword", () => {
    const ir = convertExpression(parseExpr("true"));
    expect(ir).toEqual<IRNode>({ kind: 'Literal', value: true });
  }
  );

  it("Should define convertFalseKeyword", () => {
    const ir = convertExpression(parseExpr("false"));
    expect(ir).toEqual<IRNode>({ kind: 'Literal', value: false });
  }
  );

});