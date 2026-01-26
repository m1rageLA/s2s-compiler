const ts = require("typescript");

function buildBodyText(fnArg, sf) {
    if (
        (ts.isArrowFunction(fnArg) || ts.isFunctionExpression(fnArg)) &&
        ts.isBlock(fnArg.body)
    ) {
        return fnArg.body.statements.map((s) => s.getText(sf)).join("\n");
    }

    if (ts.isArrowFunction(fnArg) || ts.isFunctionExpression(fnArg)) {
        return fnArg.body.getText(sf);
    }

    return fnArg.getText(sf);
}

module.exports = { buildBodyText };
