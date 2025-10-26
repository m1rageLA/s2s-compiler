function add(a: any): string | number {
    if (a === "string") {
        return "123" + "456";
    }
    return 123 + 456;
}

console.log(add("string"));