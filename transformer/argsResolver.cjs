const ts = require("typescript");
const vm = require("node:vm");
const path = require("node:path");

function unwrapExpression(node) {
  let current = node;
  while (
    ts.isParenthesizedExpression(current) ||
    ts.isAsExpression(current) ||
    ts.isTypeAssertionExpression(current) ||
    ts.isNonNullExpression(current)
  ) {
    current = current.expression;
  }
  return current;
}

function findInitializerInFile(identifier) {
  const sf = identifier.getSourceFile();
  let initializer = null;

  const visit = (node) => {
    if (initializer) return;

    if (
      ts.isVariableDeclaration(node) &&
      ts.isIdentifier(node.name) &&
      node.name.text === identifier.text &&
      node.initializer
    ) {
      initializer = node.initializer;
      return;
    }

    ts.forEachChild(node, visit);
  };

  visit(sf);
  return initializer;
}

function resolveArgExpression(node, checker, seen = new Set()) {
  const target = unwrapExpression(node);

  if (!ts.isIdentifier(target)) {
    return target;
  }

  if (!checker) {
    throw new Error(
      "A TypeScript program is required to resolve identifiers in heavy() args"
    );
  }
  const symbol =
    (checker.getShorthandAssignmentValueSymbol &&
      checker.getShorthandAssignmentValueSymbol(target)) ||
    checker.getSymbolAtLocation(target);
  let resolvedSymbol =
    symbol && symbol.flags && checker && ts.SymbolFlags.Alias
      ? symbol.flags & ts.SymbolFlags.Alias
        ? checker.getAliasedSymbol(symbol)
        : symbol
      : symbol;
  let decl =
    resolvedSymbol &&
    (resolvedSymbol.valueDeclaration ||
      (resolvedSymbol.declarations && resolvedSymbol.declarations[0]));

  if (decl && ts.isShorthandPropertyAssignment(decl)) {
    const valueSymbol =
      checker.getShorthandAssignmentValueSymbol &&
      checker.getShorthandAssignmentValueSymbol(decl);
    if (valueSymbol) {
      resolvedSymbol = valueSymbol;
      decl =
        valueSymbol.valueDeclaration ||
        (valueSymbol.declarations && valueSymbol.declarations[0]);
    }
  }

  if (!resolvedSymbol || !decl) {
    const fallbackInit = findInitializerInFile(target);
    if (fallbackInit) {
      return resolveArgExpression(fallbackInit, checker, seen);
    }
    throw new Error(`Unsupported identifier "${target.text}" in heavy() args`);
  }
  if (seen.has(resolvedSymbol)) {
    throw new Error(`Circular reference in heavy() args at "${target.text}"`);
  }

  seen.add(resolvedSymbol);
  if (ts.isVariableDeclaration(decl) && decl.initializer) {
    return resolveArgExpression(decl.initializer, checker, seen);
  }
  if (ts.isParameter(decl) && decl.initializer) {
    return resolveArgExpression(decl.initializer, checker, seen);
  }

  throw new Error(`Unsupported identifier "${target.text}" in heavy() args`);
}

