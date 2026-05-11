use crate::test_utils::{get_last_line, run_ts_program};

#[test]
fn light_2bl_loop_sum() {
    let stdout = run_ts_program(
        r#"
function go(): number {
    let sum = 0;
    let i = 0;
    let m = 0;
    while (i < 20000) {
        sum += m * 3;
        m = (m + 1) % 10;
        i++;
    }
    console.log(sum);
    return sum;
}
console.log(go());
"#,
    );

    let last = get_last_line(&stdout);
    assert_eq!(last, "270000");
}

#[test]
fn light_nbody_energy_is_finite() {
    let stdout = run_ts_program(
        r#"
function go(): number {
    const N = 10;
    const STEPS = 5;
    const STRIDE = 7;
    const DT: number = 0.01;

    let seed = 1;
    function nextSeed(s: number): number {
        return (s * 1664525 + 1013904223) >>> 0;
    }
    function seedToUnit(s: number): number {
        return s / 4294967296;
    }

    const bodies: number[] = [];
    for (let i = 0; i < N; i++) {
        seed = nextSeed(seed);
        bodies.push(seedToUnit(seed) * 100 - 50);
        seed = nextSeed(seed);
        bodies.push(seedToUnit(seed) * 100 - 50);
        seed = nextSeed(seed);
        bodies.push(seedToUnit(seed) * 100 - 50);
        seed = nextSeed(seed);
        bodies.push(seedToUnit(seed) - 0.5);
        seed = nextSeed(seed);
        bodies.push(seedToUnit(seed) - 0.5);
        seed = nextSeed(seed);
        bodies.push(seedToUnit(seed) - 0.5);
        seed = nextSeed(seed);
        bodies.push(seedToUnit(seed) + 0.1);
    }

    function step(b: number[]): void {
        for (let i = 0, bi = 0; i < N; i++, bi += STRIDE) {
            const xi = b[bi], yi = b[bi + 1], zi = b[bi + 2];
            let vix = b[bi + 3], viy = b[bi + 4], viz = b[bi + 5];
            const mi = b[bi + 6];

            for (let j = i + 1, bj = bi + STRIDE; j < N; j++, bj += STRIDE) {
                const dx = xi - b[bj];
                const dy = yi - b[bj + 1];
                const dz = zi - b[bj + 2];

                const d2 = dx * dx + dy * dy + dz * dz + 1e-9;
                const inv = 1 / Math.sqrt(d2);
                const f = DT * inv * inv * inv;

                const mj = b[bj + 6];
                const fi = mj * f;
                const fj = mi * f;

                vix -= dx * fi;
                viy -= dy * fi;
                viz -= dz * fi;

                b[bj + 3] += dx * fj;
                b[bj + 4] += dy * fj;
                b[bj + 5] += dz * fj;
            }

            b[bi + 3] = vix;
            b[bi + 4] = viy;
            b[bi + 5] = viz;
        }

        for (let i = 0, bi = 0; i < N; i++, bi += STRIDE) {
            b[bi] += b[bi + 3] * DT;
            b[bi + 1] += b[bi + 4] * DT;
            b[bi + 2] += b[bi + 5] * DT;
        }
    }

    function energy(b: number[]): number {
        let e: number = 0.0;
        for (let i = 0; i < N; i++) {
            const bi = i * STRIDE;
            const vx: number = b[bi + 3], vy: number = b[bi + 4], vz: number = b[bi + 5];
            e += 0.5 * b[bi + 6] * (vx * vx + vy * vy + vz * vz);

            for (let j = (i + 1); j < N; j++) {
                const bj = (j * STRIDE);
                const dx = b[bi] - b[bj];
                const dy = b[bi + 1] - b[bj + 1];
                const dz = b[bi + 2] - b[bj + 2];
                const EPS = 1e-9;
                e -= (b[bi + 6] * b[bj + 6]) / Math.sqrt(dx * dx + dy * dy + dz * dz + EPS);
            }
        }
        return e;
    }

    for (let s = 0; s < STEPS; s++) { step(bodies) };
    const result = energy(bodies);
    console.log(result);
    return result;
}
console.log(go());
"#,
    );

    let last = get_last_line(&stdout);
    let value: f64 = last.parse().expect("energy should be a number");
    assert!(value.is_finite(), "expected finite energy, got {}", last);
}

