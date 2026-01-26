const ts = require("typescript");
const { buildArgsPreamble } = require("../argsResolver.cjs");
const { buildBodyStatements } = require("./body.cjs");
const { rewriteReturns, buildOutputDeclaration } = require("./returnCapture.cjs");
const { extractReturnType } = require("./typeUtils.cjs");
const { buildLogStatement } = require("./logging.cjs");

function buildSnippet(fnArg, argsArg, sf, checker, printer) {
  const preamble = buildArgsPreamble(argsArg, sf, checker);
  const bodyStatements = buildBodyStatements(fnArg);
  const outputType = extractReturnType(fnArg);
  const transformed = rewriteReturns(bodyStatements);
  const outputDecl = buildOutputDeclaration(outputType);
  const logStmt = buildLogStatement();

  const fullStatements = [outputDecl, ...transformed, logStmt];
  const bodyText = printer.printList(
    ts.ListFormat.MultiLine,
    ts.factory.createNodeArray(fullStatements),
    sf
  );

  return preamble ? `${preamble}\n${bodyText}` : bodyText;
}

module.exports = { buildSnippet };
