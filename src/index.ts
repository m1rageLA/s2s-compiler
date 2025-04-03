import parseCodeToAST from "./parser/parser";


const ast = parseCodeToAST('src/samples/fibonacci.ts');
console.dir(ast, { depth: null });