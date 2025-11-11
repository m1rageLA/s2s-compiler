const greet = (name: string): string => {
  const message = `Hello, ${name}!`;
  return message.toUpperCase();
};

console.log(greet("Rust"));
