function headline(title: string): void {
  console.log(`--- ${title} ---`);
}

function mulByTwo(n: number): number {
  return n * 2;
}

const increment = (value: number): number => value + 1;

function sum(values: number[]): number {
  let total: number = 0;
  for (let i = 0; i < values.length; i = i + 1) {
    total = total + values[i];
  }
  return total;
}

function factorial(n: number): number {
  if (n <= 1) {
    return 1;
  }
  return n * factorial(n - 1);
}

headline("pipeline");
const input: number[] = [1, 2, 3, 4, 5];
const doubled: number[] = input.map((n) => mulByTwo(n));
const filtered: number[] = doubled.filter((n) => n > 4);
const total: number = sum(filtered);
console.log(`input=${input.length} filtered=${filtered.length} total=${total}`);

headline("array mutation");
let queue: number[] = [];
queue.push(10);
queue.push(20);
queue.push(sum(filtered));
console.log(`queue snapshot=${queue[0]},${queue[1]},${queue[2]}`);

headline("control flow");
let countdown: number = 3;
while (countdown > 0) {
  console.log(`while:${countdown}`);
  countdown = countdown - 1;
}

let seen: number = 0;
do {
  console.log(`do:${seen}`);
  seen = seen + 1;
} while (seen < 2);

headline("recursion");
console.log(`factorial(5)=${factorial(5)}`);

headline("higher-order");
const pipeline = (value: number): number => increment(mulByTwo(value));
console.log(`pipeline(7)=${pipeline(7)}`);

headline("branching");
const probe: number = total > 15 ? 1 : 0;
const branchMessage: string = probe === 1 ? "probe:high" : "probe:low";
console.log(branchMessage);
