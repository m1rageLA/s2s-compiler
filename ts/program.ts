// Самая простая версия n-body на обычных JS массивах
// Очень тяжёлая для CPU, но максимально понятная.
function nbody() {
    function sqrt(x: number): number {
        if (x <= 0) return 0;

        let guess = x; // начальное приближение
        for (let i = 0; i < 10; i++) {
            guess = 0.5 * (guess + x / guess);
        }
        return guess;
    }


    const N = 1500;         // количество тел (можешь менять)
    const STEPS = 3000;     // количество шагов
    const G = 1.0;
    const DT = 0.01;
    const EPS = 1e-9;

    //
    // Храним каждое тело как обычный JS-объект
    //
    const bodies: any[] = [];

    for (let i = 0; i < N; i++) {
        bodies.push({
            x: (Math.random() - 0.5) * 2,
            y: (Math.random() - 0.5) * 2,
            z: (Math.random() - 0.5) * 2,

            vx: (Math.random() - 0.5) * 0.1,
            vy: (Math.random() - 0.5) * 0.1,
            vz: (Math.random() - 0.5) * 0.1,

            m: Math.random() * 2 + 0.1
        });
    }

    //
    // Один шаг O(N²) — самый тяжёлый кусок
    //
    function step() {
        for (let i = 0; i < N; i++) {
            const bi = bodies[i];

            for (let j = i + 1; j < N; j++) {
                const bj = bodies[j];

                const dx = bj.x - bi.x;
                const dy = bj.y - bi.y;
                const dz = bj.z - bi.z;

                const distSq = dx * dx + dy * dy + dz * dz + EPS;
                const dist = sqrt(distSq);
                const invDist3 = 1.0 / (distSq * dist);

                const force = G * invDist3;

                const fx = dx * force;
                const fy = dy * force;
                const fz = dz * force;

                // dv = F / m * dt
                bi.vx += fx * bj.m * DT;
                bi.vy += fy * bj.m * DT;
                bi.vz += fz * bj.m * DT;

                bj.vx -= fx * bi.m * DT;
                bj.vy -= fy * bi.m * DT;
                bj.vz -= fz * bi.m * DT;
            }
        }

        for (let i = 0; i < N; i++) {
            const b = bodies[i];
            b.x += b.vx * DT;
            b.y += b.vy * DT;
            b.z += b.vz * DT;
        }
    }

    console.log(`Start: N=${N}, STEPS=${STEPS}`);

    for (let i = 0; i < STEPS; i++) step();


    let checksum = 0;
    for (let i = 0; i < bodies.length; i++) {
        const b = bodies[i];
        checksum += b.x + b.y + b.z;
    }


    console.log("Checksum:", checksum);
}

nbody();
