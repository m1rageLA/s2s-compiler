export type TsAstStatus = "implemented" | "missing-mvp" | "missing-later";
export type TsAstPriority = "mvp" | "backlog";

export type IrSpecField = {
  name: string;
  signature: string;
  note?: string;
};

export type IrSpec = {
  title?: string;
  summary?: string;
  signature?: string;
  fields?: IrSpecField[];
};

export type IrReference = {
  type: string;
  variant?: string;
  ideal?: IrSpec;
};

export type TsAstNode = {
  id: string;
  title: string;
  swcKind: string;
  description?: string;
  priority: TsAstPriority;
  ir?: IrReference[];
  children?: TsAstNode[];
};

export const TS_AST_NODES: TsAstNode[] = [
  {
    id: "program",
    title: "Program",
    swcKind: "Module",
    description: "Root node for a TypeScript file.",
    priority: "mvp",
    ir: [{ type: "IrModule" }],
    children: [
      {
        id: "declarations",
        title: "Declarations",
        swcKind: "Decl",
        description: "Top-level declarations inside a module.",
        priority: "mvp",
        children: [
          {
            id: "variable-declaration",
            title: "Variable Declaration",
            swcKind: "VarDecl",
            description: "let/const/var declarations.",
            priority: "mvp",
            ir: [
              { type: "IrItem", variant: "Variable" },
              { type: "IrVariable" },
            ],
          },
          {
            id: "function-declaration",
            title: "Function Declaration",
            swcKind: "FnDecl",
            description: "function foo() { ... }",
            priority: "mvp",
            ir: [
              { type: "IrItem", variant: "Function" },
              { type: "IrFunction" },
            ],
          },
          {
            id: "class-declaration",
            title: "Class Declaration",
            swcKind: "ClassDecl",
            description: "class Foo { ... }",
            priority: "mvp",
          },
          {
            id: "interface-declaration",
            title: "Interface Declaration",
            swcKind: "TsInterfaceDecl",
            description: "interface Foo { ... }",
            priority: "backlog",
          },
          {
            id: "enum-declaration",
            title: "Enum Declaration",
            swcKind: "TsEnumDecl",
            description: "enum Foo { A, B }",
            priority: "backlog",
          },
          {
            id: "type-alias",
            title: "Type Alias",
            swcKind: "TsTypeAliasDecl",
            description: "type Foo = ...",
            priority: "backlog",
          },
          {
            id: "import-declaration",
            title: "Import Declaration",
            swcKind: "ImportDecl",
            description: "import { Foo } from 'mod';",
            priority: "backlog",
          },
          {
            id: "export-declaration",
            title: "Export Declaration",
            swcKind: "ExportDecl",
            description: "export const Foo = ...;",
            priority: "backlog",
          },
        ],
      },
      {
        id: "statements",
        title: "Statements",
        swcKind: "Stmt",
        description: "Executable constructs inside blocks and functions.",
        priority: "mvp",
        children: [
      {
        id: "block-statement",
        title: "Block Statement",
        swcKind: "BlockStmt",
        description: "{ ... }",
        priority: "mvp",
        ir: [
          {
            type: "IrStmt",
            variant: "Block",
            ideal: {
              summary: "Block should carry the lowered statements of its scope.",
              fields: [
                { name: "0", signature: "Vec<IrStmt>", note: "Statements inside the block." },
              ],
            },
          },
        ],
      },
          {
            id: "expression-statement",
            title: "Expression Statement",
            swcKind: "ExprStmt",
            description: "Expression used as a statement.",
            priority: "mvp",
            ir: [
              { type: "IrItem", variant: "Expression" },
              { type: "IrExpression" },
            ],
          },
          {
            id: "return-statement",
            title: "Return Statement",
            swcKind: "ReturnStmt",
            description: "return value;",
            priority: "mvp",
            ir: [{ type: "IrStmt", variant: "Return" }],
          },
          {
            id: "variable-statement",
            title: "Variable Statement",
            swcKind: "VarDecl",
            description: "let/const inside a block.",
            priority: "mvp",
            ir: [{ type: "IrStmt", variant: "Leteral" }],
          },
          {
            id: "if-statement",
            title: "If Statement",
            swcKind: "IfStmt",
            description: "if (cond) { ... } else { ... }",
            priority: "mvp",
          },
          {
            id: "for-statement",
            title: "For Statement",
            swcKind: "ForStmt",
            description: "Classic for (init; test; update) loop.",
            priority: "mvp",
          },
          {
            id: "for-of-statement",
            title: "For Of Statement",
            swcKind: "ForOfStmt",
            description: "for (const item of iterable) { ... }",
            priority: "backlog",
          },
          {
            id: "while-statement",
            title: "While Statement",
            swcKind: "WhileStmt",
            description: "while (condition) { ... }",
            priority: "mvp",
          },
          {
            id: "do-while-statement",
            title: "Do While Statement",
            swcKind: "DoWhileStmt",
            description: "do { ... } while (condition);",
            priority: "backlog",
          },
          {
            id: "switch-statement",
            title: "Switch Statement",
            swcKind: "SwitchStmt",
            description: "switch (value) { case ... }",
            priority: "backlog",
          },
          {
            id: "break-statement",
            title: "Break Statement",
            swcKind: "BreakStmt",
            description: "break;",
            priority: "backlog",
          },
          {
            id: "continue-statement",
            title: "Continue Statement",
            swcKind: "ContinueStmt",
            description: "continue;",
            priority: "backlog",
          },
          {
            id: "throw-statement",
            title: "Throw Statement",
            swcKind: "ThrowStmt",
            description: "throw error;",
            priority: "backlog",
          },
          {
            id: "try-statement",
            title: "Try Statement",
            swcKind: "TryStmt",
            description: "try { ... } catch (e) { ... }",
            priority: "backlog",
          },
        ],
      },
      {
        id: "expressions",
        title: "Expressions",
        swcKind: "Expr",
        description: "Expression forms inside statements and other expressions.",
        priority: "mvp",
        children: [
          {
            id: "identifier",
            title: "Identifier",
            swcKind: "Ident",
            description: "Reference to a binding.",
            priority: "mvp",
            ir: [{ type: "IrExpression", variant: "Identifier" }],
          },
          {
            id: "literal",
            title: "Literal",
            swcKind: "Lit",
            description: "Numeric, string, boolean, null, undefined literals.",
            priority: "mvp",
            ir: [
              { type: "IrExpression", variant: "Literal" },
              { type: "IrLiteral" },
            ],
          },
          {
            id: "binary-expression",
            title: "Binary Expression",
            swcKind: "BinExpr",
            description: "Arithmetic and comparison operations.",
            priority: "mvp",
            ir: [
              { type: "IrExpression", variant: "Binary" },
              { type: "IrBinOp" },
            ],
          },
          {
            id: "logical-expression",
            title: "Logical Expression",
            swcKind: "BinExpr",
            description: "Logical operators (&&, ||, ??).",
            priority: "mvp",
            ir: [
              { type: "IrExpression", variant: "Binary" },
              { type: "IrBinOp" },
            ],
          },
          {
            id: "assignment-expression",
            title: "Assignment Expression",
            swcKind: "AssignExpr",
            description: "x = value; x += value;",
            priority: "mvp",
          },
          {
            id: "unary-expression",
            title: "Unary Expression",
            swcKind: "UnaryExpr",
            description: "Unary operators (!value, typeof value).",
            priority: "mvp",
          },
          {
            id: "update-expression",
            title: "Update Expression",
            swcKind: "UpdateExpr",
            description: "++i, i--.",
            priority: "backlog",
          },
          {
            id: "call-expression",
            title: "Call Expression",
            swcKind: "CallExpr",
            description: "foo(bar).",
            priority: "mvp",
            ir: [{ type: "IrExpression", variant: "Call" }],
          },
          {
            id: "new-expression",
            title: "New Expression",
            swcKind: "NewExpr",
            description: "new Foo(bar).",
            priority: "backlog",
          },
          {
            id: "member-expression",
            title: "Member Expression",
            swcKind: "MemberExpr",
            description: "obj.prop or obj[expr].",
            priority: "mvp",
          },
          {
            id: "optional-chain-expression",
            title: "Optional Chain",
            swcKind: "OptChainExpr",
            description: "obj?.prop.",
            priority: "backlog",
          },
          {
            id: "conditional-expression",
            title: "Conditional Expression",
            swcKind: "CondExpr",
            description: "cond ? a : b",
            priority: "mvp",
          },
          {
            id: "array-literal",
            title: "Array Literal",
            swcKind: "ArrayLit",
            description: "[a, b, c]",
            priority: "backlog",
            ir: [{ type: "IrExpression", variant: "Array" }],
          },
          {
            id: "object-literal",
            title: "Object Literal",
            swcKind: "ObjectLit",
            description: "{ key: value }",
            priority: "mvp",
          },
          {
            id: "template-literal",
            title: "Template Literal",
            swcKind: "Tpl",
            description: "`hello ${name}`",
            priority: "backlog",
          },
          {
            id: "arrow-function",
            title: "Arrow Function",
            swcKind: "ArrowExpr",
            description: "(args) => body",
            priority: "mvp",
          },
          {
            id: "function-expression",
            title: "Function Expression",
            swcKind: "FnExpr",
            description: "const foo = function() {}",
            priority: "backlog",
          },
          {
            id: "await-expression",
            title: "Await Expression",
            swcKind: "AwaitExpr",
            description: "await promise",
            priority: "backlog",
          },
          {
            id: "yield-expression",
            title: "Yield Expression",
            swcKind: "YieldExpr",
            description: "yield value",
            priority: "backlog",
          },
        ],
      },
      {
        id: "types",
        title: "Type Annotations",
        swcKind: "TsType",
        description: "Type system constructs specific to TypeScript.",
        priority: "mvp",
        children: [
          {
            id: "type-primitive",
            title: "Primitive Keywords",
            swcKind: "TsKeywordType",
            description: "number, string, boolean.",
            priority: "mvp",
            ir: [{ type: "IrType" }],
          },
          {
            id: "type-reference",
            title: "Type Reference",
            swcKind: "TsTypeRef",
            description: "Identifiers or qualified names in types.",
            priority: "mvp",
          },
          {
            id: "type-union",
            title: "Union Type",
            swcKind: "TsUnionType",
            description: "type A = B | C;",
            priority: "mvp",
          },
          {
            id: "type-intersection",
            title: "Intersection Type",
            swcKind: "TsIntersectionType",
            description: "type A = B & C;",
            priority: "backlog",
          },
          {
            id: "type-literal",
            title: "Type Literal",
            swcKind: "TsTypeLit",
            description: "Inline object type definitions.",
            priority: "backlog",
          },
          {
            id: "type-function",
            title: "Function Type",
            swcKind: "TsFnType",
            description: "(args) => Return",
            priority: "backlog",
          },
          {
            id: "type-tuple",
            title: "Tuple Type",
            swcKind: "TsTupleType",
            description: "[string, number]",
            priority: "backlog",
          },
          {
            id: "type-literal-boolean",
            title: "Boolean Literal Type",
            swcKind: "TsLitType",
            description: "type Flag = true",
            priority: "backlog",
          },
          {
            id: "type-literal-number",
            title: "Numeric Literal Type",
            swcKind: "TsLitType",
            description: "type Size = 42",
            priority: "backlog",
          },
          {
            id: "type-optional",
            title: "Optional Type",
            swcKind: "TsOptionalType",
            description: "Optional modifier in types.",
            priority: "backlog",
          },
          {
            id: "type-rest",
            title: "Rest Type",
            swcKind: "TsRestType",
            description: "Rest element in tuple types.",
            priority: "backlog",
          },
        ],
      },
      {
        id: "misc",
        title: "Miscellaneous",
        swcKind: "Misc",
        description: "Additional structures tracked for completeness.",
        priority: "backlog",
        children: [
          {
            id: "decorator",
            title: "Decorator",
            swcKind: "Decorator",
            description: "@decorator",
            priority: "backlog",
          },
          {
            id: "comment",
            title: "Comment",
            swcKind: "Comment",
            description: "Line and block comments.",
            priority: "backlog",
          },
        ],
      },
    ],
  },
];
