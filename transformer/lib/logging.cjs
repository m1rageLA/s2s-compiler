const ts = require("typescript");

function buildLogStatement() {
  return ts.factory.createExpressionStatement(
    ts.factory.createCallExpression(
      ts.factory.createPropertyAccessExpression(
        ts.factory.createIdentifier("console"),
        ts.factory.createIdentifier("log")
      ),
      undefined,
      [
        ts.factory.createStringLiteral("__HEAVY_OUTPUT__:"),
        // Log the raw value so non-arrays (string/number/object) don't get
        // coerced into repeated `undefined` when treated like an array.
        ts.factory.createIdentifier("__heavyOutput"),
      ]
    )
  );
}

module.exports = { buildLogStatement };
