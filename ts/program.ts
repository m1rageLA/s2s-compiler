const a = 5;
const b = 10;
const c = 3;
const x = a > b ? (b > c ? b : c) : (a + b < 20 ? a + b : b - c);
console.log(x);
