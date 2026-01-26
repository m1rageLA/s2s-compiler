import { heavy } from "./index.js";

const result = await heavy(() => {
    function go(): number {
        let s = "";
        for (let i = 0; i < 900000; i++) {
            s += "a";
        }
        return s.length;
    }
    return go();
});

console.log("Additional output:", result.rust);



