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

function parseExpr(code: string): ts.Expression {
  // Wrap the code so the first statement in the virtual file *is* our expression.
  const sf = ts.createSourceFile('tmp.ts', `(${code});`, ts.ScriptTarget.Latest, true)
  const stmt = sf.statements[0]
  if (!ts.isExpressionStatement(stmt)) {
    throw new Error('Expected an ExpressionStatement')
  }
  return stmt.expression
}

describe("statement converters", () => {
    it("Should define convertFunctionDeclaration", () => {
        const ir = convertExpression(parseExpr("10"));
        expect(ir).toEqual<IRNode>({kind: 'Literal', value: 10});
    }); 
});