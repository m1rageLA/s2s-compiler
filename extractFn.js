import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import ts from "typescript";

export const ENTRY_FUNCTION = "__ts2rust_entry";

const COMPILER_ROOT = path.dirname(fileURLToPath(import.meta.url));
const SUPPORTED_PARAM_TYPES = new Set(["number", "string", "boolean"]);

export function extractFunctionWithTypes(_fn, options = {}) {
  const callsite = options.callsite ?? findUserCallsite();
  const sourcePath = resolveTypeScriptSource(callsite.file);
  const { sourceFile, checker } = loadTypeScriptProgram(sourcePath);
  const call = findCompileCall(sourceFile, callsite.line);

  if (!call) {
    throw new Error(`Cannot find compile(...) call in ${sourcePath}:${callsite.line}`);
  }

  const [argument] = call.arguments;
  if (!argument) {
    throw new Error(`compile(...) requires a function argument at ${sourcePath}:${callsite.line}`);
  }

  return normalizeCompileArgument(argument, sourceFile, checker);
}

export function normalizeSourceFunction(source, options = {}) {
  const { sourceFile, checker } = loadInlineTypeScriptProgram(
    source,
    options.fileName ?? "inline.ts",
  );
  const declaration = sourceFile.statements.find(ts.isFunctionDeclaration);

  if (!declaration) {
    throw new Error("compileSource(...) expects a TypeScript function declaration");
  }

  return normalizeFunctionLike(declaration, sourceFile, checker);
}

function normalizeCompileArgument(argument, sourceFile, checker) {
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

function normalizeFunctionLike(node, sourceFile, checker) {
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

function loadTypeScriptProgram(sourcePath) {
  const compilerOptions = {
    target: ts.ScriptTarget.ES2022,
    module: ts.ModuleKind.NodeNext,
    moduleResolution: ts.ModuleResolutionKind.NodeNext,
    strict: true,
    skipLibCheck: true,
    noEmit: true,
  };
  const program = ts.createProgram([sourcePath], compilerOptions);
  const sourceFile = program.getSourceFile(sourcePath);
  if (!sourceFile) {
    throw new Error(`Cannot load TypeScript source ${sourcePath}`);
  }

  return {
    sourceFile,
    checker: program.getTypeChecker(),
  };
}

function loadInlineTypeScriptProgram(source, fileName) {
  const compilerOptions = {
    target: ts.ScriptTarget.ES2022,
    module: ts.ModuleKind.NodeNext,
    moduleResolution: ts.ModuleResolutionKind.NodeNext,
    strict: true,
    skipLibCheck: true,
    noEmit: true,
  };
  const sourcePath = path.resolve(fileName);
  const host = ts.createCompilerHost(compilerOptions);
  const getSourceFile = host.getSourceFile.bind(host);

  host.getSourceFile = (requestedFile, languageVersion, onError, shouldCreateNewSourceFile) => {
    if (path.resolve(requestedFile) === sourcePath) {
      return ts.createSourceFile(requestedFile, source, languageVersion, true, ts.ScriptKind.TS);
    }
    return getSourceFile(requestedFile, languageVersion, onError, shouldCreateNewSourceFile);
  };
  host.readFile = (requestedFile) =>
    path.resolve(requestedFile) === sourcePath ? source : ts.sys.readFile(requestedFile);
  host.fileExists = (requestedFile) =>
    path.resolve(requestedFile) === sourcePath || ts.sys.fileExists(requestedFile);

  const program = ts.createProgram([sourcePath], compilerOptions, host);
  const sourceFile = program.getSourceFile(sourcePath);
  if (!sourceFile) {
    throw new Error("Cannot load inline TypeScript source");
  }

  return {
    sourceFile,
    checker: program.getTypeChecker(),
  };
}

function inferReturnType(node, checker) {
  if (!checker) {
    return null;
  }

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

function normalizeParameter(param, index, sourceFile) {
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
  if (!SUPPORTED_PARAM_TYPES.has(type)) {
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

function renderBody(node, sourceFile) {
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

function collectTypeAliases(sourceFile) {
  return sourceFile.statements
    .filter(ts.isTypeAliasDeclaration)
    .map((statement) => statement.getText(sourceFile));
}

function findFunctionBinding(sourceFile, name) {
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

function findCompileCall(sourceFile, runtimeLine) {
  let result = null;
  let nearest = null;

  function visit(node) {
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
      if (!nearest || distance < nearest.distance) {
        nearest = { node, distance };
      }
    }

    ts.forEachChild(node, visit);
  }

  visit(sourceFile);
  return result ?? (nearest?.distance <= 3 ? nearest.node : null);
}

function isCompileCallee(expression) {
  return (
    (ts.isIdentifier(expression) && expression.text === "compile") ||
    (ts.isPropertyAccessExpression(expression) && expression.name.text === "compile")
  );
}

function isFunctionLike(node) {
  return ts.isArrowFunction(node) || ts.isFunctionExpression(node) || ts.isFunctionDeclaration(node);
}

function unwrapParentheses(node) {
  let current = node;
  while (ts.isParenthesizedExpression(current)) {
    current = current.expression;
  }
  return current;
}

function findUserCallsite() {
  const stack = new Error().stack?.split("\n").slice(1) ?? [];

  for (const line of stack) {
    const frame = parseStackLine(line);
    if (!frame) {
      continue;
    }
    if (frame.file.startsWith("node:") || frame.file.includes(`${path.sep}node_modules${path.sep}`)) {
      continue;
    }
    if (!frame.file.startsWith(COMPILER_ROOT)) {
      return frame;
    }
  }

  throw new Error("Cannot determine compile(...) callsite");
}

function parseStackLine(line) {
  const match = line.match(/\(?((?:file:\/\/)?\/.*?):(\d+):(\d+)\)?$/);
  if (!match) {
    return null;
  }

  const file = match[1].startsWith("file://") ? fileURLToPath(match[1]) : match[1];
  return {
    file,
    line: Number(match[2]),
    column: Number(match[3]),
  };
}

function resolveTypeScriptSource(runtimeFile) {
  if (runtimeFile.endsWith(".ts") || runtimeFile.endsWith(".tsx")) {
    return runtimeFile;
  }

  const parsed = path.parse(runtimeFile);
  const candidates = [
    path.join(parsed.dir, `${parsed.name}.ts`),
    path.join(parsed.dir, `${parsed.name}.tsx`),
  ];

  const source = candidates.find((candidate) => fs.existsSync(candidate));
  if (!source) {
    throw new Error(
      `Cannot find TypeScript source for ${runtimeFile}. Compile from a .ts file or keep the .ts next to emitted JS.`,
    );
  }

  return source;
}
