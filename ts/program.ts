class Point {
    x: number;
    y: number;

    constructor(x: number, y: number) {
        this.x = x;
        this.y = y;
    }

    move(dx: number) {
        this.x += dx;
    }

    toString(): string {
        return `(${this.x}, ${this.y})`;
    }
}

// 🔹 создаём объект
let p = new Point(10, 5);
console.log("Начальная точка:", p.toString());

// 🔹 двигаем на +3
p.move(3);
console.log("После move(3):", p.toString());
