import * as ts from "typescript";

type IRNode =
    |
    {
        kind: "Program";
        body: IRNode[];
    }
    |
    {
        kind: "Function";
        name: string;
        params: string[];
        body: IRNode[];
    }
    |
    {
        kind: "VariableDeclaration";
        name: string;
        value?: IRNode; //there are variable declaration without initialization
    }
    |
    {
        kind: "Return";
        value: IRNode;
    }
    |
    {
        kind: "Literal";
        value: string | number | boolean | null;
    }
    |
    {
        kind: "Identifier";
        name: string;
    }
    |
    {
        kind: "Binary";
        operator: string; // + - * / % ** == === != !== < > <= >=
        left: IRNode;
        right: IRNode;
    }
    |
    {
        kind: "If";
        condition: IRNode;
        theBlock: IRNode[];
        elseBlock?: IRNode[];
    }
    |
    {
        kind: "While";
        condition: IRNode;
        body: IRNode[];
    }
    |
    {
        kind: "For";
        init?: IRNode;
        condition?: IRNode;
        increment?: IRNode;
        body: IRNode;
    }
    |
    {
        kind: "Block";
        statements: IRNode[];
    }
    |
    {
        kind: "ExpressionStatement";
        expression: IRNode;
    }
    |
    {
        kind: "Call";
        calle: IRNode;
        args: IRNode[];
    };