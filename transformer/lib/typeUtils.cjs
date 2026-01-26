const ts = require("typescript");

function cloneTypeNode(typeNode) {
  if (!typeNode) return undefined;
  if (ts.factory && ts.factory.cloneNode) {
    return ts.factory.cloneNode(typeNode);
  }
  // Fallback for older TS versions: reuse the existing node.
  return typeNode;
}

function extractReturnType(fnArg) {
  return ts.isFunctionExpression(fnArg) || ts.isArrowFunction(fnArg)
    ? cloneTypeNode(fnArg.type)
    : undefined;
}

module.exports = { cloneTypeNode, extractReturnType };
