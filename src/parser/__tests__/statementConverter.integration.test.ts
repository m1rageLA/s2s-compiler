import { describe, it, expect } from 'vitest';
import ts from 'typescript';
import { convertStatement } from '../statementConverters';
import { convertExpression } from '../expressionConverter';
import type { IRNode } from '../coreSchema';

describe('convertStatement – integration (real IR)', () => {
    // ------------------------------------------------------------------
    // 3. SwitchStatement — полноценная проверка структуры IR
    // ------------------------------------------------------------------
    it('ExpressionStatement', () => {
        const expr = ts.factory.createNumericLiteral(42);
        const node = ts.factory.createExpressionStatement(expr);

        expect(convertStatement(node)).toEqual<IRNode>({
            kind: 'ExpressionStatement',
            expression: convertExpression(expr),
        });
    });

    it('SwitchStatement', () => {
        const sw = ts.factory.createSwitchStatement(
            ts.factory.createIdentifier('x'),
            ts.factory.createCaseBlock([
                ts.factory.createCaseClause(ts.factory.createNumericLiteral(1), [
                    ts.factory.createBreakStatement()
                ]),
                ts.factory.createDefaultClause([
                    ts.factory.createBreakStatement()
                ])
            ])
        );

        const ir = convertStatement(sw);

        expect(ir).toEqual<IRNode>({
            kind: 'Switch',
            expression: convertExpression(ts.factory.createIdentifier('x')),
            cases: [
                {
                    test: convertExpression(ts.factory.createNumericLiteral(1)),
                    consequent: [{ kind: 'BreakStatement' }]
                },
                { test: 'default', consequent: [{ kind: 'BreakStatement' }] }
            ]
        });
    });


});
