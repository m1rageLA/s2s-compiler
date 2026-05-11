import ts from "typescript";
import type {
  CompilerParamType,
  ExtractedFunction,
  FunctionLike,
  NormalizedParam,
} from "../api/types.js";
import { ENTRY_FUNCTION } from "./constants.js";

const SUPPORTED_PARAM_TYPES = new Set<CompilerParamType>(["number", "string", "boolean"]);

export function normalizeCompileArgument(
  argument: ts.Expression,
  sourceFile: ts.SourceFile,
  checker: ts.TypeChecker,
): ExtractedFunction {
  const unwrapped = unwrapParentheses(argument);

  if (isFunctionLike(unwrapped)) {
    return normalizeFunctionLike(unwrapped, sourceFile, checker);
  }

  if (ts.isIdentifier(unwrapped)) {
    const declaration = findFunctionBinding(sourceFile, unwrapped.text);
    if (!declaration) {
      throw new Error(`Cannot find TypeScript function binding "${unwrapped.text}"`);
    }
    return normalizeFunctionLike(declaration, sourceFile, checker);
  }

  throw new Error("compile(...) supports function declarations, function expressions, and typed arrows");
}

export function normalizeFunctionLike(
  node: FunctionLike,
  sourceFile: ts.SourceFile,
  checker: ts.TypeChecker,
): ExtractedFunction {
  const functionName = node.name?.text ?? null;
  const params = node.parameters.map((param, index) => normalizeParameter(param, index, sourceFile));
  const inferredReturnType = node.type ? null : inferReturnType(node, checker);
  const returnType = node.type
    ? `: ${node.type.getText(sourceFile)}`
    : inferredReturnType
      ? `: ${inferredReturnType}`
      : "";
  const body = renderBody(node, sourceFile);
  const aliases = collectTypeAliases(sourceFile);
  const aliasPrefix = aliases.length > 0 ? `${aliases.join("\n")}\n\n` : "";
  const paramList = params.map((param) => param.source).join(", ");
  const callArgs = params.map((param) => param.name).join(", ");
  const primaryName = functionName ?? ENTRY_FUNCTION;
  const primaryFunction = `function ${primaryName}(${paramList})${returnType} ${body}\n`;
  const wrapperFunction =
    functionName && functionName !== ENTRY_FUNCTION
      ? `function ${ENTRY_FUNCTION}(${paramList})${returnType} { return ${functionName}(${callArgs}); }\n`
      : "";
  const source = `${aliasPrefix}${primaryFunction}${wrapperFunction}`;

  return {
    source,
    signature: {
      params: params.map(({ name, type }) => ({ name, type })),
    },
  };
}

export function findCompileCall(
  sourceFile: ts.SourceFile,
  runtimeLine: number,
): ts.CallExpression | null {
  let result: ts.CallExpression | null = null;
  let nearestNode: ts.CallExpression | null = null;
  let nearestDistance = Number.POSITIVE_INFINITY;

  function visit(node: ts.Node): void {
    if (result) {
      return;
    }

    if (ts.isCallExpression(node) && isCompileCallee(node.expression)) {
      const start = sourceFile.getLineAndCharacterOfPosition(node.getStart(sourceFile)).line + 1;
      const end = sourceFile.getLineAndCharacterOfPosition(node.getEnd()).line + 1;
      if (start <= runtimeLine && runtimeLine <= end) {
        result = node;
        return;
      }
      const distance = runtimeLine < start ? start - runtimeLine : runtimeLine - end;
      if (distance < nearestDistance) {
        nearestNode = node;
        nearestDistance = distance;
      }
    }

    ts.forEachChild(node, visit);
  }

  visit(sourceFile);
  if (result) {
    return result;
  }
  return nearestDistance <= 3 ? nearestNode : null;
}

function inferReturnType(node: FunctionLike, checker: ts.TypeChecker): string | null {
  const signature = checker.getSignatureFromDeclaration(node);
  if (!signature) {
    return null;
  }

  const returnType = checker.typeToString(checker.getReturnTypeOfSignature(signature));
  if (returnType === "undefined") {
    return "void";
  }

  return ["number", "string", "boolean", "void"].includes(returnType) ? returnType : null;
}

function normalizeParameter(
  param: ts.ParameterDeclaration,
  index: number,
  sourceFile: ts.SourceFile,
): NormalizedParam {
  if (param.dotDotDotToken || param.questionToken || param.initializer) {
    throw new Error("compile(...) currently supports only required positional parameters");
  }
  if (!ts.isIdentifier(param.name)) {
    throw new Error("compile(...) currently supports only identifier parameters");
  }
  if (!param.type) {
    throw new Error(`Parameter "${param.name.text}" must have a TypeScript type annotation`);
  }

  const type = param.type.getText(sourceFile).trim();
  if (!isSupportedParamType(type)) {
    throw new Error(
      `Parameter "${param.name.text}" uses unsupported type "${type}". Supported: number, string, boolean`,
    );
  }

  return {
    name: param.name.text || `arg${index}`,
    type,
    source: param.getText(sourceFile),
  };
}

function renderBody(node: FunctionLike, sourceFile: ts.SourceFile): string {
  if (ts.isArrowFunction(node)) {
    if (ts.isBlock(node.body)) {
      return node.body.getText(sourceFile);
    }
    return `{ return ${node.body.getText(sourceFile)}; }`;
  }

  if (!node.body) {
    throw new Error("compile(...) cannot compile a function without a body");
  }

  return node.body.getText(sourceFile);
}

function collectTypeAliases(sourceFile: ts.SourceFile): string[] {
  return sourceFile.statements
    .filter(ts.isTypeAliasDeclaration)
    .map((statement) => statement.getText(sourceFile));
}

function findFunctionBinding(sourceFile: ts.SourceFile, name: string): FunctionLike | null {
  for (const statement of sourceFile.statements) {
    if (ts.isFunctionDeclaration(statement) && statement.name?.text === name) {
      return statement;
    }

    if (ts.isVariableStatement(statement)) {
      for (const declaration of statement.declarationList.declarations) {
        if (
          ts.isIdentifier(declaration.name) &&
          declaration.name.text === name &&
          declaration.initializer
        ) {
          const initializer = unwrapParentheses(declaration.initializer);
          if (isFunctionLike(initializer)) {
            return initializer;
          }
        }
      }
    }
  }

  return null;
}

function isCompileCallee(expression: ts.Expression): boolean {
  return (
    (ts.isIdentifier(expression) && expression.text === "compile") ||
    (ts.isPropertyAccessExpression(expression) && expression.name.text === "compile")
  );
}

function isFunctionLike(node: ts.Node): node is FunctionLike {
  return ts.isArrowFunction(node) || ts.isFunctionExpression(node) || ts.isFunctionDeclaration(node);
}

function unwrapParentheses(node: ts.Expression): ts.Expression {
  let current = node;
  while (ts.isParenthesizedExpression(current)) {
    current = current.expression;
  }
  return current;
}

function isSupportedParamType(type: string): type is CompilerParamType {
  return SUPPORTED_PARAM_TYPES.has(type as CompilerParamType);
}
