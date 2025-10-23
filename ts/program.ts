function outer() {
    return function inner() {
        return function therd() {
            return "hello";
        }
    };
}

console.log(outer());