function resolveArgLiteral(node, checker, seen = new Set()) {
  const target = unwrapExpression(node);

  if (ts.isIdentifier(target)) {
    if (!checker) {
      throw new Error(
        "A TypeScript program is required to resolve identifiers in heavy() args"
      );
    }
    const symbol =
      (checker.getShorthandAssignmentValueSymbol &&
        checker.getShorthandAssignmentValueSymbol(target)) ||
      checker.getSymbolAtLocation(target);
    let resolvedSymbol =
      symbol && symbol.flags && checker && ts.SymbolFlags.Alias
        ? (symbol.flags & ts.SymbolFlags.Alias
            ? checker.getAliasedSymbol(symbol)
            : symbol)
        : symbol;
    let decl =
      resolvedSymbol &&
      (resolvedSymbol.valueDeclaration ||
        (resolvedSymbol.declarations && resolvedSymbol.declarations[0]));

    if (decl && ts.isShorthandPropertyAssignment(decl)) {
      const valueSymbol =
        checker.getShorthandAssignmentValueSymbol &&
        checker.getShorthandAssignmentValueSymbol(decl);
      if (valueSymbol) {
        resolvedSymbol = valueSymbol;
        decl =
          valueSymbol.valueDeclaration ||
          (valueSymbol.declarations && valueSymbol.declarations[0]);
      }
    }

    if (!resolvedSymbol || !decl) {
      const fallbackInit = findInitializerInFile(target);
      if (fallbackInit) {
        return resolveArgLiteral(fallbackInit, checker, seen);
      }
      throw new Error(`Unsupported identifier "${target.text}" in heavy() args`);
    }
    if (seen.has(resolvedSymbol)) {
      throw new Error(`Circular reference in heavy() args at "${target.text}"`);
    }

    seen.add(resolvedSymbol);
    if (ts.isVariableDeclaration(decl) && decl.initializer) {
      return resolveArgLiteral(decl.initializer, checker, seen);
    }
    if (ts.isParameter(decl) && decl.initializer) {
      return resolveArgLiteral(decl.initializer, checker, seen);
    }

    throw new Error(`Unsupported identifier "${target.text}" in heavy() args`);
  }

  if (ts.isNumericLiteral(target)) {
    return ts.factory.createNumericLiteral(target.text);
  }
  if (ts.isStringLiteral(target)) {
    return ts.factory.createStringLiteral(target.text);
  }
  if (target.kind === ts.SyntaxKind.TrueKeyword) {
    return ts.factory.createTrue();
  }
  if (target.kind === ts.SyntaxKind.FalseKeyword) {
    return ts.factory.createFalse();
  }
  if (target.kind === ts.SyntaxKind.NullKeyword) {
    return ts.factory.createNull();
  }

  if (ts.isPrefixUnaryExpression(target)) {
    if (
      target.operator === ts.SyntaxKind.MinusToken ||
      target.operator === ts.SyntaxKind.PlusToken
    ) {
      return ts.factory.createPrefixUnaryExpression(
        target.operator,
        resolveArgLiteral(target.operand, checker, seen)
      );
    }
  }

  if (ts.isArrayLiteralExpression(target)) {
    if (target.elements.some((el) => ts.isOmittedExpression(el))) {
      throw new Error("Unsupported array hole in heavy() arguments");
    }
    const elements = target.elements.map((el) =>
      resolveArgLiteral(el, checker, seen)
    );
    return ts.factory.createArrayLiteralExpression(elements, true);
  }

  if (ts.isObjectLiteralExpression(target)) {
    const props = target.properties.map((prop) => {
      if (ts.isShorthandPropertyAssignment(prop)) {
        return ts.factory.createPropertyAssignment(
          ts.factory.createIdentifier(prop.name.text),
          resolveArgLiteral(prop.name, checker, seen)
        );
      }

      if (!ts.isPropertyAssignment(prop) || !ts.isIdentifier(prop.name)) {
        throw new Error(
          "Only identifier property assignments are supported in heavy() args"
        );
      }
      return ts.factory.createPropertyAssignment(
        ts.factory.createIdentifier(prop.name.text),
        resolveArgLiteral(prop.initializer, checker, seen)
      );
    });
    return ts.factory.createObjectLiteralExpression(props, true);
  }

  throw new Error(`Unsupported argument type in heavy() --> ${target.getText()}`);
}

function parseArgsMap(arg) {
  if (!arg) return new Map();

  if (!ts.isObjectLiteralExpression(arg)) {
    throw new Error(
      "heavy() args must be an object literal or heavy.prepareArgs(...)"
    );
  }

  const map = new Map();
  for (const prop of arg.properties) {
    if (ts.isShorthandPropertyAssignment(prop)) {
      map.set(prop.name.text, prop.name);
      continue;
    }

    if (ts.isPropertyAssignment(prop) && ts.isIdentifier(prop.name)) {
      map.set(prop.name.text, prop.initializer);
      continue;
    }

    throw new Error(
      "Only identifier property assignments are supported in heavy() args"
    );
  }

  return map;
}

function literalFromValue(value, pathLabel = "args") {
  if (value === null) {
    return ts.factory.createNull();
  }

  const valueType = typeof value;
  if (valueType === "number") {
    return ts.factory.createNumericLiteral(value);
  }
  if (valueType === "string") {
    return ts.factory.createStringLiteral(value);
  }
  if (valueType === "boolean") {
    return value ? ts.factory.createTrue() : ts.factory.createFalse();
  }

  if (Array.isArray(value)) {
    const elements = value.map((el, idx) => {
      if (el === undefined) {
        throw new Error(
          `Unsupported undefined at ${pathLabel}[${idx}] in heavy.prepareArgs result`
        );
      }
      return literalFromValue(el, `${pathLabel}[${idx}]`);
    });
    return ts.factory.createArrayLiteralExpression(elements, true);
  }

  if (valueType === "object") {
    const props = Object.entries(value).map(([name, val]) => {
      if (
        name === "__proto__" ||
        name === "constructor" ||
        name === "prototype"
      ) {
        throw new Error(
          `Forbidden property name "${name}" in heavy.prepareArgs result`
        );
      }
      if (!ts.isIdentifierText(name, ts.ScriptTarget.Latest)) {
        throw new Error(
          `heavy.prepareArgs keys must be valid identifiers. Got "${name}" at ${pathLabel}`
        );
      }
      if (val === undefined) {
        throw new Error(
          `Unsupported undefined at ${pathLabel}.${name} in heavy.prepareArgs result`
        );
      }
      return ts.factory.createPropertyAssignment(
        ts.factory.createIdentifier(name),
        literalFromValue(val, `${pathLabel}.${name}`)
      );
    });

    return ts.factory.createObjectLiteralExpression(props, true);
  }

  throw new Error(
    `Unsupported value of type "${valueType}" in heavy.prepareArgs result at ${pathLabel}`
  );
}