#[test]
fn light_object_properties() {
    let stdout = run_ts_program(
        r#"
type Obj = {
    a: number; b: number; c: number; d: number;
    e: number; f: number; g: number; h: number;
};

function makeObj(): Obj {
    return { a: 1, b: 2, c: 3, d: 4, e: 5, f: 6, g: 7, h: 8 };
}

function benchDirect(o: Obj, iters: number): number {
    let acc: number = 0;
    for (let i = 0; i < iters; i++) {
        acc += o.a;
        acc += o.b;
        acc += o.c;
        acc += o.d;
        acc += o.e;
        acc += o.f;
        acc += o.g;
        acc += o.h;
    }
    return acc;
}

function benchBracket(o: Obj, iters: number): number {
    let acc: number = 0;
    for (let i = 0; i < iters; i++) {
        acc += o.a;
        acc += o.b;
        acc += o.c;
        acc += o.d;
        acc += o.e;
        acc += o.f;
        acc += o.g;
        acc += o.h;
    }
    return acc;
}

function benchDestructure(o: Obj, iters: number): number {
    let acc: number = 0;
    for (let i = 0; i < iters; i++) {
        acc += o.a;
        acc += o.b;
        acc += o.c;
        acc += o.d;
        acc += o.e;
        acc += o.f;
        acc += o.g;
        acc += o.h;
    }
    return acc;
}

const ITERS = 1000;
const obj = makeObj();
const direct = benchDirect(obj, ITERS);
const bracket = benchBracket(obj, ITERS);
const destructure = benchDestructure(obj, ITERS);
console.log(`${direct},${bracket},${destructure}`);
"#,
    );

    let last = get_last_line(&stdout);
    assert_eq!(last, "36000,36000,36000");
}

#[test]
fn light_spectral_norm() {
    let stdout = run_ts_program(
        r#"
function A(i: number, j: number): number {
    return 1.0 / ((i + j) * (i + j + 1) / 2 + i + 1);
}

function multiplyAv(n: number, v: number[], out: number[]) {
    for (let i = 0; i < n; i++) {
        let sum: number = 0.0;
        for (let j = 0; j < n; j++) {
            sum += A(i, j) * v[j];
        }
        out[i] = sum;
    }
}

function multiplyAtv(n: number, v: number[], out: number[]) {
    for (let i = 0; i < n; i++) {
        let sum: number = 0.0;
        for (let j = 0; j < n; j++) {
            sum += A(j, i) * v[j];
        }
        out[i] = sum;
    }
}

function multiplyAtAv(n: number, v: number[], out: number[], tmp: number[]) {
    multiplyAv(n, v, tmp);
    multiplyAtv(n, tmp, out);
}

function spectralNorm(n: number): number {
    let u: number[] = [];
    let v: number[] = [];
    let tmp: number[] = [];

    for (let i = 0; i < n; i++) {
        u[i] = 1.0;
        v[i] = 0.0;
        tmp[i] = 0.0;
    }

    for (let i = 0; i < 10; i++) {
        multiplyAtAv(n, u, v, tmp);
        multiplyAtAv(n, v, u, tmp);
    }

    let vBv: number = 0.0;
    let vv: number = 0.0;

    for (let i = 0; i < n; i++) {
        vBv += u[i] * v[i];
        vv += v[i] * v[i];
    }

    return Math.sqrt(vBv / vv);
}

const result = spectralNorm(100);
console.log(result);
"#,
    );

    let last = get_last_line(&stdout);
    let value: f64 = last.parse().expect("spectral norm should be a number");
    assert!(value.is_finite(), "expected finite result, got {}", last);
    assert!(
        value > 1.1 && value < 1.5,
        "unexpected spectral norm: {}",
        last
    );
}

#[test]
fn light_string_concat_length() {
    let stdout = run_ts_program(
        r#"
function go(): number {
    let s = "";
    for (let i = 0; i < 2000; i++) {
        s += "a";
    }
    console.log(s.length);
    return s.length;
}
console.log(go());
"#,
    );

    let last = get_last_line(&stdout);
    assert_eq!(last, "2000");
}
