const ts = require("typescript");

function rewriteReturns(statements) {
  const transformed = [];

  for (const stmt of statements) {
    if (ts.isReturnStatement(stmt)) {
      transformed.push(
        ts.factory.createExpressionStatement(
          ts.factory.createBinaryExpression(
            ts.factory.createIdentifier("__heavyOutput"),
            ts.SyntaxKind.EqualsToken,
            stmt.expression ?? ts.factory.createIdentifier("undefined")
          )
        )
      );
      break;
    }

    transformed.push(stmt);
  }

  return transformed;
}

function buildOutputDeclaration(outputType) {
  return ts.factory.createVariableStatement(
    undefined,
    ts.factory.createVariableDeclarationList(
      [
        ts.factory.createVariableDeclaration(
          ts.factory.createIdentifier("__heavyOutput"),
          undefined,
          outputType,
          undefined
        ),
      ],
      ts.NodeFlags.Let
    )
  );
}

module.exports = { rewriteReturns, buildOutputDeclaration };
