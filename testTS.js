"use strict";
function go() {
    let s = "";
    for (let i = 0; i < 900000000; i++) {
        s += "a";
    }
    return s.length;
}
console.log("Go result:", go());
