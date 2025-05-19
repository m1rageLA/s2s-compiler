import * as ts from 'typescript';
import {
  IRNode,
  FunctionNode,
  VariableDeclarationNode,
  ReturnNode,
  IfNode,
  WhileNode,
  ForNode,
  BlockNode,
  ExpressionStatementNode,
} from './coreSchema';
import { convertExpression } from './expressionConverter';
import { convertStatement } from './statementConverters';     // если этот файл лежит отдельно

/* ────────────────────────────  helpers  ──────────────────────────── */

function singleVarDeclToIR(decl: ts.VariableDeclaration): VariableDeclarationNode {
  if (!ts.isIdentifier(decl.name)) {
    throw new Error(`Unsupported variable pattern: ${decl.name.getText()}`);
  }
  return {
    kind: 'VariableDeclaration',
    name: decl.name.text,
    value: decl.initializer ? convertExpression(decl.initializer) : undefined,
  };
}

function convertBlockLike(stmt: ts.Statement): BlockNode {
  return ts.isBlock(stmt)
    ? {
        kind: 'Block',
        statements: stmt.statements.map(convertStatement),
      }
    : {
        kind: 'Block',
        statements: [convertStatement(stmt)],
      };
}

/* ────────────────────────────  top-level converters  ──────────────────────────── */

export function convertFunctionDeclaration(node: ts.FunctionDeclaration): FunctionNode {
  const name = node.name?.text ?? 'anonymous';
  const params = node.parameters.map(p => {
    if (ts.isIdentifier(p.name)) return p.name.text;
    throw new Error(`Unsupported parameter pattern: ${p.name.getText()}`);
  });
  const bodyStmts = node.body ? node.body.statements.map(convertStatement) : [];
  return { kind: 'Function', name, params, body: bodyStmts };
}

export function convertVariableStatement(
  node: ts.VariableStatement | ts.VariableDeclarationList
): IRNode {
  const declList = ts.isVariableStatement(node) ? node.declarationList : node;
  const decls = declList.declarations;
  if (decls.length === 1) {
    return singleVarDeclToIR(decls[0]);
  }
  return {
    kind: 'Block',
    statements: decls.map(singleVarDeclToIR),
  };
}

export function convertReturnStatement(node: ts.ReturnStatement): ReturnNode {
  if (!node.expression) throw new Error('Return without expression is unsupported');
  return { kind: 'Return', value: convertExpression(node.expression) };
}

export function convertIfStatement(node: ts.IfStatement): IfNode {
  return {
    kind: 'If',
    condition: convertExpression(node.expression),
    thenBlock: convertBlockLike(node.thenStatement),
    elseBlock: node.elseStatement ? convertBlockLike(node.elseStatement) : undefined,
  };
}

export function convertWhileStatement(node: ts.WhileStatement): WhileNode {
  return {
    kind: 'While',
    condition: convertExpression(node.expression),
    body: node.statement ? [convertStatement(node.statement)] : [],
  };
}

export function convertForStatement(node: ts.ForStatement): ForNode {
  let init: IRNode | undefined;
  if (node.initializer) {
    if (ts.isVariableDeclarationList(node.initializer)) {
      init = convertVariableStatement(node.initializer);
    } else {
      init = {
        kind: 'ExpressionStatement',
        expression: convertExpression(node.initializer),
      } as ExpressionStatementNode;
    }
  }

  return {
    kind: 'For',
    init,
    condition: node.condition ? convertExpression(node.condition) : undefined,
    increment: node.incrementor ? convertExpression(node.incrementor) : undefined,
    body: convertBlockLike(node.statement),
  };
}

export function convertBlock(node: ts.Block): BlockNode {
  return {
    kind: 'Block',
    statements: node.statements.map(convertStatement),
  };
}
