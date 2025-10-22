var h = 1;
function f() {
    console.log(h);
    console.log(f());
}
console.log(h && f());
