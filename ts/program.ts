// Простейшая линейная регрессия: y = a * x + b
// Учимся подгонять a и b по данным с помощью градиентного спуска

function trainLinearModel(xs: number[], ys: number[], lr: number, steps: number) {
  let a = 0
  let b = 0

  for (let step = 0; step < steps; step++) {
    let da = 0
    let db = 0
    let n = xs.length

    for (let i = 0; i < n; i++) {
      const x = xs[i]
      const y = ys[i]
      const y_pred = a * x + b
      const error = y_pred - y

      da += (2 / n) * error * x
      db += (2 / n) * error
    }

    a = a - lr * da
    b = b - lr * db

    if (step % 100 === 0) {
      console.log("step", step, "loss", da * da + db * db)
    }
  }

  return { a, b }
}

// Демонстрация
const xs = [1, 2, 3, 4, 5]
const ys = [3, 5, 7, 9, 11] // фактически y = 2x + 1
const result = trainLinearModel(xs, ys, 0.01, 1000)
console.log("Результат:", result)
