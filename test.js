"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var index_js_1 = require("./index.js");
var heavy = index_js_1.default.heavy;
var x = 5;
var y = 5;
var z = 40;
var result = await heavy(function () {
    function fibHeavy(calcX) {
        if (calcX <= 1)
            return calcX;
        return fibHeavy(calcX - 1) + fibHeavy(calcX - 2);
    }
    function calcX(x, y, z) {
        return x + y + z;
    }
    return fibHeavy(calcX(x, y, z));
}, { x: x, y: y, z: z });
console.log("output:", result.output);
