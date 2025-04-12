import { transpileFileToIR } from "../parser/ts2core";
import * as fs from "fs";

const filePath = "./example.ts";

try {
    const ir = transpileFileToIR(filePath);
    console.log(JSON.stringify(ir, null, 2));
    fs.writeFileSync("output.ir.json", JSON.stringify(ir, null, 2));
} catch (error) {
    console.error("Error reading file:", error);
}