function evalPreparedArgs(callExpr, sf) {
  if (callExpr.arguments.length !== 1) {
    throw new Error("heavy.prepareArgs expects a single function argument");
  }

  const fnNode = unwrapExpression(callExpr.arguments[0]);
  if (!ts.isArrowFunction(fnNode) && !ts.isFunctionExpression(fnNode)) {
    throw new Error("heavy.prepareArgs requires an arrow or function expression");
  }

  const transpiled = ts.transpileModule(`(${fnNode.getText(sf)})`, {
    compilerOptions: {
      module: ts.ModuleKind.CommonJS,
      target: ts.ScriptTarget.ES2019,
    },
    fileName: sf.fileName,
  });

  let factoryFn;
  try {
    factoryFn = vm.runInNewContext(
      transpiled.outputText,
      {
        require,
        console,
        process,
        Buffer,
        __dirname: path.dirname(sf.fileName),
        __filename: sf.fileName,
        exports: {},
        module: { exports: {} },
      },
      { filename: sf.fileName }
    );
  } catch (err) {
    throw new Error(
      `Failed to evaluate heavy.prepareArgs callback: ${err.message}`
    );
  }

  if (typeof factoryFn !== "function") {
    throw new Error(
      "heavy.prepareArgs expects its argument to evaluate to a function"
    );
  }

  let value;
  try {
    value = factoryFn();
  } catch (err) {
    throw new Error(
      `Error while executing heavy.prepareArgs callback: ${err.message}`
    );
  }

  if (value === null || typeof value !== "object" || Array.isArray(value)) {
    throw new Error("heavy.prepareArgs must return an object with primitive fields");
  }

  const literal = literalFromValue(value);
  if (!ts.isObjectLiteralExpression(literal)) {
    throw new Error("heavy.prepareArgs must return an object");
  }

  return literal;
}

function isPrepareArgsCall(expr) {
  if (!ts.isCallExpression(expr)) return false;
  if (
    ts.isPropertyAccessExpression(expr.expression) &&
    expr.expression.name.text === "prepareArgs"
  ) {
    return true;
  }
  return ts.isIdentifier(expr.expression) && expr.expression.text === "prepareArgs";
}

function normalizeArgsArg(argsArg, sf, checker) {
  if (!argsArg) return undefined;

  const resolved = resolveArgExpression(argsArg, checker);
  if (isPrepareArgsCall(resolved)) {
    return evalPreparedArgs(resolved, sf);
  }

  if (ts.isObjectLiteralExpression(resolved)) {
    return resolved;
  }

  throw new Error(
    "heavy() args must be an object literal or heavy.prepareArgs(...)"
  );
}

function buildArgsPreamble(argsArg, sf, checker) {
  const normalizedArgs = normalizeArgsArg(argsArg, sf, checker);
  const argsMap = parseArgsMap(normalizedArgs);
  if (argsMap.size === 0) return "";

  const resolvedArgs = Array.from(argsMap.entries()).map(([name, initializer]) => {
    const literal = resolveArgLiteral(initializer, checker);
    return { name, literal };
  });

  const argAssignments = resolvedArgs.map(({ name, literal }) =>
    ts.factory.createVariableStatement(
      undefined,
      ts.factory.createVariableDeclarationList(
        [
          ts.factory.createVariableDeclaration(
            ts.factory.createIdentifier(name),
            undefined,
            undefined,
            literal
          ),
        ],
        ts.NodeFlags.Const
      )
    )
  );

  const printer = ts.createPrinter();
  return argAssignments
    .map((node) => printer.printNode(ts.EmitHint.Unspecified, node, sf))
    .join("\n");
}

module.exports = {
  buildArgsPreamble,
  parseArgsMap,
  resolveArgLiteral,
};
