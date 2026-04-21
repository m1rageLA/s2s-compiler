
        const double = (value: number): number => value * 2;
        const increment = (value: number): number => value + 1;

        const start = 5;
        const afterDouble = double(start);
        const afterIncrement = increment(afterDouble);

        const pipeline = (value: number): number => {
            const first = increment(value);
            return double(first);
        };

        const pipelineResult = pipeline(3);

        console.log(`double=${afterDouble}`);
        console.log(`increment=${afterIncrement}`);
        console.log(`pipeline=${pipelineResult}`);
    