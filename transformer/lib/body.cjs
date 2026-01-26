const ts = require("typescript");

function buildBodyStatements(fnArg) {
  if (ts.isBlock(fnArg.body)) {
    return Array.from(fnArg.body.statements);
  }

  if (ts.isArrowFunction(fnArg)) {
    return [
      ts.factory.createReturnStatement(
        ts.isBlock(fnArg.body) ? undefined : fnArg.body
      ),
    ];
  }

  return [ts.factory.createExpressionStatement(fnArg.body)];
}

module.exports = { buildBodyStatements };
