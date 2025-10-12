const double = (x: number) => x * 2;

const add = (a: number, b: number) => {
    const sum = a + b;
    const msg = sum + sum;
    console.log(msg);
    return double(sum);
};

console.log(add(2, 3));
