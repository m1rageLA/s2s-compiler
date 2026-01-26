// eslint-plugin-ts2rust/rules/no-dynamic-in-heavy.mjs

function getHeavyName(context) {
    return context.options?.[0]?.heavyName || "heavy";
}

export default {
    meta: {
        type: "problem",
        schema: [
            {
                type: "object",
                properties: {
                    heavyName: { type: "string" },
                },
                additionalProperties: false,
            },
        ],
        messages: {
            forbidden: "{{name}} is not supported inside {{heavy}}() (ts2rust subset)",
        },
    },

    create(context) {
        const heavyName = getHeavyName(context);
        const sourceCode = context.getSourceCode();

        function isInsideHeavyCallback(node) {
            const ancestors = sourceCode.getAncestors(node);

            for (let i = ancestors.length - 1; i >= 0; i--) {
                const fn = ancestors[i];

                if (
                    fn.type === "FunctionExpression" ||
                    fn.type === "ArrowFunctionExpression"
                ) {
                    const parent = ancestors[i - 1];
                    if (
                        parent &&
                        parent.type === "CallExpression" &&
                        parent.callee.type === "Identifier" &&
                        parent.callee.name === heavyName &&
                        parent.arguments[0] === fn
                    ) {
                        return true;
                    }
                }
            }
            return false;
        }

        function report(node, name) {
            context.report({
                node,
                messageId: "forbidden",
                data: { name, heavy: heavyName },
            });
        }

        function guard(node, name) {
            if (isInsideHeavyCallback(node)) report(node, name);
        }

        return {
            // ===== statements =====
            TryStatement(node) {
                guard(node, "try/catch/finally");
            },
            ThrowStatement(node) {
                guard(node, "throw");
            },
            WithStatement(node) {
                guard(node, "with");
            },
            // ForInStatement(node) {
            //     guard(node, "for-in");
            // },
            ForOfStatement(node) {
                guard(node, "for-of");
            },

            // ===== expressions =====
            NewExpression(node) {
                guard(node, "new");
            },
            ThisExpression(node) {
                guard(node, "this");
            },
            Super(node) {
                guard(node, "super");
            },
            ChainExpression(node) {
                guard(node, "optional chaining");
            },
            SpreadElement(node) {
                guard(node, "spread");
            },

            UnaryExpression(node) {
                if (!isInsideHeavyCallback(node)) return;
                // if (node.operator === "typeof") report(node, "typeof");
                // if (node.operator === "delete") report(node, "delete");
                // if (node.operator === "void") report(node, "void operator");
            },

            Literal(node) {
                if (!isInsideHeavyCallback(node)) return;
                if (node.regex) report(node, "RegExp literal");
            },

            ArrayExpression(node) {
                if (!isInsideHeavyCallback(node)) return;
                if (node.elements?.some((el) => el === null)) {
                    report(node, "array holes ([,,])");
                }
            },

            //async
            FunctionDeclaration(node) {
                if (node.async) guard(node, "async function");
            },
            FunctionExpression(node) {
                if (node.async) guard(node, "async function");
            },
            ArrowFunctionExpression(node) {
                if (node.async) guard(node, "async arrow function");
            },
            NewExpression(node) {
                if (
                    node.callee.type === "Identifier" &&
                    node.callee.name === "Promise"
                ) {
                    guard(node, "Promise");
                }
            },
            CallExpression(node) {
                if (!isInsideHeavyCallback(node)) return;

                if (
                    node.callee.type === "MemberExpression" &&
                    node.callee.property.type === "Identifier" &&
                    ["then", "catch", "finally"].includes(node.callee.property.name)
                ) {
                    report(node, "Promise.then/catch/finally");
                }
            },
            CallExpression(node) {
                if (!isInsideHeavyCallback(node)) return;

                if (
                    node.callee.type === "Identifier" &&
                    ["setTimeout", "setInterval", "queueMicrotask"].includes(node.callee.name)
                ) {
                    report(node, node.callee.name);
                }
            },

        };
    },
};
