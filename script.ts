// The following script is a simple test for the following conjecture:

// Let S: N -> N be the sum of the digits of a positive integer.
// For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an interger.

const readline = require('readline');

function digits(n: number) {
    return Array.from(Math.round(Math.abs(n)).toString()).map(c => parseInt(c));
}

function sum(n: number) {
    return digits(n).reduce((a, b) => a + b);
}

function test(a: number, b: number) {
    return (sum(a + b) - (sum(a) + sum(b))) % 9 == 0;
}

function ask(question: string): Promise<string> {
    const rl = readline.createInterface({
        input: process.stdin,
        output: process.stdout,
    });

    return new Promise(resolve => rl.question(question, (ans: string) => {
        rl.close();
        resolve(ans);
    }));
}

function getCounterExp(max: number) {
    const counterexmpls: [number, number][] = [];

    for (let a = 0; a <= max; a++)
        for (let b = a; b <= max; b++)
            if (!test(a, b)) counterexmpls.push([a, b]);

    return counterexmpls;
}

console.log("\nThis script is a simple test for the following conjecture:\n");
console.log("Let S: N -> N be the sum of the digits of a positive integer.");
console.log("For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an interger.\n");

ask("What value would you like to test the conjecture for? ").then(ans => {
    if (!isNaN(parseInt(ans))) {
        console.log(`\nLOADING. . .`);

        const max = parseInt(ans)
            , start = new Date
            , counterexmpls = getCounterExp(max)
            , elepsed = Math.round((new Date().getTime() - start.getTime()) / 100) / 10;

        console.log(`LOADED. . . in ${elepsed}s\n`);

        if (counterexmpls.length == 0)
            console.log(`The conjecture is proved for all natural numbers smaller or equals to ${max}!`);
        else {
            console.log("The conjecture is disproved! Here are the counter examples:");
            console.log(counterexmpls.map(pair => `(${pair[0]}, ${pair[1]})`).join(", "));
        }
    } else console.log(`'${ans}' is not a natural number!`);
}).catch(console.error);