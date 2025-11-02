// string_methods_test.ts

function testStringMethods() {
  const str = "Hello World";
  console.log("=== TEST STRING METHODS ===");

  // length
  console.log("length:", str.length === 11 ? "OK" : `FAIL (${str.length})`);

  // toUpperCase
  const upper = str.toUpperCase();
  console.log("toUpperCase:", upper === "HELLO WORLD" ? "OK" : `FAIL (${upper})`);

  // toLowerCase
  const lower = str.toLowerCase();
  console.log("toLowerCase:", lower === "hello world" ? "OK" : `FAIL (${lower})`);

  // split
  const parts = str.split(" ");
  console.log("split:", parts.length === 2 && parts[0] === "Hello" && parts[1] === "World" ? "OK" : `FAIL (${parts})`);

  // replace
  const replaced = str.replace("World", "TS");
  console.log("replace:", replaced === "Hello TS" ? "OK" : `FAIL (${replaced})`);

  // includes
  console.log("includes:", str.includes("Hello") && !str.includes("Bye") ? "OK" : "FAIL");

  // concat
  const concatenated = "Hello".concat(" ", "TypeScript");
  console.log("concat:", concatenated === "Hello TypeScript" ? "OK" : `FAIL (${concatenated})`);

  // slice
  const sliced = str.slice(0, 5);
  console.log("slice:", sliced === "Hello" ? "OK" : `FAIL (${sliced})`);

  // substr (устаревший, но проверим)
  const substrValue = str.substr(6, 5);
  console.log("substr:", substrValue === "World" ? "OK" : `FAIL (${substrValue})`);
}

// Тест базовых конструкций языка
function testBaseConstructs() {
  console.log("\n=== TEST BASE CONSTRUCTS ===");

  // if / else
  const a = 10;
  const b = 5;
  let result = "";
  if (a > b) result = "greater";
  else result = "less";
  console.log("if/else:", result === "greater" ? "OK" : "FAIL");

  // for loop
  let sum = 0;
  for (let i = 0; i < 5; i++) {
    sum += i;
  }
  console.log("for:", sum === 10 ? "OK" : `FAIL (${sum})`);

  // while loop
  let count = 0;
  let j = 0;
  while (j < 3) {
    count += j;
    j++;
  }
  console.log("while:", count === 3 ? "OK" : `FAIL (${count})`);



  // function
  function multiply(x: number, y: number): number {
    return x * y;
  }
  const mul = multiply(3, 4);
  console.log("function:", mul === 12 ? "OK" : `FAIL (${mul})`);

  // array + map
  const arr = [1, 2, 3];
  const doubled = arr.map(x => x * 2);
  console.log("map:", doubled.join(",") === "2,4,6" ? "OK" : `FAIL (${doubled})`);

  // object
  const obj = { name: "Alice", age: 25 };
  console.log("object:", obj.name === "Alice" && obj.age === 25 ? "OK" : "FAIL");
}

// Запуск тестов
testStringMethods();
testBaseConstructs();

console.log("\n=== TEST COMPLETE ===");
