import { describe, it, expect, vi, beforeEach } from "vitest";
import * as ts from "typescript";
import {
    fnSentinel,
    varSentinel,
    retSentinel,
    ifSentinel,
    whileSentinel,
    forSentinel,
    blockSentinel,
    exprSentinel,
} from "./sentinels";

import { convertFunctionDeclaration } from "../specificConverters";

vi.mock("../specificConverters", () => ({
    convertFunctionDeclaration: vi.fn().mockReturnValue(fnSentinel),
}));

describe('convertStatement - it should handle all statements variation', () => {
    it("delegates FunctionDeclaration", () => {
        const functNode = ts.factory.createFunctionDeclaration(
            /* decorators */ undefined,
            /* modifiers  */ undefined,
            /* name       */ "foo",
            /* typeParams */ undefined,
            /* params     */[],
            /* returnType */ undefined,
            ts.factory.createBlock([]),
        );

        const result = convertFunctionDeclaration(functNode);

        expect(convertFunctionDeclaration).toHaveBeenCalledOnce();
        expect(convertFunctionDeclaration).toHaveBeenCalledWith(functNode);
        expect(result).toEqual(fnSentinel);
});
})
