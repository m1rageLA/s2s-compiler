// irSchema.ts
// Discriminated-union definitions for all IR nodes

export interface ProgramNode {
  kind: "Program";
  body: IRNode[];
}

export interface FunctionNode {
  kind: "Function";
  name: string;
  params: string[];
  body: IRNode[];
}

export interface VariableDeclarationNode {
  kind: "VariableDeclaration";
  name: string;
  value?: IRNode;
}

export interface ReturnNode {
  kind: "Return";
  value: IRNode;
}

export interface LiteralNode {
  kind: "Literal";
  value: string | number | boolean | null | undefined;
}

export interface IdentifierNode {
  kind: "Identifier";
  name: string;
}

export interface BinaryNode {
  kind: "Binary";
  operator: string; // + - * / % ** == === != !== < > <= >=
  left: IRNode;
  right: IRNode;
}

export interface IfNode {
  kind: "If";
  condition: IRNode;
  thenBlock: IRNode;
  elseBlock?: IRNode;
}

export interface WhileNode {
  kind: "While";
  condition: IRNode;
  body: IRNode[];
}

export interface ForNode {
  kind: "For";
  init?: IRNode;
  condition?: IRNode;
  increment?: IRNode;
  body: IRNode;
}

export interface BlockNode {
  kind: "Block";
  statements: IRNode[];
}

export interface ExpressionStatementNode {
  kind: "ExpressionStatement";
  expression: IRNode;
}

export interface CallExpressionNode {
  kind: "CallExpression";
  callee: IRNode;
  args: IRNode[];
}

export interface AssignmentNode {
  kind: "Assignment";
  left: IRNode,
  right: IRNode
}

export interface ObjectLiteralNode {
  kind: "ObjectLiteral",
  properties: { key: string, value: IRNode }[]
}

export interface ArrayLiteralNode {
  kind: "ArrayLiteral",
  elements: IRNode[]
}

export type IRNode =
  // Statements
  | ProgramNode
  | FunctionNode
  | VariableDeclarationNode
  | ReturnNode
  | IfNode
  | WhileNode
  | ForNode
  | BlockNode
  | ExpressionStatementNode

  // Expressions
  | LiteralNode
  | IdentifierNode
  | BinaryNode
  | CallExpressionNode
  | AssignmentNode
  | ObjectLiteralNode
  | ArrayLiteralNode;