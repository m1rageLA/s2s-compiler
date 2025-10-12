function fib(n) {
    if (n <= 1) {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}
var input = 50;
console.log("".concat(fib(input)));
    