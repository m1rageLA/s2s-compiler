import { transpileFileToIR } from "./parser/ts2core";


const ast = transpileFileToIR('src/samples/fibonacci.ts');
console.dir(ast, { depth: null });