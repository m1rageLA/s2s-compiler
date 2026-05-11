import { compileToRust } from "./compiler/dist/index.js";

const heavy = compileToRust(


`    function add(x: number, y: number): number {
        return x + y;
    }`


)

console.log(heavy);
