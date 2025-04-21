import { transpileFileToIR } from "./parser/ts2core";


const ast = transpileFileToIR('src/samples/example.ts');
console.dir(ast, { depth: null });