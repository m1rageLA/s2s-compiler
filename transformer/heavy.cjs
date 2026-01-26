const ts = require("typescript");
const { buildSnippet } = require("./lib/snippet.cjs");

function heavyTransformer(program) {
  const checker = program.getTypeChecker();
  const printer = ts.createPrinter();

  return (ctx) => {
    const visit = (node) => {
      if (
        ts.isCallExpression(node) &&
        ts.isIdentifier(node.expression) &&
        node.expression.text === "heavy" &&
        (node.arguments.length === 1 || node.arguments.length === 2)
      ) {
        const fnArg = node.arguments[0];
        const argsArg = node.arguments[1];
        if (ts.isFunctionExpression(fnArg) || ts.isArrowFunction(fnArg)) {
          const sf = fnArg.getSourceFile();
          const text = buildSnippet(fnArg, argsArg, sf, checker, printer);

          return ts.factory.updateCallExpression(
            node,
            node.expression,
            node.typeArguments,
            [ts.factory.createNoSubstitutionTemplateLiteral(text)]
          );
        }
      }
      return ts.visitEachChild(node, visit, ctx);
    };

    return (sf) => ts.visitNode(sf, visit);
  };
}

module.exports = heavyTransformer;
