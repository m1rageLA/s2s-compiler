function demo(x) {
  let sum = 0;
  const data = [1, { val: 2 }, 3];
  {
    sum = x + data[0];
  }
  if (sum > 1) {
    sum = sum * data[1].val;
  } else {
    sum = sum - data[2];
  }
  for (let i = 0; i < data.length; i = i + 1) {
    if (data[i] % 2 === 0) {
      continue;
    }
    sum += data[i];
  }
  while (sum < 10) {
    sum = sum + 1;
    if (sum === 5) break;
  }
  switch (sum) {
    case 5:
      sum = 'five';
      break;
    case 6:
      sum = 'six';
      break;
    default:
      sum = 'other';
  }
  const date = new Date();
  console.log(date.getFullYear(), sum);
  return sum;
}

const objLit = { a: 1, b: "test", nested: { c: true } };
const arrLit = [objLit, demo];
demo(objLit.nested.c);
